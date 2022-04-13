

trait Time{
    fn show_time(&self) -> u8;
}

enum TrafficColor {
    Red,
    Green,
    Yellow
}

impl Time for TrafficColor{
    fn show_time(&self)->u8{
        match self {
            TrafficColor::Green => 90,
            TrafficColor::Red =>  20,
            TrafficColor::Yellow => 3
        }
    }
}

pub fn my_light_print() {
    println!("红灯：{}",TrafficColor::Red.show_time());
    println!("绿灯：{}",TrafficColor::Green.show_time());
    println!("黄灯：{}",TrafficColor::Yellow.show_time());
}
