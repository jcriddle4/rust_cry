
//  #![feature(drop_types_in_const)]

#![allow(unused_parens)]

//#![feature(step_by)]

use std::process;
//use std::sync::Arc;
//use std::sync::Mutex;
use std::fs::File;
//use std::io;
use std::io::prelude::*;
//use std::mem;
use std::env;
//use std::path::Path;
use lite_scramble::*;

use byteorder::{LittleEndian};
use byteorder::ByteOrder;

const MAX_EXPANSION_SIZE: usize = 33554432;

const EXP_BUFF_SIZE: usize = 23000000;

//pub type EXP_BUFF_TYPE = [u8; 23000000];

//static expansion_array: [u8; 9000000];

//static mut arc: Option<[u8; 3]>  = Arc::new(Mutex::new(None));

//static mut expansion_array: Option<[u8; 3]> = Arc::new(Mutex::new(None));

//static mut expansion_array: Mutex<Option<[u8; 3]>> = Mutex::new(None);

/*
fn set_expansion_array(mutex: &Mutex<[u8; 9000000]>){
    let mut guard = lock(mutex);

    let a = access(&mut guard);


}  */

pub fn expand_init() -> (Vec<u8>){
    //let mut exp_buff: Vec<u8> = Vec::with_capacity(EXP_BUFF_SIZE);

    let mut exp_buff = vec![0; EXP_BUFF_SIZE];

    //unsafe {
    //    exp_buff = mem::uninitialized();
    //}

    println!("Opening 1984\n");
    let path_result = env::current_dir().expect("Could not get cwd\n");
    println!("Cwd is:{}",path_result.display());
    let mut file = File::open("1984.tar.bz2").expect("Unable to open 1984");

    println!("Reading 1984\n");

    match file.read(&mut *exp_buff) {
        Ok(bytes_read) => {
                if(bytes_read != EXP_BUFF_SIZE) {
                    println!("Read 1984 but came up short for file length");
                    process::exit(1);
                }
            }
        ,Err(e) => {
            println!("Error reading 1984:{}", e);
            process::exit(1);
        }
    }

    exp_buff
}

pub fn expand_128(exp_buff: & Vec<u8>, key_buff: &mut [u8; 16], size: usize, buff: &mut [u8]){
    let skip_ahead = 8192;
    let min_step = 65536 + size + 1;
    let key_size = 16;
    let full_size = (min_step * (key_size + 2)) + skip_ahead + 1024;

    if(size % 32 != 0){
        println!("ERROR: Size must be a multiple of 32 given size:{} delta:{}",size,(size%32));
        process::exit(1);
    }

    if(size < 64){
        println!("ERROR: Size of expansion was less than 64");
        process::exit(1);
    }

    if(size > MAX_EXPANSION_SIZE){
        println!("ERROR: Max expansion size exceeded:{} max allowed:{}",size,MAX_EXPANSION_SIZE);
        process::exit(1);
    }

    if(full_size > exp_buff.len()){
        println!("ERROR: The initialize exp_buff is smaller then needed for expansion request:{} {}",size,full_size);
        process::exit(1);
    }

    //bzero array the rust way
    for elem in buff.iter_mut() {
        *elem = 0;
    }

    let mut key16: u16 = 0;
    for counter in 0..8{
        let ptr16 = (counter * 2) % 16;
        key16 = key16.wrapping_add(LittleEndian::read_u16(&mut key_buff[ptr16..(ptr16 + 2)]));
    }

    println!(" key16:{}",key16);
    key16 = key16.rotate_left(7);
    let file_offset = skip_ahead + (key16 as usize);
    println!("File offset:{}",file_offset);
    for counter in 0..size{
        buff[counter] = exp_buff[file_offset + counter];
    }

    //let x: u64;
    //let y: u64;
    //let z: u64;

    for counter in 0..16 {

        if(counter % 8 == 0){
            //println!(" key_buff_aaa:{}",LittleEndian::read_u32(&mut key_buff[0..4]));
            lite_scramble_128(key_buff);
            //println!(" key_buff_128:{}",LittleEndian::read_u32(&mut key_buff[0..4]));
        }

        let ptr16 = (counter * 2) % 16;
        let file_offset = skip_ahead
            + ((LittleEndian::read_u16(&mut key_buff[ptr16..(ptr16 + 2)])) as usize)
            + (min_step * (counter + 1));

        //println!(" file_offset:{} min_step:{} key16:{} counter:{}",file_offset,min_step,((LittleEndian::read_u16(&mut key_buff[ptr16..(ptr16 + 2)])) as usize),counter);
        //println!(" buff:{} {} {} {}",buff[0],buff[1],buff[20],buff[42]);
        let mut c: u64 = 0;
        for qi in (0..(size/8)){
            let i = qi * 8;
            let x = LittleEndian::read_u64(& exp_buff[(file_offset + i)..(file_offset + i + 8)]);
            let y = LittleEndian::read_u64(&mut buff[i..(i + 8)]);
            let z = x.wrapping_add(y);

            LittleEndian::write_u64(&mut buff[i..(i+8)], z.wrapping_add(c));
            c = (if(z < x || z < y) {1} else {0});
        }
    }
    println!(" my_init:{} {} {} {}",buff[0],buff[1],buff[20],buff[42]);
}