// mod stack_heap;
// mod vars;
// mod ownership;
// mod generics;
// mod lifetime;
// mod enums;
// mod structs;
// mod traits;
// mod err_handling;
mod unit_test;

fn main() {
    // println!("Hello, world!");
    // vars::run();
    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    // err_handling::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();

    use std::io;
    /*
    rand crateを使って、予想する数字を生成
    Rng Traitは乱数生成器が実装すべきメソッドを定義しており、それらのメソッドを使用するには、このトレイトがスコープ内になければなりません。
    */
    use rand::Rng;
    use std::cmp::Ordering;

    println!("Guess the number!");

    // 1~101内の乱数を生成
    let secret_number = rand::thread_rng().gen_range(1..101);

    // loopキーワードで無限ループを作成し、ユーザが数字を予想する機会を増やす
    loop {
        println!("Please input your guess.");

        // ミュータブル変数を定義し、String型の新しいインスタンスを返す
        let mut guess = String::new();

        // ioモジュールのstdin関数を呼び出して、ユーザ入力を処理できるようにする
        io::stdin()
            .read_line(&mut guess) // ユーザーからの入力を取得
            .expect("行の読み込み失敗"); // Result型で失敗の可能性を扱う（ないとwarning出る）

        /*
        guessの前の値を新しい値で覆い隠す（シャドーイング）
        型を変更したい時によく使用するらしい
        上のguess変数の文字列の先頭と末尾の空白をすべて削除する。
        なぜやるかというと、たとえばユーザが5と入力してEnterキーを押すと、guessは5\nになるため
        符号なしの32ビット数値のu32へ、parseする
        match式で、数値型が入力されればok、それ以外がくればerrにいき、loopを続ける
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数値型の入力を求む！");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // match式でguessとsecret_numberを比較し、matchしたらループを抜ける
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
