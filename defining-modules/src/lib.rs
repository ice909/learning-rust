mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// 使用use关键字来引入模块
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    hosting::add_to_waitlist();
}

// 使用as关键字来重命名模块
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// use std::cmp::Ordering;
// use std::io;
// 嵌套路径，消除大量use行
use std::{
    cmp::Ordering,
    io::{self, Write},
};
// global 运算符, 将所有的公有项引入作用域
use std::collections::*;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super 开始的相对路径，相当于../
    }

    fn cook_order() {}
}

mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}
