use std::collections::HashMap;

pub fn demonstrate_hash_maps() {
    // 创建一个空的HashMap
    let mut scores = HashMap::new();
    // 插入数据
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 遍历HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 这里 field_name 和 field_value 不再有效，
    // println!("field_name: {}", field_name);
    // println!("field_value: {}", field_value);
    println!("map: {:?}", map);

    // 根据旧值更新值

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
