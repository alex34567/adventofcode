extern crate crypto;
use crypto::md5;
use crypto::digest::Digest;

fn main() {
    let key = "ckczppom";
    let mut padding = 1;
    let mut five_zero_found = false;
    let mut md5 = md5::Md5::new();
    loop {
        md5.reset();
        let input = key.to_string() + &padding.to_string();
        let input_bytes = input.into_bytes();
        let mut output: [u8; 16] = [0; 16];
        md5.input(&input_bytes);
        md5.result(&mut output);
        if output[0] == 0 && output[1] == 0 && output[2] & 0b11110000 == 0  && !five_zero_found {
            println!("The padding that makes {} have 5 zeros is {}", key, padding);
            five_zero_found = true;
        }
        if output[0] == 0 && output[1] == 0 && output[2] == 0 {
            break;
        }
        padding += 1;
    }
    println!("The padding that makes {} have 6 zeros is {}", key, padding);
}
