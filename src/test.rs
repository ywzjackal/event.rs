#[test]
fn test_event() {
    use super::Holder;
    use std::sync::*;

    let mut holder = Holder::<String>::new();

    let handler1_counter = Arc::new(Mutex::new(0));
    let handler1_cc = handler1_counter.clone();
    let handler1 = holder.join(move |msg: &mut String| {
        println!("handler 1 got message: {}", msg);
        *handler1_cc.lock().unwrap() += 1;
    });

    let handler2_counter = Arc::new(Mutex::new(0));
    let handler2_cc = handler2_counter.clone();
    holder.join(move |msg: &mut String| {
        println!("handler 2 got message: {}", msg);
        *handler2_cc.lock().unwrap() += 1;
    });

    holder.invoke("Hello Handler1 and Handler2!".to_string());
    holder.leave(handler1);
    holder.invoke("Hello Handler2!".to_string());

    assert_eq!(*handler1_counter.lock().unwrap(), 1);
    assert_eq!(*handler2_counter.lock().unwrap(), 2);
}