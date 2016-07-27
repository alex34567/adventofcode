fn look_say(input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut last_char = input[0];
    let mut char_count = 1;
    for b in input.into_iter().skip(1) {
        if b == last_char {
            char_count += 1;
        } else {
            for x in char_count.to_string().into_bytes() {
                output.push(x);
            }
            output.push(last_char);
            last_char = b;
            char_count = 1;
        }
    }
    for x in char_count.to_string().into_bytes() {
        output.push(x);
    }
    output.push(last_char);
    output
}

fn main() {
    let input_bytes = b"3113322113";
    let mut input = Vec::new();
    for x in input_bytes {
        input.push(*x);
    }
    for _ in 0..40 {
        input = look_say(input);
    }
    println!("After 40 iterations the number is {} charecters big", input.len());
    for _ in 0..10 {
        input = look_say(input);
    }
    println!("After 50 iterations the number is {} charecters big", input.len());
}
