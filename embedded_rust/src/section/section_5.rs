pub fn ifelse() {
    let x = -1;

    if x == 42 {
        println!("x is 42");
    } else if x < 0 {
        println!("x is negative");
    } else {
       println!("x is not 42 but positive");
    }
}

//rustのif文は式であり値を返却することができる
pub fn ifelse_is_expression(){
    let x = -42;
    //let文にするため、行末にセミコロンをつける
    let abs = if x < 0 { -x } else { x };
    println!("absolute value of x = {}", abs);

    //ifブロックとelseブロックでは同じ型の値を返さなければならない
    //下記のコードはコンパイルエラー
    //let err = if x < 0 { -x } else { false };
}

pub fn rust_loop() {
    //無限ループ
    //loop{}
    
    //途中でループを抜ける
    let mut count = 0;
    loop{
        count += 1;
        if count == 4 { break }
    }

    //ループの先頭に戻る
    let mut count = 0;
    loop{
        count += 1;
        //奇数の場合
        if count % 2 == 1 {
            println!("odd");
            continue;
        }             
        println!("even");

        if count == 4 {
            break;
        }
    }
}

pub fn rust_while() {
    let mut counter = 0;

    while counter < 10 {
        counter += 1;
    }

    println!("counter = {}", counter);
}

pub fn rust_for() {
    //イテレータから受け取った各要素に対して、ブロック内の処理を繰り返す
    //..演算子はstart..endという形式で記述するとstart以上、end未満のRange型オブジェクトを生成する
    //下記の場合1~4までのRangeになる
    for i in 1..5 {
        println!("i = {}", i);
    }

    //..=演算子はstart以上、end以下のRangeオブジェクトを生成する
    //下記の場合1~5までのRangeになる
    for i in 1..=5 {
        println!{"i = {}", i};
    }

    //コレクションからiter()でイテレータを生成し、
    //要素分繰り返す
    let a: [i32;4] = [1,2,3,4];
    for element in a.iter() {
        println!("a[i] = {}", element);
    }

    //イテレータを取得するメソッドで特によく使われるもの
    //iter():     要素に&Tでアクセスするイテレータを取得する(Tを参照のみしたい場合に使用する)
    //iter_mut(): 要素に&mut Tでアクセスするイテレータを取得する(Tを変更したい場合に使用する)
    //into_iter():要素にＴでアクセスするイテレータを取得する(元のオブジェクトの所有権が移動(move)する)
    //

    //bad code
    //条件式を誤ると配列の範囲外にアクセスしてしまうため
    //forを使うほうがより好ましい
    //let a= [1,2,3,4]
    //let mut i = 0;
    //while i < 4 {
    //  println!("a[i] = {}", a[i]);
    //  i += 1;
    //}
}
