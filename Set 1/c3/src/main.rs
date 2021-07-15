use std::collections::HashMap;

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    let mut output = vec![];

    for index in 0..a.len() {
        output.push(a[index] ^ b[index]);
    }

    output
}

fn xor_key(a: &[u8], key: u8) -> Vec<u8> {
    let mut output = vec![];

    for index in 0..a.len() {
        output.push(a[index] ^ key);
    }

    output
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

fn main() {
    let ref_freq = get_freq(b"If this is your first time, the documentation for the standard library is written to be casually perused. Clicking on interesting things should generally lead you to interesting places. Still, there are important bits you don't want to miss, so read on for a tour of the standard library and its documentation!");
    
    let cipher =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();

    let mut plain = vec![];

    let mut max: f64 = 0.0;

    for key in 0..=255 {
        let xored = xor_key(&cipher, key);

        let target_freq = get_freq(&xored);

        let score = score(&ref_freq, &target_freq);

        if score > max {
            max = score;
            plain = xored;
        }
    }

    println!("{}", String::from_utf8_lossy(&plain));
}

fn score(a: &HashMap<u8, f64>, b: &HashMap<u8, f64>) -> f64 {
    let mut score = 0.0;

    for byte in 0..=255 {
        score += a.get(&byte).unwrap_or(&0.0) * b.get(&byte).unwrap_or(&0.0);
    }

    score
}
