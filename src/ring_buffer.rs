use crate::logger::{get_level_string, get_level_color, LogLevel};
use crate::{info, error, log_output};

pub const MAXBUFFERSIZE: usize = 10;

#[derive(Default)]
pub struct Buffer<T>
{
    pub head: usize,
    pub tail: usize,
    pub entries: [Option<T>;MAXBUFFERSIZE],
}

impl<T: Copy> Buffer<T>
{
    pub fn new() -> Self
    {
        Self{head: 0, tail: 0, entries: [None;MAXBUFFERSIZE]}
    }

    pub fn push_back(&mut self, value: T) -> bool
    {
        if (self.tail + 1)%MAXBUFFERSIZE == self.head
        {
            error!("Buffer size exceeded");
            return false;
        }

        self.entries[self.tail] = Some(value);
        self.tail = (self.tail + 1)%MAXBUFFERSIZE;
        true
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        //no pending requests, do nothing
        if self.head == self.tail
        {
            info!("No Pending Requests");
            return None;
        }

        let on_event = self.entries[self.head];
        self.entries[self.head] = None;
        self.head = (self.head+1)%MAXBUFFERSIZE;
        on_event
    }
}

fn on_event(code: u8){}

#[cfg(test)]
mod tests
{
    use crate::test;
    use super::*;
    #[test]
    fn build_works()
    {
        let mut buffer = Buffer::new();

        buffer.push_back(test::on_event as fn(u8));
        buffer.push_back(on_event as fn(u8));
        assert_ne!(buffer.entries[0].unwrap(), buffer.entries[1].unwrap());
    }

    #[test]
    fn pop_works()
    {
        let mut buffer = Buffer::new();
        buffer.push_back(test::on_event as fn(u8));
        buffer.push_back(on_event as fn(u8));

        buffer.pop_front();
        buffer.pop_front();
        assert_eq!(buffer.entries[0], None);
        assert_eq!(buffer.head, 2);
        assert_eq!(buffer.tail, 2);
    }
}
