pub fn comment(){
  //これは行コメント
  let _x = 42; //コードの終端にもコメントを記述できる
}

//ドキュメンテーションコメント
//markdown記法で記述できる

///Doubles the number given.
///
///# Examples
///```
///let result = docs::double(5);
///println!("result = {}", result);
///```
pub fn documentation(number:u32) -> u32{
  number * 2
}
