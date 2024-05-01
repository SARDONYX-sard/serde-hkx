# HKX types

- Offset and Size: Used to adjust the size and read/write position of the binary data to be read/write.

- Never used(on this version): `Zero`, `FunctionPointer`, `InplaceArray`, `HomogeneousArray`, `RelArray`, `Max`

| TypeKind           | C++ Type                          |  Bytes size(x86) | Bytes size(x86_64) |
| ------------------ | --------------------------------- | ---------------: | -----------------: |
| `Void`             | `void`                            |                  |                    |
| `Bool`             | `hkBool` (`bool`)                 |                1 |                  1 |
| `Char`             | `hkChar` (`signed char`)          |                1 |                  1 |
| `Int8`             | `hkInt8` (`signed char`)          |                1 |                  1 |
| `Uint8`            | `hkUint8` (`unsigned char`)       |                1 |                  1 |
| `Int16`            | `hkInt16` (`signed short`)        |                2 |                  2 |
| `Uint16`           | `hkUint16` (`unsigned short`)     |                2 |                  2 |
| `Int32`            | `hkInt32` (`signed int`)          |                4 |                  4 |
| `Uint32`           | `hkUint32` (`unsigned int`)       |                4 |                  4 |
| `Int64`            | `hkInt64` (`signed long long`)    |                8 |                  8 |
| `Uint64`           | `hkUint64` (`unsigned long long`) |                8 |                  8 |
| `Real`             | `hkReal` (`float`)                |                4 |                  4 |
| `Vector4`          | `hkVector4`                       |               16 |                 16 |
| `Quaternion`       | `hkQuaternion`                    |               16 |                 16 |
| `Matrix3`          | `hkMatrix3`                       |               48 |                 48 |
| `Rotation`         | `hkRotation`                      |               48 |                 48 |
| `QsTransform`      | `hkQsTransform`                   |               48 |                 48 |
| `Matrix4`          | `hkMatrix4`                       |               64 |                 64 |
| `Transform`        | `hkTransform`                     |               64 |                 64 |
| `Zero`             |                                   |                  |                    |
| `Pointer`          | `T*`                              |                4 |                  8 |
| `FunctionPointer`  |                                   |                4 |                  8 |
| `Array`            | `hkArray`                         |               12 |                 16 |
| `InplaceArray`     | `InplaceArray`                    |                  |                    |
| `Enum`             | `hkEnum<Enum, SizeType>`          | sizeof(SizeType) |   sizeof(SizeType) |
| `Struct`           | `class` or `struct`               |   sizeof(Struct) |     sizeof(Struct) |
| `SimpleArray`      | `hkSimpleArray`                   |                8 |                 12 |
| `HomogeneousArray` |                                   |                  |                    |
| `Variant`          | `hkVariant`                       |                8 |                 16 |
| `CString`          | `char*`                           |                4 |                  8 |
| `Ulong`            | `hkUlong`(`unsigned long`)        |                4 |                  8 |
| `Flags`            | `hkFlags<FlagEnum, SizeType>`     | sizeof(SizeType) |   sizeof(SizeType) |
| `Half`             | `hkHalf` (`hkInt16`)              |                2 |                  2 |
| `StringPtr`        | `hkStringPtr`                     |                4 |                  8 |
| `RelArray`         | `hkRelArray<T>`                   |                4 |                  4 |
| `Max`              |                                   |                  |                    |

- Which Array pattern is `hkBool[3]` etc.?

  It does not seem to be classified as `TYPE_ARRAY` or any other array.
  `vtype: TYPE_BOOL, array size: 3`, only the array size changes. There is an editing software that calls this C style Array.

## Types details

These are a summary of the assumed C++ code, the binary read/write method derived from it, and the representation method on XML.

These may change when SIMD operations are enabled.(Especially, `hkVector4`)

### `void`

No type information.

This is often used to fill in generics elements with types for which generics are not used.

- `hkArray<hkBool>` -> vtype: TYPE_ARRAY, vsubtype: TYPE_BOOL
- `hkBool` -> vtype: TYPE_BOOL, vsubtype: TYPE_VOID

---

### `hkReal`

- f32 floating point number

- C++

```cpp
typedef float hkReal;
```

- XML

Display even if the value is 0 to the 6th decimal place.

```xml
<hkparam name="maxFrictionTorque">0.000000</hkparam>
```

---

### `hkVector4`

If we use SIMD registers, we will use `__m128` and so on.

- C++

