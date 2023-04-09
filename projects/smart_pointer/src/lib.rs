use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub fn hello(name: &str) {
    println!("Hello, {}", name);
}

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {} !", self.data);
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub struct SomeMessenger {
    pub values: RefCell<Vec<String>>,
}

impl SomeMessenger {
    pub fn new() -> SomeMessenger {
        SomeMessenger {
            values: RefCell::new(vec![]),
        }
    }
}

impl Messenger for SomeMessenger {
    fn send(&self, msg: &str) {
        self.values.borrow_mut().push(String::from(msg));

        // let mut one_borrow = self.values.borrow_mut();
        // let mut two_borrow = self.values.borrow_mut();

        // one_borrow.push(String::from(msg));
        // two_borrow.push(String::from(msg));
    }
}

#[derive(Debug)]
pub enum ListV2 {
    Cons(i32, Rc<ListV2>),
    Nil,
}

#[derive(Debug)]
pub enum ListV3 {
    Cons(Rc<RefCell<i32>>, RefCell<Rc<ListV3>>),
    Nil,
}

#[derive(Debug)]
pub enum ListV4 {
    Cons(i32, RefCell<Rc<ListV4>>),
    Nil,
}

impl ListV4 {
    pub fn tail(&self) -> Option<&RefCell<Rc<ListV4>>> {
        match self {
            ListV4::Cons(_, ref item) => Some(item),
            ListV4::Nil => None,
        }
    }
}
