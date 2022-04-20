use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("rust is cool"),
        location: String::from("madrid"),
        author: String::from("sarah"),
        content: String::from("Everybody knows that rust is cool!"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", news_article.summarize());
}
