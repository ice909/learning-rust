fn main() {
    // 语句，不返回值，需要分号
    let _x = 5; // 语句
    println!("Hello, world!");
    another_function();
    another_function_with_param(5);
    another_function_with_multiple_params(5, 10);

    // 表达式，不需要分号
    let z = {
        let x = 3;
        x + 1
    };
    println!("The value of z is: {z}");

    // 函数调用
    let y = five();
    println!("The value of y is: {y}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// 定义函数
fn another_function() {
    println!("Another function.");
}

// 定义函数，带参数
fn another_function_with_param(x: i32) {
    println!("The value of x is: {x}");
}

// 定义函数，带多个参数
fn another_function_with_multiple_params(x: i32, y: i32) {
    println!("The value of x is: {x}, the value of y is: {y}");
}

// 定义函数，返回值
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
