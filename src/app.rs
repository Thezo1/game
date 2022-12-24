use x11rb::{
    connection::Connection, 
    rust_connection::RustConnection, 
    protocol::xproto::*, 
    wrapper::ConnectionExt as _,
    protocol::Event,
};
use std::{
    error::Error, 
    fs::{OpenOptions, remove_file}, 
    io::ErrorKind,
    os::unix::fs::OpenOptionsExt,
};
use x11rb::COPY_DEPTH_FROM_PARENT;
use crate::logger::*;
use crate::{error, info, warn, log_output};

#[derive(Debug)]
pub enum ErrorApp
{
    AppCreateError
}

pub struct App<'a>{
    pub app_config: AppConfig<'a>,
    app_state: AppState
}

pub struct AppConfig<'a>{
    pub app_name: &'a str,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

struct AppState{
    is_running: bool,
    plat_state: PlatformState,
    is_suspeded: bool,
    _width: u16,
    _last_time: f64,
    _height: u16,
}

struct PlatformState{
    connection: RustConnection,
    window: u32,
    screen: Screen,
    wm_protocol: u32,
    wm_delete_win: u32,
}

impl PlatformState
{
    pub fn new() -> Self
    {
        let (conn, screen_num) = x11rb::connect(None).expect("Failed to connect to X server");
        let screen = &conn.setup().roots[screen_num];
        let win_id = conn.generate_id().expect("failed to get window id");

        return Self{
            window: win_id,
            screen: screen.clone(),
            connection: conn,
            wm_protocol: 0,
            wm_delete_win: 0,
        }
    }

    fn pump_message(&self) -> bool
    {
        let mut quit = false;
        loop
        {
            let event = self.connection.wait_for_event().unwrap();
            match event
                {
                    Event::KeyPress(_) => {},
                    Event::KeyRelease(_) => (),
                    Event::ButtonPress(_) => (),
                    Event::ButtonRelease(_) => (),

                    Event::MotionNotify(_) => (),
                    Event::ConfigureNotify(_) => (),
                    Event::ClientMessage(event) => {
                        if event.data.as_data32()[0] == self.wm_delete_win 
                        {
                            info!("Window was asked to close");
                            quit = true;
                            return !quit;
                        }
                    },
                    _ => (),
                }
            return !quit;
        }
    }
    fn shutdown(&self)
    {
        remove_file("/tmp/myapp.lock").expect("Error deleteing locked file");
        self.connection.destroy_window(self.window).expect("Error destroying window");    
    }
}

impl AppState
{
    fn new() -> Self
    {
        let plat_state = PlatformState::new();
        Self{
            is_running: false,
            plat_state,
            is_suspeded: false,
            _width: 100,
            _height: 100,
            _last_time: 0.0,
        }
    }
}

impl<'a> AppConfig<'a>
{
    pub fn new(app_name: &'a str, x: i16, y: i16, width: u16, height: u16) -> Self
    {
        Self{
            app_name,
            x,
            y,
            width,
            height,
        }
    }
}

impl<'a> Default for AppConfig<'a>
{
    fn default() -> Self
    {
        Self::new("test", 0, 0, 100, 100)
    }
}

impl<'a> App<'a>
{
    pub fn new(app_config: AppConfig<'a>) -> Result<App<'a>, ErrorApp>
    {
        if !is_only_instance()
        {
            return Err(ErrorApp::AppCreateError);
        }

        let mut app_state = AppState::new();
        let plat_state = &mut app_state.plat_state;

        create_window(plat_state, &app_config).expect("Failed to create window");
        info!("App Created");

        app_state.is_running = true;
        app_state.is_suspeded = false;

        let app = Self{
            app_state,
            app_config,
        };

        Ok(app)
    }

    pub fn run(&mut self)
    {
        while self.app_state.is_running
        {
            if !self.app_state.plat_state.pump_message()
            {
                info!("Platform stopped pumping messages");
                self.app_state.is_running = false;
            }
        }
        self.app_state.is_running = false;
        self.app_state.plat_state.shutdown();
    }
}

fn create_window(mut plat_state: &mut PlatformState, app_config: &AppConfig) -> Result<(), Box<dyn Error>>
{
    let screen = &plat_state.screen;
    let conn = &plat_state.connection;

    let win_id = plat_state.window;
    let values = CreateWindowAux::default()
        .background_pixel(screen.black_pixel)
        .event_mask(EventMask::BUTTON_PRESS 
            | EventMask::BUTTON_RELEASE 
            | EventMask::KEY_PRESS 
            | EventMask::KEY_RELEASE
            | EventMask::EXPOSURE
            | EventMask::POINTER_MOTION
            | EventMask::STRUCTURE_NOTIFY
        );
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        app_config.x,
        app_config.y,
        app_config.width,
        app_config.height,
        0,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,        
        &values,
    )?;

    //set window name and icon name
    conn.change_property8(
        PropMode::REPLACE,
        win_id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        app_config.app_name.as_bytes(),
    )?;

    conn.change_property8(
        PropMode::REPLACE,
        win_id,
        AtomEnum::WM_ICON_NAME,
        AtomEnum::STRING,
        app_config.app_name.as_bytes(),
    )?;

    //sends a message if the window is asked to close
    let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS").unwrap();
    let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW").unwrap();

    let wm_protocols = wm_protocols.reply().unwrap().atom;
    let wm_delete_window = wm_delete_window.reply().unwrap().atom;

    conn.change_property32(
        PropMode::REPLACE,
        win_id,
        wm_protocols,
        AtomEnum::ATOM,
        &[wm_delete_window],
    )?;

    //set the atoms
    plat_state.wm_protocol = wm_protocols;
    plat_state.wm_delete_win = wm_delete_window;

    conn.map_window(win_id)?;
    conn.flush()?;
    Ok(())
}

fn is_only_instance() -> bool
{
    let lock_file = "/tmp/myapp.lock";

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .custom_flags(libc::O_EXCL)
        .open(lock_file)
        {
            Ok(_) => { return true },
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    warn!("Another instance of the application is already running");
                    return false;
                } else {
                    error!("Error opening lock file: {}", err);
                }
            }
        };
}
