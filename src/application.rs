use crate::logger::*;
use crate::platform::PlatformState;
use crate::{log_output, error};

static mut INITIALIZED: bool = false;

#[derive(Debug)]
pub enum ErrorApp
{
    AppCreateError
}

pub struct App<'a, 'b>
{
    pub app_config: AppConfig<'b>,
    app_state: AppState<'a>,
}

pub struct AppConfig<'b>
{
    pub app_name: &'b str,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

pub struct AppState<'a>
{
    is_running: bool,
    is_suspeded: bool,
    plat_state: &'a PlatformState,
    width: i16,
    height: i16,
    last_time: f64,
}

impl<'b> AppConfig<'b>
{
    pub fn new(app_name: &'b str, x: i16, y: i16, width: u16, height: u16) -> Self
    {
        return Self{app_name, x, y, width, height};
    }
}

impl<'a> AppState<'a>
{
    fn new(plat_state: &'a PlatformState) -> Self
    {
        return Self
            {
            is_running: true, 
            is_suspeded: false,
            plat_state,
            width: 100,
            height: 100,
            last_time: 0.
        };
    }
}

impl<'a, 'b> App<'a, 'b>
{
    pub fn create(plat_state: &'a mut PlatformState, app_config: AppConfig<'b>) -> Result<App<'a, 'b>, ErrorApp>
    {
        //TODO: find a better way to do this without static variable
        unsafe 
        {
            if INITIALIZED
            {
                error!("Create Application called more than once");
                return Err(ErrorApp::AppCreateError);
            }

            /* initialize subsystems here

            */


            if !plat_state.platform_startup(&app_config)
            {
                return Err(ErrorApp::AppCreateError);
            }
            let mut app_state = AppState::new(plat_state);

            app_state.is_running = true;
            app_state.is_suspeded = false;

            let app = App
                {
                app_config,
                app_state,
            };
            INITIALIZED = true;
            return Ok(app);
        }
    }

    pub fn run(mut self) -> bool
    {
        while self.app_state.is_running
        {
            if !self.app_state.plat_state.platform_pump_message()
            {
                self.app_state.is_running = false;
            }
        }
        self.app_state.is_running = false;
        self.app_state.plat_state.platform_shutdown();
        return true;
    }
}
