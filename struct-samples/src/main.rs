fn main() {
    //  struct_samples();

    //   enum_samples();

    //   options_samples();

    value_in_cents(Coin::Quarter(IndState::Maharashtra));

    value_in_cents(Coin::Nickel);
}

fn struct_samples() {
    let user = User {
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rect is {:#?}", rect);

    println!("area of rect is {:#?}", rect.area());

    let rect1 = Rectangle {
        width: 1,
        height: 2,
    };

    let rect2 = Rectangle {
        width: 3,
        height: 4,
    };

    println!(
        "{:#?} {:#?} can hold {}",
        rect1,
        rect2,
        rect1.can_hold_other(&rect2)
    );

    println!(
        "{:#?} {:#?} can hold {}",
        rect1,
        rect2,
        rect2.can_hold_other(&rect1)
    );

    let rect3 = Rectangle::square(5);
}

fn enum_samples() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let localhost1 = IpAddrKind1::V4(String::from("127.0.0.1"));

    let localhost2 = IpAddrKind2::V4(127, 0, 0, 1);

    Message::demo_function();
}

fn options_samples() {
    let x: i8 = 5;
    let y: Option<i8> = Some(10);

    let sum = x + y.unwrap_or(0);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold_other(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind1 {
    V4(String),
    V6(String),
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn demo_function() {
        println!("Hello, Enum!");
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}

#[derive(Debug)]
enum IndState {
    Maharashtra,
    UP,
    Gujarat,
    Delhi,
    Karnataka,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(IndState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}
