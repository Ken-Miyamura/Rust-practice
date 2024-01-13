/*
 * enumの各要素にデータ型を持たせるが、Nodeのデータ型をListにしていることで、領域が定まってないListはコンパイル時にエラーを吐く
 * それをBox pointerを用いて、サイズを確定させ、コンパイルが通るようにする
 */
enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    /*
     * Stack overflow
     * rustのstackの上限は、8メガbyteなので、それを超えてStackにデータ格納しようとするとStack Overflowエラーが起こる
     */
    // let a1: [u8; 9000000] = [1; 9000000];
    // println!("{:?}", a1);

    /*
     * Vector型
     * 配列の要素を動的に変更したい場合に使う型
     * メモリ構造はstr型と同じ。違うのは、capが要素数なことだけ
     */
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("stack address of v1 is: {:p}", &v1);
    println!("stack address of v2 is: {:p}", &v2);
    println!("Heap memory address of v1: {:p}", v1.as_ptr());
    println!("len of v1 is: {}", v1.len());
    println!("capacity of v1 is: {}", v1.capacity());
    // 1の後に10を追加
    v1.insert(1, 10);
    println!("{:?}", v1);
    // 要素を削除
    v1.remove(0);
    println!("{:?}", v1);
    // v3をv1に連結（v3の中身は空になる）
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    /*
     * box pointer
     */
    let t1 = (10, String::from("hello"));
    println!("Stack Address of t1 is: {:p}", &t1);
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr());
    println!("len of t1 is: {:?}", t1.1.len());
    println!("capacity of t1 is: {:?}", t1.1.capacity());
    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!("{} {}", b1.0, b1.1);
    println!("{} {}", b1.0, b1.1);
    println!("Stack Address of box pointer b1 is: {:p}", &b1);
    println!("Heap memory address of tbox pointer b1: {:p}", b1);
}
