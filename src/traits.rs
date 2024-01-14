trait Fruits {
    fn price(&self) -> u32;
}
trait Summary {
    fn summarize(&self) -> String;
}
trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct Apple;
struct Banana;
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Message for NewsArticle {
    // 元のtraitで具体処理してるので、これはoverrideということ
    // fn message(&self) -> String {
    //     String::from("再Message")
    // }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("ken"),
        content: String::from("いい感じに捗っている"),
        reply: false,
        retweet: false,
    };
    println!("new tweet {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("大谷翔平緊急帰国"),
        location: String::from("Japan"),
        author: String::from("Rosen"),
        content: String::from("パパラッチも焦る"),
    };
    println!("new article {}", article.summarize());

    notify(&article);
    notify_another(&article);
}

// trait境界によって、Fruitsのtraitを持つデータ型のみに限定
fn get_price<T: Fruits>(fruits: T) {
    println!("price is {}", fruits.price());
}

// サマリーtraitを実装してるデータ型であれば、itemに渡せる
fn notify(item: &impl Summary) {
    println!("Breaking News! : {}", item.summarize());
}

fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking News! : {}", item.summarize());
    println!("Message {}", item.message());
}
