
use utility;
//use rand::Rng;
use std::mem;
use byteorder::{LittleEndian};
use byteorder::ByteOrder;
use expand_rnd::*;

use utility::*;

extern crate rand;

type A256 = [u32; 256];

const PWQ_SIZE: usize = 540;

pub struct R128 {
     infrequent_small0: [u32; 34]
    ,infrequent_small1: [u32; 34]
    ,x0: u32
    ,x1: u32
    ,y0: u32
    ,y1: u32
    ,small_cycle0: u32
    ,small_cycle1: u32
    ,large_cycle0: u32
    ,large_cycle1: u32
    ,varray: [u32; 6]
    ,key_byte_r:[u8; 16]
    ,pp: [u32; 16]
    ,addr0: A256
    ,addr1: A256
    ,c1: [u32; 90]
    ,c2: [u32; 90]
    ,c3: [u32; 106]
    ,c4: [u32; 106]
    ,s0: A256
    ,s1: A256
    ,s2: A256
    ,s3: A256
    ,s4: A256
    ,s5: A256
    ,s6: A256
    ,s7: A256
    ,s8: A256
    ,s9: A256
    ,infrequent_large1: [u32; 1024]
    ,infrequent_large2: [u32; 1024]
    ,right0: [u32; 512]
    ,right1: [u32; 512]
    ,v1k: [u32; 1024]
    ,infrequent_medium0: [u32; 130]
    ,infrequent_medium1: [u32; 130]
    ,infrequent_small2: [u32; 34]
    ,infrequent_small3: [u32; 34]
    ,pwq: [u32; PWQ_SIZE]
    ,padding: [u8; 24]
}

pub struct R128_ECB {
    key_byte_r: [u8; 16]
    ,p: [u32; 16]
    ,infrequent_small0: [u32; 34]
    ,infrequent_small1: [u32; 34]
    ,s1: A256
    ,s2: A256
    ,s3: A256
    ,s4: A256
    ,s5: A256
    ,s6: A256
    ,s7: A256
    ,s8: A256
    ,s9: A256
    ,s10: A256
    ,s11: A256
    ,s12: A256
    ,s13: A256
    ,s14: A256
    ,right0: [u32; 512]
    ,right1: [u32; 512]
    ,right2: [u32; 512]
    ,right3: [u32; 512]
    ,right4: [u32; 512]
    ,right5: [u32; 512]
    ,v1k_a: [u32; 1024]
    ,v1k_b: [u32; 1024]
    ,v1k_c: [u32; 1024]
    ,infrequent_xlarge0: [u32; 1532]
    ,infrequent_xlarge1: [u32; 1532]
    ,infrequent_large0: [u32; 1024]
    ,infrequent_large1: [u32; 1024]
    ,infrequent_medium0: [u32; 130]
    ,infrequent_medium1: [u32; 130]
}

fn circle_around(mm0: u32, mm1: u32, mm2: u32, mm3: u32, v1: & A256, v2: & A256) -> (u32,u32,u32,u32) {
    let mask: u32 = 0x000000ff;

    let mm0 = mm0.wrapping_add(v1[((mm3 >> 24) & mask) as usize]);
    let mm0 = mm0.wrapping_add(v2[((mm3 >> 16) & mask) as usize]);
    let mm1 = mm1.wrapping_add(v1[((mm3 >>  8) & mask) as usize]);
    let mm2 = mm2.wrapping_add(v1[((mm3      ) & mask) as usize]);

    let mm1 = mm1.wrapping_add(v1[((mm0 >> 24) & mask) as usize]);
    let mm1 = mm1.wrapping_add(v2[((mm0 >> 16) & mask) as usize]);
    let mm2 = mm2.wrapping_add(v1[((mm0 >>  8) & mask) as usize]);
    let mm3 = mm3.wrapping_add(v1[((mm0      ) & mask) as usize]);

    let mm2 = mm2.wrapping_add(v1[((mm1 >> 24) & mask) as usize]);
    let mm2 = mm2.wrapping_add(v2[((mm1 >> 16) & mask) as usize]);
    let mm0 = mm0.wrapping_add(v1[((mm1 >>  8) & mask) as usize]);
    let mm3 = mm3.wrapping_add(v1[((mm1      ) & mask) as usize]);

    let mm3 = mm3.wrapping_add(v1[((mm2 >> 24) & mask) as usize]);
    let mm3 = mm3.wrapping_add(v2[((mm2 >> 16) & mask) as usize]);
    let mm0 = mm0.wrapping_add(v1[((mm2 >>  8) & mask) as usize]);
    let mm1 = mm1.wrapping_add(v1[((mm2      ) & mask) as usize]);

    (mm0, mm1, mm2, mm3)
}

fn z_circle_around(mm0: u32, mm1: u32, mm2: u32, mm3: u32, v1: & A256, v2: & A256) -> (u32,u32,u32,u32) {
    let mask: u32 = 0x000000ff;

    let mm3 = mm3.wrapping_sub(v1[((mm2 >> 24) & mask) as usize]);
    let mm3 = mm3.wrapping_sub(v2[((mm2 >> 16) & mask) as usize]);
    let mm0 = mm0.wrapping_sub(v1[((mm2 >>  8) & mask) as usize]);
    let mm1 = mm1.wrapping_sub(v1[((mm2      ) & mask) as usize]);

    let mm2 = mm2.wrapping_sub(v1[((mm1 >> 24) & mask) as usize]);
    let mm2 = mm2.wrapping_sub(v2[((mm1 >> 16) & mask) as usize]);
    let mm0 = mm0.wrapping_sub(v1[((mm1 >>  8) & mask) as usize]);
    let mm3 = mm3.wrapping_sub(v1[((mm1      ) & mask) as usize]);

    let mm1 = mm1.wrapping_sub(v1[((mm0 >> 24) & mask) as usize]);
    let mm1 = mm1.wrapping_sub(v2[((mm0 >> 16) & mask) as usize]);
    let mm2 = mm2.wrapping_sub(v1[((mm0 >>  8) & mask) as usize]);
    let mm3 = mm3.wrapping_sub(v1[((mm0      ) & mask) as usize]);

    let mm0 = mm0.wrapping_sub(v1[((mm3 >> 24) & mask) as usize]);
    let mm0 = mm0.wrapping_sub(v2[((mm3 >> 16) & mask) as usize]);
    let mm1 = mm1.wrapping_sub(v1[((mm3 >>  8) & mask) as usize]);
    let mm2 = mm2.wrapping_sub(v1[((mm3      ) & mask) as usize]);

    (mm0, mm1, mm2, mm3)
}

