use platform::*;
use application::*;

mod platform;
mod logger;
mod application;
mod test;

fn main()
{
    let app_config = AppConfig::new("test", 100, 100, 1200, 700);
    let mut platform = PlatformState::new();
    let app = App::create(&mut platform, app_config).unwrap();
    app.run();
}
