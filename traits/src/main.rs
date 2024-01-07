use std::fmt::Debug;

fn main() {
    let article = NewArticle {
        author: String::from("John Doe"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."),
        headline: String::from("Lorem ipsum"),
    };
    

    let tweet = Tweet {
        username: String::from("John Doe"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."),
        reply: false,
        retweet: false,
    };

    let like = Like {
        username: String::from("John Doe"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."),
    };


    // calling the summarize method on the article instance
    article.summarize();
    println!();


    // calling the summarize method on the tweet instance
    tweet.summarize();
    println!();

    // uses the default implementation of summarize
    like.summarize();
    println!();



    // calling the notify function with the article instance
    notify(&article);
    println!();

    // calling the generic notify function with the tweet instance
    generic_notify(&tweet);
    println!();


    // calling the returns_summarizable function
    let tweet = returns_summarizable();
    tweet.summarize();

}



#[derive(Debug)]
struct Like {
    username: String,
    content: String,
}

impl Summary for Like {}


#[derive(Debug)]
struct NewArticle {
    author: String,
    content: String,
    headline: String,
}

impl Summary for NewArticle {
    // overrides the default implementation of summarize
    fn summarize(&self)  {
        println!("New article: {}", self.headline);
        println!("Author: {}", self.author);
        println!("Content: {}", self.content);
    }
}


#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    // overrides the default implementation of summarize
    fn summarize(&self) {
        println!("New tweet from {}", self.username);
        println!("Content: {}", self.content);
        println!("Reply: {}", self.reply);
        println!("Retweet: {}", self.retweet);
    }
}


trait Summary {
    // the default implementation of summarize
    fn summarize(&self) {
        println!("(Read more...)");
    }
}


// creating a function that takes a type that implements the Summary trait
fn notify(item: &impl Summary) {
    println!("Breaking news!");
    item.summarize();
}

fn generic_notify<T: Summary + Debug>(item: &T) {
    println!("Breaking news!");
    item.summarize();
}

// function that returns a type that implements the Summary trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("John Doe"),
        content: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."),
        reply: false,
        retweet: false,
    }
}
