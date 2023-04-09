use smart_pointer::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Using Box<T>
    {
        println!("# Using Box<T>\n");

        let b = Box::new(5);
        println!("b = {}", b);
    }

    // ConsList
    {
        use List::{Cons, Nil};

        println!("\n# ConsList\n");

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

    // dereference-type corection
    {
        println!("\n# dereference-type corection\n");

        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    // Drop trait
    {
        println!("\n# Drop trait\n");

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

    // Rc<T> type
    {
        use ListV2::{Cons, Nil};

        println!("\n# Rc<T> type\n");

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    // RefCell<T> type
    {
        println!("\n# RefCell<T> type\n");

        let some_messenger = SomeMessenger::new();
        let mut limit_tracker = LimitTracker::new(&some_messenger, 100);
        limit_tracker.set_value(80);
        println!("values = {:?}", some_messenger.values);
    }

    // Rc<T> and RefCell<T>
    {
        use ListV3::{Cons, Nil};

        println!("\n# Rc<T> and RefCell<T>\n");

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), RefCell::new(Rc::new(Nil))));
        let b = Cons(Rc::new(RefCell::new(3)), RefCell::new(Rc::clone(&a)));
        let c = Cons(Rc::new(RefCell::new(4)), RefCell::new(Rc::clone(&a)));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    // Reference cycles
    {
        use ListV4::{Cons, Nil};

        println!("\n# Reference cycles\n");

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }
}

// RefCell<T> type
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = SomeMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.values.borrow().len(), 1);
    }
}
