pub const AFTER_HELP: &str = color_print::cstr!(
    r#"<blue><bold><underline>Examples</underline></bold></blue>
- <blue!>hkx -> xml</blue!>
  <cyan!>hkxc convert --input</cyan!> ./defaultmale.hkx <cyan!>--format</cyan!> xml

- <blue!>xml -> hkx(64bit)</blue!>
  <cyan!>hkxc convert -i</cyan!> ./defaultmale.xml <cyan!>-v</cyan!> amd64 <cyan!>--stdout --log-level</cyan!> trace

- <blue!>hkx(32bit) -> hkx(64bit)</blue!>
  <cyan!>hkxc convert -i</cyan!> ./defaultmale_x86.hkx <cyan!>-o</cyan!> ./defaultmale_x64.hkx <cyan!>-v</cyan!> amd64 <cyan!>--log-level</cyan!> debug <cyan!>--log-file</cyan!> "./convert_x86_to_x64_bytes.log"

- <blue!>hkx(64bit) -> hkx(32bit)</blue!>
  <cyan!>hkxc convert -i</cyan!> ./defaultmale_x64.hkx <cyan!>-o</cyan!> ./defaultmale_x86.hkx <cyan!>-v</cyan!>  win32 <cyan!>--log-level</cyan!> trace <cyan!>--log-file</cyan!> ./convert_x64_to_x86_bytes.log
"#
);
