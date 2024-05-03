# Hkx binary data format

- Note
  - That offset is 0 based index.
  - Endianness should always be taken into account for data larger than 1bytes.

## Hkx one file binary format

| Field                                                    | Size (bytes)    | Offset (bytes)  |
| -------------------------------------------------------- | --------------- | --------------- |
| hkx file header                                          | 64              | 0               |
| padding for `section_offset` in header(Here we assume 0) | 0               | 64              |
| section header of `__classnames__`                       | 48              | 64              |
| section header of `__type__`                             | 48              | 112             |
| section header of `__data__`                             | 48              | 160             |
| section of `__classnames__`                              | variable length | 208             |
| section of `__types__`(Here we assume 0)                 | 0               | variable length |
| section of `__data__`                                    | variable length | variable length |
| fixups of `__classnames__`                               | variable length | variable length |
| fixups of `__types__`                                    | variable length | variable length |
| fixups of `__data__`                                     | variable length | variable length |

## Hkx file header(64bytes)

| Field                              | Description                                                     | Size (bytes) | Offset (bytes) |
| ---------------------------------- | --------------------------------------------------------------- | ------------ | -------------- |
| magic0                             | First magic number (`0x57E0E057`)                               | 4            | 0              |
| magic1                             | Second magic number (`0x10C0C010`)                              | 4            | 4              |
| user_tag                           | User-defined tag.                                               | 4            | 8              |
| file_version                       | Version of the file.                                            | 4            | 12             |
| pointer_size                       | Size of pointers in bytes (4 or 8)                              | 1            | 16             |
| endian                             | Endianness of the file (0 for big-endian, 1 for little-endian). | 1            | 17             |
| padding_option                     | Padding option used in the file.                                | 1            | 18             |
| base_class                         | Base class.                                                     | 1            | 19             |
| section_count                      | Number of sections in the HKX file.                             | 4            | 20             |
| contents_section_index             | Index of the contents section.                                  | 4            | 24             |
| contents_section_offset            | Offset of the contents section.                                 | 4            | 28             |
| contents_class_name_section_index  | Index of the contents class name section.                       | 4            | 32             |
| contents_class_name_section_offset | Offset of the contents class name section.                      | 4            | 36             |
| contents_version_string            | Version string of the contents. + separator(0xFF)               | 16           | 40             |
| flags                              | Various flags.                                                  | 4            | 56             |
| max_predicate                      | Maximum predicate. None is -1 (== `0xFF 0xFF`)                  | 2            | 60             |
| section_offset                     | Section offset. None is -1 (== `0xFF 0xFF`)                     | 2            | 62             |

- padding_option:
  Alignment or not until ptr size?
  - 0 -> Do nothing
  - 1 -> `void*`, `hkArray`, `struct T*(class ptr)`, `hkStringPtr`, `CString(char*)`, means there is a blank space of ptr size at the beginning when reading/writing.

## Section header(48bytes)

| Field                 | Description                                             | Size (bytes) | Offset (bytes) |
| --------------------- | ------------------------------------------------------- | ------------ | -------------- |
| section_tag           | Section name (e.g. `__data__`).                         | 19           | 0              |
| section_tag_separator | Always must be `0xFF`                                   | 1            | 19             |
| absolute_data_start   | Section start & fixup base offset.                      | 4            | 20             |
| local_fixups_offset   | Offset from absolute offset to local fixup map.         | 4            | 24             |
| global_fixups_offset  | Offset from absolute offset to global fixup map.        | 4            | 28             |
| virtual_fixups_offset | Offset from absolute offset to virtual class fixup map. | 4            | 32             |
| exports_offset        | Offset from absolute offset to exports.                 | 4            | 36             |
| imports_offset        | Offset from absolute offset to imports.                 | 4            | 40             |
| end_offset            | End offset of the section.                              | 4            | 44             |

- `section_tag`: The string is a null-terminated string and is filled with zeros.

- `absolute_data_start`: Section start & fixup base offset.

  - Example Calculation formula:
    `Hkx header size(64) + hkx_header.section_offset(0) + Section header size(48) * section count(3) = 208(bytes) == 0xD0(bytes)`

## Section

### Classnames section

