#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
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

        let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
        println!("list = {:?}", list);
    }
}
