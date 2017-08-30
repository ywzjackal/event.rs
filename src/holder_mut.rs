use std::collections::HashMap;

pub struct HolderMut<T> {
    counter: usize,
    pub handlers: HashMap<usize, Box<FnMut(&mut T) + Send>>,
}

impl<T: 'static> HolderMut<T> {
    pub fn new() -> HolderMut<T> {
        HolderMut {
            counter: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn join<F>(&mut self, r: F) -> usize
        where F: FnMut(&mut T) + Send + 'static
    {
        self.counter += 1;
        self.handlers.insert(self.counter, Box::new(r));
        return self.counter;
    }

    pub fn leave(&mut self, id: usize) -> Option<Box<FnMut(&mut T) + Send>> {
        self.handlers.remove(&id)
    }

    pub fn invoke(&mut self, mut arg: &mut T) {
        for (_, h) in self.handlers.iter_mut() {
            (*h)(&mut arg);
        }
    }
}