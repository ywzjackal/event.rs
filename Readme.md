# Event and Multithreading Event crate for Rust
## Example
### Single Thread Event
```
#[test]
fn test_event() {
    use super::Handler;
    use super::EventHolder;
    use std::thread::current;

    // Normal Handler
    struct MyHandler {}
    // Handler<Sender Type, Event Argument Type>
    // Handler for SenderType: i32, Event Argument Type: i32
    impl Handler<i32, i32> for MyHandler {
        // on_event(&mut self, Sender Mutable Refrence, Event Mutable Argument Refrence)
        fn on_event(&mut self, s: &mut i32, v: &mut i32) {
            println!("got add event, {} -> {}, thread: {:?}",
                     *s,
                     *v,
                     current().name());
        }
    }
    // Handler for SenderType: u32, Event Argument Type: u32
    impl Handler<u32, u32> for MyHandler {
        fn on_event(&mut self, s: &mut u32, v: &mut u32) {
            println!("got sub event, {} -> {}, thread: {:?}",
                     *s,
                     *v,
                     current().name());
        }
    }

    // create two different EventHolder
    let mut add_holder = EventHolder::<MyHandler, i32, i32>::new();
    let mut sub_holder = EventHolder::<MyHandler, u32, u32>::new();

    // join Event Handler into EventHolder
    add_holder.join(MyHandler {});
    sub_holder.join(MyHandler {});
    add_holder.join(MyHandler {});

    // hold join_index, need by leave holder func
    let join_index = sub_holder.join(MyHandler {});
    // leave holder with join_index
    sub_holder.leave(join_index);

    // invoke event
    // Sender: i32, Event Argument: i32
    add_holder.invoke(&mut 333, &mut 123);
    // Sender: u32, Event Argument: u32
    sub_holder.invoke(&mut 777, &mut 987);
}

```
Output: ``` cargo test -- --nocapture ```
```
got add event, 333 -> 123, thread: Some("test::test_event")
got add event, 333 -> 123, thread: Some("test::test_event")
got sub event, 777 -> 987, thread: Some("test::test_event")
test test::test_event ... ok
```
### Multithread Event
```
#[test]
fn test_event_multithreading() {
    use super::Handler;
    use super::EventHolder;
    use std::sync::{Arc, Mutex};
    use std::thread::current;
    use std::thread::sleep;
    use std::time::Duration;

    // Multithread Able Handler impl Sync + Send + Sized + 'static
    type MyHandler = Arc<Mutex<i32>>;
    fn new_myhandler() -> MyHandler {
        Arc::new(Mutex::new(0))
    }

    // Handler for SenderType: i32, EventArugmentType: i32
    impl Handler<i32, i32> for MyHandler {
        fn on_event(&mut self, s: &mut i32, v: &mut i32) {
            match self.lock() {
                Ok(mut i) => {
                    println!("got add event, self:{:?}, event: {} -> {}, thread: {:?}",
                             *i,
                             *s,
                             *v,
                             current().name());
                    *i += 1;
                }
                Err(e) => {
                    panic!("{:?}", e);
                }
            }
        }
    }

    // create two same holder
    let mut holder = EventHolder::<MyHandler, i32, i32>::new();
    let mut holder2 = EventHolder::<MyHandler, i32, i32>::new();
    // create handler
    let myhandler = new_myhandler();

    // join handler to holders
    holder.join(myhandler.clone());
    holder2.join(myhandler.clone());

    // invoke two event holder multithreading...
    holder.invoke_multithreading(&mut 777, &mut 987);
    holder.invoke_multithreading(&mut 222, &mut 987);

    // wait threads exit.
    sleep(Duration::from_millis(100));
}
```
Output: ```cargo test -- --nocaptur```
```
got add event, self:0, event: 777 -> 987, thread: Some("EventHolder Multithread")
got add event, self:1, event: 222 -> 987, thread: Some("EventHolder Multithread")
test test::test_event_multithreading ... ok
```