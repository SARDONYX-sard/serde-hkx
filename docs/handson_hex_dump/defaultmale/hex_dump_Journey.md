# `defaultmale.hkx` hex dump Journey

The following is a hex dump by xxd.exe broken down by category.

This page attempts to explain the binary specification of hkx while performing binary analysis based on SkyrimSE's `defaultmale.hkx`.

```txt
// Hkx header
00000000: 57 e0 e0 57 10 c0 c0 10 00 00 00 00 08 00 00 00  W..W............
00000010: 08 01 00 01 03 00 00 00 02 00 00 00 00 00 00 00  ................
00000020: 00 00 00 00 4b 00 00 00 68 6b 5f 32 30 31 30 2e  ....K...hk_2010.
00000030: 32 2e 30 2d 72 31 00 ff 00 00 00 00 ff ff ff ff  2.0-r1..........

// Section headers
// `__classnames__` section header
00000040: 5f 5f 63 6c 61 73 73 6e 61 6d 65 73 5f 5f 00 00  __classnames__..
00000050: 00 00 00 ff d0 00 00 00 90 00 00 00 90 00 00 00  ................
00000060: 90 00 00 00 90 00 00 00 90 00 00 00 90 00 00 00  ................
// `__types__` section header
00000070: 5f 5f 74 79 70 65 73 5f 5f 00 00 00 00 00 00 00  __types__.......
00000080: 00 00 00 ff 60 01 00 00 00 00 00 00 00 00 00 00  ....`...........
00000090: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
// `__data__` section header
000000a0: 5f 5f 64 61 74 61 5f 5f 00 00 00 00 00 00 00 00  __data__........
000000b0: 00 00 00 ff 60 01 00 00 70 01 00 00 c0 01 00 00  ....`...p.......
000000c0: e0 01 00 00 10 02 00 00 10 02 00 00 10 02 00 00  ................

// `__classnames__` section
000000d0: f6 5e 58 75 09 68 6b 43 6c 61 73 73 00 c2 a4 7e  .^Xu.hkClass...~
000000e0: 5c 09 68 6b 43 6c 61 73 73 4d 65 6d 62 65 72 00  \.hkClassMember.
000000f0: cf 09 36 8a 09 68 6b 43 6c 61 73 73 45 6e 75 6d  ..6..hkClassEnum
00000100: 00 6c 8a 6f ce 09 68 6b 43 6c 61 73 73 45 6e 75  .l.o..hkClassEnu
00000110: 6d 49 74 65 6d 00 1e c1 72 27 09 68 6b 52 6f 6f  mItem...r'.hkRoo
00000120: 74 4c 65 76 65 6c 43 6f 6e 74 61 69 6e 65 72 00  tLevelContainer.
00000130: a7 9b a3 13 09 68 6b 62 50 72 6f 6a 65 63 74 44  .....hkbProjectD
00000140: 61 74 61 00 0a d6 6a 07 09 68 6b 62 50 72 6f 6a  ata...j..hkbProj
00000150: 65 63 74 53 74 72 69 6e 67 44 61 74 61 00 ff ff  ectStringData...
// `__data__` section
00000160: 00 00 00 00 00 00 00 00 01 00 00 00 01 00 00 80  ................
00000170: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000180: 00 00 00 00 00 00 00 00 68 6b 62 50 72 6f 6a 65  ........hkbProje
00000190: 63 74 44 61 74 61 00 00 00 00 00 00 00 00 00 00  ctData..........
000001a0: 68 6b 62 50 72 6f 6a 65 63 74 44 61 74 61 00 00  hkbProjectData..
000001b0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
000001c0: 00 00 00 00 00 00 00 00 00 00 80 3f 00 00 00 00  ...........?....
000001d0: 00 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00  ................
000001e0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
000001f0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
00000200: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
00000210: 00 00 00 00 00 00 00 00 01 00 00 00 01 00 00 80  ................
00000220: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
00000230: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000240: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000250: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
00000260: 00 00 00 00 00 00 00 00 43 68 61 72 61 63 74 65  ........Characte
00000270: 72 73 5c 44 65 66 61 75 6c 74 4d 61 6c 65 2e 68  rs\DefaultMale.h
00000280: 6b 78 00 00 00 00 00 00 00 00 00 00 00 00 00 00  kx..............
00000290: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
000002a0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
000002b0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
000002c0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................

// Fixups
// local_fixups
000002d0: 00 00 00 00 10 00 00 00 10 00 00 00 28 00 00 00  ............(...
000002e0: 18 00 00 00 40 00 00 00 b0 00 00 00 00 01 00 00  ....@...........
000002f0: 00 01 00 00 08 01 00 00 d0 00 00 00 30 01 00 00  ............0...
00000300: d8 00 00 00 40 01 00 00 e0 00 00 00 50 01 00 00  ....@.......P...
00000310: e8 00 00 00 60 01 00 00 ff ff ff ff ff ff ff ff  ....`...........
// global_fixups
00000320: 20 00 00 00 02 00 00 00 50 00 00 00 70 00 00 00   .......P...p...
00000330: 02 00 00 00 80 00 00 00 ff ff ff ff ff ff ff ff  ................
// virtual_fixups
00000340: 00 00 00 00 00 00 00 00 4b 00 00 00 50 00 00 00  ........K...P...
00000350: 00 00 00 00 65 00 00 00 80 00 00 00 00 00 00 00  ....e...........
00000360: 79 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff  y...............
```

## hkx header

```log
00000000: 57 e0 e0 57 10 c0 c0 10 00 00 00 00 08 00 00 00  W..W............
          <--------->                                      magic0: u32
                     <----------->                         magic1: u32
                                  <--------->              user_tag: i32
                                             <---------->  file_version: i32
      magic0: 0x57e0e057
      magic0: 0x10c0c010
    user_tag: 0
