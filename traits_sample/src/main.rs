fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                                hockey team in the NHL.",
        ),
        author: String::from("Iceburgh"),
    };

    let blog_post = BlogPost {
        title: String::from("My first blog post"),
        content: String::from("This is my first blog post"),
        author: String::from("John Doe"),
    };

    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());
    println!("New Blob Post: {}", blog_post.summarize());

    notify(&article);

    notify2(&blog_post);

    notify3(&article, &tweet);

    notify4(&blog_post, &tweet);
}

struct NewsArticle {
    headline: String,
    content: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} , by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} , by {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct BlogPost {
    title: String,
    content: String,
    author: String,
}

impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more...from{})", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}
