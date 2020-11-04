#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

mod actor;
mod arbiter_service;
mod message;
mod message_response;
mod supervised;
mod utils;

#[proc_macro_derive(Message, attributes(rtype))]
pub fn message_derive_rtype(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    message::expand(&ast).into()
}

#[proc_macro_derive(MessageResponse)]
pub fn message_response_derive_rtype(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    message_response::expand(&ast).into()
}

#[proc_macro_derive(Actor, attributes(actor))]
pub fn actor_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    actor::expand(&ast).into()
}

#[proc_macro_derive(Supervised)]
pub fn supervised_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    supervised::expand(&ast).into()
}

#[proc_macro_derive(ArbiterService)]
pub fn arbiter_service_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    arbiter_service::expand(&ast).into()
}
