#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangleの新しいインスタンスを作成
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // 対象のインスタンスが持ってる幅と高さをかけあわせて面積計算メソッド。
    // 引数にselfを受け取った場合は対象のインスタンスに対しメソッドとして呼び出せる
    // selfの先頭に＆つけて、インスタンスの参照を受け取るようにしてる（つけない場合、areaが呼ばれるインスタンスに引数に渡された時に所有権がムーブして元のインスタンスが使えなくなる）
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        username: String::from("Ken"),
        email: String::from("someone@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    // 構造体の所有権のムーブ
    let mut user2 = user1;
    user2.email = String::from("someone_another@gmail.com");
    println!("{:#?}", user2);

    let user3 = build_user(
        String::from("someone_3@gmail.com"),
        String::from("Miyamura"),
    );
    println!("{:#?}", user3);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    // areaが呼ばれた後でもrectのインスタンスにアクセスできるのは、引数を参照にしてるため
    println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
