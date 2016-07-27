use std::str;

fn new_password(password: &mut [u8]) {
    for c in password.into_iter().rev() {
        *c += 1;
        if *c > b'z' {
            *c = b'a';
            continue;
        }
        break;
    }
}

fn get_password(mut password: [u8; 8]) -> [u8; 8] {
    loop {
        let mut increasing_letters = false;
        for x in password.iter()
            .zip(password.iter().skip(1))
            .zip(password.iter().skip(2)) {
            let ((char1, char2), char3) = x;
            if *char1 + 1 == *char2 && *char1 + 2 == *char3 {
                increasing_letters = true;
            }
        }
        let mut bad_char = false;
        for c in &password {
            if *c == b'i' || *c == b'o' || *c == b'l' {
                bad_char = true;
            }
        }
        let mut char_pairs = 0;
        let mut overlap = false;
        for x in password.iter()
            .zip(password.iter().skip(1)) {
            if overlap == true {
                overlap = false;
                continue;
            }
            let (char1, char2) = x;
            if *char1 == *char2 {
                char_pairs += 1;
                overlap = true
            }
        }
        if increasing_letters == true && 
           bad_char == false && 
           char_pairs >= 2 {
            break;
        }
        new_password(&mut password);
    }
    password
}

fn main() {
    let password_bytes = b"cqjxjnds";
    let mut password = [0u8; 8];
    password.copy_from_slice(password_bytes);
    new_password(&mut password);
    password = get_password(password);
    println!("Santa's new password is {}", str::from_utf8(&password).unwrap());
    new_password(&mut password);
    password = get_password(password);
    println!("Santa's second new password is {}", str::from_utf8(&password).unwrap());
}
    
