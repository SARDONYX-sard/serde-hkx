use crate::cpp_info::Class;
use crate::cpp_info::Member;
use crate::ClassMap;
use proc_macro2::TokenStream;
use quote::quote;
use rayon::prelude::*;

/// Enumerate C++ class information by recursively tracing from the current class.
/// - current class name -> Myself and all parent classes.
/// - parent class name -> All parent classes.
///
/// # Returns
/// Vec sorted by deepest parent class.
pub fn get_inherited_class<'a>(class_name: &str, classes_map: &'a ClassMap) -> Vec<&'a Class<'a>> {
    // Cache variables
    let mut current_class_name = class_name;
    let mut inherited_class = Vec::new();

    // Get all parents
    while let Some(class) = classes_map.get(current_class_name) {
        inherited_class.push(class);

        if let Some(parent_name) = &class.parent {
            current_class_name = parent_name;
        } else {
            break; // No more parent to process
        }
    }

    inherited_class.reverse(); // This is because binary reads must be read from the most root parent class.
    inherited_class
}

/// Enumerate C++ class information by recursively tracing from the current class.
/// - current class name -> Myself and all parent classes.
/// - parent class name -> All parent classes.
///
/// # Returns
/// Vec sorted by deepest parent class.
pub fn get_inherited_members<'a>(
    class_name: &str,
    classes_map: &'a ClassMap,
) -> Vec<&'a Member<'a>> {
    let members = {
        // Get all parents
        let mut members = Vec::new();
        let mut current_class_name = class_name;
        while let Some(class) = classes_map.get(current_class_name) {
            members.push(class.members.as_slice());

            if let Some(parent_name) = &class.parent {
                current_class_name = parent_name;
            } else {
                break; // No more parent to process
            }
        }
        members.reverse(); // This is because binary reads must be read from the most root parent class.
        members
    };

    // NOTE:
    // If we just reverse a flattened `member`, the fields will also be in reverse order,
    // so it is necessary to split the `member` into two.
    members.into_iter().fold(Vec::new(), |mut acc, member| {
        acc.par_extend(member);
        acc
    })
}

pub fn serde_borrow_attr(has_string: bool) -> TokenStream {
    if has_string {
        quote! {
            #[cfg_attr(feature = "serde", serde(borrow))]
        }
    } else {
        quote! {}
    }
}
