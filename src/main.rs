#![feature(portable_simd)]


use std::simd::f32x4;
use bitsvec::BitVec;


fn main() {
    let a = f32x4::splat(10.0);
    let b = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", a + b);
    let mut bitvec = BitVec::ones(1000); // create a bitvec contains 0 ..= 999
    bitvec.set(900, false); // delete 900 from bitvec
    bitvec.set(1200, true); // add 1200 to bitvec (and expand bitvec to length 1201)
    println!("{:?}", bitvec.get(900));
    println!("{:?}", bitvec.get(1200));
    println!("{:?}", bitvec.get(1000));
}