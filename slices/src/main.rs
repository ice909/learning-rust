// 编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。
// 如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
fn first_word(s: &str) -> usize {
    // 转换为字节数组
    let bytes = s.as_bytes();

    // 遍历字节数组，查找第一个空格
    // 如果找到空格，返回空格的索引
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    // 如果没有找到空格，返回字符串的长度
    s.len()
}

fn main() {
    let s = "hello world";
    // 获取第一个单词的长度
    let len = first_word(s);
    // 截取第一个单词
    // 字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：
    let first_word = &s[..len];
    println!("The first word is: {}", first_word);
    // slice截取全部字符串
    let all_word = &s[..];
    println!("The all word is: {}", all_word);
    // slice截取字符串的中间几个字符
    let mid_word = &s[1..4];
    println!("The mid word is: {}", mid_word);

}
