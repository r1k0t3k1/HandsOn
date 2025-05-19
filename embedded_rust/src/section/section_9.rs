//列挙型の定義
//rustの列挙型は各variantが異なる型の値を持つことができる
pub enum Type {
    Int,
    Float,
    Boolean,
}

pub enum  DefType{
    Int(i64),
    Float(f64),
    Boolean(bool),
}

//列挙型のオブジェクト
//let int = Type::Int;
//
//variantが値を持つ場合、値を渡してオブジェクトを生成する
//let int = Type::Int(42);

//列挙型にもメソッドと関連関数を定義できる
impl Type {
    fn method(&self){
        //メソッドの定義
    }

    fn associated_func() {
        //関連関数の定義
    }
}

//列挙型はmatch演算子を使用してパターンマッチができる
pub fn print_type(t: Type){
    match t {
        Type::Int => println!("type is integer"),
        Type::Float => println!("type is Float"),
        Type::Boolean => println!("type is Boolean"),
    }
}

//値を保持している列挙型に対しては値を含めたパターンの比較が可能
//パターンが一致した場合、値を束縛できる
pub fn print_type_bind(t: DefType){
    match t {
        DefType::Int(i) => println!("integer value = {}",i),
        DefType::Float(f) => println!("float value = {}",f),
        DefType::Boolean(b) => println!("boolean value = {}",b),

    }
}
