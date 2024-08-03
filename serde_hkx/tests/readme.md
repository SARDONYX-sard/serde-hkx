# SkyrimSE parse all hkx files of `Animation.bsa`

By tokio's multi thread async(With the logger turned off), everything can be converted to XML in 1 minute.

- `data/meshes/` of extracted `Animation.bsa`

- We can find virtual_fixups corresponding to global_fixups in almost all hkx files, but global_fixups like `meshes\\actors\\ambient\\share\character assets\\skeleton.hkx` are There was a pattern that could not be found. (global_fixups.src 120 -> 56976 -> Not found virtual_fixups.name_offset)
