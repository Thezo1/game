use x11rb::
{
    rust_connection::RustConnection,
    connection::Connection,
    protocol::xproto::*,
    wrapper::ConnectionExt as _,
    protocol::Event,
};

use crate::logger::*;
use crate::{info, log_output};

struct Config<'a>
{
    name: &'a str,
    _x: i16,
    _y: i16,
    width: u16,
    height: u16,
}

pub struct Platform<'a>
{
    config: Config<'a>,
    connection: RustConnection,
    window: u32,
    screen: Screen,
    gc_id: u32,
    pixmap: u32,
    wm_protocol: u32,
    wm_delete_win: u32,
}

impl<'a> Config<'a>
{
    pub fn new(name: &'a str, width: u16, height: u16) -> Self
    {
       Self {name, _x: 0, _y: 0, width, height,}
    }
}

impl<'a> Platform<'a>
{
    pub fn new() -> Self
    {
        let config = Config::new("Game", 800, 600);
        let (connection, screen_num) = RustConnection::connect(None).unwrap();
        let screen = &connection.setup().roots[screen_num];
        let window = connection.generate_id().unwrap();
        let gc_id = connection.generate_id().unwrap();
        let pixmap = connection.generate_id().unwrap();
        Self
        {
            screen: screen.clone(),
            config,
            connection,
            window,
            gc_id,
            pixmap,
            wm_protocol: 0,
            wm_delete_win: 0,
        }
    }

    fn create_window(&mut self)-> Result<(), Box<dyn std::error::Error>>
    {
        let connection = &self.connection;
        let screen = &self.screen;
        let config = &self.config;
        let window = self.window;
        let gc_id = self.gc_id;
        let pixmap = self.pixmap;

        let values = CreateWindowAux::default()
            .background_pixel(screen.black_pixel)
            .background_pixmap(pixmap)
            .event_mask(EventMask::BUTTON_PRESS 
                | EventMask::BUTTON_RELEASE 
                | EventMask::KEY_PRESS 
                | EventMask::KEY_RELEASE
                | EventMask::EXPOSURE
                | EventMask::POINTER_MOTION
                | EventMask::STRUCTURE_NOTIFY
            );

        // create graphics context
        connection.create_gc(
            gc_id,
            screen.root,
            &CreateGCAux::default().graphics_exposures(0),
        )?;

        // create pixmap, back buffer
        connection.create_pixmap(
            screen.root_depth,
            pixmap,
            screen.root,
            config.width,
            config.height,
        )?;

        connection.create_window(
            screen.root_depth,
            window,
            screen.root,
            0,
            0,
            config.width,
            config.height,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &values,
        )?;

        //set window name and icon name
        connection.change_property8(
            PropMode::REPLACE,
            window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            config.name.as_bytes(),
        )?;
        connection.change_property8(
            PropMode::REPLACE,
            window,
            AtomEnum::WM_ICON_NAME,
            AtomEnum::STRING,
            config.name.as_bytes(),
        )?;

        //sends a message if the window is asked to close
        let wm_protocols = connection.intern_atom(false, b"WM_PROTOCOLS")?;
        let wm_delete_window = connection.intern_atom(false, b"WM_DELETE_WINDOW")?;

        let wm_protocols = wm_protocols.reply()?.atom;
        let wm_delete_window = wm_delete_window.reply()?.atom;

        connection.change_property32(
            PropMode::REPLACE,
            window,
            wm_protocols,
            AtomEnum::ATOM,
            &[wm_delete_window],
        )?;

        //set the atoms
        self.wm_protocol = wm_protocols;
        self.wm_delete_win = wm_delete_window;

        connection.map_window(window)?;
        connection.flush()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        self.create_window()?;
        loop
        {
            let event = self.connection.wait_for_event()?;
            match event
            {
                Event::KeyPress(_) => {},
                Event::KeyRelease(_) => (),
                Event::ButtonPress(_) => (),
                Event::ButtonRelease(_) => (),

                Event::MotionNotify(_) => (),

                Event::ConfigureNotify(event) =>
                {
                    println!("{:?}", event);
                }

                Event::ClientMessage(event) => {
                    if event.data.as_data32()[0] == self.wm_delete_win 
                    {
                        info!("Window was asked to close");
                        return Ok(());
                    }
                },
                _ => (),
            }
        }
    }
}
