use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use syn::{Data, DeriveInput, Fields, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(KdbSqlInserter)]
pub fn to_sql_inserter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new(
                    input.span(),
                    "GenerateTable can only be applied to structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new(input.span(), "GenerateTable can only be applied to structs")
                .to_compile_error()
                .into();
        }
    };

    let fields: Vec<_> = fields.iter().collect();

    let mut func_stream = TokenStream2::default();
    for f in fields.into_iter() {
        let field_name = f.ident.as_ref().unwrap();
        let field_name = field_name.to_string();
        let db_field_ident = format_ident!("{}", field_name.to_uppercase());
        let field_indent = format_ident!("{}", field_name);
        func_stream.extend(quote! { .field(Self::#db_field_ident, self.#field_indent) });
    }

    let expanded = quote! {
        impl crate::mapper::db::ToSqlInserter for #name {
            fn to_sql_inserter(self) -> chin_sql::SqlInserter<'static> {
                chin_sql::SqlInserter::new(Self::TABLE)
                    #func_stream
            }
        }
    };

    TokenStream::from(expanded)
}
