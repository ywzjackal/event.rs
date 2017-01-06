use std::collections::HashMap;

pub struct HolderRef<T> {
    counter: usize,
    pub handlers: HashMap<usize, Box<Fn(&T) + Send>>,
}

impl<T: 'static> HolderRef<T> {
    pub fn new() -> HolderRef<T> {
        HolderRef {
            counter: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn join<F>(&mut self, r: F) -> usize
        where F: Fn(&T) + Send + 'static
    {
        self.counter += 1;
        self.handlers.insert(self.counter, Box::new(r));
        return self.counter;
    }

    pub fn leave(&mut self, id: usize) -> Option<Box<Fn(&T) + Send>> {
        self.handlers.remove(&id)
    }

    pub fn invoke(&mut self, mut arg: &T) {
        for (_, h) in self.handlers.iter_mut() {
            (*h)(&mut arg);
        }
    }
}