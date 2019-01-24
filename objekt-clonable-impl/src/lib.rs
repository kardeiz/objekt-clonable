extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use syn::*;

#[proc_macro_attribute]
pub fn clonable(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_trait = parse_macro_input!(item as ItemTrait);


    let item_trait_ident = &item_trait.ident;

    let cloneish_paths: &[Path] = &[
        parse_quote!(Clone),
        parse_quote!(std::clone::Clone),
        parse_quote!(::std::clone::Clone),
    ];

    if let Some(path) = item_trait
        .supertraits
        .iter_mut()
        .filter_map(|x| match x {
            TypeParamBound::Trait(ref mut y) => Some(y),
            _ => None
        })
        .map(|x| &mut x.path)
        .find(|x| cloneish_paths.iter().any(|y| &y == x))
    {
        *path = parse_quote!(objekt_clonable::objekt::Clone);
    } else {
        panic!("`Clone` must be present in trait supertrait list");
    }

    let (impl_generics, ty_generics, where_clause) = item_trait.generics.split_for_impl();

    (quote! {
        #item_trait
        objekt_clonable::objekt::clone_trait_object!(#impl_generics #item_trait_ident #ty_generics #where_clause);
    })
    .into()
}
