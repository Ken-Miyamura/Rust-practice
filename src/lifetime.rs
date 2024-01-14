pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("x");
    let res2;
    {
        // st4の方がライフタイムが短いので、st4の方がst3よりも短いとみなされる
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
}

// 二つの文字列を比較し、大きい方の参照を返してくれる関数。返り値の参照のライフタイムは、引数で受け取った2つうちの短い方を適用するようにする（ジェネリクスライフタイムアノテーション）
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ある参照だけを返す関数。実体sを生成して参照をreturnすると、sは関数を抜けた時にdropされ、参照が一人歩きしてしまい、ダングリングポインタ発生する
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("dummy");
//     &s
// }

// これも同じく
// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }

fn dummy3() -> String {
    let s = String::from("dummy");
    s
}
