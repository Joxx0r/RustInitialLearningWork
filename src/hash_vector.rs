

use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 2, 3, 4];


    

    match v2.get(100)
    {
        Some(_) => println!("Found vaid value!"),
        None => println!("Invalid value"),
    }

    let mut index :i32 = 0;
    for i in &mut v {
        index += 1;
        *i += 4 * index;
        println!("Value in v: {}", i);
    }

    let mut hash_test  = HashMap::new();
    hash_test.insert(String::from("test_01"), 5);
    hash_test.insert(String::from("test_02"), 3);
    println!("{:?}", hash_test);
}