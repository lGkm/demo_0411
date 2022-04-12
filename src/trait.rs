mod  trait;

trait time{
    fn show_time(&self) -> u8;
}

enum TrafficColor {
    Red,
    Green,
    Yellow
}

impl time for TrafficColor{
    fn show_time(&self)->u8{
        match self {
            TrafficColor::Green => 90,
            TrafficColor::Red =>  20,
            TrafficColor::Yellow => 3
        }
    }
}

pub fn my_light_print() {
    println!("{}",TrafficColor::Red.show_time());
    println!("{}",TrafficColor::Green.show_time());
    println!("{}",TrafficColor::Yellow.show_time());
}
