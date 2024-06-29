#[derive(serde::Deserialize)]
enum Enum {
    A,
    B,
    C,
}

#[derive(serde::Deserialize)]
struct Struct {
    #[allow(dead_code)]
    enum_: Enum,
}

fn main() {}
