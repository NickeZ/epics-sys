#![feature(extern_crate_item_prelude)]
#![feature(concat_idents)]
#![recursion_limit="128"]
// use std::ffi::CStr;
// use std::str::Utf8Error;

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate paste;

use quote::quote;


#[proc_macro_attribute]
pub fn epics_register(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_epics_register(&ast).into()
}

fn impl_epics_register(ast: &syn::ItemFn) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    if ! name_str.ends_with("_impl") {
        return syn::parse::Error::new(name.span(), "Expected name to end with `_impl`").to_compile_error();
    }
    let name2 = syn::Ident::new(name_str.trim_end_matches(&"_impl"), name.span());
    println!("{}", name2);
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
            pub extern "C" fn #name2(precord: *mut #rec_type) -> c_long {
                match #name(unsafe {&mut *precord}) {
                    Ok(()) => 0,
                    Err(()) => 1,
                }
            }

            #[no_mangle]
            pub fn [<register_func_ #name2>]() {
                use std::mem;
                let fnname = format!("{}\0", stringify!(#name2));
                unsafe {
                    registryFunctionAdd(
                        fnname.as_ptr() as *const _,
                        Some(mem::transmute::<extern "C" fn(*mut #rec_type) -> c_long, unsafe extern "C" fn()>(#name2)));
                }
            }

            #[no_mangle]
            pub static mut [<pvar_func_register_func_ #name2>]: *const c_void = [<register_func_ #name2>] as *const c_void;

        }

        #ast
    };
    gen.into()
}

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