fn single_strike(src: u32, mm0: u32, mm1: u32, infrequent_pool: & [u32; 1024]) -> (u32, u32){
    let mut sum: u32 = 0;

    sum += src.wrapping_shr(24);
    sum += src.wrapping_shl( 8).wrapping_shr(24);
    sum += src.wrapping_shl(16).wrapping_shr(24);
    sum += src.wrapping_shl(24).wrapping_shr(24);

    (mm0.wrapping_add(infrequent_pool[sum as usize]), mm1.wrapping_add(infrequent_pool[(sum+1) as usize]))
}

fn z_single_strike(src: u32, mm0: u32, mm1: u32, infrequent_pool: & [u32; 1024]) -> (u32, u32){
    let mut sum: u32 = 0;

    sum += src.wrapping_shr(24);
    sum += src.wrapping_shl( 8).wrapping_shr(24);
    sum += src.wrapping_shl(16).wrapping_shr(24);
    sum += src.wrapping_shl(24).wrapping_shr(24);

    (mm0.wrapping_sub(infrequent_pool[sum as usize]), mm1.wrapping_sub(infrequent_pool[(sum+1) as usize]))
}



fn inside_inf_x(src1: u32, src2: u32, dest: u32, infreqx: & [u32; 1532]) -> (u32){
    let mask: u32 = 0x000000ff;
    let mut sum;

    sum = src1 & mask;
    let v = src1 >> 8;
    sum += v & mask;
    //println!(" sumpp:{}",sum);

    let mut v = src2;
    for _ in 0..4{
        sum += v & mask;
        v = v >> 8;
    }
    //println!(" sum:{}",sum);
    dest.wrapping_add(infreqx[sum as usize])
}

fn z_inside_inf_x(src1: u32, src2: u32, dest: u32, infreqx: & [u32; 1532]) -> (u32){
    let mask: u32 = 0x000000ff;
    let mut sum;

    sum = src1 & mask;
    let v = src1 >> 8;
    sum += v & mask;

    let mut v = src2;
    for _ in 0..4{
        sum += v & mask;
        v = v >> 8;
    }

    dest.wrapping_sub(infreqx[sum as usize])
}

fn inf_xlarge(mm0: u32, mm1: u32,mm2: u32,mm3: u32, infreqx: & [u32; 1532]) -> (u32,u32,u32,u32){
    let mm2 = inside_inf_x(mm1,mm0,mm2,infreqx);
    let mm0 = inside_inf_x(mm3,mm2,mm0,infreqx);
    let mm3 = inside_inf_x(mm2,mm1,mm3,infreqx);
    let mm1 = inside_inf_x(mm0,mm3,mm1,infreqx);

    (mm0,mm1,mm2,mm3)
}

fn z_inf_xlarge(mm0: u32, mm1: u32,mm2: u32,mm3: u32, infreqx: & [u32; 1532]) -> (u32,u32,u32,u32){

    let mm1 = z_inside_inf_x(mm0,mm3,mm1,infreqx);
    let mm3 = z_inside_inf_x(mm2,mm1,mm3,infreqx);
    let mm0 = z_inside_inf_x(mm3,mm2,mm0,infreqx);
    let mm2 = z_inside_inf_x(mm1,mm0,mm2,infreqx);

    (mm0,mm1,mm2,mm3)
}


fn spin_10(mm0: u32, mm1: u32, mm2: u32, mm3: u32, v1k: & [u32; 1024]) -> (u32,u32,u32,u32) {
    let rotate: u32 = ((mm0 & 0xb0000000) >> 29) + 5;

    let mm1 = mm1.rotate_right(rotate);
    let mm2 = mm2.rotate_right(rotate);
    let mm3 = mm3.rotate_right(rotate);

    let mm1 = mm1.wrapping_add(v1k[(mm0 & 0x0000003ff) as usize]);
    let mm2 = mm2.wrapping_add(v1k[((mm0 & 0x0000ffc00) >> 10) as usize]);
    let mm3 = mm3.wrapping_add(v1k[((mm0 & 0x03ff00000) >> 20) as usize]);

    (mm0,mm1,mm2,mm3)
}

fn z_spin_10(mm0: u32, mm1: u32, mm2: u32, mm3: u32, v1k: & [u32; 1024]) -> (u32,u32,u32,u32) {
    let rotate: u32 = ((mm0 & 0xb0000000) >> 29) + 5;

    let mm1 = mm1.wrapping_sub(v1k[(mm0 & 0x0000003ff) as usize]);
    let mm2 = mm2.wrapping_sub(v1k[((mm0 & 0x0000ffc00) >> 10) as usize]);
    let mm3 = mm3.wrapping_sub(v1k[((mm0 & 0x03ff00000) >> 20) as usize]);

    let mm1 = mm1.rotate_left(rotate);
    let mm2 = mm2.rotate_left(rotate);
    let mm3 = mm3.rotate_left(rotate);

    (mm0,mm1,mm2,mm3)
}



