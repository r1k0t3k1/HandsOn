pub fn hello_world() {
  println!("Hello, world!"); //標準出力に文字を出力する
}

pub fn print_array() {
    let a: [i32; 5] = [1,2,3,4,5]; //変数aに配列を束縛する
    println!("a = {:?}",a) //配列の値を表示する
}

