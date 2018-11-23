//extern crate rand;
use std::time::Instant;
fn add_key(input: &mut u32, key: u32){
    *input = *input ^ key;
}

fn sub_bytes(input: &mut u32){
    let sbox: [u32; 16] = [0x8, 0xf, 0x3, 0x0, 0xa, 0x5, 0x9, 0x6, 0xc, 0x1, 0xe, 0x2, 0x7, 0x4, 0xd, 0xb];
    *input = (sbox[((*input & 0xf000) >> 12) as usize] << 12) ^ (sbox[((*input & 0x0f00) >> 8) as usize] << 8) ^ (sbox[((*input & 0x00f0) >> 4) as usize] << 4) ^ (sbox[(*input & 0x000f) as usize]);
}

fn permute(input: &mut u32){
    let permutace: [u32; 16] = [2,3,11,5,8,0,15,9,1,12,4,7,13,6,14,10];
    let mut temp: u32 = 0x0;
    for i in 0..16 {
        temp = temp ^ (((*input >> 15-i) & 0x1) << (15-permutace[i]));
    }
    *input = temp;
}

fn encrypt(input: u32) -> u32{
    let keys: [u32; 5] = [0xb1ac, 0x3170, 0xed23, 0x1aa9, 0x52f1];
    let mut result: u32 = input;
    for i in 0..3{
        add_key(&mut result, keys[i]);
        sub_bytes(&mut result);
        permute(&mut result);
    }
    add_key(&mut result, keys[3]);
    sub_bytes(&mut result);
    add_key(&mut result, keys[4]);
    return result;
}

fn main() {
    let start = Instant::now();
    let mut temp: u32 = 0x6964;
    let mut counter: u64 = 0;
    while start.elapsed().as_secs() < 10 {
        temp = encrypt(temp);
        counter = counter+1;
    }
    println!("milis: {}", start.elapsed().as_secs());
    println!("count: {}", counter/10);
}
