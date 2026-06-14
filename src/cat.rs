use std::ffi::CString;
use std::io;
use rand::random_range;

pub fn main(catstring: &str, ) {
    println!("{} Is the value of catstring", catstring);
}

pub fn returnval(oneplus:u32) -> u32 {
    println!("{} Is the value of oneplus", oneplus);
    let adddatshit = oneplus + 1;
    println!("{} Is after we adddatshit", adddatshit);
    adddatshit
}