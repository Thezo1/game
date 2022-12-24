use crate::app::*;

mod logger;
mod test;
mod app;

fn main()
{
    let app_config = AppConfig::default();
    let mut app = App::new(app_config).unwrap();
    app.run();
}
