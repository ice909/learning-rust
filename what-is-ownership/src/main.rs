fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{s1}, world!"); // s1 被移动了，不能再使用
    println!("{s2}, world!"); // s2 可以使用

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里， 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，但 i32 是 Copy 的， 所以在后面可继续使用 x

    println!("{x}"); // x 可以使用
                     // println!("{s}");            // s 不可用

    let _s3 = gives_ownership(); // gives_ownership 将返回值
                                // 转移给 s3

    let s4 = String::from("hello"); // s4 进入作用域

    let s5 = takes_and_gives_back(s4); // s4 被移动到
                                       // takes_and_gives_back 中，
                                       // 它也将返回值移给 s5
    println!("{s5}"); // s5 可以使用
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string，并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}
