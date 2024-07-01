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
