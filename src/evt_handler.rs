pub trait Handler<S, E>
    where S: Sized,
          E: Sized
{
    fn on_event(&mut self, &mut S, &mut E);
}

impl<S, E, F> Handler<S, E> for F
    where S: 'static,
          E: 'static,
          F: FnMut(&mut S, &mut E) + 'static
{
    fn on_event(&mut self, mut s: &mut S, mut e: &mut E) {
        self(&mut s, &mut e)
    }
}