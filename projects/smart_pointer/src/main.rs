use std::ops::Deref;
use std::rc::Rc;

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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {} !", self.data);
    }
}

#[derive(Debug)]
enum ListV2 {
    Cons(i32, Rc<ListV2>),
    Nil,
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

    // Drop trait
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created");
    }

    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main");
    }

    // Rc<T> struct
    {
        use ListV2::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
        println!("b = {:?}, c = {:?}", b, c);
    }
}
