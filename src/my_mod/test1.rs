pub fn test1() -> String {
    println!("Male: {:?}", TestCategory::Male);
    println!("Female: {:?}", TestCategory::Female);
    let p1 = Person {
        name: "测试用户".to_string(),
        gender: TestCategory::Male,
    };
    println!("{:?}", p1);
    String::from("test1 result")
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: TestCategory,
}

#[derive(Debug)]
enum TestCategory {
    Male,
    Female,
}
