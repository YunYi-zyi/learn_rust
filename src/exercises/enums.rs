use std::fmt;

pub fn run() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home: {home}, loopback {loopback}");

    let home_another = IpAddrAnother::V4(127, 0, 0, 1);
    let loopback_another = IpAddrAnother::V6(String::from("::1"));

    println!("Here is another way: Home: {home_another}, loopback {loopback_another}");
    
    /*
    Rust中还有一种叫Option<T>的枚举，包含在prelude中，用来代表其他语言中的NULL
    let some_n = Some(1);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    */
}

enum IpAddrKind {
    V4,
    V6,
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IpAddrKind::V4 => "IPv4",
            IpAddrKind::V6 => "IPv6",
        };
        write!(f, "{s}")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kind: {}, Address: {}", self.kind, self.address)
    }
}

enum IpAddrAnother {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl fmt::Display for IpAddrAnother {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddrAnother::V4(a, b, c, d) => write!(f, "{a}.{b}.{c}.{d}"),
            IpAddrAnother::V6(address) => write!(f, "{address}"),
        }
    }
}

// 枚举也可以有不同类型
/*
* enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
*/
