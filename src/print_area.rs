pub trait Area{
    fn calculated_area(&self) -> f64;
}

pub struct Triangle {
    pub high: f64,
    pub buttom: f64
}


impl Area for Triangle{
    fn calculated_area(&self) -> f64 {
        (self.high*self.buttom)/2.0
    }
}

pub struct Circular {
    pub diameter:f64
}


impl Area for Circular{
    fn calculated_area(&self) -> f64 {
        self.diameter*self.diameter*std::f64::consts::PI/4.0
    }
}

pub struct Square {
    pub diameter:f64
}


impl Area for Square{
    fn calculated_area(&self) -> f64 {
        self.diameter*self.diameter
    }
}

pub fn print_area <T:Area>(t:T){
    println!("图形面积为：{}",t.calculated_area());
}