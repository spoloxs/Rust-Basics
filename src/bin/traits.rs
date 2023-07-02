// Traits allow us to define methods that can be shared with multiple classes

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}

impl Summary for Tweet{
    fn summarize(self) -> String {
        format!("")
    }
}

pub trait Summary {
    fn summarize(self) -> String; // We are defining only the struct of method and not the body
}

fn main(){
    let tweet = 
        NewsArticle{author: String::from("Chintu"),
        headline: String::from("Elon Musk is Alien"),
        content: String::from("Nothing") 
    };
    println!("{}", tweet.summarize())
}
