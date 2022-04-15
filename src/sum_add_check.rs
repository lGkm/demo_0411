
// 求和
fn add_sum(nums: &[u32]) -> Option<u32> {
    // 默认0
    let mut sum: Option<u32> = Some(0);
    for a in nums.iter() {
        //
        sum = a.checked_add(match sum {
            Some(i) => i,
            None => { return  None },
        });
    }
    return  sum;
}


pub fn my_println(){

    // 最大值
    let arrays_check: [u32; 5] = [u32::MAX, 2, 3, 4, 5];
    // 正常数据
    let arrays_new: [u32; 5] = [222, 2, 3, 4, 5];

    println!("sum: {:?}", add_sum(&arrays_check));
    println!("sum: {:?}", add_sum(&arrays_new));

}