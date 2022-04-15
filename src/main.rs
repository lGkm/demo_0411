//pub mod trait_TrafficLight;
//pub mod sum_add_check;
pub mod print_area;




fn main() {
    // 灯
    // trait_TrafficLight::my_light_print();
    //求和
    // sum_add_check::my_println();
    // 三角形
    let triangle_a= print_area::Triangle{
        high:4.0,
        buttom:3.0
    };
    print_area::print_area(triangle_a);

    // 圆
    let triangle_b= print_area::Circular{
         diameter:10.0
    };
    print_area::print_area(triangle_b);

    // 正方形
    let triangle_c= print_area::Square{
        diameter:10.0
    };
    print_area::print_area(triangle_c);


}








