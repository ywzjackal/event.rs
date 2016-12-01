use std::collections::HashMap;
use std::any::Any;

pub trait Raiser {
    type Type: Sized;
    fn on_event(&mut self, &mut Self::Type);
    fn as_any(&mut self) -> &mut Any;
}

pub struct Holder<T> {
    counter: usize,
    pub handlers: HashMap<usize, Box<Raiser<Type = T>>>,
}

impl<T: Send + 'static> Holder<T> {
    pub fn new() -> Holder<T> {
        Holder {
            counter: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn join<L>(&mut self, r: L) -> usize
        where L: Raiser<Type = T> + 'static
    {
        self.counter += 1;
        self.handlers.insert(self.counter, Box::new(r));
        return self.counter;
    }

    pub fn leave(&mut self, id: usize) -> Option<Box<Raiser<Type = T>>> {
        self.handlers.remove(&id)
    }

    pub fn invoke(&mut self, mut arg: T) {
        for (_, h) in self.handlers.iter_mut() {
            h.on_event(&mut arg);
        }
    }
}