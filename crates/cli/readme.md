# CLI

## Examples

```shell
# - hkx -> xml
./hkxc convert --input ./defaultmale.hkx --format xml

# - xml -> hkx(32bit)
./hkxc convert -i ./defaultmale.xml -o ./defaultmale.hkx --format win32 --log-level "debug" --log-file "./convert_to_x64_bytes.log"
# - xml -> hkx(64bit)
./hkxc convert -i ./defaultmale.xml -v amd64 --stdout --log-level "trace"

# - hkx(32bit) -> hkx(64bit)
./hkxc convert -i ./defaultmale_x86.hkx -o ./defaultmale_x64.hkx -v amd64 --log-level "debug" --log-file "./convert_x86_to_x64_bytes.log"
# - hkx(64bit) -> hkx(32bit)
./hkxc convert -i ./defaultmale_x64.hkx -o ./defaultmale_x86.hkx -v win32 --log-level "trace" --log-file "./convert_x64_to_x86_bytes.log"
```