file_version: 8

00000010: 08 01 00 01 03 00 00 00 02 00 00 00 00 00 00 00  ................
         <->                                               ptr_size: u8
            <->                                            endian: u8
                <->                                        padding_option: u8
                   <->                                     base_class: u8
                      <--------->                          section_count: i32
                                 <---------->              contents_section_index: i32
                                             <---------->  contents_contents_section_offset: i32
                        ptr_size: 8 -> 8bytes -> This hkx file is 64bit.
                          endian: 0 -> little endian
                  padding_option: 0
                      base_class: 1
                   section_count: 3
          contents_section_index: 2
contents_contents_section_offset: 0

00000020: 00 00 00 00 4b 00 00 00 68 6b 5f 32 30 31 30 2e  ....K...hk_2010.
          <--------->                                      contents_class_name_section_index: i32
                      <--------->                          contents_class_name_section_offset: i32
                                 <-----------------------  contents_contents_version_string: [u8;16]
 contents_class_name_section_index: 0
contents_class_name_section_offset: 0x4b(75)

00000030: 32 2e 30 2d 72 31 00 ff 00 00 00 00 ff ff ff ff  2.0-r1..........
          ---------------------->                          contents_contents_version_string: [u8;16]
                                  <--------->              flags: i32
                                              <--->        max_predicate: i16
                                                    <--->  section_offset: i16
contents_contents_version_string: "hk_2010.2.0-r1\0" + separator 0xFF
                           flags: 0
                   max_predicate: 0xffff -> -1
                  section_offset: 0xffff -> -1

```

## Section headers

### `classnames` section header

```txt

00000040: 5f 5f 63 6c 61 73 73 6e 61 6d 65 73 5f 5f 00 00  __classnames__..
          <----------------------------------------------  contents_contents_version_string: [u8;19]

00000050: 00 00 00 ff d0 00 00 00 90 00 00 00 90 00 00 00  ................
          -------->                                        contents_contents_version_string: [u8;19]
                   <->                                     section_tag_separator: u8
                      <--------->                          absolute_data_start: u32
                                  <--------->              local_fixups_offset: u32
                                              <--------->  global_fixups_offset: u32
