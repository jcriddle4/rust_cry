

#![allow(unused_parens)]

use cry128::*;

use std::process;
use std::mem;
use time::PreciseTime;
use lite_scramble::*;
use expand_rnd::*;

pub fn test_expansion(){
    let exp_buff = expand_init();
    let mut key_buff: [u8; 16];
    const SIZE : usize = 56864;
    let mut buff: [u8; SIZE];

    unsafe {
        key_buff = mem::uninitialized();
        buff     = mem::uninitialized();
    }
    for i in 0..16{
        key_buff[i] = ((3*i + 1) as u8);
    }

    expand_128(& exp_buff,&mut key_buff,SIZE,&mut buff);

    for i in 0..32{
        print!(" {}",(buff[i] as u32));
    }
    println!(" ");
    //process::exit(0);
}

pub fn test_cry128_ecb(){
    let exp_buff = expand_init();
    let mut rr: R128_ECB;
    let mut key_buff: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

    unsafe {
        rr = mem::uninitialized();
    }

    init_r128_ecb(&mut rr,& exp_buff,&mut key_buff);

    let mut buff: [u8; 16] = [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];

    let old_buff = buff.clone();

    let test_loops : u32 = 3000;

    for _ in (0..test_loops){
        cry128_ecb(&rr, &mut buff);
    }

    for _ in (0..test_loops){
        z_cry128_ecb(&rr, &mut buff);
    }

    if(buff != old_buff){
        println!(" ERROR: Mismatch comparing buffers R128_ECB \n");
        process::exit(1);
    } else {
        println!(" PASS: Buffers matched R128_ECB\n");
    }
}

pub fn test_lite_scramble(){
    let mut buff: [u8; 16] = [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];

    let old_buff = buff.clone();

    let test_loops : u32 = 3000;

    for _ in (0..test_loops){
        lite_scramble_128(&mut buff);
    }

    for _ in (0..test_loops){
        z_lite_scramble_128(&mut buff);
    }

    if(buff != old_buff){
        println!(" ERROR: Mismatch comparing buffers lite_scramble \n");
        process::exit(1);
    } else {
        println!(" PASS: Buffers matched lite_scramble\n");
    }
}

pub fn test_cry128_print(){
    let mut rr128: R128;
    //const size: usize = 34144;
    let mut output_buff: [u8; 16] = [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
    let mut key_buff: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

    unsafe {
        rr128 = mem::uninitialized();
    }

    let exp_buff: Vec<u8> = expand_init();

    init_r128(&mut rr128,& exp_buff,&mut key_buff);

    for i in 0..1000 {
        cry128(&rr128, &mut output_buff, (i/500), i);
    }

    print!("cy128 output:");
    for v in output_buff.iter() {
        print!(" {}",v)
    }
}

pub fn test_cry128_ecb_print(){
    let mut rr: R128_ECB;
    //const size: usize = 34144;
    let mut output_buff: [u8; 16] = [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
    let mut key_buff: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

    unsafe {
        rr = mem::uninitialized();
    }

    let exp_buff: Vec<u8> = expand_init();

    init_r128_ecb(&mut rr,& exp_buff,&mut key_buff);

    //for i in 0..1000 {
        cry128_ecb(&rr, &mut output_buff);
    //}

    print!("cy128 output:");
    for v in output_buff.iter() {
        print!(" {}",v)
    }

    //process::exit(0);
}

pub fn test_cry128(){
    let mut rr128: R128;
    let mut key_buff: [u8; 16] = [15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];

    unsafe {
        rr128 = mem::uninitialized();
    }

    let exp_buff = expand_init();

    init_r128(&mut rr128,& exp_buff,&mut key_buff);

    let mut buff: [u8; 16] = [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];

    let old_buff = buff.clone();

    let test_loops : u32 = 400;

    for i in (0..test_loops){
        for j in (0..7) {
            cry128(&rr128, &mut buff, j, i);
        }
    }

    for i in (0..test_loops).rev() {
        for j in (0..7).rev() {
            z_cry128(&rr128, &mut buff, j, i);
        }
    }

    if(buff != old_buff){
        println!(" ERROR: Mismatch comparing buffers\n");
        process::exit(1);
    } else {
        println!(" PASS: Buffers matched\n");
    }
}

pub fn test_speed(){
    let mut rr128: R128;
    let mut key_buff: [u8; 16] = [15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];

    unsafe {
        rr128 = mem::uninitialized();
    }

    let exp_buff = expand_init();

    init_r128(&mut rr128,& exp_buff,&mut key_buff);

    let test_count: u32 = 8000000;

    let mut buff: [u8; 16] = [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];

    let start = PreciseTime::now();
    for count in 0..30 {
        for i in 0..test_count {
            cry128(& rr128, &mut buff, count, i);
        }
    }
    let end = PreciseTime::now();
    println!("Runtime      cry128:{}", start.to(end));

    //----------------------------------------------------------------------------------
    //----------------------------------------------------------------------------------

    let mut r_ecb: R128_ECB;
    unsafe {
        r_ecb = mem::uninitialized();
    }
    init_r128_ecb(&mut r_ecb,& exp_buff,&mut key_buff);

    let start = PreciseTime::now();
    for _ in 0..30 {
        for _ in 0..test_count {
            cry128_ecb(& r_ecb, &mut buff);
        }
    }
    let end = PreciseTime::now();
    println!("Runtime  cry128_ecb:{}", start.to(end));

}

