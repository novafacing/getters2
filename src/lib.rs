//! Derive Macro for automatically implementing getter and setter patterns
//! for structs and enums.
//!
//! # Examples
//!
//! `getters2` supports both structs and enums, unlike other crates that claim to implement this
//! pattern!
//!
//! ## Structs
//!
//! `getters2` supports structs with named fields, newtype structs, and tuple structs.
//!
//! ### Named Fields
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! struct Vector3 {
//!    x: f32,
//!    y: f32,
//!    z: f32,
//! }
//!
//! let mut v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
//! assert_eq!(v.x_ref(), &1.0);
//! assert_eq!(v.y_ref(), &2.0);
//! assert_eq!(v.z_ref(), &3.0);
//! ```
//!
//! ### Tuple Structs
//!
//! Tuple structs are automatically named from first to last (there is technically a
//! limit of 20 elements, but if you hit it, you may just want to refactor).
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! struct Vector3(f32, f32, f32);
//!
//! let mut v = Vector3(1.0, 2.0, 3.0);
//! assert_eq!(v.first_ref(), &1.0);
//! assert_eq!(v.second_ref(), &2.0);
//! assert_eq!(v.last_ref(), &3.0);
//! ```
//!
//! ### Newtype Structs
//!
//! Newtype structs work like a single-element tuple struct.
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! struct Vector1(f32);
//!
//! let mut v = Vector1(1.0);
//! assert_eq!(v.first_ref(), &1.0);
//! ```
//!
//! ### Mutable, Clone, and Deref Getters
//!
//! We don't always want to return an immutable reference to a field. Sometimes we want
//! to return a mutable reference, dereference the field, or clone the field. `getters2`
//! supports all of these patterns at either a struct level or a field level. Note that
//! the field-level attributes will override the struct-level attributes, and will only
//! work on named structs (there is no way to specify attributes on tuple or newtype
//! struct elements).
//!
//! ### Mutable, Clone, and Deref Struct Getters
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! #[getters(deref, clone, mutable)]
//! struct Vector3 {
//!   x: f32,
//!   y: f32,
//!   z: f32,
//! }
//!
//! let mut v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
//! assert_eq!(v.x_ref(), &1.0);
//! assert_eq!(v.y_ref(), &2.0);
//! assert_eq!(v.z_ref(), &3.0);
//! assert_eq!(v.x_deref(), 1.0);
//! assert_eq!(v.y_deref(), 2.0);
//! assert_eq!(v.z_deref(), 3.0);
//! assert_eq!(v.x_clone(), 1.0);
//! assert_eq!(v.y_clone(), 2.0);
//! assert_eq!(v.z_clone(), 3.0);
//! *v.x_mut() = 4.0;
//! *v.y_mut() = 5.0;
//! *v.z_mut() = 6.0;
//! assert_eq!(v.x_ref(), &4.0);
//! assert_eq!(v.y_ref(), &5.0);
//! assert_eq!(v.z_ref(), &6.0);
//! ```
//!
//! ### Mutable, Clone, and Deref Field Getters
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! struct Vector3 {
//!   #[getters(deref)]
//!   x: f32,
//!   #[getters(deref)]
//!   y: f32,
//!   z: f32,
//! }
//!
//! let mut v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
//! assert_eq!(v.x_ref(), &1.0);
//! assert_eq!(v.y_ref(), &2.0);
//! assert_eq!(v.z_ref(), &3.0);
//! assert_eq!(v.x_deref(), 1.0);
//! assert_eq!(v.y_deref(), 2.0);
//! // No z_deref method!
//! // assert_eq!(v.z_deref(), 3.0);
//! ```
//!
//! ### Skipping all Getters for a Field
//!
//! Sometimes we want to skip generating all getters for certain fields. We can do this
//! by adding the skip attributes for all the getters we have enabled for the struct:
//!
//! * `skip` - Skips the immutable reference getter
//! * `skip_mutable` - Skips the mutable reference getter
//! * `skip_deref` - Skips the dereference getter
//! * `skip_clone` - Skips the clone getter
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! #[getters(deref, clone, mutable)]
//! struct Vector3 {
//!   #[getters(skip, skip_mutable, skip_deref, skip_clone)]
//!   x: f32,
//!   y: f32,
//!   z: f32,
//! }
//!
//! let mut v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
//! // No x_ref method!
//! // assert_eq!(v.x_ref(), &1.0);
//! assert_eq!(v.y_ref(), &2.0);
//! assert_eq!(v.z_ref(), &3.0);
//! // No x_deref method!
//! // assert_eq!(v.x_deref(), 1.0);
//! assert_eq!(v.y_deref(), 2.0);
//! assert_eq!(v.z_deref(), 3.0);
//! // No x_clone method!
//! // assert_eq!(v.x_clone(), 1.0);
//! assert_eq!(v.y_clone(), 2.0);
//! assert_eq!(v.z_clone(), 3.0);
//! // No x_mut method!
//! // *v.x_mut() = 4.0;
//! *v.y_mut() = 4.0;
//! *v.z_mut() = 5.0;
//! // No x_ref method!
//! // assert_eq!(v.x_ref(), &4.0);
//! assert_eq!(v.y_ref(), &4.0);
//! assert_eq!(v.z_ref(), &5.0);
//! ```
//!
//! ## Enums
//!
//! Of course, everything we just saw for structs also works for enums. Because we don't know
//! which variant of an enum we have, we have to return an `Option` for each getter.
//!
//! ### Named Enums
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! #[getters(deref, clone, mutable)]
//! enum Animal {
//!   Dog {
//!     #[getters(skip_deref)]
//!     name: String,
//!     age: u8
//!   },
//!   Cat {
//!     #[getters(skip_deref)]
//!     name: String,
//!     age: u8
//!   },
//! }
//! let mut dog = Animal::Dog { name: "Rover".to_string(), age: 5 };
//! let mut cat = Animal::Cat { name: "Mittens".to_string(), age: 3 };
//! assert_eq!(dog.dog_name_ref(), Some(&"Rover".to_string()));
//! assert_eq!(dog.dog_name_clone(), Some("Rover".to_string()));
//! assert_eq!(dog.dog_age_ref(), Some(&5));
//! assert_eq!(dog.dog_age_clone(), Some(5));
//!
//! let Some(dog_name) = dog.dog_name_mut() else {
//!    panic!("Expected Some");
//! };
//! *dog_name = "Spot".to_string();
//!
//! assert_eq!(dog.dog_name_ref(), Some(&"Spot".to_string()));
//!
//! assert_eq!(cat.cat_name_ref(), Some(&"Mittens".to_string()));
//! assert_eq!(cat.cat_name_clone(), Some("Mittens".to_string()));
//! assert_eq!(cat.cat_age_ref(), Some(&3));
//! assert_eq!(cat.cat_age_deref(), Some(3));
//! assert_eq!(cat.cat_age_clone(), Some(3));
//!
//! let Some(cat_name) = cat.cat_name_mut() else {
//!   panic!("Expected Some");
//! };
//! *cat_name = "Whiskers".to_string();
//!
//! assert_eq!(cat.cat_name_ref(), Some(&"Whiskers".to_string()));
//! ```
//!
//! ## Tuple and Newtype Enums
//!
//! Tuple and newtype enums work just like tuple and newtype structs. Note that we can't
//! skip individual fields in tuple and newtype enums, but we can skip entire variants.
//! Unofrtunately, this isn't a crate limitation, just a Rust syntax limitation.Here, we
//! just turn of deref, because we can't dereference strings.
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! #[getters(clone, mutable)]
//! enum Animal {
//!   Dog(String, u8),
//!   Cat(String, u8),
//! }
//! let mut dog = Animal::Dog("Rover".to_string(), 5);
//! let mut cat = Animal::Cat("Mittens".to_string(), 3);
//! assert_eq!(dog.dog_first_ref(), Some(&"Rover".to_string()));
//! assert_eq!(dog.dog_first_clone(), Some("Rover".to_string()));
//! assert_eq!(dog.dog_last_ref(), Some(&5));
//! assert_eq!(dog.dog_last_clone(), Some(5));
//! assert_eq!(cat.cat_first_ref(), Some(&"Mittens".to_string()));
//! assert_eq!(cat.cat_first_clone(), Some("Mittens".to_string()));
//! assert_eq!(cat.cat_last_ref(), Some(&3));
//! assert_eq!(cat.cat_last_clone(), Some(3));
//! ```
//!
//! ### Skipping Enum Variants
//!
//! In addition to skipping a field in a named enum variant, we can skip entire variants.
//!
//! ```rust
//! # use getters2::Getters;
//! #[derive(Getters)]
//! #[getters(deref, clone, mutable)]
//! enum Animal {
//!   #[getters(skip, skip_mutable, skip_deref, skip_clone)]
//!   Dog {
//!     name: String,
//!     age: u8
//!   },
//!   #[getters(skip, skip_mutable, skip_deref, skip_clone)]
//!   Cat(i64),
//!   #[getters(skip, skip_mutable, skip_deref, skip_clone)]
//!   Person(String, i64, i64),
//!   
//! }
//! ```

use darling::{
    ast::{Data, Fields},
    util::Flag,
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_error::abort;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, DeriveInput, Expr, Generics, Ident, Index, Member, Type,
    Visibility,
};

#[derive(Debug, FromField)]
#[darling(attributes(getters))]
struct GettersField {
    ident: Option<Ident>,
    #[allow(unused)]
    vis: Visibility,
    ty: Type,
    #[allow(unused)]
    attrs: Vec<Attribute>,
    mutable: Flag,
    deref: Flag,
    clone: Flag,
    skip: Flag,
    skip_mutable: Flag,
    skip_deref: Flag,
    skip_clone: Flag,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(getters))]
struct GettersVariant {
    ident: Ident,
    discriminant: Option<Expr>,
    fields: Fields<GettersField>,
    #[allow(unused)]
    attrs: Vec<Attribute>,
    #[allow(unused)]
    mutable: Flag,
    #[allow(unused)]
    deref: Flag,
    #[allow(unused)]
    clone: Flag,
    skip: Flag,
    skip_mutable: Flag,
    skip_deref: Flag,
    skip_clone: Flag,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(getters),
    supports(
        struct_named,
        struct_newtype,
        struct_tuple,
        enum_named,
        enum_newtype,
        enum_tuple,
        enum_unit
    ),
    // NOTE: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
    forward_attrs(
        cfg,
        derive,
        allow,
        warn,
        deny,
        forbid,
        deprecated,
        must_use,
        doc,
        non_exhaustive
    )
)]
struct GettersInput {
    ident: Ident,
    #[allow(unused)]
    vis: Visibility,
    generics: Generics,
    data: Data<GettersVariant, GettersField>,
    #[allow(unused)]
    attrs: Vec<Attribute>,
    mutable: Flag,
    clone: Flag,
    deref: Flag,
}

