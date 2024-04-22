# defaultmale x86_64 C++ Class Layout

[See MemoryLayout](https://godbolt.org/z/MnzEqz7xn)

```txt
*** Dumping AST Record Layout
         0 | class hkArray<struct hkRootLevelContainerNamedVariant>
         0 |   struct hkRootLevelContainerNamedVariant * m_data
         8 |   int m_size
        12 |   int m_capacityAndFlags
           | [sizeof=16, dsize=16, align=8,
           |  nvsize=16, nvalign=8]

*** Dumping AST Record Layout
         0 | struct hkRootLevelContainer
         0 |   class hkArray<struct hkRootLevelContainerNamedVariant> namedVariants
         0 |     struct hkRootLevelContainerNamedVariant * m_data
         8 |     int m_size
        12 |     int m_capacityAndFlags
           | [sizeof=16, dsize=16, align=8,
           |  nvsize=16, nvalign=8]

*** Dumping AST Record Layout
         0 | class hkStringPtr
         0 |   const char * m_stringAndFlag
           | [sizeof=8, dsize=8, align=8,
           |  nvsize=8, nvalign=8]

*** Dumping AST Record Layout
         0 | struct hkRootLevelContainerNamedVariant
         0 |   class hkStringPtr name
         0 |     const char * m_stringAndFlag
         8 |   class hkStringPtr className
         8 |     const char * m_stringAndFlag
        16 |   struct hkReferencedObject * variant
           | [sizeof=24, dsize=24, align=8,
           |  nvsize=24, nvalign=8]

*** Dumping AST Record Layout
         0 | class hkBaseObject
         0 |   (hkBaseObject vtable pointer)
           | [sizeof=8, dsize=8, align=8,
           |  nvsize=8, nvalign=8]

*** Dumping AST Record Layout
         0 | class hkReferencedObject
         0 |   class hkBaseObject (primary base)
         0 |     (hkBaseObject vtable pointer)
         8 |   hkUint16 memSizeAndFlags
        10 |   hkUint16 referenceCount
        12 |   char[4] _pad0
           | [sizeof=16, dsize=16, align=8,
           |  nvsize=16, nvalign=8]

*** Dumping AST Record Layout
         0 | struct hkVector4
         0 |   hkReal x
         4 |   float y
         8 |   float z
        12 |   float w
           | [sizeof=16, dsize=16, align=16,
           |  nvsize=16, nvalign=16]

*** Dumping AST Record Layout
         0 | class hkEnum<enum EventMode, char>
         0 |   char storage
           | [sizeof=1, dsize=1, align=1,
           |  nvsize=1, nvalign=1]

*** Dumping AST Record Layout
         0 | struct hkbProjectData
         0 |   class hkReferencedObject (primary base)
         0 |     class hkBaseObject (primary base)
         0 |       (hkBaseObject vtable pointer)
         8 |     hkUint16 memSizeAndFlags
        10 |     hkUint16 referenceCount
        12 |     char[4] _pad0
        16 |   struct hkVector4 worldUpWS
        16 |     hkReal x
        20 |     float y
        24 |     float z
        28 |     float w
        32 |   struct hkbProjectStringData * stringData
        40 |   class hkEnum<enum EventMode, char> defaultEventMode
        40 |     char storage
        41 |   char[7] _pad0
           | [sizeof=48, dsize=48, align=16,
           |  nvsize=48, nvalign=16]

*** Dumping AST Record Layout
         0 | class hkArray<class hkStringPtr>
         0 |   class hkStringPtr * m_data
         8 |   int m_size
        12 |   int m_capacityAndFlags
           | [sizeof=16, dsize=16, align=8,
           |  nvsize=16, nvalign=8]

*** Dumping AST Record Layout
         0 | struct hkbProjectStringData
         0 |   class hkReferencedObject (primary base)
         0 |     class hkBaseObject (primary base)
         0 |       (hkBaseObject vtable pointer)
         8 |     hkUint16 memSizeAndFlags
        10 |     hkUint16 referenceCount
        12 |     char[4] _pad0
        16 |   class hkArray<class hkStringPtr> animation_filenames
        16 |     class hkStringPtr * m_data
        24 |     int m_size
        28 |     int m_capacityAndFlags
        32 |   class hkArray<class hkStringPtr> behavior_filenames
        32 |     class hkStringPtr * m_data
        40 |     int m_size
        44 |     int m_capacityAndFlags
        48 |   class hkArray<class hkStringPtr> character_filenames
        48 |     class hkStringPtr * m_data
        56 |     int m_size
        60 |     int m_capacityAndFlags
        64 |   class hkArray<class hkStringPtr> event_names
        64 |     class hkStringPtr * m_data
        72 |     int m_size
        76 |     int m_capacityAndFlags
        80 |   class hkStringPtr animation_path
        80 |     const char * m_stringAndFlag
        88 |   class hkStringPtr behavior_path
        88 |     const char * m_stringAndFlag
        96 |   class hkStringPtr character_path
        96 |     const char * m_stringAndFlag
       104 |   class hkStringPtr full_path_to_source
       104 |     const char * m_stringAndFlag
       112 |   class hkStringPtr root_path
       112 |     const char * m_stringAndFlag
           | [sizeof=120, dsize=120, align=8,
           |  nvsize=120, nvalign=8]
```
