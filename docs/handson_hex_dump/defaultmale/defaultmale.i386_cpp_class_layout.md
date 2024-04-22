# defaultmale x86 C++ Class Layout

[See MemoryLayout](https://godbolt.org/z/fr3nPT769)

```txt
** Dumping AST Record Layout
         0 | class hkArray<struct hkRootLevelContainerNamedVariant>
         0 |   struct hkRootLevelContainerNamedVariant * m_data
         4 |   int m_size
         8 |   int m_capacityAndFlags
           | [sizeof=12, dsize=12, align=4,
           |  nvsize=12, nvalign=4]

*** Dumping AST Record Layout
         0 | struct hkRootLevelContainer
         0 |   class hkArray<struct hkRootLevelContainerNamedVariant> namedVariants
         0 |     struct hkRootLevelContainerNamedVariant * m_data
         4 |     int m_size
         8 |     int m_capacityAndFlags
           | [sizeof=12, dsize=12, align=4,
           |  nvsize=12, nvalign=4]

*** Dumping AST Record Layout
         0 | class hkStringPtr
         0 |   const char * m_stringAndFlag
           | [sizeof=4, dsize=4, align=4,
           |  nvsize=4, nvalign=4]

*** Dumping AST Record Layout
         0 | struct hkRootLevelContainerNamedVariant
         0 |   class hkStringPtr name
         0 |     const char * m_stringAndFlag
         4 |   class hkStringPtr className
         4 |     const char * m_stringAndFlag
         8 |   struct hkReferencedObject * variant
           | [sizeof=12, dsize=12, align=4,
           |  nvsize=12, nvalign=4]

*** Dumping AST Record Layout
         0 | class hkBaseObject
         0 |   (hkBaseObject vtable pointer)
           | [sizeof=4, dsize=4, align=4,
           |  nvsize=4, nvalign=4]

*** Dumping AST Record Layout
         0 | class hkReferencedObject
         0 |   class hkBaseObject (primary base)
         0 |     (hkBaseObject vtable pointer)
         4 |   hkUint16 memSizeAndFlags
         6 |   hkUint16 referenceCount
         8 |   char[0] _pad0
           | [sizeof=8, dsize=8, align=4,
           |  nvsize=8, nvalign=4]

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
         4 |     hkUint16 memSizeAndFlags
         6 |     hkUint16 referenceCount
         8 |     char[0] _pad0
        16 |   struct hkVector4 worldUpWS
        16 |     hkReal x
        20 |     float y
        24 |     float z
        28 |     float w
        32 |   struct hkbProjectStringData * stringData
        36 |   class hkEnum<enum EventMode, char> defaultEventMode
        36 |     char storage
        37 |   char[11] _pad0
           | [sizeof=48, dsize=48, align=16,
           |  nvsize=48, nvalign=16]

*** Dumping AST Record Layout
         0 | class hkArray<class hkStringPtr>
         0 |   class hkStringPtr * m_data
         4 |   int m_size
         8 |   int m_capacityAndFlags
           | [sizeof=12, dsize=12, align=4,
           |  nvsize=12, nvalign=4]

*** Dumping AST Record Layout
         0 | struct hkbProjectStringData
         0 |   class hkReferencedObject (primary base)
         0 |     class hkBaseObject (primary base)
         0 |       (hkBaseObject vtable pointer)
         4 |     hkUint16 memSizeAndFlags
         6 |     hkUint16 referenceCount
         8 |     char[0] _pad0
         8 |   class hkArray<class hkStringPtr> animation_filenames
         8 |     class hkStringPtr * m_data
        12 |     int m_size
        16 |     int m_capacityAndFlags
        20 |   class hkArray<class hkStringPtr> behavior_filenames
        20 |     class hkStringPtr * m_data
        24 |     int m_size
        28 |     int m_capacityAndFlags
        32 |   class hkArray<class hkStringPtr> character_filenames
        32 |     class hkStringPtr * m_data
        36 |     int m_size
        40 |     int m_capacityAndFlags
        44 |   class hkArray<class hkStringPtr> event_names
        44 |     class hkStringPtr * m_data
        48 |     int m_size
        52 |     int m_capacityAndFlags
        56 |   class hkStringPtr animation_path
        56 |     const char * m_stringAndFlag
        60 |   class hkStringPtr behavior_path
        60 |     const char * m_stringAndFlag
        64 |   class hkStringPtr character_path
        64 |     const char * m_stringAndFlag
        68 |   class hkStringPtr full_path_to_source
        68 |     const char * m_stringAndFlag
        72 |   class hkStringPtr root_path
        72 |     const char * m_stringAndFlag
           | [sizeof=76, dsize=76, align=4,
           |  nvsize=76, nvalign=4]
```
