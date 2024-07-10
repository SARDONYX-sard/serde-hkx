pub mod deserialize;
pub mod serialize;

/// n == 3 => `.parent.parent.parent`
pub(super) fn n_time_parent_ident(n: usize) -> proc_macro2::TokenStream {
    let mut ident = Vec::new();
    for _ in 0..n {
        ident.push(quote::quote! { parent });
    }

    quote::quote! {
        #(.#ident)*
    }
}

/// Cast str to literal.
///
/// # Purpose
/// Number string to hex number.
pub(super) fn str2lit(s: &str) -> syn::Lit {
    use core::str::FromStr as _;
    syn::Lit::new(proc_macro2::Literal::from_str(s).unwrap())
}
