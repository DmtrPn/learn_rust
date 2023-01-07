// Нужно импортить в том числе и типаж Summary !!!
use sandbox::aggregator::{Summary, Tweet, NewsArticle};
use core::fmt::Debug;
use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    notify(&tweet);
    notify(&article);
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Это эквивалентно этому:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// Если нужно несколько типажей, то объединяем их через +
// pub fn notify(item: &(impl Summary + Display)) {

// Обобщение типов ключевым словом  where
#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 34 }

#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    #[allow(dead_code)]
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    #[allow(dead_code)]
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

