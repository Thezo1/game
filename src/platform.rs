use x11rb::connection::Connection;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::wrapper::ConnectionExt as _;

struct InternalState<'a>
{
    connection: &'a RustConnection,
    window: u32,
    screen: &'a Screen,
    wm_protocol: u32,
    wm_delete_win: u32,
}

pub struct PlatformState<'a>
{
    internal_state: InternalState<'a>,
}

impl<'a> InternalState<'a>
{
    fn new(connection: &'a RustConnection, screen: &'a Screen) -> Self
    {
        Self
        {
            connection,
            window: 0,
            screen,
            wm_protocol: 0,
            wm_delete_win: 0,
        }
    }
}

impl<'a> PlatformState<'a>
{
    fn new(connection: &'a RustConnection, screen: &'a Screen) -> Self
    {
        let internal_state = InternalState::new(connection, screen);
        Self
        {
            internal_state
        }
    }


    pub fn platform_startup(
        app_name: &str,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    )
    {
        let (conn, screen_num) = x11rb::connect(None).expect("Failed to connect");
        let screen = &conn.setup().roots[screen_num];
        let win_id = conn.generate_id().expect("failed to get window id");


        let mut platform = PlatformState::new(&conn, screen);
        platform.internal_state.window = win_id;

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
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            screen.root_visual,        
            &values,
        ).expect("Window Creation Failed");

        conn.change_property8(
            PropMode::REPLACE,
            win_id,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            app_name.as_bytes(),
        ).expect("Error changing name");

        conn.change_property8(
            PropMode::REPLACE,
            win_id,
            AtomEnum::WM_ICON_NAME,
            AtomEnum::STRING,
            app_name.as_bytes(),
        ).unwrap();

        x11rb::atom_manager! {
            Atoms: AtomsCookie {
                WM_PROTOCOLS,
                WM_DELETE_WINDOW,
            }
        }

        let atoms = Atoms::new(&conn).unwrap().reply().unwrap();
        conn.change_property32(
            PropMode::REPLACE,
            win_id,
            atoms.WM_PROTOCOLS,
            AtomEnum::ATOM,
            &[atoms.WM_DELETE_WINDOW],
        ).unwrap();

        platform.internal_state.wm_protocol = atoms.WM_PROTOCOLS;
        platform.internal_state.wm_delete_win = atoms.WM_DELETE_WINDOW;

        conn.map_window(win_id).expect("Map Window Error");
        conn.flush().expect("Error Flushing");

        loop
        {
        }
    }
}
