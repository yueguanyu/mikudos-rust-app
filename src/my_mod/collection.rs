use std::collections::HashMap;

pub fn test_collection() {
    let mut instance_name: HashMap<&str, &str> = HashMap::new();
    instance_name.insert("name", "testName");
    for (key, val) in instance_name.iter() {
        println!("elem: {} value: {}", key, val);
    }
    match instance_name.get(&"name") {
        Some(value) => {
            println!("value with {} is {}", "name", value);
        }
        None => println!("name is none"),
    }
    println!("instance:{:?}", instance_name)
}
