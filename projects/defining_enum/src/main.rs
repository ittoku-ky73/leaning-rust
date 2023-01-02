#[derive(Debug)]
struct Ipv4Addr {
    _address: String,
}

impl Ipv4Addr {
    fn new(_address: String) -> Self {
        Self {
            _address
        }
    }
}

#[derive(Debug)]
struct Ipv6Addr {
    _address: String,
}

impl Ipv6Addr {
    fn new(_address: String) -> Self {
        Self {
            _address
        }
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message call: {:?}", self);
    }
}

fn main() {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(String::from("127.0.0.1")));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(String::from("::1")));
    let m1 = Message::Quit;
    let m2 = Message::Move { _x: 12, _y: 34 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(12, 34, 56);
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    println!("v4 address is: {:?}", localhost_v4);
    println!("v6 address is: {:?}", localhost_v6);
    m1.call();
    m2.call();
    m3.call();
    m4.call();
    println!("some_number is: {:?}", some_number);
    println!("some_char is: {:?}", some_char);
    println!("absent_number is: {:?}", absent_number);
}
