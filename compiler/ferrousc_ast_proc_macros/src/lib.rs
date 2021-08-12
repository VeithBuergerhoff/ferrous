extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;
    
    let code = quote!(
        #struct_node
        node!(#name);
    );

    code.into()
}

#[proc_macro_attribute]
pub fn stat(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;
    
    let code = quote!(
        #struct_node
        node!(#name);
        stat!(#name);
    );

    code.into()
}

#[proc_macro_attribute]
pub fn expr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;
    
    let code = quote!(
        #struct_node
        node!(#name);
        expr!(#name);
    );

    code.into()
}