# Havok Class information for `hk_2010.2.0-r1` (32bit/64bit)

source of C++ information

- 32bit info: [`hkxcmd Report` output files](https://github.com/figment/hkxcmd)
- 64bit info: [`HKX2Library` (This is SKSE dump info)](https://github.com/ret2end/HKX2Library)

Furthermore, from the information in hkxcmd, I created my own automatic calculation program to calculate the missing classes in HKX2Library, taking into account the C++ conventions. (In other words, there is no guarantee that the data is correct.)

We then generated a set of Json files here using this offset & size information.

Json files prefixed with `BS` are `Bethesda` proprietary extension classes.

## Details of this Json format

C++ Class members

- `name`: C++ member name.

- `class_ref`: If `vtype` is `TYPE_POINTER` or `TYPE_STRUCT`, the structure name used is entered. Otherwise, this entry does not exist.

- `vtype`: The type of TypeKind defined in the [hkx_types.md](../docs/specification/hkx_types.md) specification will be entered. (However, it starts with `TYPE_` at the beginning in UPPER_CASE)

- `vsubtype`: The type inside the generics of Array, the size that holds the enum data will be entered.

- `arrsize`: Used only when the size of arrays is predetermined, such as `hkBool[3]`. Otherwise, 0.

  Note: **This is not used when `vtype` is `TYPE_ARRAY`.**

  - For example `hkBool[3]`

    ```json
    {
      "vtype": "TYPE_BOOL",
      "arrsize": 3
    }
    ```

- `flags`: flags for information such as owned type, alignment, skip serialization, etc.

  The following types of flags exist.(Note that any flag will contain `FLAGS_NONE` if it ends in 0)

  ```c
  enum FlagValues {
    FLAGS_NONE = 0;
    ALIGN_8 = 128;
    ALIGN_16 = 256;
    NOT_OWNED = 512;
    SERIALIZE_IGNORED = 1024;
  };
  ```

- `default`: The default value. The default value is basically 0, but some values are different.

## Exceptions

- Reflection data in `hk_2010.2.0-r1`, but some data hidden by multiple inheritance of parent classes etc. do not exist.

```jsonc
{
  "name": "hkpCollisionFilter",
  "parent": ["hkReferencedObject", "hkpCollidableCollidableFilter", "hkpShapeCollectionFilter", "hkpRayShapeCollectionFilter", "hkpRayCollidableFilter"]
},
{
  "name": "hkpShapeCollection",
  "parent": ["hkpShape", "hkpShapeContainer"]
},
{
  "name": "hkpConvexListShape",
  "parent": ["hkpConvexShape", "hkpShapeContainer"]
},
{
  "name": "hkpRemoveTerminalsMoppModifier",
  // NOTE: "hkpMoppModifier" is not exist in reflection data.
  "parent": ["hkReferencedObject", "hkpMoppModifier"]
},
{
  "name": "hkpTriggerVolume",
  // NOTE: "hkpContactListener", "hkpWorldPostSimulationListener", "hkpEntityListener"
  //       is not exist in reflection data.
  "parents": ["hkReferencedObject", "hkpContactListener", "hkpWorldPostSimulationListener", "hkpEntityListener"]
}
```

- Unknown paddings

```rust
// Unknown padding(char _unknown_padding[4];(+4) before 0th field)
"BSRagdollContactListenerModifier" => cpp_class.members[0].offset_x86_64 = 88,

// Offset by field that does not appear in reflection data(struct hkpWorldDynamicsStepInfo m_dynamicsStepInfo;(+328))
"hkpWorld" => cpp_class.members[43].offset_x86_64 = 1008,
```

- Different signature

There is one place where the signature of Class is different between hkxcmd(32bit info) and HKX2Library(64bit info).(Reflection data is adopted from hkxcmd.)

hkpCompressedMeshShape

- `hkxcmd`: 0xa62d5e6e
- `HKX2Library`: 0xe3d1dba

## Licenses

First of all, the 64-bit information is extracted from the comments from 'HKX2Library' and is clearly part of the source code. Therefore, the license is clearly stated here.

```txt
SPDX-License-Identifier: MIT
SPDX-FileCopyrightText: 2021 kreny
```

The source code for “hkxcmd” is BSD, but the SDK is under the Havok license.
However, it is not likely that the data output by the `hkxcmd Report` is under the influence of the license.

The compiler and linker do not impose a license on the output data.

The same is true for the types section information in HavokBehaviorProcess.exe.
