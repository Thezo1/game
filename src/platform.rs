use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::wrapper::ConnectionExt as _;
use crate::logger::*;
use crate::{info, log_output};
use crate::application::AppConfig;

struct InternalState
{
    connection: RustConnection,
    window: u32,
    graphics_context: u32,
    screen: Screen,
    wm_protocol: u32,
    wm_delete_win: u32,
}

pub struct PlatformState
{
    internal_state: InternalState,
}

impl InternalState
{
    fn new() -> Self
    {
        let (conn, screen_num) = x11rb::connect(None).expect("Failed to connect");
        let screen = &conn.setup().roots[screen_num];
        let win_id = conn.generate_id().expect("failed to get window id");
        let gc_id = conn.generate_id().expect("failed to get gc id");

        Self
        {
            window: win_id,
            screen: screen.clone(),
            graphics_context: gc_id,
            connection: conn,
            wm_protocol: 0,
            wm_delete_win: 0,
        }
    }
}

impl PlatformState
{
    pub fn new() -> Self
    {
        let internal_state = InternalState::new();
        Self
        {
            internal_state
        }
    }


    pub fn platform_startup(&mut self, app_config: &AppConfig) -> bool
    {
        //get state from struct
        let screen = &self.internal_state.screen;
        let conn = &self.internal_state.connection;
        let gc_id = &self.internal_state.graphics_context;

        //create graphics context to draw to foreground
        let gc_win = screen.root;
        let values = CreateGCAux::default()
            .foreground(screen.white_pixel)
            .graphics_exposures(0);
        conn.create_gc(*gc_id, gc_win, &values).expect("Failed creating a grapics context");

        //create pixmap
        let pid = conn.generate_id().unwrap();
        conn.create_pixmap(
            screen.root_depth,
            pid,
            screen.root,
            600,
            800,
        ).unwrap();

        //create window
        let win_id = &self.internal_state.window;
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
            *win_id,
            screen.root,
            app_config.x,
            app_config.y,
            app_config.width,
            app_config.height,
            0,
            WindowClass::INPUT_OUTPUT,
            screen.root_visual,        
            &values,
        ).expect("Window Creation Failed");

        //set window name and icon name
        conn.change_property8(
            PropMode::REPLACE,
            *win_id,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            app_config.app_name.as_bytes(),
        ).expect("Error changing name");

        conn.change_property8(
            PropMode::REPLACE,
            *win_id,
            AtomEnum::WM_ICON_NAME,
            AtomEnum::STRING,
            app_config.app_name.as_bytes(),
        ).unwrap();

        //sends a message if the window is asked to close
        let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS").unwrap();
        let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW").unwrap();

        let wm_protocols = wm_protocols.reply().unwrap().atom;
        let wm_delete_window = wm_delete_window.reply().unwrap().atom;

        conn.change_property32(
            PropMode::REPLACE,
            *win_id,
            wm_protocols,
            AtomEnum::ATOM,
            &[wm_delete_window],
        ).unwrap();

        //set the atoms
        self.internal_state.wm_protocol = wm_protocols;
        self.internal_state.wm_delete_win = wm_delete_window;

        conn.map_window(*win_id).expect("Map Window Error");
        conn.flush().unwrap();

        return true;
    }

    pub fn platform_shutdown(&self)
    {
        self.internal_state.connection.destroy_window(self.internal_state.window).expect("Error destroying window");    
    }

    pub fn platform_pump_message(&self) -> bool
    {
        let mut quit = false;
        loop
        {
            let event = self.internal_state.connection.wait_for_event().unwrap();
            match event
            {
                Event::KeyPress(_) => {},
                Event::KeyRelease(_) => (),
                Event::ButtonPress(_) => (),
                Event::ButtonRelease(_) => (),

                Event::MotionNotify(_) => (),
                Event::ConfigureNotify(_) => (),
                Event::ClientMessage(event) => {
                    if event.data.as_data32()[0] == self.internal_state.wm_delete_win 
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
}
