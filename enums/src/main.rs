// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 更简洁的定义方式
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

// 枚举每个成员都可以处理不同类型和数量的数据
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// match 控制流语句
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
        Coin::Quarter => 25,
    }
}

// 匹配Option<T>类型
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 通配模式和_占位符

fn main() {
    // 枚举的成员位于其标识符的命名空间中，并使用两个冒号分开
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 使用更简洁的定义方式
    let home2 = IpAddrKind2::V4(String::from("127.0.0.0"));
    let loopback2 = IpAddrKind2::V6(String::from("::1"));

    let home3 = IpAddrKind3::V4(127, 0, 0, 1);
    let loopback3 = IpAddrKind3::V6(String::from("::1"));

    // 使用 match 控制流语句
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);

    // 使用 Option<T> 类型
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // 除了 3 和 7 以外的所有值
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // 可以匹配任意值而不绑定到该值，空元组
    }

    // if let 简洁控制流
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// 定义一个函数，可以接收任何IpAddrKind类型的参数
fn route(ip_kind: IpAddrKind) {
    // 函数体
    match ip_kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}
