use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote, format_ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", ast);

    // Parse struct names.
    let struct_name = ast.ident;
    let builder_name = format_ident!("{}Builder", &struct_name);

    // Parse field names.
    let fields = match ast.data {
        syn::Data::Struct(data_struct) => {
            data_struct.fields
        },
        _ => todo!()
    };
    let field_names: Vec<_> = match fields {
        syn::Fields::Named(ref named_fields) => {
            named_fields.named.iter()
                .map(|field| field.ident.clone())
                .collect()
        },
        _ => todo!()
    };
    // println!("{:#?}", field_names);

    // Parse field types.
    let field_types: Vec<_> = match fields {
        syn::Fields::Named(ref named_fields) => {
            named_fields.named.iter()
                .map(|field| field.ty.clone())
                .collect()
        },
        _ => todo!()
    };

    let generated = quote! {
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: None),*
                }
            }
        }

        pub struct #builder_name {
            #(#field_names: Option<#field_types>),*
        }

        impl #builder_name {
            #(
            fn #field_names(&mut self, #field_names: #field_types) -> &mut Self {
                self.#field_names= Some(#field_names);
                self
            }
            )*

            pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                Ok(#struct_name {
                    #(
                        #field_names: self.#field_names.clone()
                            .ok_or_else(|| format!("Field {} not set", stringify!(#field_names)))?
                    ),*
                })
            }
        }
    };
    // println!("{:#?}", generated);
    println!("{}", generated.to_string());

    TokenStream::from(generated)
}
