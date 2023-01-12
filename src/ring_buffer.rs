use crate::logger::{get_level_string, get_level_color, LogLevel};
use crate::{info, error, log_output};
use crate::events::*;

// the last entry will be empty due to my wrap around logic
// TODO:(Zourt) when filling up array MAXBUFFERSIZE - 1
pub const MAXBUFFERSIZE: usize = 10;

#[derive(Clone, Copy)]
pub struct Buffer<T>
{
    pub head: usize,
    pub tail: usize,
    pub entries: [Option<T>;MAXBUFFERSIZE],
}

impl<T: Copy> Default for Buffer<T>
{
    fn default() -> Self
    {
        Self{head: 0, tail: 0, entries: [None;MAXBUFFERSIZE]}
    }
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

    pub fn insert(&mut self, value: T) -> bool
    {
        if self.tail > MAXBUFFERSIZE
        {
            error!("Buffer size exceeded");
            return false;
        }

        let mut x = 0;
        for entry in self.entries
        {
            if entry.is_none()
            {
                self.entries[x] = Some(value);
                return true;
            }
            x+=1;
        }
        false
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        //no pending requests, do nothing
        if self.head == self.tail
        {
            info!("No Pending Requests");
            return None;
        }

        let value = self.entries[self.head];
        self.entries[self.head] = None;
        self.head = (self.head+1)%MAXBUFFERSIZE;
        value
    }

    pub fn remove_at_index(&mut self, index: usize) -> bool
    {
        if self.head == self.tail
        {
            info!("Buffer Empty");
            return false;
        }

        self.entries[index] = None;
        true
    }

    pub fn clear(&mut self)
    {
        let mut x = 0;
        while x < MAXBUFFERSIZE
        {
            self.entries[x] = None;
            x+=1;
        }
        self.head = 0;
        self.tail = 0;
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

        let event = test::Test::create_event(32, Data{u64: [3;2]});
        buffer.push_back(event);
        assert!(buffer.entries[0].is_some());
        assert!(buffer.entries[1].is_none());
        assert_eq!(buffer.head, 0);
        assert_eq!(buffer.tail, 1);
    }

    #[test]
    fn pop_works()
    {
        let mut buffer = Buffer::new();
        buffer.push_back(test::on_event as fn(u8));

        assert_eq!(buffer.head, 0);
        assert_eq!(buffer.tail, 1);
        assert!(buffer.entries[1].is_none());
    }

    #[test]
    fn test_test()
    {
        let mut buffer = Buffer::new();
        for _ in 0..MAXBUFFERSIZE - 1
        {
            //println!("head ({}) tail ({})", buffer.head, buffer.tail);
            buffer.push_back(test::on_event as fn(u8));
        }
        buffer.remove_at_index(5);
        buffer.remove_at_index(2);
        buffer.insert(on_event as fn(u8));
        buffer.insert(on_event as fn(u8));
        assert_eq!(buffer.entries[5].unwrap() as u32, buffer.entries[2].unwrap() as u32);
    }

    #[test]
    fn test_clear()
    {
        let mut buffer = Buffer::new();
        for _ in 0..MAXBUFFERSIZE - 1
        {
            buffer.push_back(test::on_event as fn(u8));
        }

        buffer.clear();
        for i in 0..MAXBUFFERSIZE - 1
        {
            println!("entries {:?}", buffer.entries[i]);
        }
        assert_eq!(buffer.head, 0);
        assert_eq!(buffer.tail, 0);
    }
}
