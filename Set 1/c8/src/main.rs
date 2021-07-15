fn main() {
    let chiper = include_str!("c8.txt");
    let mut found = false;
    for line in chiper.lines() {
        for i in (0..line.len()).step_by(32) {
            for k in (i + 32..line.len()).step_by(32) {
                if &line[i..i + 32] == &line[k..k + 32] {
                    println!("Detected: {} ", line);
                    println!("Using: {}", &line[i..i + 32]);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
}
