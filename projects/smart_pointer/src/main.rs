enum List {
    Cons(i32, List),
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

        let list = Cons(1, Cons(2, Cons(3, Nil)));
    }
}
