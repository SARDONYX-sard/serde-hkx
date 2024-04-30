# HKX types(unverified)

- Bytes Size: When reading/writing a binary file, read/write is done only for the size, so it should be clear.

| TypeKind       | C++ Type                              |  Bytes size(x86) | Bytes size(x86_64) |
| -------------- | ------------------------------------- | ---------------: | -----------------: |
| `Void`         |                                       |                  |                    |
| `Bool`         | `hkBool` (`bool`)                     |                1 |                  1 |
| `Char`         | `hkChar` (`signed char`)              |                1 |                  1 |
| `Int8`         | `hkInt8` (`signed char`)              |                1 |                  1 |
| `Uint8`        | `hkUint8` (`unsigned char`)           |                1 |                  1 |
| `Int16`        | `hkInt16` (`signed short`)            |                2 |                  2 |
| `Uint16`       | `hkUint16` (`unsigned short`)         |                2 |                  2 |
| `Int32`        | `hkInt32` (`signed int`)              |                4 |                  4 |
| `Uint32`       | `hkUint32` (`unsigned int`)           |                4 |                  4 |
| `Int64`        | `hkInt64` (`signed long long`)        |                8 |                  8 |
| `Uint64`       | `hkUint64` (`unsigned long long`)     |                8 |                  8 |
| `Real`         | `hkReal` (`float`)                    |                4 |                  4 |
| `Vector4`      | `hkVector4`                           |               16 |                 16 |
| `Quaternion`   | `hkQuaternion`                        |               16 |                 16 |
| `Matrix3`      | `hkMatrix3`                           |               48 |                 48 |
| `Rotation`     | `hkRotation`                          |               48 |                 48 |
| `QsTransform`  | `hkQsTransform`                       |               48 |                 48 |
| `Matrix4`      | `hkMatrix4`                           |               64 |                 64 |
| `Transform`    | `hkTransform`                         |               64 |                 64 |
| `Array`        | `hkArray`                             |               12 |                 16 |
| `InplaceArray` | `InplaceArray`                        |    sizeof(Array) |      sizeof(Array) |
| `Enum`         | `hkEnum<Enum, SizeType>`              | sizeof(SizeType) |   sizeof(SizeType) |
| `Struct`       | `Class T`                             |   sizeof(Struct) |     sizeof(Struct) |
| `SimpleArray`  | `T[N]` (e.g. `hkBool[3]`)             |    sizeof(Array) |      sizeof(Array) |
| `Variant`      | `hkVariant` (`void*` & `hkClass*`)    |                4 |                  8 |
| `CString`      | `char*`                               |                4 |                  8 |
| `Ulong`        | `hkUlong`(`unsigned long`)            |                4 |                  8 |
| `Flags`        | `hkFlags<Enum, SizeType>` (`hkInt16`) |                2 |                  2 |
| `Half`         | `hkHalf` (`hkInt16`)                  |                2 |                  2 |
| `StringPtr`    | `hkStringPtr`                         |                4 |                  8 |
| `RelArray`     | `hkRelArray<T>`                       |                4 |                  4 |
| `Max`          |                                       |                  |                    |

## Container types details

### `hkVector4`

### `hkQuaternion`

### `hkMatrix3`

### `hkRotation`

### `hkQsTransform`

```cpp
// This is  closer code.

/**
 * - byte size: 48(x86)/ 48(x86_64)
 * - ownership: Owned
 */
class hkQsTransform {
    /**
     * `Vector4::w`(4th) isn't used(`w` is always 0.0).
     * -    offset:  0
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 transition;
    /**
     * -    offset: 16(x86)/16(x86_64)
     * - byte size: 16(x86)/16(x86_64)
     */
    hkQuaternion rotation;
    /**
     * `Vector4::w`(4th) isn't used(`w` is always 0.0).
     * -    offset: 32(x86)/32(x86_64)
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 scale;
};
```

In XML, it would be as follows.(Note that the `w` in `transition` and `scale` are not used, so those two have only three elements.)

```xml
<!--                            < - vector4 transition - - >< - - - - quaternion rotation - - - - > < - vector4 scale - - - - >    -->
<hkparam name="aFromBTransform">(0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000)</hkparam>
```

### `hkMatrix4`

In XML, it would be as follows.

```xml
<tag>(0.000000 0.000000 0.000000 0.000000)(-0.000000 0.000000 -0.000000 1.000000)(1.000000 1.000000 1.000000 0.000000)(1.000000 1.000000 1.000000 0.000000)</tag>
```

### `hkTransform`

### `hkArray`

- Since `T *m_data` is a raw pointer, it is not clear whether it is assigned to the stack or heap segment.

```cpp
// This is  closer code.

/**
 * - byte size: 12(x86)/ 16(x86_64)
 * - ownership: Owned
 */
template <typename T>
class hkArray {
    /**
     * -    offset: 0
     * - byte size: 4(x86)/ 8(x86_64)
     */
    T *m_data;
    /**
     * -    offset: 4(x86)/ 8(x86_64)
     * - byte size: 4(x86)/ 4(x86_64)
     */
    int m_size;
    /**
     * The upper 2bits are flags indicating the allocation status.
     * -    offset: 8(x86)/12(x86_64)
     * - byte size: 4(x86)/ 4(x86_64)
     */
    int m_capacityAndFlags;
};
```

- XML Example

```xml
<hkparam name="variableNames" numelements="6">
  <hkcstring>blendDefault</hkcstring>
  <hkcstring>blendFast</hkcstring>
  <hkcstring>blendSlow</hkcstring>
  <hkcstring>Direction</hkcstring>
  <hkcstring>IsBlocking</hkcstring>
  <hkcstring>Speed</hkcstring>
</hkparam>
```

### `InplaceArray`

### `hkEnum<Enum, SizeType>`

- enum type that stores only the size of `SizeType` in memory.

```cpp
template <typename Enum, typename SizeType>
class hkEnum {
    SizeType storage;
};
```

```xml
<hkparam name="type">TYPE_ANG_FRICTION</hkparam>
```

### `Struct`

### `SimpleArray`

### `hkVariant`

### CString(`char*`)

- Null-terminated string type.
- It is unclear which segment (stack, heap, or other) is being pointed to because of the raw pointer.

### `hkUlong`(`unsigned long`)

### `hkFlags<Enum, SizeType>`(`hkInt16`)

### `hkHalf`

- Represents a 16-bit floating-point number

```cpp
// This is  closer code.

/**
 * - byte size: variable length
 * - byte size: 2(x86)/ 2(x86_64)
 * - ownership: Owned
 */
class hkHalf {
    unsigned short m_data;
};
```

### `hkStringPtr`

- Null-terminated string type.
- There is a flag `StringFlags::OWNED_FLAG = 0x1` defined in the class, so `Owned` is also possible.
- It is unclear which segment (stack, heap, or other) is being pointed to because of the raw pointer.

```cpp
// This is  closer code.

/**
 * - byte size: variable length
 * - byte size: 4(x86)/ 8(x86_64)
 * - ownership: Borrowed | Owned
 */
class hkStringPtr {
    const char *m_stringAndFlag;
};
```

- XML Example

```xml
<hkparam name="name">Ragdoll_Wisp L Hand01</hkparam>
```

### `hkRelArray<T>`

```cpp
template <typename T>
class hkRelArray<T> {
  unsigned short m_size;
  unsigned short m_offset;
};
```