impl GettersInput {
    fn method_field(&self, field: &GettersField, index: usize, max: usize) -> TokenStream2 {
        let ty = &field.ty;
        let immutable = !field.skip.is_present();
        let mutable = (field.mutable.is_present() || self.mutable.is_present())
            && !field.skip_mutable.is_present();
        let clone =
            (field.clone.is_present() || self.clone.is_present()) && !field.skip_clone.is_present();
        let deref =
            (field.deref.is_present() || self.deref.is_present()) && !field.skip_deref.is_present();

        let (immutable, maybe_mutable, maybe_clone, maybe_deref) =
            if let Some(ident) = field.ident.as_ref() {
                let ident_ref = format_ident!("{}_ref", ident);
                let ident_mut = format_ident!("{}_mut", ident);
                let ident_clone = format_ident!("{}_clone", ident);
                let ident_deref = format_ident!("{}_deref", ident);
                (
                    immutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_ref(&self) -> &#ty {
                                &self.#ident
                            }
                        })
                        .unwrap_or_default(),
                    mutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_mut(&mut self) -> &mut #ty {
                                &mut self.#ident
                            }
                        })
                        .unwrap_or_default(),
                    clone
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_clone(&self) -> #ty {
                                self.#ident.clone()
                            }
                        })
                        .unwrap_or_default(),
                    deref
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_deref(&self) -> #ty {
                                self.#ident
                            }
                        })
                        .unwrap_or_default(),
                )
            } else {
                // Field with no ident, we generate a named method
                let name = method_name(index, max);
                let name_ref = format_ident!("{}_ref", name);
                let name_mut = format_ident!("{}_mut", name);
                let name_clone = format_ident!("{}_clone", name);
                let name_deref = format_ident!("{}_deref", name);
                let index = Member::Unnamed(Index {
                    index: index as u32,
                    span: Span::call_site(),
                });

                (
                    immutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_ref(&self) -> &#ty {
                                &self.#index
                            }
                        })
                        .unwrap_or_default(),
                    mutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_mut(&mut self) -> &mut #ty {
                                &mut self.#index
                            }
                        })
                        .unwrap_or_default(),
                    clone
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_clone(&self) -> #ty {
                                self.#index.clone()
                            }
                        })
                        .unwrap_or_default(),
                    deref
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_deref(&self) -> #ty {
                                self.#index
                            }
                        })
                        .unwrap_or_default(),
                )
            };

        quote! {
            #immutable
            #maybe_mutable
            #maybe_clone
            #maybe_deref
        }
    }

    fn method_variant(
        &self,
        field: &GettersField,
        index: usize,
        max: usize,
        enum_ident: &Ident,
        variant_ident: &Ident,
        skip: bool,
        skip_mutable: bool,
        skip_clone: bool,
        skip_deref: bool,
    ) -> TokenStream2 {
        let ty = &field.ty;
        let immutable = !field.skip.is_present() && !skip;
        let mutable = (field.mutable.is_present() || self.mutable.is_present())
            && !field.skip_mutable.is_present()
            && !skip_mutable;
        let clone = (field.clone.is_present() || self.clone.is_present())
            && !field.skip_clone.is_present()
            && !skip_clone;
        let deref = (field.deref.is_present() || self.deref.is_present())
            && !field.skip_deref.is_present()
            && !skip_deref;
        let prefix = variant_ident.to_string().to_ascii_lowercase();

        let (immutable, maybe_mutable, maybe_clone, maybe_deref) =
            if let Some(ident) = field.ident.as_ref() {
                #[allow(unused)]
                let ident_ref = format_ident!("{}_{}_ref", prefix, ident);
                #[allow(unused)]
                let ident_mut = format_ident!("{}_{}_mut", prefix, ident);
                #[allow(unused)]
                let ident_clone = format_ident!("{}_{}_clone", prefix, ident);
                #[allow(unused)]
                let ident_deref = format_ident!("{}_{}_deref", prefix, ident);
                (
                    immutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_ref(&self) -> Option<&#ty> {
                                if let #enum_ident::#variant_ident { #ident, .. } = self {
                                    Some(#ident)
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                    mutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_mut(&mut self) -> Option<&mut #ty> {
                                if let #enum_ident::#variant_ident { ref mut #ident, .. } = self {
                                    Some(#ident)
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                    clone
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_clone(&self) -> Option<#ty> {
                                if let #enum_ident::#variant_ident { #ident, .. } = self {
                                    Some(#ident.clone())
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                    deref
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #ident_deref(&self) -> Option<#ty> {
                                if let #enum_ident::#variant_ident { #ident, .. } = self {
                                    Some(*#ident)
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                )
            } else {
                // Field with no ident, we generate a named method
                let name = method_name(index, max);
                let name_ref = format_ident!("{}_{}_ref", prefix, name);
                let name_mut = format_ident!("{}_{}_mut", prefix, name);
                let name_clone = format_ident!("{}_{}_clone", prefix, name);
                let name_deref = format_ident!("{}_{}_deref", prefix, name);
                let elements = tuple_elements(max);
                let elements_mut = tuple_elements_mut(max);
                let element = tuple_element_name(index);

                (
                    immutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_ref(&self) -> Option<&#ty> {
                                if let #enum_ident::#variant_ident(#elements) = self {
                                    Some(#element)
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                    mutable
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_mut(&mut self) -> Option<&mut #ty> {
                                if let #enum_ident::#variant_ident(#elements_mut) = self {
                                    Some(#element)
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                    clone
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_clone(&self) -> Option<#ty> {
                                if let #enum_ident::#variant_ident(#elements) = self {
                                    Some(#element.clone())
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                    deref
                        .then_some(quote! {
                            #[inline(always)]
                            pub fn #name_deref(&self) -> Option<#ty> {
                                if let #enum_ident::#variant_ident(#elements) = self {
                                    Some(*#element)
                                } else {
                                    None
                                }
                            }
                        })
                        .unwrap_or_default(),
                )
            };

        quote! {
            #immutable
            #maybe_mutable
            #maybe_clone
            #maybe_deref
        }
    }

    fn methods_struct(&self, fields: &Fields<&GettersField>) -> TokenStream2 {
        fields
            .iter()
            .enumerate()
            .map(|(i, f)| self.method_field(f, i, fields.len()))
            .collect::<TokenStream2>()
    }

    fn methods_enum(&self, variants: &[&GettersVariant]) -> TokenStream2 {
        variants
            .iter()
            .map(|v| {
                let variant_ident = &v.ident;

                if let Some(discriminant) = v.discriminant.as_ref() {
                    abort!(
                        discriminant,
                        "Getters cannot be derived for enums with discriminants"
                    )
                }

                v.fields
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        self.method_variant(
                            f,
                            i,
                            v.fields.len(),
                            &self.ident,
                            variant_ident,
                            v.skip.is_present(),
                            v.skip_mutable.is_present(),
                            v.skip_clone.is_present(),
                            v.skip_deref.is_present(),
                        )
                    })
                    .collect::<TokenStream2>()
            })
            .collect::<TokenStream2>()
    }
}

impl ToTokens for GettersInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let methods = if let Some(ref fields) = self.data.as_ref().take_struct() {
            self.methods_struct(fields)
        } else if let Some(ref variants) = self.data.as_ref().take_enum() {
            self.methods_enum(variants)
        } else {
            abort!(
                self.ident,
                "Getters can only be derived for structs and enums"
            )
        };

        tokens.extend(quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #methods
            }
        })
    }
}

