fn main() {
    let x = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut base_64 = String::from("");
    let mut y = String::from("");
    let to_decrypt = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    for char in to_decrypt.chars() {
        y.push_str(to_binary(char));
    }
    let mut pad = 0;
    let mut c = 0;
    while c < y.len() {
        if c + 6 > y.len() {
            y.push_str("00");
            pad += 1;
            if c + 6 > y.len() {
                y.push_str("00");
                pad += 1;
            }
            let slice = &y[c..c + 6];
            let ch = x.chars().nth(to_decimal(slice.to_string())).unwrap();
            base_64.push(ch);
            if pad == 2 {
                base_64.push_str("==");
                break;
            } else {
                base_64.push('=');
                break;
            }
        } else {
            let slice = &y[c..c + 6];
            c += 6;

            let ch = x.chars().nth(to_decimal(slice.to_string())).unwrap();
            base_64.push(ch);
        }
    }
    println!("{}", base_64);
}

fn to_decimal(s: String) -> usize {
    let mut x = 0;
    if s.chars().nth(5).unwrap() == '1' {
        x += 1;
    }
    if s.chars().nth(4).unwrap() == '1' {
        x += 2;
    }
    if s.chars().nth(3).unwrap() == '1' {
        x += 4;
    }
    if s.chars().nth(2).unwrap() == '1' {
        x += 8;
    }
    if s.chars().nth(1).unwrap() == '1' {
        x += 16;
    }
    if s.chars().nth(0).unwrap() == '1' {
        x += 32;
    }
    x
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "",
    }
}
