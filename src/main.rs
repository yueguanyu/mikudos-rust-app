mod test {
    include!("../proto/greeter/test.rs");
}

fn main() {
    let random_boolean = rand::random();
    println!("You {}!", if random_boolean { "win" } else { "lose" });
    crate::test::test();
}
