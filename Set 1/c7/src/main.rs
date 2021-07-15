use aes::cipher::{generic_array::GenericArray, BlockDecrypt, NewBlockCipher};
use aes::{Aes128, Block};
fn main() {
    let cipher = include_str!("cipher.txt");
    let key = GenericArray::from_slice("YELLOW SUBMARINE".as_bytes());
    let aes128 = Aes128::new(&key);
    let mut encrypted: Vec<u8> = vec![];
    let mut block = Block::default();

    for line in cipher.lines() {
        for vec in base64::decode(line).unwrap() {
            encrypted.push(vec);
        }
    }

    for chunk in encrypted.chunks(16) {
        block.copy_from_slice(chunk);
        aes128.decrypt_block(&mut block);
        print!("{}", String::from_utf8(block.to_vec()).unwrap());
    }
}
