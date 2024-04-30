# `defaultmale.hkx` hex dump Journey

The following is a hex dump by xxd.exe broken down by category.

This page attempts to explain the binary specification of hkx while performing binary analysis based on SkyrimSE's `defaultmale.hkx`.

```log
// HKX header
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

// Sections
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
// Local fixups
000002d0: 00 00 00 00 10 00 00 00 10 00 00 00 28 00 00 00  ............(...
000002e0: 18 00 00 00 40 00 00 00 b0 00 00 00 00 01 00 00  ....@...........
000002f0: 00 01 00 00 08 01 00 00 d0 00 00 00 30 01 00 00  ............0...
00000300: d8 00 00 00 40 01 00 00 e0 00 00 00 50 01 00 00  ....@.......P...
00000310: e8 00 00 00 60 01 00 00 ff ff ff ff ff ff ff ff  ....`...........
// Global fixups
00000320: 20 00 00 00 02 00 00 00 50 00 00 00 70 00 00 00   .......P...p...
00000330: 02 00 00 00 80 00 00 00 ff ff ff ff ff ff ff ff  ................
// Virtual fixups
00000340: 00 00 00 00 00 00 00 00 4b 00 00 00 50 00 00 00  ........K...P...
00000350: 00 00 00 00 65 00 00 00 80 00 00 00 00 00 00 00  ....e...........
00000360: 79 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff  y...............
```

## HKX header

First read the 17th endian byte, which is the endian byte. Then we can read u32, etc. as little endian or as big endian.

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
                          endian: 1 -> little endian
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

### `__classnames__` section header

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
                                        <->                separator
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
                      <--------->                          signature
                                  <->                      separator
                                     <-------------------  class_name
signature: 0x076ad60a
separator: 0x09
class_name: `hkbProjectStringData\0`

00000150: 65 63 74 53 74 72 69 6e 67 44 61 74 61 00 ff ff  ectStringData...
          ---------------------------------------->        class_name
                                                    <---->  16bytes alignments by filled with 0xff
```

## `__data__` section fixups

Get information on fixups first. Otherwise, some of the field's content location will not be known and the class information will not be read accurately.

The byte location of the fixups is after the `__data__` section, so it can be determined from the abs + local_fixup offset in the `__data__` section header.

This time the fixup abs in the `__data__` section is 0x160 and the local_fixups_offset is 0x170.

0x160(352) + 0x170(368) = 0x2d0(720)

Therefore, there is binary data of fixups from 0x2d0.

### Local fixups

Map with current seek position as src and actual content location as dst.
This is used by `hkStringPtr`, `hkArray`, etc. to access data via pointers.

- How about ptr of class?
  This is dynamically added as an index during binary deserialization.

  Some mod editing software starts with the number 50, but the Havok SDK name
  index rule is currently unknown. Since it does not exist in the binary data,
  this is usually not a problem, but since the motion patch tool uses the name
  index (e.g. `#0050`) to perform diff patch, it is necessary to figure it out.

  TODO: Figure out Havok SDK's name index rule.

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

```log
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

- Why is the name `virtual' the actual class?

  The SDK description says that `hkBaseObject`, a 'class without fields', is inherited by all Havok classes so that the vtable comes first before the data.

  And All Havok managed objects inherit from `hkReferencedObject`(which inherits from `hkBaseObject`), stores memory size and reference count.

- section_index: 0 means that virtualFixup's name_offset(dst) indicates a `__classnames__` section.

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

N: for loop index.(for N in virtual_fixups)
virtual_fixups[N].class_name_offset -> class_names_map[class_name_offset] -> Call constructor of class.

- The current class_names map(key: class_name_start_position) is as follows

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

