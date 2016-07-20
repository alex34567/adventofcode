extern crate crypto;
use crypto::md5;
use crypto::digest::Digest;

fn main() {
    let key = "ckczppom";
    let mut padding = 1;
    let mut md5 = md5::Md5::new();
    loop {
        md5.reset();
        let input = key.to_string() + &padding.to_string();
        let input_bytes = input.into_bytes();
        let mut output: [u8; 16] = [0; 16];
        md5.input(&input_bytes);
        md5.result(&mut output);
        if output[0] == 0 && output[1] == 0 && output[2] == 0 {
            break;
        }
        padding += 1;
    }
    println!("{}", padding);
}