fn medium_inf(src0: u32, src1: u32, mm0: u32, mm1: u32, medium: & [u32; 130]) -> (u32,u32){
    let idx: u32= (src0.count_ones() + src1.count_ones()) << 1;

    (mm0.wrapping_add(medium[idx as usize]),mm1.wrapping_add(medium[(idx+1) as usize]))
}

fn z_medium_inf(src0: u32, src1: u32, mm0: u32, mm1: u32, medium: & [u32; 130]) -> (u32,u32){
    let idx: u32 = (src0.count_ones() + src1.count_ones()) << 1;

    (mm0.wrapping_sub(medium[idx as usize]),mm1.wrapping_sub(medium[(idx+1) as usize]))
}

fn twist_and_turn(mm0: u32, my_rotate: u8, zz: u32) -> (u32){
    mm0.rotate_right((my_rotate as u32)).wrapping_add(zz)
}

fn z_twist_and_turn(mm0: u32, my_rotate: u8, zz: u32) -> (u32){
    mm0.wrapping_sub(zz).rotate_left((my_rotate as u32))
}

fn add_counters(mm0: u32,mm1: u32,mm2: u32,mm3: u32, addr0: & A256, addr1: & A256, counter0: u32, counter1: u32) -> (u32,u32,u32,u32){
    let mm0 = mm0.wrapping_add(addr0[ (counter0 & 0x000000ff)         as usize]);
    let mm1 = mm1.wrapping_add(addr0[((counter0 & 0x0000ff00) >>   8) as usize]);
    let mm2 = mm2.wrapping_add(addr0[((counter0 & 0x00ff0000) >>  16) as usize]);
    let mm3 = mm3.wrapping_add(addr0[((counter0 & 0xff000000) >>  24) as usize]);

    let mm0 = mm0.wrapping_add(addr1[ (counter1 & 0x000000ff)         as usize]);
    let mm1 = mm1.wrapping_add(addr1[((counter1 & 0x0000ff00) >>   8) as usize]);
    let mm2 = mm2.wrapping_add(addr1[((counter1 & 0x00ff0000) >>  16) as usize]);
    let mm3 = mm3.wrapping_add(addr1[((counter1 & 0xff000000) >>  24) as usize]);

    (mm0,mm1,mm2,mm3)
}

fn z_add_counters(mm0: u32,mm1: u32,mm2: u32,mm3: u32, addr0: & A256, addr1: & A256, counter0: u32, counter1: u32) -> (u32,u32,u32,u32){
    let mm0 = mm0.wrapping_sub(addr1[ (counter1 & 0x000000ff)         as usize]);
    let mm1 = mm1.wrapping_sub(addr1[((counter1 & 0x0000ff00) >>   8) as usize]);
    let mm2 = mm2.wrapping_sub(addr1[((counter1 & 0x00ff0000) >>  16) as usize]);
    let mm3 = mm3.wrapping_sub(addr1[((counter1 & 0xff000000) >>  24) as usize]);

    let mm0 = mm0.wrapping_sub(addr0[ (counter0 & 0x000000ff)         as usize]);
    let mm1 = mm1.wrapping_sub(addr0[((counter0 & 0x0000ff00) >>   8) as usize]);
    let mm2 = mm2.wrapping_sub(addr0[((counter0 & 0x00ff0000) >>  16) as usize]);
    let mm3 = mm3.wrapping_sub(addr0[((counter0 & 0xff000000) >>  24) as usize]);

    (mm0,mm1,mm2,mm3)
}

