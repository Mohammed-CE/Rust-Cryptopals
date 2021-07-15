fn main() {

    let op1 = "1c0111001f010100061a024b53535009181c";
    let op2 = "686974207468652062756c6c277320657965";

    let mut x = String::from("");
    let mut y = String::from("");

    for char in op1.chars(){
        x.push_str(to_binary(char));  
    }
    for char in op2.chars(){
        y.push_str(to_binary(char));  
        }
    
    let mut result = String::from("");

        
        let mut c = 0;
        while c <  y.len(){   
                let to_hex =  format!("{:02x}", to_decimal(y[c..c+8].to_string()) ^ to_decimal(x[c..c+8].to_string()));
                result.push_str(&to_hex.to_string());
                c +=8;

            }
    
            println!("{}", result);

    
    


}

fn to_decimal(s: String) -> i32 {
    let mut x = 0;
    if s.chars().nth(7).unwrap() == '1' {
        x +=1;
    }
    if s.chars().nth(6).unwrap() == '1' {
        x +=2;
    }
    if s.chars().nth(5).unwrap() == '1' {
        x +=4;
    }
    if s.chars().nth(4).unwrap() == '1' {
        x +=8;
    }
    if s.chars().nth(3).unwrap() == '1' {
        x +=16;
    }
    if s.chars().nth(2).unwrap() == '1' {
        x +=32;
    }
    if s.chars().nth(1).unwrap() == '1' {
        x +=64;
    }
    if s.chars().nth(0).unwrap() == '1' {
        x +=128;
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
        _  => "",
    }
}

