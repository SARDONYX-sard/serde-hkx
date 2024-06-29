#![allow(non_camel_case_types, non_snake_case, unused)]

mod generated {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let class = crate::generated::BGSGamebryoSequenceGenerator::default();
        dbg!(class);
    }
}
