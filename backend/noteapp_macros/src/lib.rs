extern crate proc_macro2;
use proc_macro::TokenStream;
use syn::{ parse, Fields, Ident, ItemEnum, Variant };
use quote::quote;

#[proc_macro_derive(StorageAbstract)]
pub fn custom_enum_dispatch(item: TokenStream) -> TokenStream {
    let ast: ItemEnum = match parse(item) {
        Ok(parsed) => parsed,
        Err(_) => panic!("#[derive(StorageAbstract)] can only be used with `Enums`"),
    };
    let output = impl_macro(&ast);
    TokenStream::from(output)
}

fn impl_macro(ast: &ItemEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let methods = vec![("get_contents", vec!["path"]), ("write_contents", vec!["path", "data"])];

    let match_lex = methods
        .into_iter()
        .map(|(method_name, args)| {
            ast.variants
                .iter()
                .filter_map(move |variant| generate_match(variant, method_name, &args, name))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let a = &match_lex[0];
    let b = &match_lex[1];

    quote! {
            impl StorageAbstract for #name {
                async fn get_contents(&self, path: &str) -> Result<Option<String>, Error> {
                    match self {
                        #(#a)*
                    }
                }
                async fn write_contents(&self, path: &str, data: String) -> Result<(), Error> {
                    match self {
                        #(#b)*
                    }
                }
            }
        }
}

fn generate_match(
    variant: &Variant,
    method_name: &str,
    args: &[&str],
    enum_name: &Ident
) -> Option<proc_macro2::TokenStream> {
    match &variant.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let name = &variant.ident;
            let method_ident = Ident::new(method_name, proc_macro2::Span::call_site());
            let args_tokens = args
                .iter()
                .map(|&arg| Ident::new(arg, proc_macro2::Span::call_site()));
            Some(
                quote! {
                    #enum_name::#name(inner) => inner.#method_ident(#(#args_tokens),*).await,
                }
            )
        }
        _ => None,
    }
}
