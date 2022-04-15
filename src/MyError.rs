pub(crate) struct MyError {
    code: i32,
    msg: String
}
use std::fmt::{Display,Debug,Formatter};
impl std::error::Error for MyError { }
impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]{}", self.code, self.msg)
    }
}
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]{}", self.code, self.msg)
    }
}


pub fn main() -> Result<(), MyError> {
    let oneError: Result<(), MyError>= Err(MyError{
        code: -1,
        msg: "error".to_owned(),
    });
    // 问号操作符是针对Result<T,E>类型的一个语法糖
    // 本质是一个match匹配
    // 如果Result是一个E类型的错误值，则提前返回错误，结束当前函数
    // 如果Result是一个T类型的正确值，则提取出值，方便后续进行链式调用
    oneError?;

    Ok(())
}

//fn getSum(x:Option<i32>)->Result<(), MyError>{
//    match x {
//        None => None,  //必须处理None, 不能操作，返回None
//        Some(i) => Some(i + 1), //Some变成加一的Some,仍旧是Option<T>
//    }
//}
