use my_mod::traits::Area;

mod my_mod;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}

fn main() {
    // let random_boolean = rand::random();
    // println!("You {}!", if random_boolean { "win" } else { "lose" });
    // let r = my_mod::test1::test1();
    // println!("r: {}", r.len());
    // let rect = Rectangle::create(30, 50);
    // println!("{:?}", rect);
    // my_mod::collection::test_collection();
    // let circle = my_mod::traits::Circle { r: 3.0 };
    let circle = my_mod::traits::Circle::new(3.0);
    let round = circle.area();
    println!("round11 is {}", round);
    // my_mod::traits::run();
}
