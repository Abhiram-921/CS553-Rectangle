/*
 * RECTANGLE
 *
 */

use std::convert::TryInto;
const SBOX: [u16; 16] = [6, 5, 0xc, 0xa, 1, 0xe, 7, 9, 0xb, 0, 3, 0xd, 8, 0xf, 4, 2];
const RC: [u16; 25] = [0x01, 0x02, 0x04, 0x09, 0x12, 0x05, 0x0B, 0x16, 0x0C, 0x19, 0x13, 0x07, 0x0F, 0x1F, 0x1E, 0x1C, 0x18, 0x11, 0x03, 0x06, 0x0D, 0x1B, 0x17, 0x0E, 0x1D];

fn main() {
    // third parameter = 0 for bit slice and 1 for normal lookup version
    println!("{:b}", cipher(0, 0, 0));
    println!("{:b}", cipher(0, 0, 1));
}

fn sbox(input: u16) -> u16 {
    SBOX[input as usize]
}

fn next_subkey(initial: &mut [u16; 5], round: usize) {
    // apply sbox
    for i in 0..4 {
        let sb = sbox(((initial[0] >> i) & 1) | ((initial[1] >> i) & 1) << 1 | ((initial[2] >> i) & 1) << 2 | ((initial[3] >> i) & 1) << 3);
        for k in 0..4 {
            initial[k] = (initial[k] & !(0b1 << i)) | ((sb >> k) & 1) << i;
        }
    }

    // apply Feistal transformation
    let row0_copy: u16 = initial[0];
    initial[0] = ((initial[0] >> 8) | (initial[0] << 8)) ^ initial[1];
    initial[1] = initial[2];
    initial[2] = initial[3];
    initial[3] = ((initial[3] >> 4) | (initial[3] << 12)) ^ initial[4];
    initial[4] = row0_copy;

    // add 5-bit round constant
    initial[0] = initial[0] ^ RC[round];
}

fn add_round_key(input: &mut [u16; 4], k: [u16; 4]) {
    for i in 0..4 {
        input[i] = input[i] ^ k[i];
    }
}

fn sub_column_bs(input: &mut [u16; 4]) {
    let mut t: [u16; 12] = [0; 12];
    let mut b: [u16; 4] = [0; 4];

    t[1] = !input[1];
    t[2] = input[0] & t[1];
    t[3] = input[2] ^ input[3];
    b[0] = t[2] ^ t[3];

    t[5] = input[3] | t[1];
    t[6] = input[0] ^ t[5];
    b[1] = input[2] ^ t[6];
    t[8] = input[1] ^ input[2];

    t[9] = t[3] & t[6];
    b[3] = t[8] ^ t[9];
    t[11] = b[0] | t[8];
    b[2] = t[6] ^ t[11];

    for i in 0..4 {
        input[i] = b[i];
    }
}

fn sub_column_nr(input: &mut [u16; 4]) {
    for i in 0..16 {
        let temp: u16 = sbox(((input[0] >> i) & 1) << 0 | ((input[1] >> i) & 1) << 1 | ((input[2] >> i) & 1) << 2 | ((input[3] >> i) & 1) << 3);
        for j in 0..4 {
            input[j] = (input[j] & !(1 << i)) | ((temp >> j) & 1) << i;
        }
    }
}

fn shift_row(input: &mut [u16; 4]) {
    input[1] = (input[1] << 1) | (input[1] >> 15);
    input[2] = (input[2] << 12) | (input[2] >> 4);
    input[3] = (input[3] << 13) | (input[3] >> 3);
}

fn cipher(input: u64, iv: u128, mo: i32) -> u64 {
    let mut key: [u16; 5] = [
        (iv & 0xffff) as u16,
        ((iv >> 16) & 0xffff) as u16,
        ((iv >> 32) & 0xffff) as u16,
        ((iv >> 48) & 0xffff) as u16,
        ((iv >> 64) & 0xffff) as u16
    ];
    let mut state: [u16; 4] = [
        (input & 0xffff) as u16,
        ((input >> 16) & 0xffff) as u16,
        ((input >> 32) & 0xffff) as u16,
        ((input >> 48) & 0xffff) as u16,
    ];
    if(mo == 0) {
        for i in 0..25 {
            add_round_key(&mut state, key[0..4].try_into().expect(""));
            sub_column_bs(&mut state);
            shift_row(&mut state);
            next_subkey(&mut key, i);
        }
    }
    else {
        for i in 0..25 {
            add_round_key(&mut state, key[0..4].try_into().expect(""));
            sub_column_nr(&mut state);
            shift_row(&mut state);
            next_subkey(&mut key, i);
        }
    }
    add_round_key(&mut state, key[0..4].try_into().expect(""));
    (state[0] as u64) | ((state[1] as u64) << 16) | ((state[2] as u64) << 32) | ((state[3] as u64) << 48)
}
