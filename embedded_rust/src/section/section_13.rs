fn convert(){
  let x: i32 = 42;
  // let y: i64 = x; //型変換できない　コンパイルエラー
  let y: i64 = x.into(); //into()による型変換
  let y = i64::from(x); //その他『type::from(value)』で型変換が可能
  let y = x as i64; //コンパイラが知っている範囲での安全な型変換
  println!("y = {}", y);

  //into()とfrom()メソッドによる型変換はIntoトレイト、Fromトレイトによって提供される
  //T => U への型変換『From<T> for U』を実装すると『Into<U> for T』も実装される
}

fn type_conversions_that_can_fail(){
  // let x: i64 = 42;
  // let y: i32 = x.into(); //i64=>i32のIntoトレイトは実装されていないためコンパイルエラー
  // println!("y = {}", y);

  //失敗するかもしれない型変換ではTryInto/TryFromトレイトを使用する
  //TryInto/TryFromはデフォルトでインポートされていないので明示的にインポートする
  use std::convert::TryInto;

  let x: i64 = 42;
  let y: i32 = x.try_into().unwrap(); //try_into()/try_from()の戻り値はResultなのでunwrap()で中身を取り出す
  //let y = i32::try_from(x).unwrap(); 同義
  println!("y = {}", y);
}