```cpp
/**
 * - byte size: 48(x86)/ 48(x86_64)
 * - ownership: Owned
 */
class __attribute__((aligned(16))) hkVector4 {
    /**
     * -    offset:  0
     * - byte size:  4(x86)/ 4(x86_64)
     */
    hkReal x;
    /**
     * -    offset:  8(x86)/ 8(x86_64)
     * - byte size:  4(x86)/ 4(x86_64)
     */
    hkReal y;
    /**
     * -    offset: 12(x86)/12(x86_64)
     * - byte size:  4(x86)/ 4(x86_64)
     */
    hkReal z;
    /**
     * -    offset: 16(x86)/16(x86_64)
     * - byte size:  4(x86)/ 4(x86_64)
     */
    hkReal w;
};
```

- XML

```xml
<hkparam name="">
  <!-- x         y        z         w -->
  (-0.000000 0.000000 -0.000000 1.000000)
</hkparam>
```

---

### `hkQuaternion`

- C++

```cpp
/**
 * - byte size: 16(x86)/ 16(x86_64)
 * - ownership: Owned
 */
class hkQuaternion {
    /**
     * Vector part
     * -    offset:  0
     * - byte size: 12(x86)/12(x86_64)
     */
    hkVector3 v;
    /**
     * Scaler part
     * - byte size: 16(x86)/16(x86_64)
     */
    hkReal s;
};
```

- XML

The w component, which is unused on XML, is not displayed.

```xml
<hkparam>
  <!--       Vector3 x      --><!-- scale -->
  (-0.000000 0.000000 -0.000000 1.000000)
</hkparam>
```

---

### `hkMatrix3`

- f32(4bytes) must be read/written for 4 \* 3 = 12 times = 48 bytes size.

- The reason for daring to use Vector4 seems to be for alignment.

- C++

```cpp
/**
 * - byte size: 48(x86)/ 48(x86_64)
 * - ownership: Owned
 */
class hkMatrix3 {
    /**
     * -    offset:  0
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 x;
    /**
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 y;
    /**
     * -    offset: 32(x86)/32(x86_64)
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 z;
};
```

- XML

The w component, which is unused on XML, is not displayed.

```xml
<hkparam>
  <!--       Vector4 x     -->
  (0.000000 0.000000 0.000000)
  <!--       Vector4 y       -->
  (-0.000000 0.000000 -0.000000)
  <!--       Vector4 z     -->
  (1.000000 1.000000 1.000000)
</hkparam>
```

---

### `hkRotation`

Same as `hkMatrix3`.

- There is a 16-byte alignment in the class, but Vector4 is used in the hkMatrix3 field, which is considered the same since it does 16-byte alignment internally.

```cpp
/**
 * - byte size: 48(x86)/ 48(x86_64)
 * - ownership: Owned
 */
class hkRotation: public hkMatrix3 {
};
```

---

### `hkQsTransform`

- C++

```cpp
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

- XML

```xml
<hkparam name="aFromBTransform">
  <!--  Vector4 transition -->
  (0.000000 0.000000 0.000000)
  <!--       Quaternion rotation      -->
  (-0.000000 0.000000 -0.000000 1.000000)
  <!--  Vector4 scale      -->
  (1.000000 1.000000 1.000000)
</hkparam>
```

---

### `hkMatrix4`

- C++

```cpp
/**
 * - byte size: 64(x86)/ 64(x86_64)
 * - ownership: Owned
 */
