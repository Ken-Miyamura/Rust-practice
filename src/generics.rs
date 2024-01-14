// rustでは構造体はstructで定義できる。genericsでどんな型の構造体も作ることが可能
struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

// 構造体にメソッド追加可能（classにメソッド追加するのと似たイメージ）
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    /*
     * ジェネリクス
     */
    // number_listの一番大きい値を取得する処理を書く
    let number_list = vec![34, 58, 25, 100, 65];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is: {largest}");
    // println!("The largest is: {}", largest_i32(number_list));
    println!("The largest is: {}", largest(number_list));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("The char is: {}", largest(char_list));

    // 構造体のインスタンスを生成している
    let p1 = Point { x: 1, y: 2 };
    let p2 = PointAnother { x: 1.0, y: 2 };
    let p3 = PointAnother { x: "Rust", y: 'a' };
    let p4: PointAnother<f64, _> = p2.mixup(p3);
    println!("The p4 is: {} {}", p4.x, p4.y);
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Tの比較が可能なデータ型をgenericsとして許容することができる（Trait境界）
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