pub fn cry128(rr: & R128, buff: &mut [u8; 16], counter0: u32, counter1: u32){

    let r0 = LittleEndian::read_u32(&mut buff[0..4]);
    let r1 = LittleEndian::read_u32(&mut buff[4..8]);
    let r2 = LittleEndian::read_u32(&mut buff[8..12]);
    let r3 = LittleEndian::read_u32(&mut buff[12..16]);

    //for i in 0..11 {
    //    println!(" rr infrequent_small0:{}", rr.infrequent_small0[i]);
    //}

    let c1_idx = counter0 % 89;
    let c2_idx = counter1 % 89;

    let v0 = rr.varray[0].wrapping_add(counter0);
    let v1 = rr.varray[1].wrapping_add(counter1);
    let v2 = rr.varray[2].wrapping_add(counter0);
    let v3 = rr.varray[3].wrapping_add(counter1);
    let v4 = rr.varray[4].wrapping_add(counter0);
    let v5 = rr.varray[5].wrapping_add(counter1);

    let counter0_idx = v2 % rr.x0;
    let counter1_idx = v3 % rr.x1;
    let counter2_idx = ((v0 / rr.large_cycle0) % rr.y0) + rr.x0;
    let counter3_idx = ((v1 / rr.large_cycle1) % rr.y1) + rr.x1;
    let counter4_idx = (v4 / rr.small_cycle0) % 103;
    let counter5_idx = (v5 / rr.small_cycle1) % 103;

    //println!("step__    r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    //println!(" addr:{}",rr.addr0[0]);

    let (r0,r1,r2,r3) =
        add_counters(r0,r1,r2,r3,& rr.addr0,& rr.addr1,counter0,counter1);              //step0
    //println!("step0     r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r0 = twist_and_turn(r0,rr.key_byte_r[0],rr.pp[0]);                              //step1.1
    let r1 = twist_and_turn(r1,rr.key_byte_r[1],rr.pp[1]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[2],rr.pp[2]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[3],rr.pp[3]);
    //println!("step1.00  rr.right0:{} rr.v1k:{}",rr.right0[0],rr.v1k[0]);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s0,& rr.s1);                     //step1.2
    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s2,& rr.s3);
    //println!("step1.2   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r2 = r2.wrapping_add(rr.infrequent_small0[r0.count_ones() as usize]);           //step2
    let r3 = r3.wrapping_add(rr.infrequent_small1[r1.count_ones() as usize]);
    //println!("step2.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r0 = r0.wrapping_add(rr.c1[c1_idx as usize]);                                   //step3
    let r1 = r1.wrapping_add(rr.c2[c2_idx as usize]);
    //println!("step3.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s4,& rr.s5);                     //step3.1
    //println!("step3.1   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = spin_right4(r0,r1,r2,r3, & rr.right0);                          //step4
    let (r2,r1,r3,r0) = spin_10(r2,r1,r3,r0, & rr.v1k);
    let (r1,r3,r0,r2) = spin_right4(r1,r3,r0,r2, & rr.right1);
    //println!("step4.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0, r1) = single_strike(r3,r0,r1,& rr.infrequent_large1);                      //step5
    let r0 = r0.wrapping_add(rr.pwq[counter0_idx as usize]);
    let r1 = r1.wrapping_add(rr.pwq[counter1_idx as usize]);
    //println!("step5.0   r0:{} r1:{} r2:{} r3:{} counter0_idx:{}",r0,r1,r2,r3,counter0_idx);

    let r0 = twist_and_turn(r0,rr.key_byte_r[4],rr.pp[4]);                              //step5.1
    let r1 = twist_and_turn(r1,rr.key_byte_r[5],rr.pp[5]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[6],rr.pp[6]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[7],rr.pp[7]);
    //println!("step5.1   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r2 = r2.wrapping_add(rr.pwq[counter2_idx as usize]);                            //step5.2
    let r3 = r3.wrapping_add(rr.pwq[counter3_idx as usize]);
    //println!("step5.2   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r2,r3,r0,r1) = spin_right4(r2,r3,r0,r1, & rr.right0);                          //step6
    let (r1,r2,r3,r0) = spin_10(r1,r2,r3,r0, & rr.v1k);
    let (r3,r1,r2,r0) = spin_right4(r3,r1,r2,r0, & rr.right1);
    //println!("step6.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0, r1) = medium_inf(r2,r3,r0,r1, & rr.infrequent_medium1);                    //step7
    //println!("step7.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r0 = r0.wrapping_add(counter1);                                                 //step8
    let r1 = r1.wrapping_add(counter0);
    let (r2, r3) = medium_inf(r0,r1,r2,r3, & rr.infrequent_medium0);
    //println!("step8.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,&  rr.s2,& rr.s3);                    //step9
    let (r0,r1,r2,r3) = spin_right4(r0,r1,r2,r3, & rr.right0);
    let (r2,r0,r3,r1) = spin_right4(r2,r0,r3,r1, & rr.right1);
    let (r1,r3,r0,r2) = spin_10(r1,r3,r0,r2, & rr.v1k);
    //println!("step9.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = infreqen_5(r0,r1,r2,r3, & rr.infrequent_large1);                //step10
    //println!("step10   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = spin_right4(r0,r1,r2,r3, & rr.right0);                          //step11
    let (r1,r0,r3,r2) = spin_10(r1,r0,r3,r2, & rr.v1k);
    let (r3,r1,r2,r0) = spin_right4(r3,r1,r2,r0, & rr.right1);
    //println!("step11   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r0 = twist_and_turn(r0,rr.key_byte_r[8],rr.pp[8]);                              //step11.1
    let r1 = twist_and_turn(r1,rr.key_byte_r[9],rr.pp[9]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[10],rr.pp[10]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[11],rr.pp[11]);

    let r2 = r2.wrapping_add(rr.c3[counter4_idx as usize]);                             //step12
    let r3 = r3.wrapping_add(rr.c4[counter5_idx as usize]);

    let r0 = r0.wrapping_add(rr.infrequent_small2[r2.count_ones() as usize]);           //step13
    let r1 = r1.wrapping_add(rr.infrequent_small3[r3.count_ones() as usize]);
    //println!("step13   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = infreqen_4(r0,r1,r2,r3, & rr.infrequent_large2);                //step14
    //println!("step14   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s4,& rr.s5);                     //step15
    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s6,& rr.s7);
    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s8,& rr.s9);

    let r0 = twist_and_turn(r0,rr.key_byte_r[12],rr.pp[12]);                            //step16
    let r1 = twist_and_turn(r1,rr.key_byte_r[13],rr.pp[13]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[14],rr.pp[14]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[15],rr.pp[15]);
    //println!("step16   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    LittleEndian::write_u32(&mut buff[0..4],   r0);
    LittleEndian::write_u32(&mut buff[4..8],   r1);
    LittleEndian::write_u32(&mut buff[8..12],  r2);
    LittleEndian::write_u32(&mut buff[12..16], r3);
}

