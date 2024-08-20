use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Ident};

pub fn derive_cli_command(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;

    match input.data {
        Data::Enum(ref de) => {
            let g = impl_cli_command(ident, de);
            Ok(g)
        }
        _ => {
            let err = syn::Error::new(
                proc_macro2::Span::call_site(),
                "`#[derive(shinc_derive::CliCommand)]` only supports enums",
            );
            Err(err)
        }
    }
}

fn impl_cli_command(ident: &Ident, de: &DataEnum) -> TokenStream {
    let run = gen_run(ident, de);
    quote! {
        impl CliCommand for #ident {
            #run
        }
    }
}

fn gen_run(ident: &Ident, de: &DataEnum) -> TokenStream {
    let subcommands = de.variants.iter().map(|variant| &variant.ident);
    quote! {
        fn run(&self) -> CliResult {
            match self {
                #(#ident::#subcommands(cmd) => cmd.run()),*
            }
        }
    }
}
