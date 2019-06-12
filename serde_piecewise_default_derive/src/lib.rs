//! See the [parent crate documentation](https://docs.rs/serde_piecewise_default/) for more information.
#![recursion_limit="512"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate serde;

use crate::proc_macro::TokenStream as RustTS;
use crate::proc_macro2::TokenStream;
use syn::*;
use syn::spanned::Spanned;
use proc_macro2::Span;

/// See the [parent crate documentation](https://docs.rs/serde_piecewise_default/) for a high-level overview.
///
/// # Implementation Details
///
/// ```rust,no_run
/// # use serde_piecewise_default_derive::DeserializePiecewiseDefault;
/// # use serde::Deserialize;
/// #[derive(DeserializePiecewiseDefault)]
/// struct Example {
///     item1: i8,
///     item2: String,
/// }
/// # impl Default for Example {
/// #     fn default() -> Example { Example { item1: 10, item2: "Hi".to_string() }}
/// # }
/// ```
/// will expand to
/// ```rust,no_run
/// struct Example {
///     item1: i8,
///     item2: String,
/// }
/// # impl Default for Example {
/// #     fn default() -> Example { Example { item1: 10, item2: "Hi".to_string() }}
/// # }
///
/// # use serde::Deserialize;
/// #[derive(Deserialize)]
/// struct OptionExample {
///     item1: Option<i8>,
///     item2: Option<String>,
/// }
///
/// impl<'de> Deserialize<'de> for Example {
///     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
///         <OptionExample as Deserialize>::deserialize(deserializer)
///             .map(|raw_result| {
///                 let OptionExample { item1, item2 } = raw_result;
///                 let default = <Example as Default>::default();
///                 let item1 = item1.unwrap_or(default.item1);
///                 let item2 = item2.unwrap_or(default.item2);
///                 Example { item1, item2 }
///             })
///     }
/// }
/// ```
#[proc_macro_derive(DeserializePiecewiseDefault)]
pub fn deserialize_piecewise_default_derive(input: RustTS) -> RustTS {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_deserialize_piecewise_default(&ast).into()
}

fn impl_deserialize_piecewise_default(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match ast.data {
        Data::Struct(ref data) => {
            let optional_struct = make_optional_struct(data, name);
            let optional_name = get_optional_name(name);
            let field_binding = make_structure(data, &optional_name);
            let field_movements = make_field_movements(data);
            let result = make_structure(data, name);

            quote! {
                #optional_struct
                impl<'de> Deserialize<'de> for #name {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                        <#optional_name as Deserialize>::deserialize(deserializer)
                            .map(|raw_result| {
                                let #field_binding = raw_result;
                                let default = <#name as Default>::default();
                                #field_movements
                                #result
                            })
                    }
                }
            }
        },
        _ => panic!("can only use piecewise default on structs")
    }
}

fn get_structure_name((i, field): (usize, &Field)) -> Ident {
    match field.ident {
        Some(ref ident) => ident.clone(),
        None => {
            let name = format!("f{}", i);
            Ident::new(&name, Span::call_site())
        }
    }
}

fn get_optional_name(orig_name: &Ident) -> Ident {
    Ident::new(&format!("Option{}", orig_name), orig_name.span())
}

fn make_optional_struct(data: &DataStruct, orig_name: &Ident) -> TokenStream {
    let new_name = get_optional_name(orig_name);
    let fields = data.fields.iter().map(make_field_optional).collect::<Vec<_>>();
    let struct_decl_body = match data.fields {
        Fields::Named(_) => quote!{
            {
                #(#fields),*
            }
        },
        Fields::Unnamed(_) => {
            // wasn't my idea, serde itself doesn't like deserializing tuples of Option with some holes
            panic!("can only use piecewise default with named fields")
        }
//        quote!{
//            (#(#fields),*);
//        },
        Fields::Unit => quote!{;}
    };
    quote! {
        #[derive(Deserialize)]
        struct #new_name #struct_decl_body
    }
}

fn make_field_optional(field: &Field) -> TokenStream {
    let Field { attrs, vis, ident, colon_token, ty } = field;
    let span = field.span();
    quote_spanned!(span=> #(#attrs)* #vis #ident #colon_token Option<#ty>)
}

fn make_structure(data: &DataStruct, name: &Ident) -> TokenStream {
    let fields = data.fields.iter().enumerate().map(make_field_structure).collect::<Vec<_>>();
    match data.fields {
        Fields::Unnamed(_) => quote!(#name(#(#fields),*)),
        Fields::Named(_) => quote!(#name{#(#fields),*}),
        Fields::Unit => quote!(#name()),
    }
}

fn make_field_structure((i, field): (usize, &Field)) -> TokenStream {
    let ident = get_structure_name((i, field));
    quote!(#ident)
}

fn make_field_movements(data: &DataStruct) -> TokenStream {
    let movements = data.fields.iter().enumerate()
        .map(|x| (x, get_structure_name(x)))
        .map(make_field_movement)
        .collect::<Vec<_>>();
    quote! {
        #(#movements
        )*
    }
}

fn make_field_movement((orig_field, structure_name): ((usize, &Field), Ident)) -> TokenStream {
    let orig_name = match orig_field.1.ident {
        Some(ref ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(Index { index: orig_field.0 as u32, span: orig_field.1.span() })
    };
    quote!(let #structure_name = #structure_name.unwrap_or(default.#orig_name);)
}
