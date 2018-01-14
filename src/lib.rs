use std::ffi::CStr;
use std::str::Utf8Error;

// Cannot create function names until concat_idents!() is fixed
// https://github.com/rust-lang/rust/issues/29599
#[macro_export]
macro_rules! epics_register_function {
    ( $func:ident, $func_priv:ident, $type:ty, $regfunc:ident, $pvarregfunc:ident ) => {
        #[no_mangle]
        pub extern "C" fn $func(precord: *mut $type) -> ::std::os::raw::c_long {
            match $func_priv(unsafe {&mut *precord}) {
                Ok(()) => 0,
                Err(()) => 1,
            }
        }

        #[no_mangle]
        pub fn $regfunc() {
            unsafe {
                registryFunctionAdd(
                    "$func\0".as_ptr() as *const _,
                    Some(::std::mem::transmute::<extern "C" fn(*mut $type) -> ::std::os::raw::c_long, unsafe extern "C" fn()>($func)));
            }
        }

        #[no_mangle]
        pub static mut $pvarregfunc: *const ::std::os::raw::c_void = $regfunc as *const ::std::os::raw::c_void;

    };
}

// AsRef is not implemneted on [i8, 61]
pub fn str_from_epics(input: &[i8]) -> Result<&str, Utf8Error>
{
    unsafe {CStr::from_ptr(input.as_ptr())}.to_str()
}

#[cfg(test)]
mod tests {
    use ::str_from_epics;
    #[test]
    fn it_works() {
        let x : [i8; 6] = ['h' as i8, 'e' as i8, 'l' as i8, 'l' as i8, 'o' as i8, '\0' as i8];
        assert_eq!(str_from_epics(&x).unwrap(), "hello");
    }
}
