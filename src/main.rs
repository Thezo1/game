use platform::*;
use application::*;
use events::*;


mod platform;
mod logger;
mod application;
mod events;
mod ring_buffer;
mod test;

fn main()
{

    let app_config = AppConfig::new("test", 100, 100, 1200, 700);
    let mut platform = PlatformState::new();
    let app = App::create(&mut platform, app_config).unwrap();
    app.run();
}