pub fn z_cry128(rr: & R128, buff: &mut [u8; 16], counter0: u32, counter1: u32){

    let r0 = LittleEndian::read_u32(&mut buff[0..4]);
    let r1 = LittleEndian::read_u32(&mut buff[4..8]);
    let r2 = LittleEndian::read_u32(&mut buff[8..12]);
    let r3 = LittleEndian::read_u32(&mut buff[12..16]);

    let c1_idx = counter0 % 89;
    let c2_idx = counter1 % 89;

    let v0 = rr.varray[0].wrapping_add(counter0);
    let v1 = rr.varray[1].wrapping_add(counter1);
    let v2 = rr.varray[2].wrapping_add(counter0);
    let v3 = rr.varray[3].wrapping_add(counter1);
    let v4 = rr.varray[4].wrapping_add(counter0);
    let v5 = rr.varray[5].wrapping_add(counter1);

    let counter0_idx = v2 % rr.x0;
    let counter1_idx = v3 % rr.x1;
    let counter2_idx = ((v0 / rr.large_cycle0) % rr.y0) + rr.x0;
    let counter3_idx = ((v1 / rr.large_cycle1) % rr.y1) + rr.x1;
    let counter4_idx = (v4 / rr.small_cycle0) % 103;
    let counter5_idx = (v5 / rr.small_cycle1) % 103;

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[12],rr.pp[12]);                  //step16
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[13],rr.pp[13]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[14],rr.pp[14]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[15],rr.pp[15]);

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s8,& rr.s9);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s6,& rr.s7);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s4,& rr.s5);           //step15

    let (r0,r1,r2,r3) = z_infreqen_4(r0,r1,r2,r3, & rr.infrequent_large2);      //step14

    let r0 = r0.wrapping_sub(rr.infrequent_small2[r2.count_ones() as usize]);   //step13
    let r1 = r1.wrapping_sub(rr.infrequent_small3[r3.count_ones() as usize]);

    let r2 = r2.wrapping_sub(rr.c3[counter4_idx as usize]);                     //step12
    let r3 = r3.wrapping_sub(rr.c4[counter5_idx as usize]);

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[8],rr.pp[8]);                    //step11.1
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[9],rr.pp[9]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[10],rr.pp[10]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[11],rr.pp[11]);

    let (r3,r1,r2,r0) = z_spin_right4(r3,r1,r2,r0, & rr.right1);
    let (r1,r0,r3,r2) = z_spin_10(r1,r0,r3,r2, & rr.v1k);
    let (r0,r1,r2,r3) = z_spin_right4(r0,r1,r2,r3, & rr.right0);                //step11

    let (r0,r1,r2,r3) = z_infreqen_5(r0,r1,r2,r3, & rr.infrequent_large1);      //step10

    let (r1,r3,r0,r2) = z_spin_10(r1,r3,r0,r2, & rr.v1k);
    let (r2,r0,r3,r1) = z_spin_right4(r2,r0,r3,r1, & rr.right1);
    let (r0,r1,r2,r3) = z_spin_right4(r0,r1,r2,r3, & rr.right0);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3, & rr.s2,& rr.s3);          //step9

    let (r2, r3) = z_medium_inf(r0,r1,r2,r3, & rr.infrequent_medium0);          //step8
    let r0 = r0.wrapping_sub(counter1);
    let r1 = r1.wrapping_sub(counter0);

    let (r0, r1) = z_medium_inf(r2,r3,r0,r1, & rr.infrequent_medium1);          //step7

    let (r3,r1,r2,r0) = z_spin_right4(r3,r1,r2,r0, & rr.right1);
    let (r1,r2,r3,r0) = z_spin_10(r1,r2,r3,r0, & rr.v1k);
    let (r2,r3,r0,r1) = z_spin_right4(r2,r3,r0,r1, & rr.right0);                //step6

    let r2 = r2.wrapping_sub(rr.pwq[counter2_idx as usize]);                    //step5.2
    let r3 = r3.wrapping_sub(rr.pwq[counter3_idx as usize]);

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[4],rr.pp[4]);                    //step5.1
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[5],rr.pp[5]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[6],rr.pp[6]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[7],rr.pp[7]);

    let r0 = r0.wrapping_sub(rr.pwq[counter0_idx as usize]);
    let r1 = r1.wrapping_sub(rr.pwq[counter1_idx as usize]);
    let (r0, r1) = z_single_strike(r3,r0,r1,& rr.infrequent_large1);            //step5

    let (r1,r3,r0,r2) = z_spin_right4(r1,r3,r0,r2, & rr.right1);
    let (r2,r1,r3,r0) = z_spin_10(r2,r1,r3,r0, & rr.v1k);
    let (r0,r1,r2,r3) = z_spin_right4(r0,r1,r2,r3, & rr.right0);                //step4

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s4,& rr.s5);           //step3.1

    let r0 = r0.wrapping_sub(rr.c1[c1_idx as usize]);                           //step3
    let r1 = r1.wrapping_sub(rr.c2[c2_idx as usize]);

    let r2 = r2.wrapping_sub(rr.infrequent_small0[r0.count_ones() as usize]);
    let r3 = r3.wrapping_sub(rr.infrequent_small1[r1.count_ones() as usize]);   //step2

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s2,& rr.s3);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s0,& rr.s1);           //step1.2

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[0],rr.pp[0]);
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[1],rr.pp[1]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[2],rr.pp[2]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[3],rr.pp[3]);                    //step1.1

    let (r0,r1,r2,r3) =
        z_add_counters(r0,r1,r2,r3,& rr.addr0,& rr.addr1,counter0,counter1);    //step0

    LittleEndian::write_u32(&mut buff[0..4],   r0);
    LittleEndian::write_u32(&mut buff[4..8],   r1);
    LittleEndian::write_u32(&mut buff[8..12],  r2);
    LittleEndian::write_u32(&mut buff[12..16], r3);
}

