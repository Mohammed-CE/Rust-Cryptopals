use std::{collections::HashMap};
fn main() {
    let text = include_str!("c6.txt");
    assert_eq!(get_hamming(b"wokka wokka!!!",b"this is a test"),37.);
    let message = remove_linefeed(text);
    let cipher = &base64::decode(&message).unwrap();
    let key=  get_key(transpose(&cipher,get_keysize(&cipher)));
    let xored = repeat_key(base64::decode(message).unwrap(),String::from_utf8_lossy(&key).to_string());
    println!("{}", String::from_utf8_lossy(&xored));
}

fn remove_linefeed(s:&str) -> String {
    let mut no_lines = String::from("");
    for line in s.lines(){
        no_lines.push_str(line);
    }
    no_lines
}

fn get_keysize(cipher: &[u8]) -> usize{
    let mut min = f64::MAX;
    let mut keysize= 0;
    let mut hamming_distance: f64 = 0.0;

    for i in 2..40{
                hamming_distance = get_hamming(&cipher[0..i],&cipher[i..i*2]) 
                + get_hamming(&cipher[0..i],&cipher[i*2..i*3]) 
                + get_hamming(&cipher[0..i],&cipher[i*3..i*4]) 
                + get_hamming(&cipher[i..i*2],&cipher[i*2..i*3])
                + get_hamming(&cipher[i..i*2],&cipher[i*3..i*4])
                + get_hamming(&cipher[i*2..i*3],&cipher[i*3..i*4]);

               hamming_distance /=  6.0 * i as f64;

                if hamming_distance <  min{
                    min = hamming_distance;
                    keysize=i;
                }
            
    }
       keysize

}

fn get_hamming(x: &[u8], y: &[u8]) -> f64 {
    x.iter().zip(y).fold(0., |a, (b, c)| a + (*b ^ *c).count_ones() as f64)
}

fn transpose(cipher: &[u8],s: usize) -> Vec<String> {
    let mut transposed = vec![String::from(""); s];
    for i in 0..cipher.len(){
        transposed[i%s].push_str(&format!("{:02x}", cipher[i]));
    }
    transposed
}

fn get_freq(text: &[u8]) -> HashMap<u8, f64> {
    let mut output = HashMap::new();

    for byte in text {
        *output.entry(*byte).or_insert(0.0) += 1.0;
    }

    for values in output.values_mut() {
        *values /= text.len() as f64;
    }

    output
}

fn get_score(a: &HashMap<u8, f64>, b: &HashMap<u8, f64>) -> f64 {
    let mut score = 0.0;

    for byte in 0..=255 {
        score += a.get(&byte).unwrap_or(&0.0) * b.get(&byte).unwrap_or(&0.0);
    }

    score
}

fn xor_key(a: &[u8], key: u8) -> Vec<u8> {
    let mut output = vec![];

    for index in 0..a.len() {
        output.push(a[index] ^ key);
    }

    output
}

fn get_key(cipher: Vec<String>) -> Vec<u8>{
    let ref_freq = get_freq("Exclusive-or is also heavily used in block ciphers such as AES (Rijndael) or Serpent and in block cipher implementation (CBC, CFB, OFB or CTR). ".as_bytes());
    let mut plain = vec![];

    for i in 0..cipher.len() {
        let cipher = hex::decode(cipher[i].to_string()).unwrap();
        let mut max: f64 = 0.0;
        let mut k = 0;
        for key in 0..=255 {
            let xored = xor_key(&cipher, key);
            let target_freq = get_freq(&xored);
            let score = get_score(&ref_freq, &target_freq);
    
            if score > max {
                max = score;
                k = key;
            }
        }
        plain.push(k);

        
    }
    println!("Key is: {}", String::from_utf8_lossy(&plain));

plain

}

fn repeat_key(t: Vec<u8>, key: String) -> Vec<u8> {
    let text = t;
    let mut encrypt_key = key;

    let mut c = 0;
    let end = encrypt_key.len();
    while encrypt_key.len() < text.len() {
        encrypt_key.push(encrypt_key.chars().nth(c).unwrap());
        c += 1;
        if c == end {
            c = 0;
        }
    }
    
    xor(&text, encrypt_key.as_bytes())
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    let mut output = vec![];

    for index in 0..a.len() {
        output.push(a[index] ^ b[index]);
    }

    output
}