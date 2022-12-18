use ring_buffer::*;
use platform::*;
use application::*;
use events::*;
use event_manager::*;


mod platform;
mod logger;
mod application;
mod events;
mod ring_buffer;
mod test;
mod event_manager;

fn callback_physics(code: u32, data: Data)
{
    println!("Event for physics: {code}, Data: data");
}

fn callback_audio(code: u32, data: Data)
{
    println!("Event for audio: {code}, Data: data");
}

fn callback_renderer(code: u32, data: Data)
{
    println!("Event for renderer: {code}, Data: data");
}
struct Physics;
struct Audio;
struct Renderer;
impl Sender for Physics{}
impl Sender for Audio{}
impl Sender for Renderer{}

fn main()
{
    let app_config = AppConfig::new("test", 100, 100, 1200, 700);
    let mut g_event_manager = EventManager::new();

    //register_listener
    g_event_manager.register_listener(12, callback_audio);
    g_event_manager.register_listener(2, callback_renderer);
    let event = Physics::create_event(12, Data{u32: [3;4]});
    let event_audio = Physics::create_event(2, Data{u32: [3;4]});
    for _ in 0..3
    {
        g_event_manager.queue_event(event);
    }
    g_event_manager.process_event();
    

    let mut platform = PlatformState::new();
    let app = App::create(&mut platform, app_config).unwrap();
    app.run();
}
