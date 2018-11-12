#![feature(extern_crate_item_prelude)]
#![feature(concat_idents)]
#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate paste;

use quote::quote;
use syn::spanned::Spanned;


#[proc_macro_attribute]
pub fn epics_register(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_epics_register(&ast).into()
}

fn impl_epics_register(ast: &syn::ItemFn) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    if ! name_str.ends_with("_impl") {
        return syn::parse::Error::new(name.span(), "expected name to end with `_impl`").to_compile_error();
    }
    let name2 = syn::Ident::new(name_str.trim_end_matches(&"_impl"), name.span());

    if ast.decl.inputs.len() != 1 {
        return syn::parse::Error::new(
            ast.decl.paren_token.span,
            "expected function to have 1 argument"
        ).to_compile_error();

    }
    let rec_type = &ast.decl.inputs.first().unwrap().into_value();
    let rec_type = match rec_type {
        syn::FnArg::Captured(arg) => Some(&arg.ty),
        _ => {
            return syn::parse::Error::new(
                rec_type.span(),
                "unknown argument"
            ).to_compile_error();
        },
    }.unwrap();
    let rec_type = match rec_type {
        syn::Type::Reference(ty) => Some(ty.elem.as_ref()),
        _ => {
            return syn::parse::Error::new(
                rec_type.span(),
                "expected reference"
            ).to_compile_error();
        },
    }.unwrap();
    let rec_type = match rec_type {
        syn::Type::Path(ty) => Some(ty),
        _ => {
            return syn::parse::Error::new(
                rec_type.span(),
                "expected path/type"
            ).to_compile_error();
        },
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
