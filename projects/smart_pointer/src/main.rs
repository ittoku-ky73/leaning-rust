use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // Using Box<T>
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    // ConsList
    {
        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);
    }

    // using `*`
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // Box<T>
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // Box<T> struct
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // dereference-type coercion
    fn hello(name: &str) {
        println!("Hello, {}", name);
    }
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}
