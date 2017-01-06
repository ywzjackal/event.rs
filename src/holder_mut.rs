use std::collections::HashMap;

pub struct HolderMut<T> {
    counter: usize,
    pub handlers: HashMap<usize, Box<Fn(&mut T)>>,
}

impl<T: 'static> HolderMut<T> {
    pub fn new() -> HolderMut<T> {
        HolderMut {
            counter: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn join<F>(&mut self, r: F) -> usize
        where F: Fn(&mut T) + 'static
    {
        self.counter += 1;
        self.handlers.insert(self.counter, Box::new(r));
        return self.counter;
    }

    pub fn leave(&mut self, id: usize) -> Option<Box<Fn(&mut T)>> {
        self.handlers.remove(&id)
    }

    pub fn invoke(&mut self, mut arg: &mut T) {
        for (_, h) in self.handlers.iter_mut() {
            (*h)(&mut arg);
        }
    }
}