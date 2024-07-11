// enum IpAddrKind {
//     V4(u8,u8,u8,u8),
//     V6(String),
// }

// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddrKind::V6(String::from("::1"));


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter =>25,
    }
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some (i+1),
    }
}

// concise control flow with if let
