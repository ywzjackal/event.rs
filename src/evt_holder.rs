use std::collections::HashMap;
use super::Handler;

pub struct EventHolder<H, S, E>
    where H: Handler<S, E> + Sized,
          E: Sized
{
    _e: Option<(S, E)>,
    counter: usize,
    pub handlers: HashMap<usize, H>,
}

impl<H, S, E> EventHolder<H, S, E>
    where H: Handler<S, E> + Sized + 'static,
          E: Sized
{
    pub fn new() -> EventHolder<H, S, E> {
        EventHolder {
            _e: None,
            counter: 0,
            handlers: HashMap::new(),
        }
    }
    pub fn join(&mut self, f: H) -> usize {
        self.counter += 1;
        self.handlers.insert(self.counter, f);
        return self.counter;
    }

    pub fn leave(&mut self, id: usize) -> Option<H> {
        self.handlers.remove(&id)
    }

    pub fn invoke(&mut self, mut sender: &mut S, mut arg: &mut E) {
        for (_, h) in self.handlers.iter_mut() {
            h.on_event(&mut sender, &mut arg);
        }
    }
}

impl<H, S, E> EventHolder<H, S, E>
    where H: Handler<S, E> + Sized + Clone + Sync + Send + 'static,
          S: Clone + Sized + Sync + Send + 'static,
          E: Clone + Sized + Sync + Send + 'static
{
    pub fn invoke_multithreading(&self, sender: &mut S, arg: &mut E) {
        use std::thread::Builder;
        for (_, h) in self.handlers.iter() {
            let mut sc = sender.clone();
            let mut hc = h.clone();
            let mut ac = arg.clone();
            Builder::new()
                .name("EventHolder Multithread".to_string())
                .spawn(move || hc.on_event(&mut sc, &mut ac))
                .unwrap();
        }
    }
}