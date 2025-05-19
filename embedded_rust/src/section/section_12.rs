//トレイトとジェネリクス

//ジェネリクスを使用しない構造体定義
//struct Point {
//    x: i32,
//    y: i32,
//}
//ジェネリクスを使用した構造体定義
struct Point<T> {
    x: T,
    y: T,
}

pub fn generics() {
    let p_i32 = Point::<i32>{ x: 300, y: 400 };
    let p_i8 = Point::<i8>{ x: 10, y: 20 };

    //コンパイラが型推論可能であれば型は省略可能
    let p_f632 = Point{ x: 30.5, y: 123.8 };

    use std::mem::size_of_val;

    println!("size of x in p_i32 = {}", size_of_val(&p_i32));
    println!("size of x in p_i8 = {}", size_of_val(&p_i8));

}

//ジェネリックを使用したメソッドや関連関数
impl<T> Point<T> {
    //関連関数
    fn new(x: T, y: T) -> Point<T> {
        Point{ x, y }
    } 
    //メソッド
    fn x(&self) -> &T {
        &self.x
    }
}


//トレイト(インターフェース)
trait Sensor {
    fn read(&self) -> u32;

    //トレイトメソッドのデフォルト実装
    //オーバーライドされない限りこれが呼ばれる
    fn fill(&self, buffer: &mut [u32]) {
        for element in buffer.iter_mut(){
            *element = self.read();
        }
    }
}

//トレイトを実装する構造体
struct LightSensor{
    value: u32,
}
struct TemperatureSensor{
    value: u32,
}


//トレイトの実装
impl Sensor for LightSensor {
    fn read(&self) -> u32 {
        self.value
    }
}
impl Sensor for TemperatureSensor {
    fn read(&self) -> u32 {
        self.value
    }
}

fn read_light_sensor() {
    let light_sensor = LightSensor { value: 100 };
    //トレイトのreadを呼び出し
     println!("light_sensor.value = {}", light_sensor.read());
}


//トレイトを関数パラメータとして使用する
fn print_sensor_value(sensor: impl Sensor) {
    println!("sensor value = {}", sensor.read());
}
//『impl trait_name』を使う実装の書き方
// fn print_sensors(a: imple Sensor, b: imple Sensor){}
//トレイト境界を使う実装の書き方
// fn print_sensors<S: Sensor>(a: S, b: S){}

// 複数のトレイトの実装を強制する書き方
// fn print_sensors(sensor: impl Sensor + Debug){}
// fn print_sensors<S:Sensor + Debug>(sensor: s){}

// where句を使った実装
// fn print_sensors<S, T>(sensor: S, t: T)
//  where S: Sensor + Debug,
//        T: Debug + Clone,
// {}
fn call_trait_read(){
    let light_sensor = LightSensor { value: 42 };
    let temperature_sensor = TemperatureSensor { value: 20 };

    print_sensor_value(light_sensor);
    print_sensor_value(temperature_sensor);
}




