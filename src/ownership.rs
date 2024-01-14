pub fn run() {
    /*
     * 所有権
     */
    let s1 = String::from("hello");
    // 所有権のムーブ
    let s2 = s1;
    println!("{}", s2);

    // 所有権のコピー（整数型がコピーtraitを実装されているので、Stack内に値がコピーされるのみでムーブは起こらない）
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address i1 is: {:p}", &i1);
    println!("Stack address i2 is: {:p}", &i2);

    // コピーのみ（これは文字列スライスなので、所有権なし。参照となる）
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address sl1 is: {:p}", &sl1);
    println!("Stack address sl2 is: {:p}", &sl2);

    // ディープコピー（heapにある他領域にデータをまるまるコピーし、新しい領域の先頭アドレスを参照。cloneメソッドで実現可能）
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address s3 is: {:p}", &s3);
    println!("Stack address s4 is: {:p}", &s4);
    println!("Heap memory address s3 is: {:?}", s3.as_ptr());
    println!("Heap memory address s4 is: {:?}", s4.as_ptr());

    let s5 = String::from("hello");
    println!("Stack address s5 is: {:p}", &s5);
    println!("Heap memory address s5 is: {:?}", s5.as_ptr());
    println!("len of s5 is: {:?}", s5.len());
    println!("capacity of s5 is: {:?}", s5.capacity());
    // この関数に所有権が移動するので、これ以降s5はアクセスできない
    take_ownership(s5);
    // println!("{}", s5);

    let s6 = String::from("hello");
    println!("Stack address s6 is: {:p}", &s6);
    println!("Heap memory address s6 is: {:?}", s6.as_ptr());
    println!("len of s6 is: {:?}", s6.len());
    println!("capacity of s6 is: {:?}", s6.capacity());

    // s7が所有権持ちになる。所有権の流れはこの場合、s6→s→s7となる
    let s7 = take_giveback_ownership(s6);
    println!("Stack address s7 is: {:p}", &s7);
    println!("Heap memory address s7 is: {:?}", s7.as_ptr());
    println!("len of s7 is: {:?}", s7.len());
    println!("capacity of s7 is: {:?}", s7.capacity());

    // immutableな参照
    let s8 = String::from("hello");
    // &を変数につけることによって、参照可能となるため、関数定義以降もs8にアクセス可能。s8を格納されてる先頭のアドレスを取得している意味している
    let len = caluculate_length(&s8);
    println!("len of {} is: {}", s8, len);

    // mutableな参照
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    // immutableな参照は複数作成可能
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;

    println!("{} {} {}", s10, r1, r2);

    // mutableとimmutableは共存できない（一人でもデータを参照してれば、値を変更できなくするため）
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let mut r2 = &mut s10;
    // println!("{} {} {}", s10, r1, r2);

    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // 所有権者（この場合s11）であっても、mutableな参照が有効な領域内ではアクセスできない。
    println!("{}", r1);
    // 参照の領域外なのでアクセス可能
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{r1} and {r2}");
    // r1とr2の参照の領域外なので、r1とr2を使用しなければ、mutableな参照を作成可能
    let r3 = &mut s12;
    // 参照はポインタの値が入っているため、参照外しをしてhelloの実データを書き換える。
    *r3 = String::from("hello_updated");
    println!("{}", r3);
}

// 引数でstring型のデータを受け取り出力する
fn take_ownership(s: String) {
    println!("{}", s);
    // stack上では違うアドレスにあるが、heapは同じ
    println!("Stack address s is: {:p}", &s);
    println!("Heap memory address s is: {:?}", s.as_ptr());
    println!("len of s is: {:?}", s.len());
    println!("capacity of s is: {:?}", s.capacity());
}

//渡されてきた所有権持ちの変数をそのままreturnする（Rustでは;つけないことでreturnとみなす）
fn take_giveback_ownership(s: String) -> String {
    s
}

// 引数でうけとった文字の長さを計算して返す。関数の引数に参照を取っているので、これは借用となる
fn caluculate_length(s: &String) -> usize {
    s.len()
}

// mutableなstrign型の参照を受け取って、_worldという文字を追加する
fn change(s: &mut String) {
    s.push_str("_world");
}
