use platform::PlatformState;

mod platform;
mod logger;

fn main()
{
    let mut platform = PlatformState::new();
    if platform.platform_startup("test", 0, 0, 100, 100)
    {
        while platform.platform_pump_message()
        {
        }
    }
    platform.platform_shutdown();
}
