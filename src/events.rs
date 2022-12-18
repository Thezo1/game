pub trait Sender
{
    fn create_event(code: u32, data: Data) -> Event
    {
        Event::new(code, data)
    }
}

#[derive(Copy, Clone)]
pub union Data
{
    pub i64: [i64;2],
    pub u64: [u64;2],
    pub f64: [f64;2],

    pub i32: [i32;4],
    pub u32: [u32;4],
    pub f32: [f32;4],

    pub i16: [i16;8],
    pub u16: [u16;8],

    pub i8: [i8;16],
    pub u8: [u8;16],

    pub char: [char;16],
}

//code is a guid
#[derive(Copy, Clone)]
pub struct Event{
    pub code: u32,
    pub data: Data,
}

impl Event
{
    //code is a guid
    fn new(code: u32, data: Data) -> Self
    {
        Self
        {
            code,
            data,
        }
    }
}

impl Default for Event
{
    fn default() -> Self
    {
        Event::new(0, Data{i64:[0;2]})
    }
}