#[proc_macro_derive(Getters, attributes(getters))]
#[allow(non_snake_case)]
pub fn Getters(input: TokenStream) -> TokenStream {
    let getters = match GettersInput::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(g) => g,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let mut tokens = TokenStream2::new();

    getters.to_tokens(&mut tokens);

    tokens.into()
}

const NUMERAL_TO_ORDINAL: [&str; 20] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eigth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
    "thirteenth",
    "fourteenth",
    "fifteenth",
    "sixteenth",
    "seventeenth",
    "eighteenth",
    "nineteenth",
    "twentieth",
];
const LAST: &str = "last";

/// Given an index (0, 1, 2, ...) return the name of the method
/// (first, second, third, ..., last)
fn method_name(i: usize, max: usize) -> Ident {
    if i == max - 1 && max != 1 {
        Ident::new(LAST, Span::call_site())
    } else {
        Ident::new(NUMERAL_TO_ORDINAL[i], Span::call_site())
    }
}

const TUPLE_ELEMENTS: [&str; 20] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t",
];

fn tuple_element_name(index: usize) -> Ident {
    Ident::new(TUPLE_ELEMENTS[index], Span::call_site())
}

fn tuple_elements(max: usize) -> TokenStream2 {
    (0..max)
        .map(tuple_element_name)
        .enumerate()
        .map(|(i, n)| {
            if i == max - 1 {
                quote!(ref #n)
            } else {
                quote!(ref #n,)
            }
        })
        .collect::<TokenStream2>()
}

fn tuple_elements_mut(max: usize) -> TokenStream2 {
    (0..max)
        .map(tuple_element_name)
        .enumerate()
        .map(|(i, n)| {
            if i == max - 1 {
                quote!(ref mut #n)
            } else {
                quote!(ref mut #n,)
            }
        })
        .collect::<TokenStream2>()
}
