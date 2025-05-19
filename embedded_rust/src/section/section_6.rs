pub fn function() {
  let x = double(5);
  let y: u32 = double_early_return(6);
  println!("5 * 2 = {}", x);
  println!("5 * 2 = {}", y);
}

fn double(number: u32) -> u32 {
  number * 2 //セミコロンを付けるとコンパイルエラー
  //もしくは　return number * 2;
  //関数内の最後の式が関数の戻り値となり、セミコロンを付けると
  //その式はユニット型として評価されてしまう

  //Rustでは通常、returnを用いないスタイルが好まれる
  //return を利用するのはエラー処理のEarly Returnなど処理の途中で関数から戻る場合
}

//Early Returnのユースケース
fn double_early_return(number: u32) -> u32 {
  //オーバーフロー対策
  if number > u32::MAX {
    return u32::MAX;
  }
  number * 2
}