pub fn cry128_ecb(rr: & R128_ECB, buff: &mut [u8; 16]){

    let r0 = LittleEndian::read_u32(&mut buff[0..4]);
    let r1 = LittleEndian::read_u32(&mut buff[4..8]);
    let r2 = LittleEndian::read_u32(&mut buff[8..12]);
    let r3 = LittleEndian::read_u32(&mut buff[12..16]);

    let r0 = twist_and_turn(r0,rr.key_byte_r[0],rr.p[0]);
    let r1 = twist_and_turn(r1,rr.key_byte_r[1],rr.p[1]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[2],rr.p[2]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[3],rr.p[3]);
    //println!("step___   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s1,& rr.s2);                     //step0
    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s3,& rr.s4);
    //println!("step0.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r2 = r2.wrapping_add(rr.infrequent_small0[r0.count_ones() as usize]);           //step1
    let r3 = r3.wrapping_add(rr.infrequent_small1[r1.count_ones() as usize]);
    //println!("step1.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s5,& rr.s6);                     //step2
    let r0 = twist_and_turn(r0,rr.key_byte_r[4],rr.p[4]);
    let r1 = twist_and_turn(r1,rr.key_byte_r[5],rr.p[5]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[6],rr.p[6]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[7],rr.p[7]);
    //println!("step2.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0, r1) = single_strike(r3,r0,r1,& rr.infrequent_large0);                      //step2.1
    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s7,& rr.s8);
    //println!("step2.1   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0, r1) = medium_inf(r2,r3,r0,r1, & rr.infrequent_medium0);                    //step3
    //println!("step3.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r2, r3) = medium_inf(r0,r1,r2,r3, & rr.infrequent_medium1);                    //step3.1

    let (r0,r1,r2,r3) = spin_right4(r0,r1,r2,r3, & rr.right0);                          //step4
    let (r2,r1,r3,r0) = spin_10(r2,r1,r3,r0, & rr.v1k_a);
    let (r3,r1,r2,r0) = spin_right4(r3,r1,r2,r0, & rr.right1);
    //println!("step4.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = inf_xlarge(r0,r1,r2,r3, & rr.infrequent_xlarge0);               //step5
    //println!(" xlarge0:{}",rr.infrequent_xlarge0[0]);
    //println!("step5.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r2,r1,r3,r0) = spin_right4(r2,r1,r3,r0, & rr.right2);                          //step6
    let (r0,r1,r2,r3) = spin_10(r0,r1,r2,r3, & rr.v1k_b);
    let (r3,r1,r0,r2) = spin_right4(r3,r1,r0,r2, & rr.right3);
    //println!("step6.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = infreqen_5(r0,r1,r2,r3, & rr.infrequent_large0);                //step7

    let (r0,r2,r1,r3) = spin_right4(r0,r2,r1,r3, & rr.right4);                          //step8
    let (r1,r0,r2,r3) = spin_10(r1,r0,r2,r3, & rr.v1k_c);
    let (r3,r1,r2,r0) = spin_right4(r3,r1,r2,r0, & rr.right5);
    //println!("step8.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r0 = twist_and_turn(r0,rr.key_byte_r[8],rr.p[8]);                               //step8.1
    let r1 = twist_and_turn(r1,rr.key_byte_r[9],rr.p[9]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[10],rr.p[10]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[11],rr.p[11]);

    let (r0,r1,r2,r3) = infreqen_4(r0,r1,r2,r3, & rr.infrequent_large1);                //step9
    //println!("step9.0   r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s9,& rr.s10);                    //step10

    let (r0,r1,r2,r3) = inf_xlarge(r0,r1,r2,r3, & rr.infrequent_xlarge1);               //step11

    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s11,& rr.s12);                   //step12
    let (r0,r1,r2,r3) = circle_around(r0,r1,r2,r3,& rr.s13,& rr.s14);
    //println!("step12.0  r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    let r0 = twist_and_turn(r0,rr.key_byte_r[12],rr.p[12]);
    let r1 = twist_and_turn(r1,rr.key_byte_r[13],rr.p[13]);
    let r2 = twist_and_turn(r2,rr.key_byte_r[14],rr.p[14]);
    let r3 = twist_and_turn(r3,rr.key_byte_r[15],rr.p[15]);
    //println!("step_fff  r0:{} r1:{} r2:{} r3:{}",r0,r1,r2,r3);

    LittleEndian::write_u32(&mut buff[0..4],   r0);
    LittleEndian::write_u32(&mut buff[4..8],   r1);
    LittleEndian::write_u32(&mut buff[8..12],  r2);
    LittleEndian::write_u32(&mut buff[12..16], r3);
}


