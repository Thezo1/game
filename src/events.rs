type Callback = fn(code: EventCodes) -> bool;

enum EventCodes
{
    // platform event codes
    ApplicationQuit = 0x01,
    KeyPressed = 0x02,
    KeyReleased = 0x03,
    ButtonPressed = 0x04,
    ButtonReleased = 0x05,
    MouseMoved = 0x06, 
    MouseWheel = 0x07,
    WindowResized = 0x08,
    MaxEvent = 0xFF,
}
