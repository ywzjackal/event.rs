use std::collections::HashMap;

pub struct Holder<T> {
    counter: usize,
    pub handlers: HashMap<usize, Box<Fn(T)>>,
}

impl<T: Clone + 'static> Holder<T> {
    pub fn new() -> Holder<T> {
        Holder {
            counter: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn join<F>(&mut self, r: F) -> usize
        where F: Fn(T) + 'static
    {
        self.counter += 1;
        self.handlers.insert(self.counter, Box::new(r));
        return self.counter;
    }

    pub fn leave(&mut self, id: usize) -> Option<Box<Fn(T)>> {
        self.handlers.remove(&id)
    }

    pub fn invoke(&mut self, arg: T) {
        for (_, h) in self.handlers.iter_mut() {
            (*h)(arg.clone());
        }
    }
}