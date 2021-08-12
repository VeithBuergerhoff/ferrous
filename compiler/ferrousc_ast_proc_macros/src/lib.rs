extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

// TODO: build accesors for fields and ::new() method
#[proc_macro_attribute]
pub fn node(_attr: TokenStream, item: TokenStream) -> TokenStream {  
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;

    let code = quote!(
        #struct_node

        impl Node for #name {}
        impl SyntaxNode for #name { 
            fn get_trivia(&self) -> &Vec<Box<dyn Trivia>> { 
                &self.trivia 
            }
        } 
    );
    code.into()
}

#[proc_macro_attribute]
pub fn trivia(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;
    
    let code = quote!(
        #struct_node

        impl Node for #name {} 
        impl Trivia for #name {} 
    );

    code.into()
}

#[proc_macro_attribute]
pub fn statement(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;
    
    let code = quote!(
        #struct_node

        impl Node for #name {}
        impl SyntaxNode for #name { 
            fn get_trivia(&self) -> &Vec<Box<dyn Trivia>> { 
                &self.trivia 
            }
        } 
        impl Statement for #name {} 
    );

    code.into()
}

#[proc_macro_attribute]
pub fn expression(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_node = parse_macro_input!(item as ItemStruct);   
    let name = struct_node.clone().ident;
    
    let code = quote!(
        #struct_node

        impl Node for #name {}
        impl SyntaxNode for #name { 
            fn get_trivia(&self) -> &Vec<Box<dyn Trivia>> { 
                &self.trivia 
            }
        } 
        impl Expression for #name {} 
    );

    code.into()
}