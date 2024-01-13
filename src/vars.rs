const MAX_POINTS: u32 = 100000;

pub fn run() {
    let mut x = 5;
    println!("the value is {}", x);
    x = 6;
    println!("the value is {x}");

    println!("{}", usize::BITS);

    // MAX_POINTSの値が格納されてるメモリのアドレスの値
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("Stack Address of i2 is: {:p}", &i2);
    println!("Stack Address of i3 is: {:p}", &i3);

    /*
     * シャドーイング
     * ・シャドーイングされた変数は、違うアドレスに格納されている（Stack上の新しい領域に格納されて）
     */
    let y = 5;
    println!("Stack Address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack Address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack Address of y is: {:p}", &y);
    // 同じスコープ内でシャドーイング可能
    {
        let y = 0;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    /*
     * タプル型
     */
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {:?}", t1);

    let mut t2 = ((0, 1), (2, 3));
    // ポインタの値も同時に取得（0, 1のみ取得）
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    println!("The value of t2 is: {} {}", x1_ptr, y1_ptr);
    // 0だけ更新したい場合(参照外し)
    *x1_ptr = 5;
    *y1_ptr = 6;
    println!("The value of t2 is: {} {}", x1_ptr, y1_ptr);
    println!("The value of t2 is: {:?}", t2);

    /*
     * 配列について
     * ・　Rustでは配列のサイズと要素数を変更不可。コンパイル時に決定している必要あり。値は必ずStackに積まれる
     */
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {}", a1, a2, a1[2]);
}