contents_contents_version_string: `__classnames__\0\0\0\0\0`
           section_tag_separator: 0xff
             absolute_data_start: 0xd0
             local_fixups_offset: 0x90
            global_fixups_offset: 0x90

00000060: 90 00 00 00 90 00 00 00 90 00 00 00 90 00 00 00  ................
          <--------->                                      virtual_fixups_offset: u32
                      <--------->                          exports_offset: u32
                                  <--------->              imports_offset: u32
                                              <--------->  end_offset: u32
virtual_fixups_offset: 0x90
       exports_offset: 0x90
       imports_offset: 0x90
           end_offset: 0x90

```

### `__types__` section header

```log
00000070: 5f 5f 74 79 70 65 73 5f 5f 00 00 00 00 00 00 00  __types__.......
          <----------------------------------------------  contents_contents_version_string: [u8;19]

00000080: 00 00 00 ff 60 01 00 00 00 00 00 00 00 00 00 00  ....`...........
          -------->                                        contents_contents_version_string: [u8;19]
                   <->                                     section_tag_separator: u8
                      <--------->                          absolute_data_start: u32
                                  <--------->              local_fixups_offset: u32
                                              <--------->  global_fixups_offset: u32
contents_contents_version_string: `__types__\0\0\0\0\0\0\0\0\0\0`
           section_tag_separator: 0xff
             absolute_data_start: 0x160
             local_fixups_offset: 0x00
            global_fixups_offset: 0x00

00000090: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------->                                      virtual_fixups_offset: u32
                      <--------->                          exports_offset: u32
                                  <--------->              imports_offset: u32
                                              <--------->  end_offset: u32
virtual_fixups_offset: 0x00
       exports_offset: 0x00
       imports_offset: 0x00
           end_offset: 0x00
```

### `__data__` section header

```log
000000a0: 5f 5f 64 61 74 61 5f 5f 00 00 00 00 00 00 00 00  __data__........
          <----------------------------------------------  contents_contents_version_string: [u8;19]

000000b0: 00 00 00 ff 60 01 00 00 70 01 00 00 c0 01 00 00  ....`...p.......
          -------->                                        contents_contents_version_string: [u8;19]
                   <->                                     section_tag_separator: u8
                      <--------->                          absolute_data_start: u32
                                  <--------->              local_fixups_offset: u32
                                              <--------->  global_fixups_offset: u32
contents_contents_version_string: `__data__\0\0\0\0\0\0\0\0\0\0\0`
           section_tag_separator: 0xff
             absolute_data_start: 0x160
             local_fixups_offset: 0x170
            global_fixups_offset: 0x1c0

000000c0: e0 01 00 00 10 02 00 00 10 02 00 00 10 02 00 00  ................
          <--------->                                      virtual_fixups_offset: u32
                      <--------->                          exports_offset: u32
                                  <--------->              imports_offset: u32
                                              <--------->  end_offset: u32
virtual_fixups_offset: 0x1e0
       exports_offset: 0x210
       imports_offset: 0x210
           end_offset: 0x210
```

## `__classnames__` section

```log
000000d0: f6 5e 58 75 09 68 6b 43 6c 61 73 73 00 c2 a4 7e  .^Xu.hkClass...~
          <--------->                                      signature
                      <->                                  separator
                         <--------------------->           class_name
                                                 <-------  signature
signature: 0x75585ef6
separator: 0x09
class_name: `hkClass\0`

000000e0: 5c 09 68 6b 43 6c 61 73 73 4d 65 6d 62 65 72 00  \.hkClassMember.
          ->                                               signature
            <->                                            separator
                <--------------------------------------->  class_name
signature: 0x5c7ea4c2
separator: 0x09
class_name: `hkClassMember\0`

000000f0: cf 09 36 8a 09 68 6b 43 6c 61 73 73 45 6e 75 6d  ..6..hkClassEnum
          <--------->                                      signature
                      <->                                  separator
                         <-------------------------------- class_name
