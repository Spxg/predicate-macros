use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::Data;
use syn::DataEnum;
use syn::DeriveInput;
use syn::Variant;

#[proc_macro_attribute]
pub fn add_fields(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as DeriveInput);
    let item_name = item.ident.clone();

    let unit_field = format!("Unit(OpUnitRcType<OpUnit<{}>>)", item_name);
    let unit_field: TokenStream = unit_field.parse().expect("parse error");
    let unknown_field: TokenStream = "Unknown".parse().expect("parse error");
    let unit = parse_macro_input!(unit_field as Variant);
    let unknown = parse_macro_input!(unknown_field as Variant);

    match item.data {
        Data::Enum(DataEnum {
            ref mut variants, ..
        }) => {
            variants.push(unit);
            variants.push(unknown);
        }
        _ => panic!("only impl for enum"),
    }

    let tokens: proc_macro2::TokenStream = item.into_token_stream();
    quote!(
        #tokens

        impl std::default::Default for #item_name {
            fn default() -> Self {
                Self::Unknown
            }
        }
    )
    .into()
}

#[proc_macro_derive(OpUnitTrait)]
pub fn impl_op_unit_trait(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let item_name = &item.ident;

    quote!(
        impl OpUnitTrait for #item_name {
            fn get_op_unit(self: &OpUnitRcType<Self>) -> OpUnitRcType<OpUnit<Self>> {
                match self.as_ref() {
                    #item_name::Unit(unit) => unit.clone(),
                    _ => OpUnitRcType::new(OpUnit::new(Some(self.clone()), None, Operation::Single)),
                }
            }
        }
    )
    .into()
}

#[proc_macro_derive(BitAnd)]
pub fn impl_bit_and(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let item_name = &item.ident;

    quote!(
        impl std::ops::BitAnd for #item_name {
            type Output = Self;
        
            fn bitand(self, rhs: Self) -> Self::Output {
                let node = OpUnit::new(
                    Some(OpUnitRcType::new(self)),
                    Some(OpUnitRcType::new(rhs)),
                    Operation::And,
                );
                #item_name::Unit(OpUnitRcType::new(node))
            }
        }
    )
    .into()
}

#[proc_macro_derive(BitOr)]
pub fn impl_bit_or(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let item_name = &item.ident;

    quote!(
        impl std::ops::BitOr for #item_name {
            type Output = Self;
        
            fn bitor(self, rhs: Self) -> Self::Output {
                let node = OpUnit::new(
                    Some(OpUnitRcType::new(self)),
                    Some(OpUnitRcType::new(rhs)),
                    Operation::Or,
                );
                #item_name::Unit(OpUnitRcType::new(node))
            }
        }
    )
    .into()
}
