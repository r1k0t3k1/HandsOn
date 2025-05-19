//attribute
//attributeはクレート、モジュール、アイテムにメタデータを付与する
//利用用途
//・条件付きコンパイル
//・Lintツールの設定
//・コンパイラ組み込み機能の利用
//・ユニットテスト関数の指定


//#[test]atrtributeを付けるとテストビルド時のみビルドされる
#[test]
fn this_is_a_test(){
  //...
}

//deriveによってDebugトレイトの標準的実装を提供
#[derive(Debug)]
struct Sensor{
  active: bool,
  latest: u32,
}

fn derive(){
  let s = Sensor{ active: false, latest: 0 };
  println!("{:?}", s); //derive(Debug)により{:?}フォーマッタが使用可能に
}

//頻出トレイト
// Debug          :{:?}フォーマッタを使用し絵t値をフォーマットするためのトレイト
// Eq/PartialEq   :オブジェクト同氏の一致比較をするためのトレイト
// Ord/PartialOrd :オブジェクト同士の大小比較をできるようにする
// Clone          :コピーによって&TからTを作成するトレイト
// Copy　　　　　 :オブジェクトを暗黙的にコピーするためのトレイト
// Default        :空っぽのインスタンスを作成するためのトレイト