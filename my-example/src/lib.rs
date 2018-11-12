#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;
use std::os::raw::c_char;

extern crate epics_sys;

use epics_sys::epics_register;

#[epics_register]
pub fn my_subroutine_impl(record: &mut subRecord) -> Result<(), ()> {
    let rec_name = unsafe {CStr::from_ptr(&record.name as *const c_char)};
    println!("Hello, I was called! {}", rec_name.to_str().unwrap());
    Ok(())
    //Err(())
}

// #[epics_register]
// pub fn invalid(record: &mut subRecord) -> Result<(),()> {
//     Ok(())
// }
//
// #[epics_register]
// pub fn invalid2_impl() -> Result<(),()> {
//     Ok(())
// }
//
// #[epics_register]
// pub fn invalid3_impl(record: subRecord) -> Result<(),()> {
//     Ok(())
// }
//
// TODO: Add support for rust structs. Put opaque pointer to self in dpvt
// struct Temp;
//
// impl Temp {
//     #[epics_register]
//     pub fn invalid3_impl(self, rec: subRecord) -> Result<(),()> {
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use crate::my_subroutine;
    use crate::subRecord;
    use std::mem;
    #[test]
    fn it_works() {
        let mut record:subRecord = unsafe{mem::zeroed()};
        let a = my_subroutine(&mut record as *mut subRecord);

        let res = match a {
            1 => "Failed",
            0 => "Success",
            _ => "Unknown",
        };
        println!("{}", res)

    }
}
