// Just the queue which is added to via listeners and processed via systems
use shipyard::*;
use web_sys::KeyboardEvent;
use std::collections::VecDeque;

use super::data::Input;

pub type InputQueueViewMut<'a> = UniqueViewMut<'a, InputQueue>;


#[derive(Component, Unique, Debug)]
pub struct InputQueue(pub VecDeque<Input>);
impl InputQueue {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    //The typical use case - will *replace* what was there
    //idea being if a user moves their mouse a bunch of times in a tick
    //we only care about the most recent position
    //if they move, click, move - we still want the click to happen _after_ the move
    //of course this isn't the only use case - others are below
    pub fn insert_replace(&mut self, input:Input) {
        let queue = &mut self.0;
        
        let entry = queue.iter_mut().find(|q_input| {
            std::mem::discriminant(*q_input) == std::mem::discriminant(&input)
        });

        if let Some(entry) = entry {
            //replace what was there
            *entry = input;
        } else {
            self.0.push_back(input);
        }
    }

    //remove what was there, and add the new one to the end
    fn _insert_move(&mut self, input:Input) {
        self.insert_always(input);
        //let queue = &mut self.0;
        
        //queue.retain(|q_input| {
            //std::mem::discriminant(q_input) != std::mem::discriminant(&input)
        //});

        //self.0.push_back(input);
    }

    //careful - this can create long lists!
    //but it's also crucial for situations where we want to accumulate delta
    pub fn insert_always(&mut self, input:Input) {
        self.0.push_back(input);
    }
}
