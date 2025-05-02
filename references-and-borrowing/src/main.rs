fn main() {
    let s1 = String::from("hello");
    // 引用，允许我们使用变量而不获取所有权， 也叫借用，不能修改
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // 可变引用，允许我们修改变量
    let mut s2 = String::from("hello");
    let s3 = &mut s2;
    // let s4 = &mut s2;// 不能同时有两个可变引用
    

    println!("{}, {}", s3, s4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
