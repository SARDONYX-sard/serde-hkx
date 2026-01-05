default: build

build:
	cl.exe /std:c11 "./main.c" "./serde_hkx_ffi.lib" -I"./serde_hkx_ffi.h"

convert:
	./main.exe -i ./defaultmale.hkx -o ./out/defaultmale.xml -v xml
	./main.exe -i ./defaultmale.hkx -o ./out/defaultmale_win32.hkx -v win32
	./main.exe -i ./defaultmale.hkx -o ./out/defaultmale_amd64.hkx -v amd64
