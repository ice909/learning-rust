mod summary;
use summary::{
    Summary,
    Tweet,
    NewsArticle,
};

// 泛型函数
// 求最大值
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest { // 还不能编译，因为没有实现 PartialOrd trait
//             largest = item;
//         }
//     }

//     largest
// }

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// 泛型方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 泛型结构体定义多个类型
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
