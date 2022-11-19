use platform::PlatformState;

mod platform;

fn main()
{
    PlatformState::platform_startup("test", 0, 0, 100, 100);
}
