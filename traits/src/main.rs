pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: u64,
    pub retweet: u64,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}



// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T>(item: &T)
where T: Summary 
{
    println!("Breaking news! {}", item.summarize());
}


fn main() {
    let tweet = Tweet {
        username: String::from("@horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: 0,
        retweet: 0,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("Victor Phos"),
        headline: String::from("Govt release UFO files"),
        content: String::from("The government has released a massive collection of UFO files"),
    };

    println!("1 new article: {}", article.summarize());

    notify(&tweet);
    notify(&article);
    
}
