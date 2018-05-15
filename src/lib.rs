#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use syn::*;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn thread_spawn(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: ItemFn = match parse(input) {
        Ok(input) => input,
        Err(..) => panic!("#[spawn()] must be applied on functions"),
    };

    if input.decl.generics.where_clause.is_some() {
        panic!("#[spawn()] does not work with where clauses")
    }

    let mut args = vec![];

    let mut where_args = vec![];
    let mut pat_args = vec![];
    for arg in input.decl.inputs.iter() {
        match *arg {
            FnArg::Captured(ref cap) => {
                let ty = &cap.ty;
                let pat = &cap.pat;
                where_args.push(quote!(#ty));
                pat_args.push(quote!(#pat));
                args.push(quote!(#pat: #ty));
            }
             _ => panic!("Unexpected argument {:?}", arg)
        }
    }

    let funcname = &input.ident;
    let attributes = &input.attrs;
    let vis = &input.vis;
    let constness = &input.constness;
    let unsafety = &input.unsafety;
    let abi = &input.abi;
    let output = match input.decl.output {
        syn::ReturnType::Default => {
            panic!("expected return type");
        }
        syn::ReturnType::Type(_, ref ret) => {
            ret
        }
    };
    let body = &input.block;

    quote!(
        #(#attributes),*
        #vis #constness #unsafety #abi fn #funcname (#(#args),*) -> ::std::thread::JoinHandle<#output> {
            ::std::thread::Builder::new()
                .name(stringify!(#funcname).to_string())
                .spawn(move || {
                    #body
                })
                .unwrap()
        }
    ).into()
}
