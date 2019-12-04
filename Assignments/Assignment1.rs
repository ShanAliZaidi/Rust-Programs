struct NewsArticle {
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

trait Summary {
    fn summarize(&self) ;
}

impl Summary for NewsArticle {
    fn summarize(&self) {
        println!("{} says, \n {}",self.author, self.content )
    }
}

impl Summary for Tweet {
    fn summarize(&self)  {
        println!("{} tweets, \n {}",self.username, self.content )
    }
}

fn main() {

    let news = NewsArticle {
        author: String::from("Mr A"),
        content: String::from("More...."),
    };

    let msg = Tweet {
        username: String::from("Someone"),
        content: String::from("More...."),
    };

    news.summarize();
    msg.summarize();
}