#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate epics_sys;

use epics_sys::epics_register;

#[epics_register]
pub fn my_subroutine(record: &mut subRecord) -> Result<(), ()> {
    println!("Hello!");
    Ok(())
    //Err(())
}

#[cfg(test)]
mod tests {
    use crate::my_subroutine_priv;
    use crate::my_subroutine;
    use crate::subRecord;
    use std::mem;
    #[test]
    fn it_works() {
        let mut record:subRecord = unsafe{mem::zeroed()};
        let a = my_subroutine_priv(&mut record as *mut subRecord);

        let res = match a {
            1 => "Failed",
            0 => "Success",
            _ => "Unknown",
        };
        println!("{}", res)

    }
}
