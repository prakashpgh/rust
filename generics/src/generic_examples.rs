//generic to get the max
fn get_max<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut max = &list[0];
    for i in list {
        if i > max {
            max = i;
        }
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

pub fn generic_demo() {
    println!("generics demo");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let max = get_max(&numbers);
    println!("{}", max);

    let p = Point { x: 5, y: 10 };
}

/////////////traits
///
//1) traits are like interface... function declarations without impl.
//2) they can have default implementation like abstract classes in c++

pub trait Summary {
    //default implementation
    fn summarize(&self) -> String {
        String::from("default summary...")
    }
}

//pub trait Display {
//    fn display(&self) -> String {
//        String::from("default summary...")
//    }
//}

pub trait Debug {
    fn debug(&self) -> String {
        String::from("default debug...")
    }
}

pub trait Clone {
    fn clone(&self) -> String {
        String::from("default clone...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct WebPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for WebPost {}
///////

//passing trait as parameter.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//multiple traits...
pub fn notify_multiple_traits(item: &(impl Summary + Display)) {}

//traits as parameters...
pub fn notify_multiple_traits_template<T: Summary + Display>(item: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    -1
}

pub fn trait_demo() {
    let social = SocialPost {
        username: String::from("test_user"),
        content: String::from("test_content"),
        reply: false,
        repost: false,
    };
    println!("{}", social.summarize());

    //web post uses the default summary...
    let web = WebPost {
        username: String::from("test_user"),
        content: String::from("test_content"),
        reply: false,
        repost: false,
    };
    println!("web-post: {}", web.summarize());
    notify(&web);
}

//combining generics and the traits
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