pub fn z_cry128_ecb(rr: & R128_ECB, buff: &mut [u8; 16]){

    let r0 = LittleEndian::read_u32(&mut buff[0..4]);
    let r1 = LittleEndian::read_u32(&mut buff[4..8]);
    let r2 = LittleEndian::read_u32(&mut buff[8..12]);
    let r3 = LittleEndian::read_u32(&mut buff[12..16]);

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[12],rr.p[12]);
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[13],rr.p[13]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[14],rr.p[14]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[15],rr.p[15]);

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s13,& rr.s14);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s11,& rr.s12);                 //step12

    let (r0,r1,r2,r3) = z_inf_xlarge(r0,r1,r2,r3, & rr.infrequent_xlarge1);             //step11

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s9,& rr.s10);                  //step10

    let (r0,r1,r2,r3) = z_infreqen_4(r0,r1,r2,r3, & rr.infrequent_large1);              //step9

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[8],rr.p[8]);                             //step8.1
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[9],rr.p[9]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[10],rr.p[10]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[11],rr.p[11]);

    let (r3,r1,r2,r0) = z_spin_right4(r3,r1,r2,r0, & rr.right5);
    let (r1,r0,r2,r3) = z_spin_10(r1,r0,r2,r3, & rr.v1k_c);
    let (r0,r2,r1,r3) = z_spin_right4(r0,r2,r1,r3, & rr.right4);                         //step8

    let (r0,r1,r2,r3) = z_infreqen_5(r0,r1,r2,r3, & rr.infrequent_large0);              //step7

    let (r3,r1,r0,r2) = z_spin_right4(r3,r1,r0,r2, & rr.right3);
    let (r0,r1,r2,r3) = z_spin_10(r0,r1,r2,r3, & rr.v1k_b);
    let (r2,r1,r3,r0) = z_spin_right4(r2,r1,r3,r0, & rr.right2);                        //step6

    let (r0,r1,r2,r3) = z_inf_xlarge(r0,r1,r2,r3, & rr.infrequent_xlarge0);             //step5

    let (r3,r1,r2,r0) = z_spin_right4(r3,r1,r2,r0, & rr.right1);
    let (r2,r1,r3,r0) = z_spin_10(r2,r1,r3,r0, & rr.v1k_a);
    let (r0,r1,r2,r3) = z_spin_right4(r0,r1,r2,r3, & rr.right0);                        //step4

    let (r2, r3) = z_medium_inf(r0,r1,r2,r3, & rr.infrequent_medium1);                  //step3.1

    let (r0, r1) = z_medium_inf(r2,r3,r0,r1, & rr.infrequent_medium0);                  //step3

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s7,& rr.s8);
    let (r0, r1) = z_single_strike(r3,r0,r1,& rr.infrequent_large0);                    //step2.1

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[4],rr.p[4]);
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[5],rr.p[5]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[6],rr.p[6]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[7],rr.p[7]);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s5,& rr.s6);                   //step2

    let r2 = r2.wrapping_sub(rr.infrequent_small0[r0.count_ones() as usize]);           //step1
    let r3 = r3.wrapping_sub(rr.infrequent_small1[r1.count_ones() as usize]);

    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s3,& rr.s4);
    let (r0,r1,r2,r3) = z_circle_around(r0,r1,r2,r3,& rr.s1,& rr.s2);                   //step0

    let r0 = z_twist_and_turn(r0,rr.key_byte_r[0],rr.p[0]);
    let r1 = z_twist_and_turn(r1,rr.key_byte_r[1],rr.p[1]);
    let r2 = z_twist_and_turn(r2,rr.key_byte_r[2],rr.p[2]);
    let r3 = z_twist_and_turn(r3,rr.key_byte_r[3],rr.p[3]);

    LittleEndian::write_u32(&mut buff[0..4],   r0);
    LittleEndian::write_u32(&mut buff[4..8],   r1);
    LittleEndian::write_u32(&mut buff[8..12],  r2);
    LittleEndian::write_u32(&mut buff[12..16], r3);
}


pub fn init_r128_ecb(r: &mut R128_ECB, exp_buff: & Vec<u8>,key_buff: &mut [u8; 16]) {
    let mut ee: [u8; EXPECTED_R_ECB_SIZE];

    unsafe{
        ee = mem::uninitialized();
    }
    expand_128(exp_buff, key_buff, EXPECTED_R_ECB_SIZE, &mut ee);

    let mut ptr: usize = 0;
    fn whack_ar(a: &mut [u32], ptr: &mut usize, ee: &mut [u8; EXPECTED_R_ECB_SIZE]){
        for elem in a.iter_mut() {
            *elem = LittleEndian::read_u32(&mut ee[*ptr..(*ptr+4)]);
            *ptr = *ptr + 4;
        }
    }

    fn whack_u8(a: &mut [u8], ptr: &mut usize, ee: &mut [u8; EXPECTED_R_ECB_SIZE]){
        for elem in a.iter_mut() {
            *elem = (ee[*ptr] % 31) + 1;
            *ptr = *ptr + 1;
        }
    }

    whack_u8(&mut r.key_byte_r,&mut ptr, &mut ee);

    whack_ar(&mut r.p, &mut ptr, &mut ee);

    whack_ar(&mut r.infrequent_small0, &mut ptr, &mut ee);
    whack_ar(&mut r.infrequent_small1, &mut ptr, &mut ee);

    whack_ar(&mut r.s1, &mut ptr, &mut ee);
    whack_ar(&mut r.s2, &mut ptr, &mut ee);
    whack_ar(&mut r.s3, &mut ptr, &mut ee);
    whack_ar(&mut r.s4, &mut ptr, &mut ee);
    whack_ar(&mut r.s5, &mut ptr, &mut ee);
    whack_ar(&mut r.s6, &mut ptr, &mut ee);
    whack_ar(&mut r.s7, &mut ptr, &mut ee);
    whack_ar(&mut r.s8, &mut ptr, &mut ee);
    whack_ar(&mut r.s9, &mut ptr, &mut ee);
    whack_ar(&mut r.s10, &mut ptr, &mut ee);
    whack_ar(&mut r.s11, &mut ptr, &mut ee);
    whack_ar(&mut r.s12, &mut ptr, &mut ee);
    whack_ar(&mut r.s13, &mut ptr, &mut ee);
    whack_ar(&mut r.s14, &mut ptr, &mut ee);

    whack_ar(&mut r.right0, &mut ptr, &mut ee);
    whack_ar(&mut r.right1, &mut ptr, &mut ee);
    whack_ar(&mut r.right2, &mut ptr, &mut ee);
    whack_ar(&mut r.right3, &mut ptr, &mut ee);
    whack_ar(&mut r.right4, &mut ptr, &mut ee);
    whack_ar(&mut r.right5, &mut ptr, &mut ee);

    whack_ar(&mut r.v1k_a, &mut ptr, &mut ee);
    whack_ar(&mut r.v1k_b, &mut ptr, &mut ee);
    whack_ar(&mut r.v1k_c, &mut ptr, &mut ee);

    whack_ar(&mut r.infrequent_xlarge0, &mut ptr, &mut ee);
    whack_ar(&mut r.infrequent_xlarge1, &mut ptr, &mut ee);

    whack_ar(&mut r.infrequent_large0, &mut ptr, &mut ee);
    whack_ar(&mut r.infrequent_large1, &mut ptr, &mut ee);

    whack_ar(&mut r.infrequent_medium0, &mut ptr, &mut ee);
    whack_ar(&mut r.infrequent_medium1, &mut ptr, &mut ee);

}

