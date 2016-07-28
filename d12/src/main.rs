extern crate json;
use std::fs::File;
use std::io::Read;

fn process_object(object: &json::object::Object, red: bool) -> i32 {
    if !red {
        for i in object.iter() {
            let (_, value) = i;
            match value.clone() {
                json::JsonValue::Short(short) => {
                    if &short == "red" {
                        return 0
                     }
                }
                _ => {}
            };
        }
    }
    let mut sum = 0;
    for i in object.iter() {
        let (_, value) = i;
        match value.clone() {
            json::JsonValue::Array(array) => sum += process_array(&array, red),
            json::JsonValue::Number(n) => sum += n.into(),
            json::JsonValue::Object(object) => sum += process_object(&object, red),
            _ => {}
        };
    }
    sum
}

fn process_array(array: &Vec<json::JsonValue>, red: bool) -> i32 {
    let mut sum = 0;
    for value in array {
        match value.clone() {
            json::JsonValue::Array(array) => sum += process_array(&array, red),
            json::JsonValue::Number(n) => sum += n.into(),
            json::JsonValue::Object(object) => sum += process_object(&object, red),
            _ => {}
        };
    }
    sum
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    drop(file);
    let jsoned = json::parse(&string).unwrap();
    let j_object;
    if let json::JsonValue::Object(object) = jsoned {
        j_object = object;
    } else {
        panic!("Bad input");
    }
    println!("The sum of all the numbers is {}",
             process_object(&j_object, true));
    println!("The sum of all the numbers without reds is {}",
             process_object(&j_object, false));
}