class hkMatrix4 {
    /**
     * -    offset:  0
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 x;
    /**
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 y;
    /**
     * -    offset: 32(x86)/32(x86_64)
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 z;
    /**
     * -    offset: 48(x86)/48(x86_64)
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 w;
};
```

- XML

```xml
<hkparam>
  <!--           Vector4 x          -->
  (0.000000 0.000000 0.000000 0.000000)
  <!--           Vector4 y          -->
  (-0.000000 0.000000 -0.000000 1.000000)
  <!--           Vector4 z          -->
  (1.000000 1.000000 1.000000 0.000000)
  <!--           Vector4 w          -->
  (1.000000 1.000000 1.000000 0.000000)
</hkparam>
```

---

### `hkTransform`

- C++

```cpp
/**
 * - byte size: 64(x86)/ 64(x86_64)
 * - ownership: Owned
 */
class hkTransform {
    /**
     * `Vector4::w`(4th) isn't used(`w` is always 0.0).
     * -    offset:  0
     * - byte size: 48(x86)/48(x86_64)
     */
    hkRotation rotation;
    /**
     * `Vector4::w`(4th) isn't used(`w` is always 0.0).
     * -    offset: 48(x86)/48(x86_64)
     * - byte size: 16(x86)/16(x86_64)
     */
    hkVector4 transition;
};
```

- XML

```xml
<hkparam name="aFromBTransform">
    <!--   Matrix3 rotation  -->
    (0.000000 0.000000 0.000000)
    (0.000000 0.000000 0.000000)
    (0.000000 0.000000 0.000000)
    <!--   Vector4 transition  -->
    (-0.000000 0.000000 -0.000000)
</hkparam>
```

---

### `Zero`

It is said to be set to 0 during serialization, but it is a deprecated item and never used.

---

### `Pointer`

- XML example

Pointers are indicated by using the index of name.

```xml
<hkobject name="#0858" class="hkbVariableBindingSet" signature="0x338ad4ff">
  <hkparam name="bindings" numelements="2">
    <hkobject>
      <hkparam name="memberPath">startStateId</hkparam>
      <hkparam name="variableIndex">6</hkparam>
      <hkparam name="bitIndex">-1</hkparam>
      <hkparam name="bindingType">BINDING_TYPE_VARIABLE</hkparam>
    </hkobject>
    <!-- ... ... -->
  </hkparam>
  <hkparam name="indexOfBindingToEnable">-1</hkparam>
</hkobject>

<hkobject name="#0859" class="hkbStateMachine" signature="0x816c1dcb">
  <hkparam name="variableBindingSet">#0858</hkparam>
</hkobject>
```

---

### `FunctionPointer`

---

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

- XML

  There seem to be 5 XML patterns for arrays among those we have identified.

  - `hkArray<hkReal>`

  ```xml
  <hkparam name="boneWeights" numelements="99">
    1.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
    0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
    0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
    0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
    0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
    0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000 0.000000
    0.000000 0.000000 0.000000
  </hkparam>
  ```

  - `hkArray<hkClass*>`
    (It can be regarded as the same as `hkArray<hkReal>` in the sense that it is space free.)

  ```xml
  <hkparam name="variantVariableValues" numelements="2">
    <!-- class pointers -->
    #0063 #0064
  </hkparam>
  ```

  - `hkArray<hkClass>`
    (e.g. `hkArray<hkRootLevelContainerNamedVariant> namedVariants` field of `hkRootLevelContainer` class)

  ```xml
  <hkparam name="namedVariants" numelements="6">
     <!--         kRootLevelContainerNamedVariant            -->
     <hkobject>
       <hkparam name="name">Merged Animation Container</hkparam>
       <hkparam name="className">hkaAnimationContainer</hkparam>
       <hkparam name="variant">#0051</hkparam>
     </hkobject>

