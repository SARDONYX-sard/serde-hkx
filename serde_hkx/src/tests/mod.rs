pub(crate) mod mocks;
mod re_convert;
mod verify;

/// - key: class index(e.g. `#0001`)
/// - value: class
pub(crate) type ClassMap<'a> = indexmap::IndexMap<usize, havok_classes::Classes<'a>>;

/// Returns a diff string of two strings.
pub(crate) fn diff(old: impl AsRef<str>, new: impl AsRef<str>) -> String {
    let diff = similar::TextDiff::from_lines(old.as_ref(), new.as_ref());
    let mut output_diff = String::new();
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            similar::ChangeTag::Delete => "<",
            similar::ChangeTag::Insert => ">",
            similar::ChangeTag::Equal => " ",
        };
        output_diff += &format!("{sign}{change}");
    }
    output_diff
}
