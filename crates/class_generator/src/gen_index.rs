use crate::get_class_map::serde_borrow_attr;
use quote::quote;
use syn::Result;

/// # Args
/// (class_name, has_string)
pub fn gen_index(class_index_map: &[(&String, bool)]) -> Result<String> {
    let mut mods = Vec::new();
    let mut enum_variants = Vec::new();
    let mut enum_match_variants = Vec::new();
    let mut class_names = Vec::new();
    let mut de_matcher = Vec::new();

    let mut default_impl = quote! {};
    for (index, (class_name, has_string)) in class_index_map.iter().enumerate() {
        class_names.push(class_name);

        let class_name_ident = quote::format_ident!("{class_name}");
        let file_name_ident = quote::format_ident!("{class_name}_");
        mods.push(quote! {
            pub mod #file_name_ident;
            pub use #file_name_ident::*;
        });

        let serde_borrow_attr = serde_borrow_attr(*has_string);
        let lifetime = if *has_string {
            quote! { <'a> }
        } else {
            quote! {}
        };

        if index == 0 {
            default_impl = quote! {
                impl Default for Classes<'_> {
                    fn default() -> Self {
                        Self::#class_name_ident(#class_name_ident::default())
                    }
                }
            };
        };

        enum_variants.push(quote! {
            #serde_borrow_attr
            #class_name_ident(#class_name_ident #lifetime)
        });

        enum_match_variants.push(quote! { Classes::#class_name_ident });

        de_matcher.push(quote! {
            #class_name => Ok(Classes::#class_name_ident(map.next_value()?))
        });
    }

    let tokens = quote! {
            /// Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
            mod class_requires {
                pub use havok_serde::{
                    ser::{Error as _, SerializeFlags, SerializeStruct, Serializer, TypeSize},
                    de::{self, Error as _, Deserializer},
                    HavokClass,
                };
                pub use havok_types::*;

                #[cfg(feature = "json_schema")]
                pub fn make_large_int_array_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                    let mut schema = <Vec<i32> as schemars::JsonSchema>::json_schema(generator);
                    let mut map = schema.ensure_object();
                    schema
                }
            }
            #(#mods)*

            use havok_serde as _serde;
            #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Debug, Clone, PartialEq)]
            pub enum Classes<'a> {
                #(#enum_variants,)*
            }

            #default_impl

            impl _serde::HavokClass for Classes<'_> {
                fn name(&self) -> &'static str {
                    match &self {
                        #(#enum_match_variants(class) => class.name(),)*
                    }
                }

                fn signature(&self) -> _serde::__private::Signature {
                    match &self {
                        #(#enum_match_variants(class) => class.signature(),)*
                    }
                }

                fn deps_indexes(&self) -> Vec<usize> {
                    match &self {
                        #(#enum_match_variants(class) => class.deps_indexes(),)*
                    }
                }
            }

            impl<'a> _serde::Serialize for Classes<'a> {
                fn serialize<S: _serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    match self {
                        #(#enum_match_variants(class) => class.serialize(serializer),)*
                    }
                }
            }

            impl<'a, 'de: 'a> _serde::Deserialize<'de> for Classes<'a> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: _serde::Deserializer<'de>,
                {
                    struct ClassesVisitor<'a> {
                        marker: core::marker::PhantomData<Classes<'a>>,
                    }

                    impl<'a, 'de: 'a> _serde::de::Visitor<'de> for ClassesVisitor<'a> {
                        type Value = Classes<'a>;

                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                            formatter.write_str("a valid class enum")
                        }

                        fn visit_class_index<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                        where
                            A: _serde::de::ClassIndexAccess<'de>,
                        {
                            let class_name = map.next_key()?;
                            match class_name {
                                #(#de_matcher,)*
                                _ => Err(_serde::de::Error::unknown_field(
                                    class_name,
                                    &[ #(#class_names,)* ],
                                )),
                            }
                        }
                    }

                    deserializer.deserialize_class_index(ClassesVisitor {
                        marker: core::marker::PhantomData,
                    })
                }
            }
    };

    let ast = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&ast))
}
