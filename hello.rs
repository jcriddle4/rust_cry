
mod utility;
mod cry128;
mod lite_scramble;
mod test;
mod expand_rnd;

extern crate byteorder;
extern crate rand;
extern crate time;

//use std::mem;
//use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
//use byteorder::ByteOrder;
//use rand::Rng;
//use std::process;
//use time::PreciseTime;

use cry128::*;
use test::*;
//use expand_rnd::*;
//use std::io;
//use std::io::Write;
//use std::io::stdout;

fn main() {

    println!("Does this start???\n");

    test_expansion();

    println!("Starting expand_init\n");
    //let exp_buff = expand_init();

    println!("The size of rr128:{}",std::mem::size_of::<R128>());

    test_cry128_ecb_print();

    test_cry128_print();

    test_cry128_ecb_print();

    test_cry128();

    test_cry128_ecb();

    test_lite_scramble();

    test_speed();

    //let mut buff: [u8; 128];
    //let key: [u8; 16];

    //expand_128(key,128,&mut buff);

	println!("Hello World!\n");

}




