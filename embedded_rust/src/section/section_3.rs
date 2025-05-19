pub fn imutable_valiable() {
    let x = 42; //不変変数の宣言
    println!("x = {}", x);

}

pub fn mutable_valiable(){
    let mut x  = 42; //可変変数の宣言
    println!("x = {}", x);
    x = 24;
    println!("x = {}", x);
}

const FOURTY_TWO: i32 = 42; //定数の宣言
//定数はコンパイル時に値が束縛されている
//定数は型を明示する必要がある
//定数の名前は全て大文字にするのが慣習
pub fn constant_valiable() {
    println!("FOURTY_TWO = {}", FOURTY_TWO);
}

static mut X: i32 = 42; //静的変数の宣言
//静的変数はコンパイル時にメモリアドレスが決定される
//可変なグローバル変数のアクセスは安全ではないためunsafeブロックでくくる必要がある
//静的変数の名前も全て大文字とするのが慣習
pub fn static_valiable() {
    unsafe {
        X += 1;
        println!("X = {}", X);
    }
}
