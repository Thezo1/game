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
        assert!((self.tail + 1)%MAXBUFFERSIZE != self.head);
        if self.tail>=MAXBUFFERSIZE
        {
            error!("Buffer size exceeded");
            return false;
        }

        self.entries[self.tail] = Some(value);
        self.tail+=(self.tail + 1)%MAXBUFFERSIZE;
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
        self.head+=(self.head+1)%MAXBUFFERSIZE;
        on_event
    }
}

#[cfg(test)]
mod tests
{
    use crate::test::*;
    use super::*;
    #[test]
    fn build_works()
    {
    }
}
