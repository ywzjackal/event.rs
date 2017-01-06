#[test]
fn test_() {
    use super::Holder;

    let mut holder_ref = Holder::<String>::new();

    let handler1 = holder_ref.join(move |msg: String| {
        println!("handler 1 got message: {}", msg);
    });

    holder_ref.join(move |msg: String| {
        println!("handler 2 got message: {}", msg);
    });

    holder_ref.invoke("Hello Handler1 and Handler2!".to_string());
    holder_ref.leave(handler1);
    holder_ref.invoke("Hello Handler2!".to_string());
}
#[test]
fn test_ref() {
    use super::HolderRef;

    let mut holder_ref = HolderRef::<String>::new();

    let handler1 = holder_ref.join(move |msg: &String| {
        println!("handler 1 got message: {}", msg);
    });

    holder_ref.join(move |msg: &String| {
        println!("handler 2 got message: {}", msg);
    });

    holder_ref.invoke(&mut "Hello Handler1 and Handler2!".to_string());
    holder_ref.leave(handler1);
    holder_ref.invoke(&mut "Hello Handler2!".to_string());
}
#[test]
fn test_mut() {
    use super::HolderMut;
    use std::sync::*;

    let mut holder_ref = HolderMut::<String>::new();

    let handler1_counter = Arc::new(Mutex::new(0));
    let handler1_cc = handler1_counter.clone();
    let handler1 = holder_ref.join(move |msg: &mut String| {
        println!("handler 1 got message: {}", msg);
        *handler1_cc.lock().unwrap() += 1;
    });

    let handler2_counter = Arc::new(Mutex::new(0));
    let handler2_cc = handler2_counter.clone();
    holder_ref.join(move |msg: &mut String| {
        println!("handler 2 got message: {}", msg);
        *handler2_cc.lock().unwrap() += 1;
    });

    holder_ref.invoke(&mut "Hello Handler1 and Handler2!".to_string());
    holder_ref.leave(handler1);
    holder_ref.invoke(&mut "Hello Handler2!".to_string());

    assert_eq!(*handler1_counter.lock().unwrap(), 1);
    assert_eq!(*handler2_counter.lock().unwrap(), 2);
}