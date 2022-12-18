//TODO: Implement Idiomatic error handling
use crate::logger::*;
use crate::{log_output, info, error};
use crate::events::{Data, Event};
use crate::ring_buffer::{Buffer, MAXBUFFERSIZE};
use std::collections::HashMap;

pub const MAXQUEUES: usize = 2;

pub trait Listener{}

//TODO:(zourt) add a way to link to listener
pub type Callback = fn(u32, Data);

type EventListenersList = Buffer<Callback>;
type EventHashMap = HashMap<u32, EventListenersList>;
type EventQueue = Buffer<Event>;

#[derive(Default)]
pub struct EventManager{
    _event_listener_map: EventHashMap,
    _event_queue: Vec<EventQueue>,
    _active_queue: usize,
}

impl EventManager
{
    pub fn new() -> Self
    {
        let _event_listener_map = EventHashMap::default();
        let _active_queue = 0;
        let mut _event_queue = Vec::with_capacity(MAXQUEUES);
        let buffer = Buffer::new();
        _event_queue.push(buffer.clone());
        _event_queue.push(buffer);
        Self
        {
            _event_listener_map,
            _event_queue,
            _active_queue,
        }
    }

    pub fn register_listener(&mut self, code: u32, callback: Callback) -> bool
    {
        let event_listener_list = self._event_listener_map
            .entry(code)
            .or_insert(Buffer::new());

        // register function pointer
        event_listener_list.insert(callback);
        true
    }

    pub fn unregister_listener(&mut self, code: u32, callback: Callback) -> bool
    {
        let event_listener_list = self._event_listener_map.get_mut(&code).unwrap();
        let mut x = 0;
        while x < MAXBUFFERSIZE
        {
            if event_listener_list.entries[x].unwrap() as u32 == callback as u32
            {
                event_listener_list.remove_at_index(x);
                return true
            }
            x+=1;
        }

        false
    }

    pub fn queue_event(&mut self, event: Event) -> bool
    {
        assert!(self._active_queue < MAXQUEUES);
        let event_listener_list = self._event_listener_map.get_mut(&event.code);
        if event_listener_list.is_some()
        {
            self._event_queue[self._active_queue].push_back(event);
            info!("Event added to queue");
            return true;
        }
        error!("Something went wrong");
        false
    }

    //TODO: Implement abort event
    //pub fn abort_event(&mut self, event: Event) -> bool
    //{
    //    assert!(self.active_queue < MAXQUEUES);
    //    let event_listener_list = self.event_listener_map.get_mut(&event.code);
    //    if event_listener_list.is_some()
    //    {
    //        let event_queue = self.event_queue[self.active_queue].as_mut();
    //        
    //        return true;
    //    }
    //    error!("Something went wrong");
    //    false
    //}

    pub fn process_event(&mut self) -> bool
    {
        let queue_to_process = self._active_queue;
        self._active_queue = (queue_to_process + 1)%MAXQUEUES;
        self._event_queue[self._active_queue].clear();

        let events = &mut self._event_queue[queue_to_process];

        //process queue
        let mut x = 0;
        while events.entries[x].is_some()
        {
            let event = events.pop_front().unwrap();
            let event_type = event.code;
            let event_listener_list = self._event_listener_map.get(&event_type);

            if event_listener_list.is_some()
            {
                let callbacks = event_listener_list.unwrap().entries;
                for callback in callbacks
                {
                    if callback.is_some()
                    {
                        let callback = callback.unwrap();
                        callback(event_type, event.data);
                    }
                }
                let y = x+1;
                info!("Process Count: {y} ");
            }
            x+=1;
        }

        true
    }
}
