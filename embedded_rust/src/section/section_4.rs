pub fn interger(){
    let x: u32 = 42; //整数型を指定して変数を宣言する場合、letで束縛するときに型を指定する
    let y = 42u32; //または整数リテラルの末尾に型を追加してもOK

    println!("x = {}, y = {}", x, y);

    //10進数 => 12345、1234_5678 桁数の多いリテラルの場合はアンダーバーで区切れる
    //16進数 => 0xff、0xFF
    //8進数  => 0o77
    //2進数  => 0b0101、0b0000_1111
    //バイト => b'A'
}

pub fn float() {
    let x = 4.24242; //f64
    let y: f32 = 2.42424; //f32
    
    //浮動小数点数をprintln!()で表示するときは
    //フォーマットを{:.2}のように書くことで小出力する小数点以下桁数を指定することができる。
    //{:.2}は小数点以下を2桁まで出力する
    println!("x = {}, y = {:.2}", x, y);
}

pub fn bool() {
    let x = true;
    let y: bool = false;

    println!("x = {}, y = {}", x, y);
}

pub fn array() {
    //配列は[要素の型;要素数]で定義する
    let array: [i32; 5] = [1,2,3,4,5];
    println!("array = {:?}", array); 
    println!("array[0] = {}", array[0]); //要素数の指定
}

pub fn slice() {
    let array = [1,2,3,4,5];
    //スライスは配列の一部の参照
    //スライス型は&[i32]と記述する
    //配列からスライスを作成するには&配列変数名[参照範囲の指定]と記述する
    let slice: &[i32] = &array[1..3]; 
    println!("slice = {:?}", slice);
    println!("slice[0] = {}", slice[0]);

}

pub fn mutable_slice() {
    let mut array = [1,2,3,4,5];
    let slice = &mut array[1..3];

    slice[0] = 6; //スライスから元の配列の値を書き換える

    println!("slice = {:?}", slice);
    println!("array = {:?}", array);
}

pub fn tuple() {
    let t: (u8, i32, usize) = (1, -42, 1_000);
    println!("u8 = {}", t.0);
    println!("i32 = {}", t.1);
    println!("usize = {}", t.2);
}

pub fn string() {
    let greeting = "hello";
    println!("{}", greeting);
}

pub fn string_to_bytearray() {
    //文字列リテラルをu8のバイト配列として変数に格納する
    let greeting = b"hello";
    //文字列ではなくバイト配列なので{}での出力はできない
    //代わりに{:?}で配列の内容を出力する
    println!("{:?}", greeting);
}