- The following classes are always present in a single file.

  - `hkClass`
  - `hkClassMember`
  - `hkClassEnum`
  - `hkClassEnumItem`

The `hkRootLevelContainer` is present in most hkx files, but not always as far as the C# implementation is concerned.

- Note
  If you think that "signature" is a unique number of a class, you will make an error.
  The same signature is used in C++ class inheritance in some cases, and it is dangerous to judge a class by its signature.
  It is better to use the class name to identify the class, since the class name itself is never covered by the signature.

| Field                     | Description                                                 | Size (bytes)    | Offset (bytes)  |
| ------------------------- | ----------------------------------------------------------- | --------------- | --------------- |
| signature/class_name pair | `0x75585EF6` + `0x09` + `hkClass\0`(8bytes)                 | 13              | 208             |
| signature/class_name pair | `0x5C7EA4C2` + `0x09` + `hkClassMember\0`(14bytes)          | 19              | 227             |
| signature/class_name pair | `0x8A3609CF` + `0x09` + `hkClassEnum\0`(12bytes)            | 17              | 244             |
| signature/class_name pair | `0xCE6F8A6C` + `0x09` + `hkClassEnumItem\0`(16bytes)        | 21              | 265             |
| signature/class_name pair | `0x2772C11E` + `0x09` + `hkRootLevelContainer\0`(21bytes)   | 26              | 291             |
| ...                       | ...                                                         | ...             | ...             |
| 16bytes alignments        | Fill bytes with `0xFF` until the number is a multiple of 16 | variable length | variable length |

- Signature & ClassName pair

| Field      | Description                                              | Size (bytes)    | Offset (bytes) |
| ---------- | -------------------------------------------------------- | --------------- | -------------- |
| signature  | e.g.`0x2772C11E`                                         | 4               | 0              |
| separator  | Always `0x09`                                            | 1               | 4              |
| class name | A null-terminated string.(e.g. `hkRootLevelContainer\0`) | variable length | 5              |

### Types section

Basically, we can assume that it does not exist.

### Data section

Fields of havok class

## Fixups

A. In binary data, a fixup refers to making adjustments or corrections to the binary representation of a program or data.

Here, it is a key/value pair where 'the current location where the binary data is being read' is the key and 'the location where the binary data exists' is the value.

- `local fixup`(4 \* 2 = 8bytes):
  This information indicates where the data of the field in the Class will be placed.

  ```rust
  pub struct LocalFixup {
      /// Current reader seek position
      pub src: u32,
      /// This position indicates where the data of the field in the Class will be placed.
      ///
      /// Example: the start of `hkStringPtr`, etc.
      /// - Relative position from `section_data`(This is start with `absolute_data_offset` position).
      /// `data_section[dst]` == The field's content in Class
      pub dst: u32,
  }
  ```

- `global fixup`(4 \* 3 = 12bytes):
  Location information needed when referencing class pointer, etc.

  ```rust
  #[repr(C)]
  pub struct GlobalFixup {
      /// Current reader seek position
      pub src: u32,
      ///  Index Section ID
      ///
      /// # Examples
      /// - 0: `__class_names__`
      /// - 1: `__type__`
      /// - 2: `__data__`
      pub dst_section_index: u32,
      /// Location information needed when referencing class pointer, etc.
      ///
      /// # Examples
      /// - When global map dst is 128
      ///
      /// `128` =>
      /// If `virtualFixup.src == 128` => `virtualFixup.class_name_offset` =>
      /// `class_names[class_name_offset]` => class name
      pub dst: u32,
  }
  ```

- `virtual fixup`(4 \* 3 = 12bytes):
  Location information for the name of the C++ class that must call the constructor.

  ```rust
  #[repr(C)]
  pub struct VirtualFixup {
      /// Current reader seek position
      ///
      /// It is partially the same as the value in `globalFixup.dst`.
      pub src: u32,
      ///  Index Section ID
      ///
      /// # Examples
      /// - 0: `__class_names__`
      /// - 1: `__type__`
      /// - 2: `__data__`
      pub section_index: u32,
      /// Havok Class name start position in `__class_name__` section.
      pub name_offset: u32,
  }
  ```
