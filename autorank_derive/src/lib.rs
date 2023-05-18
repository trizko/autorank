use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Card)]
pub fn card_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_card_macro(&ast)
}

fn impl_card_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Card for #name {
            fn get_attack(&self) -> u32 {
                self.attack
            }

            fn get_health(&self) -> u32 {
                self.health
            }
        }
    };
    gen.into()
}