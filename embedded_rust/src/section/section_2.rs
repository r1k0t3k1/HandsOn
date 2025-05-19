pub fn compile_error(){
    let x =42; //rustの変数はデフォルトで不変のため再代入は不可能
    println!("x = {}",x);
    //x = 24;   //ここでコンパイルエラーが発生
    println!("x = {},",x);
}