- The following code is almost identical to the actual definitions of the Havok types and Havok classes required for this project.
  (The reason this is necessary is that **the binary read size is highly dependent on the C++ class offset and size**.)

  View Assembly and C++ code in Compiler Explorer

  - [64bit version](https://godbolt.org/z/dz9cj5GEs)
  - [32bit version](https://godbolt.org/z/xqYs7hYs5)

  ```cpp
  // INFO: To see the 32-bit version, add `-m32` to the compiler flags.
  #include <iostream>

  // havok_2010_2_0/Source/Common/Base/Container/String/hkStringPtr.h
  class hkStringPtr {
     private:
      const char *m_stringAndFlag;
  };

  // havok_2010_2_0/Source/Common/Base/Types/hkBaseTypes.h
  typedef unsigned short hkUint16;
  typedef float hkReal;
  template <typename ENUM, typename N>
  class hkEnum {
     private:
      N storage;
  };

  // havok_2010_2_0/Source/Common/Base/Container/Array/hkArray.h
  // - size: 32bit: 12, 64bit: 16bytes
  //   ptr size(32bit: 4, 64bit: 8) + array  size(4: u32) + cap&flags(4: u32)
  //
  // Similar to Rust's `std::Vec` configuration. The difference is that size and
  // capacity are not size_t. The difference is that size and capacity are not size_t,
  // so even hkx binary data for AMD64 is 32bits.
  template <typename T>
  class hkArray {
     private:
      T *m_data;
      int m_size;
      /** The upper two bits are flags indicating the allocation status. */
      int m_capacityAndFlags;
  };

   /// Align 16 bytes to account for SIMD operations using f32 * 4 = 128
   /// XMM registers.
  struct hkVector4 {
      hkReal x __attribute__((aligned(16)));
      float y, z, w;
  };

  // havok_2010_2_0/compat/hkbTransitionEffect_0.h
  enum EventMode {
      EVENT_MODE_DEFAULT = 0,
      EVENT_MODE_PROCESS_ALL = 1,
      EVENT_MODE_IGNORE_FROM_GENERATOR = 2,
      EVENT_MODE_IGNORE_TO_GENERATOR = 3
  };

  /// Havok Classes

  /// -      size: 16(if 32bit -> 12)
  /// -    vtable: false
  /// - signature: `0x2772c11e`
  /// -   version: 0
  struct hkRootLevelContainer {
      // - offset: 0
      // -  flags: `FLAGS_NONE`
      hkArray<struct hkRootLevelContainerNamedVariant> namedVariants;
  };
  #if defined __i386__
  static_assert(sizeof(hkRootLevelContainer) == 12,
                "hkRootLevelContainer size mismatch");
  #elif defined __x86_64__
  static_assert(sizeof(hkRootLevelContainer) == 16,
                "hkRootLevelContainer size mismatch");
  #endif

  // -      size: 32bit: 12, 64bit: 24
  // -    vtable: false
  // - signature: `0xb103a2cd`
  // -   version: 0
  struct hkRootLevelContainerNamedVariant {
      // - offset: 0
      // -  flags: `FLAGS_NONE`
      hkStringPtr name;
      // - offset: 32bit: 4, 64bit: 8
      // -  flags: `FLAGS_NONE`
      hkStringPtr className;
      // - offset: 32bit: 8, 64bit: 16
      // -  flags: `FLAGS_NONE`
      struct hkReferencedObject *variant;
  };
  #if defined __i386__
  static_assert(sizeof(hkRootLevelContainerNamedVariant) == 12,
                "hkRootLevelContainerNamedVariant size mismatch");
  #elif defined __x86_64__
  static_assert(sizeof(hkRootLevelContainerNamedVariant) == 24,
                "hkRootLevelContainerNamedVariant size mismatch");
  #endif

  /// The class size is pointer size.
  /// The SDK description says that the `hkBaseObject`, a virtual function without
  /// a field, is the source of inheritance for all Havok Classes so that the
  /// vtable does not come after the data.
  ///
  /// - size: 32bit: 4, 64bit: 8
  class hkBaseObject {
     public:
      virtual ~hkBaseObject() {}
  };
  #if defined __i386__
  static_assert(sizeof(hkBaseObject) == 4, "hkBaseObject size mismatch");
  #elif defined __x86_64__
  static_assert(sizeof(hkBaseObject) == 8, "hkBaseObject size mismatch");
  #endif

  /// Stores memory size and reference count.
  /// # C++ Class Info
  /// -      size: 32bit: 8, 64bit: 12
  /// -    vtable: true
  /// -    parent: `hkBaseObject`/`0xe0708a00`
  /// - signature: `0x3b1c1113`
  /// -   version: 0
  class hkReferencedObject : hkBaseObject {
      /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) info
      /// - offset: 32bit: 0, 64bit: 0
      /// size_t parent vtable ptr;

      /// - offset: 32bit: 4, 64bit:  8
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      hkUint16 memSizeAndFlags;
      /// - offset: 32bit: 6, 64bit: 10
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      hkUint16 referenceCount;

      /// Since the vtable size of hkBaseObject is the largest, the struct
      /// alignment will be to the vtable of hkBaseObject, so (32bit: 4bytes,
      /// 64bit: 8bytes) is needed.
  #if defined __i386__
      // 32bit: 2 + 2 = 4
      char _pad0[0];
  #elif defined __x86_64__
      // 64bit: 2 + 2 + 4 = 8
      char _pad0[4];
  #endif
  };
  #if defined __i386__
  static_assert(sizeof(hkReferencedObject) == 8,
                "hkReferencedObject size mismatch");
  #elif defined __x86_64__
  static_assert(sizeof(hkReferencedObject) == 16,
                "hkReferencedObject size mismatch");
  #endif

  /// # C++ Class Info
  /// -      size: 32bit: 48
  /// -    vtable: true
  /// -    parent: `hkReferencedObject`/`0x3b1c1113`
  /// - signature: `0x13a39ba7`
  /// -   version: 2
  struct hkbProjectData : hkReferencedObject {
      /// C++ Parent class(`hkBaseObject` => parent: `None`) has no fields but
      /// size is ptr size(32bit: 4, 64bit: 8)

      /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field
      /// Info
      /// -   name:`"memSizeAndFlags"`
      /// -   type: `hkUint16`
      /// - offset: 32bit:  4, 64bit: 8
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      ///
      /// -   name:`"referenceCount"`
      /// -   type: `hkInt16`
      /// - offset: 32bit:  6, 64bit: 10
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      ///
      /// Alignment at the end with the maximum size of the field.
      /// In this case, hkBaseObject's vtable size is the largest, so it will be
      /// a multiple of ptr size. Therefore,
      /// char _pad0[0] <- 32bit
      /// char _pad0[4] <- 64bit

      /// The offset here is 16 bytes because Vector4 performs a 16-bytes
      /// alignment to account for SIMD operations using f32 * 4 = 128
      /// XMM registers.
      ///
      /// - offset: 32bit: 16, 64bit: 16
      /// -  flags: `FLAGS_NONE`
      hkVector4 worldUpWS;
      /// - offset: 32bit: 32, 64bit: 32
      /// -  flags: `FLAGS_NONE`
      /// -   size: 32bit:  4, 64bit:  8
      struct hkbProjectStringData *stringData;
      /// - offset: 36
      /// - offset: 32bit: 36, 64bit: 40
      /// -  flags: `FLAGS_NONE`
      /// -   size: 32bit:  1, 64bit:  1
      hkEnum<EventMode, char> defaultEventMode;

      /// struct alignment
      /// Alignment at the end with the maximum size of the field.
      // In this case, Vector4 size 16 is the largest, so it will be a multiple
      // of 16. Therefore, 32bit/64bit: 48
  #if defined __i386__
      char _pad0[11];
  #elif defined __x86_64__
      char _pad0[7];
  #endif
  };
  #if defined __i386__
  static_assert(sizeof(hkbProjectData) == 48, "hkbProjectData size mismatch");
  #elif defined __x86_64__
  static_assert(sizeof(hkbProjectData) == 48, "hkbProjectData size mismatch");
  #endif

  /// `hkbProjectStringData`
  ///
  /// - In C++, it represents the name of one field in the class.
  /// - In XML, the value of the `name` attribute of the `hkparam` tag.
  ///
  /// # C++ Class Info
  /// -      size: 76
  /// -    vtable: true
  /// -    parent: `hkReferencedObject`/`0x3b1c1113`
  /// - signature: `0x76ad60a`
  /// -   version: 1
  struct hkbProjectStringData : hkReferencedObject {
      /// C++ Parent class(`hkBaseObject` => parent: `None`) has no fields but
      /// size is ptr size(32bit: 4, 64bit: 8)

      /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`)
      /// -   size: 32bit: 8, 64bit: 16
      ///
      /// Field Info
      /// -   name:`"memSizeAndFlags"`
      /// -   type: `hkUint16`
      /// - offset: 32bit:  4, 64bit: 8
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      ///
      /// -   name:`"referenceCount"`
      /// -   type: `hkInt16`
      /// - offset: 32bit:  6, 64bit: 10
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      ///
      /// Alignment at the end with the maximum size of the field.
      /// In this case, hkBaseObject's vtable size is the largest, so it will be
      /// a multiple of ptr size. Therefore,
      /// char _pad0[0] <- 32bit
      /// char _pad0[4] <- 64bit

      /// - offset: 32bit:  8, 64bit: 16
      /// -  flags: `FLAGS_NONE`
      hkArray<hkStringPtr> animation_filenames;

      /// - offset: 32bit: 20, 64bit: 32
      /// -  flags: `FLAGS_NONE`
      hkArray<hkStringPtr> behavior_filenames;
      /// - offset: 32bit: 32, 64bit: 48
      /// -  flags: `FLAGS_NONE`
      hkArray<hkStringPtr> character_filenames;
      /// - offset: 32bit: 44, 64bit: 64
      /// -  flags: `FLAGS_NONE`
      hkArray<hkStringPtr> event_names;

      /// - offset: 32bit: 56, 64bit: 80
      /// -  flags: `FLAGS_NONE`
      hkStringPtr animation_path;
      /// - offset: 32bit: 60, 64bit: 88
      /// -  flags: `FLAGS_NONE`
      hkStringPtr behavior_path;
      /// - offset: 32bit: 64, 64bit: 96
      /// -  flags: `FLAGS_NONE`
      hkStringPtr character_path;
      /// - offset: 32bit: 68, 64bit: 104
      /// -  flags: `FLAGS_NONE`
      hkStringPtr full_path_to_source;
      /// - offset: 32bit: 72, 64bit: 112
      /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
      hkStringPtr root_path;
  };
  #if defined __i386__
  static_assert(sizeof(hkbProjectStringData) == 76,
                "hkbProjectStringData size mismatch");
  #elif defined __x86_64__
  static_assert(sizeof(hkbProjectStringData) == 120,
                "hkbProjectStringData size mismatch");
  #endif

  int main() {
      auto root = hkRootLevelContainer();
      auto root_arg = hkRootLevelContainerNamedVariant();
      auto class0 = hkbProjectData();
      auto class1 = hkbProjectStringData();

      return 0;
  }
  ```

- `hkArray<T>`: read_ptr_size & move current seek position(+=ptr size)
- `hkStringPtr`:

  if ptr is null?

  - true -> move current seek position(+ptr size)
  - false ->read `char*` of local_fixup.dst position -> move current seek position(+ptr size) -> align 16 for local_fixup.dst position

Basically, rather than reading them all together, it may be necessary to implement a position move by repeatedly using `read_f32` and so on.
In other words, the final read length must be the same, but the length of the seek to be read at once is **implementation dependent**.

- current position seek(reader/writer): <=>
- where the actual field data resides: <->

### Notes

- Reading and writing binary data in the havok class requires that we always know which position we are currently reading or writing a fixup.
- `hkStringPtr` and so on are read/written to the coordinates vacated by local_fixup after reading the seek position of all fields of the class and meta information such as `hkArray`.

### Read binary

```log
- `hkRootLevelContainer``
00000160: 00 00 00 00 00 00 00 00 01 00 00 00 01 00 00 80  ................
          <--------------------->                          consume pointer size: always 0
                                  <--------->              array size: u32 -> 1
                                              <--------->  capacity & flags -> 1 | 0x80
          <=============================================>  move seek 16bytes for namedVariants(`hkArray<struct hkRootLevelContainerNamedVariant>`)
First, the 0th byte from abs is read.
Then it reads forward for ptr because of `hkArray<T>` and looks for `dst` corresponding to `src: 0` in local_fixups.
Then the actual content exists in that `dst`(abs(0x160 == 352) + dst(16) = 368 = 0x170).

00000170: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------->                          consume ptr size(name: `hkStringPtr`)
                                  <--------------------->  consume ptr size(class_name: `hkStringPtr`)
          <=====================>                          move seek 8bytes for name(current position: 16 -> dst: 40)
                                  <=====================>  move seek 8bytes for class_name(current position: 24 -> dst: 64)
Q. The 0x170th binary data is name and has no content?
A. Yes, there is no `hkStringPtr` here yet. After reading `hkArray<T>` meta information (16bytes), use that position to read `hkStringPtr`.
1. Take out dst from local_fixup with current seek position as src.
2. Jump to the content location and get it as a string until `\0` comes.
2.5(On write) Write in 0 until it becomes a multiple of 16
3. Read `hkStringPtr` first, advance position by ptr.

Therefore,
current seek position: 16 -> dst: 40
abs(0x160 == 352) + dst(40) = 392 = 0x188

00000180: 00 00 00 00 00 00 00 00 68 6b 62 50 72 6f 6a 65  ........hkbProje
          <--------------------->                          consume ptr size(variant: `struct hkReferencedObject*`)
                                  <----------------------  content of name(`hkStringPtr`)
          <=====================>                          move seek 8bytes for variant(current position: )
                                  <- This is 0x188 position

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
                                        <--->              referenceCount: u16
                                              <--------->  0 fill until align ptr size
          <=====================>                          seek `hkBaseObject`(Pointer size to vtable)
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
Why is `00 00 80 3f` 1.0?
First of all, it's little-endian, so 0x3f800000.

0x3f800000 = 0b00111111_10000000_00000000_00000000

The bits breakdown of the IEEE754 single-precision floating-point numbers is as follows.
-     Sign bit(Bit 31): 0 -> unsigned
- Exponent(Bits 30-23): 0b01111111 -> 127
-  Mantissa(Bits 22-0): 00000000000000000000000

M = Mantissa, E = Exponent

1.M * 2^(E - 127) = 1.0 * 2^(127 - 127) = 1.0 * 1 = 1.0

The first one is 1 to save bits. (since we can just move the decimal point by exponent).

000001d0: 00 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00  ................
          <--------------------->                          string_data: struct hkbProjectStringData*
                                  <->                      enum EventMode: u8
                                     <------------------>  0 fill until align 16(For class alignment)
          <=====================>
                                  <=>
                                     <==================>
Note that C++ class alignment constraints directly affect binary reads. The end of a class must be aligned with the maximum size of that field.
In this case, Vector4 is the largest, so the size of the class must be a multiple of the size of Vector4.
Therefore, a 16-bytes alignment is required.

- `hkbProjectStringData` => `hkReferencedObject`(parent) => `hkBaseObject`(parent of parent)
The next virtual_fixup is 128. Therefore, 0x160 + 0x80(128) = 0x1e0
000001e0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <--------------------->                          pointer size of `hkBaseObject`

                                  <--------------------->  `hkReferencedObject`
                                  <--->                    memSizeAndFlags: u16
                                        <--->              referenceCount: u16
                                              <--------->  0 fill until align ptr size(char _pad0[0(32bit) or 4(64bit)])

          <=====================>                          seek `hkBaseObject`
                                  <======================> seek `hkReferencedObject`

000001f0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
          <--------------------------------------------->  animationFilenames: hkArray<hkStringPtr>
          <--------------------->                          consume pointer size
                                  <--------->              array size: u32 -> 0
                                              <--------->  capacity & flags -> 0 | 0x80
          <=============================================>  seek animationFilenames(current position: 208 -> dst: 304)

00000200: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
          <--------------------------------------------->  behaviorFilenames: hkArray<hkStringPtr>
          <=============================================>  seek behaviorFilenames(current position: 216 -> dst: 336)


00000210: 00 00 00 00 00 00 00 00 01 00 00 00 01 00 00 80  ................
          <--------------------------------------------->  characterFilenames: hkArray<hkStringPtr>
          <=============================================>  seek characterFilenames(current position: 232 -> dst: 352)

00000220: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80  ................
          <--------------------------------------------->  eventNames: hkArray<hkStringPtr>
          <=============================================>  seek eventNames(current position: 232 -> dst: 352)

00000230: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <=====================>                          seek ptr size for animationPath: hkStringPtr
                                  <=====================>  seek ptr size for behaviorPath: hkStringPtr

00000240: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <=====================>                          seek ptr size characterPath: hkStringPtr
                                  <=====================>  seek ptr size fullPathToSource: hkStringPtr

00000250: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
          <=====================>                          seek ptr size rootPath: hkStringPtr
                                  <=====================>  TODO: Unknown

00000260: 00 00 00 00 00 00 00 00 43 68 61 72 61 63 74 65  ........Characte
          <=====================>                          TODO: Unknown
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

NOTE:
Since `rootPath` has a null string, the binary data size for the pointer is allocated and the contents area does not exist.
If it existed, the binary data would come at this position.
```

- With the above binary analysis, we were able to determine which data_section.local_fixups information was relevant.

```rust
// `hkRootLevelContainer`
LocalFixup {
    src: 0,
    dst: 16, // hkArray<struct hkRootLevelContainerNamedVariant>(length: 1)
},
// `hkRootLevelContainerNamedVariant`
LocalFixup {
    src: 16,
    dst: 40, // name: hkStringPtr
},
LocalFixup {
    src: 24,
    dst: 64, // className: hkStringPtr
},
// `hkbProjectStringData`
LocalFixup {
    src: 176,
    dst: 256, // characterFilenames: hkArray<hkStringPtr>(length: 1)
},
LocalFixup {
    src: 256,
    dst: 264, // 1th hkStringPtr in character_filenames: hkArray<hkStringPtr>
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
// src: 240, rootPath: hkStringPtr is None
```
