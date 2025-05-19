//Option<T>は戻り値がT、またはNoneを返す
fn double(number: u32) -> Option<u32>{
    //引数の値を二倍する前に二倍した値がu32の最大値を超えないかチェック
    if number > (u32::MAX / 2) {
        return None;
    }
    Some(number * 2)
}

//Resultを返す関数の書き方
//エラーを返す場合はErr(Errorオブジェクト)
//値を返す場合はOk(オブジェクト)
fn double_return_result(number:u32) -> Result<u32,Error> {
    if number == 0 {
        Err(Error::Zero)
    } else if number > (u32::MAX / 2) {
        Err(Error::Overflow)
    } else {
        Ok(number * 2)
    }
}

pub fn use_option() {
    //Option<T>からTを取り出すにはunwrapを使用する
    //ただし、Option<T>がNoneを返す場合unwrapはpanicを起こす
    let x = double(10).unwrap();
    //let x = double(u32::MAX).unwrap(); この場合はpanic
    println!("x = {}", x);

    //Option<T>はSomeとNone、二つのvariantからなる列挙型のため
    //match式でパターンマッチができる
    match double(5) {
        Some(x) => println!("double(5) = {}", x),
        None => println!("double failed"),
    }

    //if letで値が格納されている時のみ特定の処理を実行することができる
    if let Some(x) = double(3) {
        println!("double(3) = {}", x);
    }
    if let Some(x) = double(u32::MAX) {
        //この処理は実行されない
        println!("double(u32::MAX) = {}", x);
    }
}

#[derive(Debug)]
pub enum Error {
    Overflow,
    Zero,
}
//Result<T,E>は返り値かエラーのどちらかが格納されているオブジェクト
pub fn use_result() {
   let x = double_return_result(32).unwrap();
   //let x = double_return_result(u32::MAX).unwrap(); unwrapをErrに対して使用するとpanicが発生
   println!("x = {}", x);

   //Resultと同じくmatchでのパターンマッチやif letでの処理が可能
   match double_return_result(10) {
      Ok(x) => println!("10 * 2 = {}",x),
      Err(e) => println!("doubled failed: {:?}",e),
   }

   if let Ok(x) = double_return_result(u32::MAX) {
       //この処理は実行されない
        println!("x = {}", x)
   }
}


pub fn error_raise() -> Result<u32,Error> {
    //エラーが発生した場合呼び出し元にerrorを返却
    let doubled = double_return_result(u32::MAX)?;
    //エラーが発生しなかった場合の処理
    Ok(doubled)
}
