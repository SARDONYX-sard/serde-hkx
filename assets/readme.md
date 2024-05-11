# Havok Class information for `hk_2010.2.0-r1` (32bit/64bit)

The [`hkxcmd Report` (32bit info)](https://github.com/figment/hkxcmd) and the SKSE dump of [`HKX2Library` (64bit info)](https://github.com/ret2end/HKX2Library) gave x86 and x86_64 offset information.

We then generated a set of Json files here using this offset & size information.

As the Json file prefixed with `BS`, it contains some `Bethesda` proprietary extension classes.

## Details of this Json format

C++ Class members

- `name`: C++ member name.

- `class_ref`: If `vtype` is `TYPE_POINTER` or `TYPE_STRUCT`, the structure name used is entered. Otherwise, this entry does not exist.

- `vtype`: The type of TypeKind defined in the [hkx_types.md](../docs/specification/hkx_types.md) specification will be entered. (However, it starts with `TYPE_` at the beginning in UPPER_CASE)

- `vsubtype`: The type inside the generics of Array, the size that holds the enum data will be entered.

- `arrsize`: Used only when the size of arrays is predetermined, such as `hkBool[3]`. Otherwise, 0.
  Note: this is not used when `vtype` is `TYPE_ARRAY`.
  For example, `hkBool[3]` => vtype: `TYPE_BOOL`, `arrsize`: 3.

- `flags`: flags for information such as owned type, alignment, skip serialization, etc.

  The following types of flags exist.(Note that any flag will contain `FLAGS_NONE` if it ends in 0)

  ```c
  enum Flags {
    FLAGS_NONE = 0;
    ALIGN_8 = 128;
    ALIGN_16 = 256;
    NOT_OWNED = 512;
    SERIALIZE_IGNORED = 1024;
  };
  ```

- `default`: The default value. The default value is basically 0, but some values are different.

## Exertions

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
