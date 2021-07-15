fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    let mut output = vec![];

    for index in 0..a.len() {
        output.push(a[index] ^ b[index]);
    }

    output
}

fn main() {
    let xored = repeat_key(
        String::from(
            "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal",
        ),
        String::from("ICE"),
    );

    for element in xored {
        print!("{:02x}", element);
    }
    println!("");
}

fn repeat_key(t: String, key: String) -> Vec<u8> {
    let text = t;
    let mut encrypt_key = key;

    // repeat key
    let mut c = 0;
    let end = encrypt_key.len();
    while encrypt_key.len() < text.len() {
        encrypt_key.push(encrypt_key.chars().nth(c).unwrap());
        c += 1;
        if c == end {
            c = 0;
        }
    }

    xor(text.as_bytes(), encrypt_key.as_bytes())
}
