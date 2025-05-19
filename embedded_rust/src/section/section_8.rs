pub struct Sensor {
  //センサが有効化されていれば、true 
  active: bool,
  //センサの最新の値を持っている
  latest: u32
}

pub fn struct_practice(){
  //フィールドに値を与えて構造体オブジェクトを生成する
  let sensor = Sensor {
    active: false,
    latest: 0,
  };
  //フィールドにアクセスする
  println!("active = {}, latest = {}", sensor.active,sensor.latest);

  //可変な構造体を生成
  let mut mut_sensor:Sensor = Sensor { active: false, latest: 1 };
  //フィールドの変更
  mut_sensor.active = true;
  println!("active = {}, latest = {}", mut_sensor.active,mut_sensor.latest);
}

pub fn implementation() {
  let mut sensor: Sensor = Sensor {
    active: true,
    latest: 0,
  };
  sensor.init();
  println!("read() = {}, latest = {}", sensor.read(),sensor.latest);
}

pub fn related_function(){
    let sensor = Sensor::new();
    let latest = sensor.read();
    println!("latest = {}", latest);
}

//構造体のメソッドと関連関数を定義する
impl Sensor {
  //メソッドを定義する場合はメソッドの第一引数にself or &
  //オブジェクトの値を変更しない場合、&selfを引数に取る
  fn read(&self) -> u32 {
    self.latest
  }
  //オブジェクトの値を変更する場合、&mut selfを引数に取る
  fn init(&mut self){
    self.active = true;
    self.latest = 42;
  }


  //関連関数
  //引数でselfを受け取らない構造体の関数
  //主にコンストラクタで使用する
  fn new() -> Sensor {
    Sensor{ active: false, latest: 0, }
  }
}
