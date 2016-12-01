# Event crate for Rust
## Example
```rust
#[test]
fn test_event() {
    use std::any::Any;
    use super::Raiser;
    use super::Holder;
    /// this is an custom event trait
    pub trait MessageEvent {
        fn on_message(&mut self, &mut String);
    }

    impl<E> Raiser for E
        where E: MessageEvent + 'static
    {
        type Type = String;
        fn on_event(&mut self, msg: &mut String) {
            self.on_message(msg)
        }
        fn as_any(&mut self) -> &mut Any {
            self
        }
    }

    /// the trait event client
    struct MessageClient {
        message_list: Vec<String>,
    }
    /// logic for MessageEvent
    impl MessageEvent for MessageClient {
        fn on_message(&mut self, msg: &mut String) {
            self.message_list.push(msg.clone())
        }
    }

    /// server is MessageEvent Riser
    let mut server = Holder::<String>::new();

    /// create two client for MessageEvent
    let client1 = MessageClient { message_list: Vec::new() };
    let client2 = MessageClient { message_list: Vec::new() };

    /// join clients to MessageEvent Riser
    let client1_index = server.join(client1);
    let client2_index = server.join(client2);

    /// Invoke MessageEvent Riser
    server.invoke("Hello".to_string());
    let mut c1 = server.leave(client1_index).unwrap();
    let client1 = c1.as_any().downcast_ref::<MessageClient>().unwrap();
    let mut c2 = server.leave(client2_index).unwrap();
    let client2 = c2.as_any().downcast_ref::<MessageClient>().unwrap();
    assert_eq!(client1.message_list.len(), 1);
    assert_eq!(client2.message_list.len(), 1);
    assert_eq!(client1.message_list.as_slice(), ["Hello".to_string()]);
    assert_eq!(client2.message_list.as_slice(), ["Hello".to_string()]);
    println!("test success!");
}

```