signature: 0x8a3609cf
separator: 0x09
class_name: `hkClassEnum\0`

00000100: 00 6c 8a 6f ce 09 68 6b 43 6c 61 73 73 45 6e 75  .l.o..hkClassEnu
          ->                                               class_name
             <--------->                                   signature
                         <->                               separator
                            <----------------------------- class_name
signature: 0xce6f8a6c
separator: 0x09
class_name: `hkClassEnumItem\0`

00000110: 6d 49 74 65 6d 00 1e c1 72 27 09 68 6b 52 6f 6f  mItem...r'.hkRoo
          ---------------->                                class_name
                            <--------->                    signature
                                       <->                 separator
                                           <-------------- class_name
signature: 0x2772c11e
separator: 0x09
class_name: `hkRootLevelContainer\0`

00000120: 74 4c 65 76 65 6c 43 6f 6e 74 61 69 6e 65 72 00  tLevelContainer.
          ---------------------------------------------->  class_name

00000130: a7 9b a3 13 09 68 6b 62 50 72 6f 6a 65 63 74 44  .....hkbProjectD
          <--------->                                      signature
                      <->                                  separator
                         <-------------------------------- class_name
signature: 0x13a39ba7
separator: 0x09
class_name: `hkbProjectData\0`

00000140: 61 74 61 00 0a d6 6a 07 09 68 6b 62 50 72 6f 6a  ata...j..hkbProj
          ----------->                                     class_name
                            <--------->                    signature
                                       <->                 separator
                                           <-------------- class_name
signature: 0x076ad60a
separator: 0x09
class_name: `hkbProjectStringData\0`

00000150: 65 63 74 53 74 72 69 6e 67 44 61 74 61 00 ff ff  ectStringData...
          ----------->                                     class_name
                      <--------------------------------->  16bytes alignments by filled with 0xff
