pub fn demonstrate_strings() {
    // 创建一个空字符串
    let mut s = String::new();
    println!("s: {}", s);

    // 使用字符串字面量创建字符串
    let s = "hello".to_string();
    println!("s: {}", s);

    // 使用宏创建字符串
    let s = String::from("hello");
    println!("s: {}", s);

    // 连接字符串
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("s3: {}", s3);

    // 更新字符串
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("s: {}", s);
    // push_str方法只能添加字符串切片
    // push方法可以添加单个字符
    let mut s = String::from("hello");
    let s2 = " world";
    s.push_str(s2);
    println!("s: {}", s);
    println!("s2: {}", s2);
    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);

    // 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("hello: {}", hello);

    // 使用+或format!宏连接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s: {}", s);
}