// 接口
pub trait Area {
    fn area(&self) -> f64;
}

pub trait Factory {
    fn test() -> i32;
}

// 具体类
pub struct Circle {
    pub r: f64,
}

// 让【具体类】实现【接口】
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r // 作为返回值 => 必须使用 () 括起来，并不能写 ;
    }
}

impl Circle {
    pub fn new(r: f64) -> Circle {
        Circle { r: r }
    }
}

pub fn run() {
    let circle = Circle { r: 2.0 };
    let round = circle.area();
    println!("round is {}", round)
}