const EXPECTED_R_SIZE: usize = 34144;
const EXPECTED_R_ECB_SIZE: usize = 60768;

pub fn init_r128(rr128: &mut R128, exp_buff: & Vec<u8>,key_buff: &mut [u8; 16]){
    //let mut rng = rand::thread_rng();
    let mut ee: [u8; EXPECTED_R_SIZE];

    unsafe{
        ee = mem::uninitialized();
    }
    expand_128(exp_buff, key_buff, EXPECTED_R_SIZE, &mut ee);

    //let r3 = LittleEndian::read_u32(&mut buff[12..16]);
    let mut ptr: usize = 0;
    fn whack_ar(a: &mut [u32], ptr: &mut usize, ee: &mut [u8; EXPECTED_R_SIZE]){
        for elem in a.iter_mut() {
            *elem = LittleEndian::read_u32(&mut ee[*ptr..(*ptr+4)]);
            *ptr = *ptr + 4;
        }
    }

    fn single_u32(ptr: &mut usize, ee: &mut [u8; EXPECTED_R_SIZE])->(u32){
        let r = LittleEndian::read_u32(&mut ee[*ptr..(*ptr+4)]);
        *ptr = *ptr + 4;
        r
    }

    fn whack_u8(a: &mut [u8], ptr: &mut usize, ee: &mut [u8; EXPECTED_R_SIZE]){
        for elem in a.iter_mut() {
            *elem = (ee[*ptr] % 31) + 1;
            *ptr = *ptr + 1;
        }
    }

    whack_ar(&mut rr128.infrequent_small0, &mut ptr, &mut ee);
    whack_ar(&mut rr128.infrequent_small1, &mut ptr, &mut ee);

    rr128.x0 = utility::get_prime(((single_u32(&mut ptr, &mut ee) % 32) + 56) as usize);
    rr128.y0 = (PWQ_SIZE as u32) - rr128.x0;
    single_u32(&mut ptr, &mut ee);

    rr128.x1 = utility::get_prime(((single_u32(&mut ptr, &mut ee) % 32) + 56) as usize);
    rr128.y1 = (PWQ_SIZE as u32) - rr128.x1;
    single_u32(&mut ptr, &mut ee);

    rr128.small_cycle0 = utility::get_prime(((single_u32(&mut ptr, &mut ee) % 11) + 11) as usize);
    rr128.small_cycle1 = utility::get_prime(((single_u32(&mut ptr, &mut ee) % 11) + 11) as usize);

    rr128.large_cycle0 =  utility::get_prime(((single_u32(&mut ptr, &mut ee) % 420) + 190) as usize);
    rr128.large_cycle1 =  utility::get_prime(((single_u32(&mut ptr, &mut ee) % 420) + 190) as usize);

    whack_ar(&mut rr128.varray, &mut ptr, &mut ee);

    //println!(" ptrs_small:{} {}",ptr,272);
    whack_ar(&mut rr128.addr0, &mut ptr, &mut ee);
    whack_ar(&mut rr128.addr1, &mut ptr, &mut ee);
    //println!(" ptrs_addr:{} {}",ptr,2320);
    whack_ar(&mut rr128.s0, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s1, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s2, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s3, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s4, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s5, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s6, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s7, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s8, &mut ptr, &mut ee);
    whack_ar(&mut rr128.s9, &mut ptr, &mut ee);
    //println!(" ptrs_s:{} {}",ptr,12560);

    whack_ar(&mut rr128.right0, &mut ptr, &mut ee);
    whack_ar(&mut rr128.right1, &mut ptr, &mut ee);
    //println!(" ptrs_s:{} {}",ptr,16656);

    whack_ar(&mut rr128.v1k, &mut ptr, &mut ee);
    //println!(" ptrs_s:{} {}",ptr,20752);

    whack_ar(&mut rr128.infrequent_large1, &mut ptr, &mut ee);
    whack_ar(&mut rr128.infrequent_large2, &mut ptr, &mut ee);
    //println!(" ptrs_s:{} {}",ptr,28944);

    whack_u8(&mut rr128.key_byte_r, &mut ptr, &mut ee);
    //println!(" ptrs_s:{} {}",ptr,28976);

    whack_ar(&mut rr128.pp, &mut ptr, &mut ee);
    //println!(" ptrs_pp:{} {}",ptr,29136);

    whack_ar(&mut rr128.pwq, &mut ptr, &mut ee);
    //println!(" ptrs_pwq:{} {}",ptr,31296);

    whack_ar(&mut rr128.c1, &mut ptr, &mut ee);
    whack_ar(&mut rr128.c2, &mut ptr, &mut ee);
    //println!(" ptrs_c_1_2:{} {}",ptr,32016);

    whack_ar(&mut rr128.c3, &mut ptr, &mut ee);
    whack_ar(&mut rr128.c4, &mut ptr, &mut ee);
    //println!(" ptrs_c_3_4:{} {}",ptr,32864);

    //println!(" memory size:{}",mem::size_of::<R128>());
    //println!(" ptr0:{}",ptr);

    whack_ar(&mut rr128.infrequent_medium0, &mut ptr, &mut ee);
    //println!(" ptr1:{}",ptr);
    whack_ar(&mut rr128.infrequent_medium1, &mut ptr, &mut ee);

    //println!(" ptr2:{}",ptr);

    whack_ar(&mut rr128.infrequent_small2, &mut ptr, &mut ee);
    whack_ar(&mut rr128.infrequent_small3, &mut ptr, &mut ee);

}




