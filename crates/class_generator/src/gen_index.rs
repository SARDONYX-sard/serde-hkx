use quote::quote;

/// # Args
/// (class_name, has_string)
pub fn gen_index(class_index_map: &[(&String, bool)]) -> String {
    let mut mods = Vec::new();
    let mut enum_variants = Vec::new();
    let mut enum_match_variants = Vec::new();
    let mut class_names = Vec::new();
    let mut de_matcher = Vec::new();

    for (class_name, has_string) in class_index_map {
        class_names.push(class_name);

        let class_name_ident = quote::format_ident!("{class_name}");
        let file_name_ident = quote::format_ident!("{class_name}_");
        mods.push(quote! {
            pub mod #file_name_ident;
            pub use #file_name_ident::*;
        });

        let lifetime = if *has_string {
            quote! { <'a> }
        } else {
            quote! {}
        };
        enum_variants.push(quote! {
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
                    ser::{Error as _, SerializeFlags, SerializeStruct, Serializer},
                    de::{self, Error as _, Deserializer},
                    HavokClass,
                };
                pub use havok_types::*;
            }
            #(#mods)*

            use havok_serde as _serde;
            #[derive(Debug, Default, Clone, PartialEq)]
            pub enum Classes<'a> {
                /// For binary writing, the youngest pointer index must be first after sorting in reverse order.
                ///
                /// To speed up the process, swap the first and last indexes instead of using shift.
                /// This dummy class exists to reserve space for this purpose.
                #[default]
                SwapDummy,
                #(#enum_variants,)*
            }

            impl _serde::HavokClass for Classes<'_> {
                fn name(&self) -> &'static str {
                    match &self {
                        Classes::SwapDummy => panic!("The dummy class is used only for sorting, so being called name is not a good use of the API."),
                        #(#enum_match_variants(class) => class.name(),)*
                    }
                }

                fn signature(&self) -> _serde::__private::Signature {
                    match &self {
                        Classes::SwapDummy => panic!("The dummy class is used only for sorting, so being called name is not a good use of the API."),
                        #(#enum_match_variants(class) => class.signature(),)*
                    }
                }
            }

            impl<'a> _serde::Serialize for Classes<'a> {
                fn serialize<S: _serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    match self {
                        Classes::SwapDummy => panic!("The dummy class is used only for sorting, so being called name is not a good use of the API."),
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

    let ast = match syn::parse2(tokens) {
        Ok(ast) => ast,
        Err(err) => panic!("{err}"),
    };
    prettyplease::unparse(&ast)
}
