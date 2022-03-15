use std::collections::HashMap;

pub fn main() {
    // println!("{}", find_loop_size(17807724, 7));
    let card_public_key = 6930903;
    let door_public_key = 19716708;
    let card_loop = find_loop_size(card_public_key, 7);
    let door_loop = find_loop_size(door_public_key, 7);
    let encrypt_key = transform(door_public_key, card_loop);
    assert_eq!(encrypt_key, transform(card_public_key, door_loop));
    println!("{}", encrypt_key);
}

fn find_loop_size(public_key: u64, subject_number: u64) -> usize {
    let mut loop_size = 1;
    let mut val = transform_memo(subject_number, loop_size, 0);
    while public_key != val {
        loop_size += 1;
        val = transform_memo(subject_number, loop_size, val);
    }
    loop_size
}

fn transform_memo(subject_number: u64, loop_size: usize, prev_val: u64) -> u64 {
    if loop_size == 1 { return subject_number % 20201227; }

    (prev_val * subject_number) % 20201227
}

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    let mut val = 1;
    for i in 0..loop_size {
        val = val * subject_number;
        val = val % 20201227;
    }
    val
}
