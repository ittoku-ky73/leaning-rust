use traits::{self, Summary, NewsArticle, Tweet, notify, largest, Pair};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
        ),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", news_article.summarize());
    
    notify(&tweet);
    notify(&news_article);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'l'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let pair = Pair::new(10, 20);
    pair.cmp_display();
}
