use std::fmt::Display;

struct Tweet {
    user: String,
    content: String
}

impl Summary for Tweet {
    fn author_fmt(&self) -> String {
        format!("@{}", self.user)
    }
}

struct News {
    author: String,
    headline: String,
    content: String
}

impl Summary for News {
    fn author_fmt(&self) -> String {
        format!("{}", self.author)
    }
}

trait Summary {
    fn author_fmt(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.author_fmt())
    }
}

fn notify_print<T: Summary>(text_piece: &T) {
    println!("{}", text_piece.summarize());
}

fn notify_string<T>(text_piece: &T) -> String
    where T: Summary
{
    text_piece.summarize()
}


pub fn larger<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 { v1 }
    else { v2 }
}


// fn first_word<'a>(string: &'a str) -> &'a str {
fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &byte) in  bytes.iter().enumerate() {
        if byte == b' ' {
            return &string[0..i]
        }
    }

    string
}

pub fn longer<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

pub struct Pair<T, U> {
    pub item1: T,
    pub item2: U
}

impl<T, U> Pair<T, U>
where T: Display, U: Display
{
    pub fn print(&self) {
        println!("1: {} | 2: {}", self.item1, self.item2);
    }
}

impl<T, U> Pair<T, U>
where T: Display + Copy, U: Display + Copy
{
    pub fn mixup<V, W>(pair1: &Pair<T, U>, pair2: &Pair<V, W>) -> Pair<T, W> 
    where V: Display + Copy, W  : Display + Copy
    {
        Pair {
            item1: pair1.item1,
            item2: pair2.item2
        }
    }
}