```

## `__data__` section fixups

Get information on fixups first. Otherwise, some of the field's content location will not be known and the class information will not be read accurately.

The byte location of the fixups is after the `__data__` section, so it can be determined from the abs + local_fixup offset in the `__data__` section header.

### local fixups

Used to read binary data that has an actual state at the pointer end, such as `hkStringPtr`, `hkArray`, etc.
How about class's ptr? This is an index which is attached to a real name in DRAM or XML, so it does not exist in the binary data. Therefore, there is no local_fixup for class's ptr.

```log
000002d0: 00 00 00 00 10 00 00 00 10 00 00 00 28 00 00 00  ............(...
          <--------->                                      src
                      <--------->                          dst
                                  <--------->              src
                                              <--------->  dst
src: 0x00
dst: 0x10

src: 0x10
dst: 0x28

000002e0: 18 00 00 00 40 00 00 00 b0 00 00 00 00 01 00 00  ....@...........
          <--------->                                      src
                      <--------->                          dst
                                  <--------->              src
                                              <--------->  dst
src: 0x18
dst: 0x40

src: 0xb0
dst: 0x0100

000002f0: 00 01 00 00 08 01 00 00 d0 00 00 00 30 01 00 00  ............0...
          <--------->                                      src
                      <--------->                          dst
                                  <--------->              src
                                              <--------->  dst
src: 0x100
dst: 0x108

src: 0xd0
dst: 0x0130

00000300: d8 00 00 00 40 01 00 00 e0 00 00 00 50 01 00 00  ....@.......P...
          <--------->                                      src
                      <--------->                          dst
                                  <--------->              src
                                              <--------->  dst
src: 0xd8
dst: 0x140

src: 0xe0
dst: 0x0150

00000310: e8 00 00 00 60 01 00 00 ff ff ff ff ff ff ff ff  ....`...........
          <--------->                                      src
                      <--------->                          dst
                                  <--------------------->  16bytes alignments by filled with 0xff
src: 0xe8
dst: 0x160

```

### Global fixups

Location information needed when referencing class pointer, etc.

```txt
00000320: 20 00 00 00 02 00 00 00 50 00 00 00 70 00 00 00   .......P...p...
          <--------->                                      src
                      <--------->                          section_index
                                  <--------->              dst
                                              <--------->  src
              src: 0x20
dst_section_index: 2
              dst: 0x50

src: 0x70

00000330: 02 00 00 00 80 00 00 00 ff ff ff ff ff ff ff ff  ................
          <--------->                                      section_index
                      <--------->                          dst
                                  <--------------------->  16bytes alignments by filled with 0xff
dst_section_index: 2
              dst: 0x80
```

### Virtual fixups

Location information for the name of the C++ class that must call the constructor.

```log
00000340: 00 00 00 00 00 00 00 00 4b 00 00 00 50 00 00 00  ........K...P...
          <--------->                                      src
                      <--------->                          section_index
                                  <--------->              name_offset
                                              <--------->  src
          src: 0x00
section_index: 0x00
  name_offset: 0x4b

          src: 0x50

00000350: 00 00 00 00 65 00 00 00 80 00 00 00 00 00 00 00  ....e...........
          <--------->                                      section_index
                      <--------->                          name_offset
                                  <--------->              src
                                              <--------->  section_index
section_index: 0x00
  name_offset: 0x65

          src: 0x80
section_index: 0x00

00000360: 79 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff  y...............
          <--------->                                      name_offset
                      <--------------------------------->  16bytes alignments by filled with 0xff
  name_offset: 0x79
```

## `__data__` section

1. Explore the classes that must call the constructor from the mapping information in virtual_fixup.
2. Identify class_name using this as a key, and instantiate the corresponding class from class_name by reading the binary data.

- The current class_names map is as follows

```rust
{
    5: ClassPair {
        signature: 1968725750,
        class_name: "hkClass",
    },
    18: ClassPair {
        signature: 1551803586,
        class_name: "hkClassMember",
    },
    37: ClassPair {
        signature: 2318797263,
        class_name: "hkClassEnum",
    },
    54: ClassPair {
        signature: 3463416428,
        class_name: "hkClassEnumItem",
    },
    75: ClassPair {
        signature: 661831966,
        class_name: "hkRootLevelContainer",
    },
    101: ClassPair {
        signature: 329489319,
        class_name: "hkbProjectData",
    },
    121: ClassPair {
        signature: 124442122,
        class_name: "hkbProjectStringData",
    },
}
```

- So, according to virtual_fixup, the following classes of data are contained.

```rust
VirtualFixup {
    src: 0,
    section_index: 0,
    name_offset: 75, // -> `hkRootLevelContainer`
},
VirtualFixup {
    src: 80,
    section_index: 0,
    name_offset: 101, // -> `hkbProjectData`
},
VirtualFixup {
    src: 128,
    section_index: 0,
    name_offset: 121, // -> `hkbProjectStringData`
}
```

- The C++ pseudo code of `hkRootLevelContainer` is shown below.

```cpp
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x2772c11e`
/// -   version: 0
struct hkRootLevelContainer {
    // # C++ Class Fields Info
    // -   name:`"namedVariants"`
    // -   type: `hkArray<struct hkRootLevelContainerNamedVariant>`
    // - offset: 0
    // -  flags: `FLAGS_NONE`
    hkArray<struct hkRootLevelContainerNamedVariant> namedVariants,
}
```

- `hkRootLevelContainerNamedVariant`

```cpp
// # C++ Class Info
// -      size: 12
// -    vtable: false
// - signature: `0xb103a2cd`
// -   version: 0
struct hkRootLevelContainerNamedVariant {
    // # C++ Class Fields Info
    // -   name:`"name"`
    // -   type: `hkStringPtr`
    // - offset: 0
    // -  flags: `FLAGS_NONE`
    hkStringPtr name,
    // # C++ Class Fields Info
    // -   name:`"className"`
    // -   type: `hkStringPtr`
    // - offset: 4
    // -  flags: `FLAGS_NONE`
    hkStringPtr class_name,
    // # C++ Class Fields Info
    // -   name:`"variant"`
    // -   type: `struct hkReferencedObject*`
    // - offset: 8
    // -  flags: `FLAGS_NONE`
    struct hkReferencedObject* variant,
}
```

- `hkArray<T>`: read_ptr_size & move current seek position(+ptr size)
- `hkStringPtr`:
  if ptr is null?
  null -> move current seek position(+ptr size)

  not null ->read `char*` of local_fixup.dst position
  -> move current seek position(+ptr size)
  -> align 16 for local_fixup.dst position

Basically, rather than reading them all together, it may be necessary to implement a position move by repeatedly using `read_f32` and so on.
In other words, the length of a seek read at a time **depends on the implementation**.

- current position seek(reader/writer): <=>
- where the actual field data resides: <->

### Notes

- Reading and writing binary data in the havok class requires that you always know which position you are currently reading or writing a fixup.
- `hkStringPtr` and so on are read/written to the coordinates vacated by local_fixup after reading the seek position of all fields of the class and meta information such as `hkArray`.

### Read binary

```log
- `hkRootLevelContainer``
00000160: 00 00 00 00 00 00 00 00 01 00 00 00 01 00 00 80  ................
          <--------------------->                          consume pointer size: always 0
                                  <--------->              array size: u32 -> 1
                                              <--------->  capacity & flags -> 1 & 0x80
          <=============================================>  move seek 16bytes for namedVariants(`hkArray<struct hkRootLevelContainerNamedVariant>`)

00000170: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------->                          consume ptr size(name)
                                  <--------------------->  consume ptr size(class_name)
          <=====================>                          move seek 8bytes for name(`hkStringPtr`)
                                  <=====================>  move seek 8bytes for class_name(`hkStringPtr`)

00000180: 00 00 00 00 00 00 00 00 68 6b 62 50 72 6f 6a 65  ........hkbProje
          <--------------------->                          consume ptr size(variant)
                                  <----------------------  name of String ptr
          <=====================>                          move seek 8bytes for variant(`struct hkReferencedObject*`)

00000190: 63 74 44 61 74 61 00 00 00 00 00 00 00 00 00 00  ctData..........
          ------------------->                             name
                               <------------------------>  0 fill until align 16

000001a0: 68 6b 62 50 72 6f 6a 65 63 74 44 61 74 61 00 00  hkbProjectData..
          <------------------------------------------>     name
                                                       <-> 0 fill until align 16

- `hkbProjectData` => `hkReferencedObject`(parent) => `hkBaseObject`(parent of parent)
The next virtual_fixup is 80, which means the 80th byte with data_section as the 0th.
It is 0x160 + 0x50(80) = 0x1b0
000001b0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------->                          pointer size of `hkBaseObject`

                                  <--------------------->  `hkReferencedObject`
                                  <--->                    mem_size_and_flags: u16
                                        <--->              reference_count: u16
                                              <--------->  0 fill until align 16
          <=====================>                          seek `hkBaseObject`
                                  <======================> seek `hkReferencedObject`

000001c0: 00 00 00 00 00 00 00 00 00 00 80 3f 00 00 00 00  ...........?....
          <--------------------------------------------->  world_up_ws: Vector4<f32>
          <--------->                                      f32: 0.0
                      <--------->                          f32: 0.0
                                  <--------->              f32: 1.0
                                              <--------->  f32: 0.0
          <=========>                                      seek f32
                      <=========>                          seek f32
                                  <=========>              seek f32
                                              <=========>  seek f32


000001d0: 00 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00  ................
          <--------------------->                          string_data: struct hkbProjectStringData*
                                  <->                      enum EventMode: u8
                                     <------------------>  0 fill until align 16
          <=====================>
                                  <=>
                                     <==================>

- `hkbProjectStringData` => `hkReferencedObject`(parent) => `hkBaseObject`(parent of parent)
The next virtual_fixup is 128. Therefore, 0x160 + 0x80(128) = 0x1e0
000001e0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------->                          pointer size of `hkBaseObject`

                                  <--------------------->  `hkReferencedObject`
                                  <--->                    mem_size_and_flags: u16
                                        <--->              reference_count: u16
                                              <--------->  0 fill until align 16

          <=====================>                          seek `hkBaseObject`
                                  <======================> seek `hkReferencedObject`

000001f0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
          <--------------------------------------------->  animationFilenames: hkArray<hkStringPtr>
          <--------------------->                          consume pointer size: always 0
                                  <--------->              array size: u32 -> 0
                                              <--------->  capacity & flags -> 0 & 0x80
          <=============================================>  seek animationFilenames

00000200: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
          <--------------------------------------------->  behaviorFilenames: hkArray<hkStringPtr>
          <=============================================>  behaviorFilenames


00000210: 00 00 00 00 00 00 00 00 01 00 00 00 01 00 00 80  ................
          <--------------------------------------------->  characterFilenames: hkArray<hkStringPtr>
          <=============================================>  seek characterFilenames

00000220: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
          <--------------------------------------------->  eventNames: hkArray<hkStringPtr>
          <=============================================>  seek eventNames

00000230: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <=====================>                          seek for animationPath: hkStringPtr
                                  <=====================>  seek for behaviorPath: hkStringPtr

00000240: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <=====================>                          seek characterPath: hkStringPtr
                                  <=====================>  seek fullPathToSource: hkStringPtr

00000250: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <=====================>                          seek rootPath: hkStringPtr
                                  <=====================>  seek align 16

00000260: 00 00 00 00 00 00 00 00 43 68 61 72 61 63 74 65  ........Characte
          <--------------------->
                                  <----------------------  hkStringPtr 1th of characterFilenames

00000270: 72 73 5c 44 65 66 61 75 6c 74 4d 61 6c 65 2e 68  rs\DefaultMale.h
          -----------------------------------------------  hkStringPtr 1th

00000280: 6b 78 00 00 00 00 00 00 00 00 00 00 00 00 00 00  kx..............
          ------->                                         hkStringPtr 1th (in characterFilenames)
                   <------------------------------------>  0 fill until align 16 for hkStringPtr 1th (in characterFilenames)

00000290: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------------------------------->  0 fill until align 16 for animationPath: hkStringPtr

000002a0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------------------------------->  0 fill until align 16 for behaviorPath: hkStringPtr

000002b0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------------------------------->  0 fill until align 16 for characterPath: hkStringPtr
000002c0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------------------------------->  0 fill until align 16 for fullPathToSource: hkStringPtr
```

- With the above binary analysis, we were able to determine which data_section.local_fixups information was relevant.

```rust
// `hkRootLevelContainer`
LocalFixup {
    src: 0,
    dst: 16, // hkArray<hkRootLevelContainerNamedVariant>(length: 1)
},
// `hkRootLevelContainerNamedVariant`
LocalFixup {
    src: 16,
    dst: 40, // name: hkStringPtr
},
LocalFixup {
    src: 24,
    dst: 64, // class_name: hkStringPtr
},
// `hkbProjectStringData`
LocalFixup {
    src: 176,
    dst: 256, // character_filenames: hkArray<hkStringPtr>(length: 1)
},
LocalFixup {
    src: 256,
    dst: 264, // 1th hkStringPtr in character_filenames: hkStringPtr
},
LocalFixup {
    src: 208,
    dst: 304, // animationPath: hkStringPtr
},
LocalFixup {
    src: 216,
    dst: 320, // behaviorPath: hkStringPtr
},
LocalFixup {
    src: 224,
    dst: 336, // characterPath: hkStringPtr
},
LocalFixup {
    src: 232,
    dst: 352, // fullPathToSource: hkStringPtr
},
// src: 240, root_path: hkStringPtr in None
```