     <!--           kRootLevelContainerNamedVariant              -->
     <hkobject>
       <hkparam name="name">Resource Data</hkparam>
       <hkparam name="className">hkMemoryResourceContainer</hkparam>
       <hkparam name="variant">#0054</hkparam>
    </hkobject>
  </hkparam>
  ```

  - `hkArray<hkStringPtr>`

    ```xml
    <hkparam name="variableNames" numelements="6">
      <hkcstring>blendDefault</hkcstring> <!-- hkStringPtr -->
      <hkcstring>blendFast</hkcstring>    <!-- hkStringPtr -->
      <hkcstring>blendSlow</hkcstring>    <!-- hkStringPtr -->
      <hkcstring>Direction</hkcstring>    <!-- hkStringPtr -->
      <hkcstring>IsBlocking</hkcstring>   <!-- hkStringPtr -->
      <hkcstring>Speed</hkcstring>        <!-- hkStringPtr -->
    </hkparam>
    ```

  - `hkArray<Vector4>`(e.g. `hkpRigidBody` class's field), `hkArray<Matrix4`, etc.

    ```xml
    <hkparam name="deactivationRefPosition" numelements="2">
      (0.000000 0.000000 0.000000 0.000000) <!-- Vector4 -->
      (0.000000 0.000000 0.000000 0.000000) <!-- Vector4 -->
    </hkparam>
    ```

---

### `InplaceArray`

---

### `hkEnum<Enum, SizeType>`

- enum type that stores only the size of `SizeType` in memory.

```cpp
/**
 * - byte size: sizeof(SizeType)(x86)/ sizeof(SizeType)(x86_64)
 * - ownership: Owned
 */
template <typename Enum, typename SizeType>
class hkEnum {
    SizeType storage;
};
```

- XML

  The tag in the enum becomes UPPER_SNAKE_CASE.

```xml
<hkparam name="type">TYPE_ANG_FRICTION</hkparam>
```

---

### `Struct`

---

### `SimpleArray`

Inline defined pointer and size type.

- Types used in the five classes

  - `hkbCharacter`(`poseLocal`)
  - `hkClass`(`declaredEnums: class hkClassEnum*`, `declaredMembers: class hkClassMember*`)
  - `hkClassEnum`(`items`)
  - `hkClassMember`(enum item)
  - `khkCustomAttributes`(`attributes: struct Attribute*`)

- This can be viewed as a structure consisting of a pointer to a certain class and immediately following it, an `int` representing the number of elements in an array.

  We used the term "viewed as" because this class doesn't actually exist; its fields are directly written into each class.

- The size is always defined as `int`, not `size_t`, which is the same as `hkArray`. In `hkArray`, the size is 16 bytes even in 64-bit environment because of this.

- Assumed C++

```cpp
/**
 * - byte size: 8(x86)/ 12(x86_64)
 * - ownership: Owned
 */
struct hkSimpleArray {
  /**
   * -    offset: 0(x86)/ 0(x86_64)
   * - byte size: 4(x86)/ 8(x86_64)
   */
   const SomeClass* m_someFieldName;
  /**
   * -    offset: 4(x86)/ 8(x86_64)
   * - byte size: 4(x86)/ 4(x86_64)
   */
   int m_numSomeFieldName;
};
```

---

### `hkVariant`

- It seems to have class and object pointers, but details are unknown.
- Only used for `value` of `hkCustomAttributesAttribute`.

- C++

```cpp
/**
 * - byte size: 8(x86)/ 16(x86_64)
 * - ownership: Owned
 */
struct hkVariant {
  /**
   * -    offset: 0(x86)/ 0(x86_64)
   * - byte size: 4(x86)/ 8(x86_64)
   */
	void* m_object;
  /**
   * -    offset: 4(x86)/ 8(x86_64)
   * - byte size: 4(x86)/ 8(x86_64)
   */
	hkClass* m_class;
};
```

---

### CString(`char*`)

- Null-terminated string type.
- It is unclear which segment (stack, heap, or other) is being pointed to because of the raw pointer.

---

### `hkUlong`(`unsigned long`)

- C++

```cpp
typedef unsigned long hkUlong;
```

---

### `hkFlags<Enum, SizeType>`

- Signed (or unsigned) size flags of 8, 16, or 32

- enum flag type that stores only the size of `SizeType` in memory.

```cpp
/**
 * - byte size: sizeof(SizeType)(x86)/ sizeof(SizeType)(x86_64)
 * - ownership: Owned
 */
template <typename FlagEnum, typename SizeType>
class hkFlags {
    SizeType storage;
};
```

- XML

```xml
<!-- pattern 1: If there are no flags at all (0bits), 0 is entered. -->
<hkparam>0</hkparam>

<!-- pattern 2: UPPER_SNAKE_CASE as in enum, but this one can also express OR bitwise operations with `|`.  -->
<hkparam>ALIGN8|ALIGN16|SERIALIZE_IGNORED</hkparam>

<!-- pattern 3: Can include arbitrary bits in addition to other bit flags. (Not sure if this makes sense) -->
<hkparam>ALIGN8|ALIGN16|SERIALIZE_IGNORE|64</hkparam>
```

---

### `hkHalf`

- Represents a 16-bit floating-point number

```cpp
/**
 * - byte size: 2(x86)/ 2(x86_64)
 * - ownership: Owned
 */
class hkHalf {
    unsigned short m_data;
};
```

---

### `hkStringPtr`

- Null-terminated string type.
- There is a flag `StringFlags::OWNED_FLAG = 0x1` defined in the class, so `Owned` is also possible.
- It is unclear which segment (stack, heap, or other) is being pointed to because of the raw pointer.

- C++

```cpp
/**
 * - byte size: 4(x86)/ 8(x86_64)
 * - ownership: Borrowed | Owned
 */
class hkStringPtr {
    const char *m_stringAndFlag;
};
```

- XML Example

  An arbitrary string is entered.

```xml
<hkparam name="name">Ragdoll_Wisp L Hand01</hkparam>
```

---

### `hkRelArray<T>`

```cpp
/**
 * - byte size: 4(x86)/ 4(x86_64)
 * - ownership:
 */
template <typename T>
class hkRelArray<T> {
  unsigned short m_size;
  unsigned short m_offset;
};
```
