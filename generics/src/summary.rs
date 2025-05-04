pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait 作为参数
pub fn notify2(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 上面的等价于
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 通过+ 指定多个 trait
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}