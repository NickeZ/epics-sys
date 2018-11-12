#![feature(extern_crate_item_prelude)]
#![feature(concat_idents)]
#![recursion_limit="128"]
// use std::ffi::CStr;
// use std::str::Utf8Error;

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate paste;

use proc_macro::TokenStream;
use quote::quote;


#[proc_macro_attribute]
pub fn epics_register(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_epics_register(&ast)
}

fn impl_epics_register(ast: &syn::ItemFn) -> TokenStream {
    let name = &ast.ident;
    let rec_type = &ast.decl.inputs.first().unwrap().into_value();
    let rec_type = match rec_type {
        syn::FnArg::Captured(arg) => Some(&arg.ty),
        _ => None,
    }.unwrap();
    let rec_type = match rec_type {
        syn::Type::Reference(ty) => Some(ty.elem.as_ref()),
        _ => None,
    }.unwrap();
    let rec_type = match rec_type {
        syn::Type::Path(ty) => Some(ty),
        _ => None,
    }.unwrap();
    //println!("{:#?}", rec_type);

    let gen = quote! {
        use std::os::raw::{c_long, c_void};
        paste::item! {
            #[no_mangle]
            pub extern "C" fn [<#name _priv>](precord: *mut #rec_type) -> c_long {
                match #name(unsafe {&mut *precord}) {
                    Ok(()) => 0,
                    Err(()) => 1,
                }
            }

            #[no_mangle]
            pub fn [<register_func_ #name _priv>]() {
                use std::mem;
                let fnname = format!("{}_priv\0", stringify!(#name));
                unsafe {
                    registryFunctionAdd(
                        fnname.as_ptr() as *const _,
                        Some(mem::transmute::<extern "C" fn(*mut #rec_type) -> c_long, unsafe extern "C" fn()>([<#name _priv>])));
                }
            }

            #[no_mangle]
            pub static mut [<pvar_func_register_func_ #name _priv>]: *const c_void = [<register_func_ #name _priv>] as *const c_void;

        }

        #ast
    };
    gen.into()
}

//
// Cannot create function names until concat_idents!() is fixed
// https://github.com/rust-lang/rust/issues/29599
//#[macro_export]
//macro_rules! epics_register_function {
//    ( $func:ident, $func_priv:ident, $type:ty, $regfunc:ident, $pvarregfunc:ident ) => {
//        #[no_mangle]
//        pub extern "C" fn $func(precord: *mut $type) -> ::std::os::raw::c_long {
//            match $func_priv(unsafe {&mut *precord}) {
//                Ok(()) => 0,
//                Err(()) => 1,
//            }
//        }
//
//        #[no_mangle]
//        pub fn $regfunc() {
//            unsafe {
//                registryFunctionAdd(
//                    "$func\0".as_ptr() as *const _,
//                    Some(::std::mem::transmute::<extern "C" fn(*mut $type) -> ::std::os::raw::c_long, unsafe extern "C" fn()>($func)));
//            }
//        }
//
//        #[no_mangle]
//        pub static mut $pvarregfunc: *const ::std::os::raw::c_void = $regfunc as *const ::std::os::raw::c_void;
//
//    };
//}

// AsRef is not implemneted on [i8, 61]
//pub fn str_from_epics(input: &[i8]) -> Result<&str, Utf8Error>
//{
//    unsafe {CStr::from_ptr(input.as_ptr())}.to_str()
//}
//
//#[cfg(test)]
//mod tests {
//    use ::str_from_epics;
//    #[test]
//    fn it_works() {
//        let x : [i8; 6] = ['h' as i8, 'e' as i8, 'l' as i8, 'l' as i8, 'o' as i8, '\0' as i8];
//        assert_eq!(str_from_epics(&x).unwrap(), "hello");
//    }
//}
