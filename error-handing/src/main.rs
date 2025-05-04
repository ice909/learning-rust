use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    //panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // 用Result处理可恢复的错误
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // 使用闭包和 unwrap_or_else
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {e:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // 失败时panic的简写unwrap和expect
    let greeting_file = File::open("hello2.txt").unwrap();

    let greeting_file = File::open("hello2.txt")
        .expect("hello.txt should be included in this project");
}

// 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 传播错误的简写，使用?运算符
// ?运算符的作用是：如果Result是Ok，返回Ok中的值并继续执行；如果Result是Err，Err 将作为整个函数的返回值
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

// 在?之后直接使用链式方法调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Option<T>上使用?运算符
// 如果text是空字符串，next 调用会返回 None
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
