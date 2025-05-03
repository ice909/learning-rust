// 定义结构体
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// 没有命名字段的元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// 没有任何字段的单元结构体
#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    // 创建一个结构体实例
    // 不可变
    let user = User {
        active: true,
        username: String::from("ice909"),
        email: String::from("tonimayloneya@gmail.com"),
        sign_in_count: 1,
    };
    println!("User: {:?}", user);

    // 可变
    let mut user2 = User {
        active: true,
        username: String::from("ice909"),
        email: String::from("tonimayloneya@gmail.com"),
        sign_in_count: 1,
    };
    user2.username = String::from("ice9092");
    println!("User2: {:?}", user2);

    user2 = build_user(String::from("tonimayloneya@gmail.com"), String::from("ice"));
    println!("User2: {:?}", user2);

    // 结构体更新语法
    let user3 = User {
        email: String::from("example.qq.com"),
        ..user2
    };

    println!("User3: {:?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black: {:?}", black);
    println!("Origin: {:?}", origin);

    let subject = AlwaysEqual;
    println!("Subject: {:?}", subject);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "矩形的面积是{}个像素。",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
