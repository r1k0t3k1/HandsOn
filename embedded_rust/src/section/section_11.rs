//所有権のポイント
//・全ての値は、常に唯一の所有権を持つ所有者が存在する
//・所有者がスコープを抜けると、所有していた値は破棄され、値にアクセスできなくなる
//・コピーするかムーブするかで振る舞いが異なる

pub fn ownership(){
    //変数xはまだ宣言されていないので、使用できない
    {
        //このスコープ内でxは使用できる
        let x = 42;
        println!("{}", x);
    }
    //スコープを抜けているため、変数xは使用できない
    //println!({}, x); これはコンパイルエラー
}

pub fn copy(){
    //i32のようなプリミティブ型は変数の束縛時に値のコピーが作られる
    //下記の場合、xは元の値の所有権を持ち、yはコピーされた値の所有権を持つ
    let x = 42;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);
}

pub struct Sensor {
    active: bool,
    latest: u32,
}

impl Sensor {
  fn new() -> Sensor {
    Sensor{ active: false, latest: 0, }
  }
}

pub fn _move() {
    let x = Sensor::new();
    let y = x;
    //println!("x.latest = {}", x.latest); //xはyにムーブされているため値を参照できない
    println!("y.latest = {}", y.latest);
}

struct SensorFusion {
    temperature: Sensor,
    light: Sensor,
}

pub fn struct_ownership() {
    let sensors = SensorFusion {
        temperature: Sensor::new(),
        light: Sensor::new(),
    };
    //sensorsはlightの所有権を保有している
    println!("sensors.light.latest = {}", sensors.light.latest);

    //lightの所有権をムーブする
    let _l = sensors.light;
    //コンパイルエラー sensorsはlightの所有権を既に放棄している
    //println!("light sensor latest = {}", sensors.light.latest);
}

pub fn inconvinient_example() {
    let sensor = Sensor::new();
    use_sensor(sensor);
    
    //関数にsensorを渡した後もsensorを使いたいがコンパイルエラー
    //println!("latest = {}", sensor.latest);
}

fn use_sensor (s: Sensor) {
    //sensorを使って何かをする
    println!("latest = {}", s.latest);
}//ここで変数sがスコープを抜ける

pub fn bollow_example() {
    let sensor = Sensor::new();
    //sensorの参照を貸し出す
    bollow_sensor(&sensor);
    //参照は貸出した後も使用できる
    println!("latest = {}", sensor.latest);

    let mut mut_sensor = Sensor::new();

    println!("active = {}", mut_sensor.active);
    mut_bollow_sensor(&mut mut_sensor);
    println!("active = {}", mut_sensor.active);
}

fn bollow_sensor(s: &Sensor) {
    //sensorの参照を使って処理する
    println!("latest = {}", s.latest);
}

fn mut_bollow_sensor(s: &mut Sensor){
    //可変参照を受け取っているので値を書き換えられる
    s.active = true;
}

fn lifetime() {
    //値の所有者(x)は参照(rx)よりも生存期間が長くなければならない
    //let rx;
    {
        let x = 42;
        //rx = &x; 
    }//ここでxは破棄される

    //破棄されたxを参照しようとする
    //println!("rx = {}", rx);
}


//参照を含む構造体
//'aはライフタイムパラメータ
//多くの場合ライフタイムパラメータはコンパイラが推論するため書く必要が無い場合が多い
//構造体が参照をフィールドとして持つときすべての参照にライフタイムパラメータを記述する
struct Image<'a> { 
    //rawは'aのライフタイムを持っており、これはImage<'a>のスコープはrawが参照するオブジェクトのライフタイムより短いことを意味する
    //ライフタイム比較 rawの参照先 > Image<'a> 
    raw: &'a [u8; 256]
}









