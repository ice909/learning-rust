pub fn demonstrate_vectors() {
    // 创建一个空vector，必须指定类型
    let vec: Vec<i32> = Vec::new();

    println!("vec: {:?}", vec);

    // 会自动推导类型
    let v = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v);

    let mut v = Vec::new();
    // 更新vector的值
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    // 读取vector的值
    // 1. 使用索引
    let first = &v[0];
    println!("first: {}", first);
    // 2. 使用get方法
    match v.get(0) {
        Some(first) => println!("first: {}", first),
        None => println!("no value"),
    }
    match v.get(3) {
        Some(first) => println!("first: {}", first),
        None => println!("no value"),
    }

    // 当我们获取了 vector 的第一个元素的不可变引用
    // 并尝试在 vector 末尾增加一个元素的时候，如果尝试在函数的后面引用这个元素是行不通的
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // println!("first: {}", first);

    // 3. 遍历vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // 遍历时修改vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);
    // 4. 删除vector的值
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v.pop();
    println!("v: {:?}", v);

    // 使用枚举来存储多种类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);
}