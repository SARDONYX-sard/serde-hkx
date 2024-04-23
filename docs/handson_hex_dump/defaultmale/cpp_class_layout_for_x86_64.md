# defaultmale x86_64 C++ Class Layout

[See MemoryLayout](https://godbolt.org/z/dz9cj5GEs)

```log
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

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xc9e6c38 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkRootLevelContainerNamedVariant'
| `-RecordType 0xc9e38d0 'hkRootLevelContainerNamedVariant'
|   `-CXXRecord 0xc9e7498 'hkRootLevelContainerNamedVariant'
|-CXXRecordDecl 0xc9e6e80 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xc9e6f30 <line:23:4, col:11> col:4 private
|-FieldDecl 0xc9e7008 <line:24:5, col:8> col:8 m_data 'hkRootLevelContainerNamedVariant *'
|-FieldDecl 0xc9e7060 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xca007e0 <line:25:8, col:78>
|   `-ParagraphComment 0xca007b0 <col:8, col:78>
|     `-TextComment 0xca00780 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xc9e70b8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xca008d0 <line:28:8, line:29:22>
|   `-ParagraphComment 0xca008a0 <line:28:8, line:29:22>
|     |-TextComment 0xca00850 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xca00870 <line:29:7, col:22> Text=" for the array. "
|-CXXConstructorDecl 0xc9f9508 <line:22:7> col:7 implicit constexpr hkArray 'void () noexcept' inline default trivial
|-CXXDestructorDecl 0xc9f9600 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9f99b8 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkRootLevelContainerNamedVariant> &)' inline default trivial noexcept-unevaluated 0xc9f99b8
| `-ParmVarDecl 0xc9f9af8 <col:7> col:7 'const hkArray<hkRootLevelContainerNamedVariant> &'
`-CXXConstructorDecl 0xc9f9be8 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkRootLevelContainerNamedVariant> &&)' inline default trivial noexcept-unevaluated 0xc9f9be8
  `-ParmVarDecl 0xc9fa188 <col:7> col:7 'hkArray<hkRootLevelContainerNamedVariant> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9e3630 <<source>:53:1, line:57:1> line:53:8 referenced struct hkRootLevelContainer definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-FullComment 0xca00a70 <line:49:4, line:52:18>
| `-ParagraphComment 0xca00a40 <line:49:4, line:52:18>
|   |-TextComment 0xca009a0 <line:49:4, col:35> Text=" -      size: 16(if 32bit -> 12)"
|   |-TextComment 0xca009c0 <line:50:4, col:22> Text=" -    vtable: false"
|   |-TextComment 0xca009e0 <line:51:4, col:29> Text=" - signature: `0x2772c11e`"
|   `-TextComment 0xca00a00 <line:52:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xc9e3770 <line:53:1, col:8> col:8 implicit struct hkRootLevelContainer
|-CXXRecordDecl 0xc9e3820 parent 0xc96a548 <line:56:13, col:20> col:20 struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xc9e7128 <col:5, col:54> col:54 namedVariants 'hkArray<struct hkRootLevelContainerNamedVariant>':'hkArray<hkRootLevelContainerNamedVariant>'
|-CXXConstructorDecl 0xc9f93f8 <line:53:8> col:8 implicit referenced constexpr hkRootLevelContainer 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9f9768 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (const hkRootLevelContainer &)' inline default trivial noexcept-unevaluated 0xc9f9768
| `-ParmVarDecl 0xc9f98a8 <col:8> col:8 'const hkRootLevelContainer &'
|-CXXConstructorDecl 0xc9fa288 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (hkRootLevelContainer &&)' inline default trivial noexcept-unevaluated 0xc9fa288
| `-ParmVarDecl 0xc9fa3c8 <col:8> col:8 'hkRootLevelContainer &&'
`-CXXDestructorDecl 0xc9fa548 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainer 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9c2008 <<source>:4:1, line:7:1> line:4:7 referenced class hkStringPtr definition
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xc9c2140 <col:1, col:7> col:7 implicit class hkStringPtr
|-AccessSpecDecl 0xc9c21f0 <line:5:4, col:11> col:4 private
|-FieldDecl 0xc9c2238 <line:6:5, col:17> col:17 m_stringAndFlag 'const char *'
|-CXXMethodDecl 0xc9f6898 <line:4:7> col:7 implicit constexpr operator= 'hkStringPtr &(const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xc9f6898
| `-ParmVarDecl 0xc9f69c8 <col:7> col:7 'const hkStringPtr &'
|-CXXMethodDecl 0xc9f6aa8 <col:7> col:7 implicit constexpr operator= 'hkStringPtr &(hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xc9f6aa8
| `-ParmVarDecl 0xc9f6bd8 <col:7> col:7 'hkStringPtr &&'
|-CXXDestructorDecl 0xc9f8eb0 <col:7> col:7 implicit constexpr ~hkStringPtr 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9fa948 <col:7> col:7 implicit used constexpr hkStringPtr 'void () noexcept' inline default trivial
| `-CompoundStmt 0xca002a8 <col:7>
|-CXXConstructorDecl 0xc9fac60 <col:7> col:7 implicit constexpr hkStringPtr 'void (const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xc9fac60
| `-ParmVarDecl 0xc9fad98 <col:7> col:7 'const hkStringPtr &'
`-CXXConstructorDecl 0xc9fae20 <col:7> col:7 implicit constexpr hkStringPtr 'void (hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xc9fae20
  `-ParmVarDecl 0xc9faf58 <col:7> col:7 'hkStringPtr &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkStringPtr = type { ptr }
  NonVirtualBaseLLVMType:%class.hkStringPtr = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9e7498 prev 0xc9e3820 <<source>:70:1, line:80:1> line:70:8 referenced struct hkRootLevelContainerNamedVariant definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xc9e7598 <col:1, col:8> col:8 implicit struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xc9e7688 <line:73:5, col:17> col:17 name 'hkStringPtr'
|-FieldDecl 0xc9e76f0 <line:76:5, col:17> col:17 className 'hkStringPtr'
|-CXXRecordDecl 0xc9e7748 parent 0xc96a548 <line:79:5, col:12> col:12 struct hkReferencedObject
|-FieldDecl 0xc9e78f0 <col:5, col:32> col:32 variant 'struct hkReferencedObject *'
|-CXXConstructorDecl 0xc9fa838 <line:70:8> col:8 implicit referenced constexpr hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9faaa8 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (const hkRootLevelContainerNamedVariant &)' inline default trivial noexcept-unevaluated 0xc9faaa8
| `-ParmVarDecl 0xc9fabe8 <col:8> col:8 'const hkRootLevelContainerNamedVariant &'
|-CXXConstructorDecl 0xc9fb058 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (hkRootLevelContainerNamedVariant &&)' inline default trivial noexcept-unevaluated 0xc9fb058
| `-ParmVarDecl 0xc9fc1d8 <col:8> col:8 'hkRootLevelContainerNamedVariant &&'
`-CXXDestructorDecl 0xc9fc328 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9ea218 <<source>:95:1, line:98:1> line:95:7 referenced class hkBaseObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor can_const_default_init
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment
| `-Destructor non_trivial user_declared
|-FullComment 0xca0f050 <line:89:4, line:94:30>
| |-ParagraphComment 0xca0efd0 <line:89:4, line:92:40>
| | |-TextComment 0xca0ef30 <line:89:4, col:35> Text=" The class size is pointer size."
| | |-TextComment 0xca0ef50 <line:90:4, col:80> Text=" The SDK description says that the `hkBaseObject`, a virtual function without"
| | |-TextComment 0xca0ef70 <line:91:4, col:75> Text=" a field, is the source of inheritance for all Havok Classes so that the"
| | `-TextComment 0xca0ef90 <line:92:4, col:40> Text=" vtable does not come after the data."
| `-ParagraphComment 0xca0f020 <line:94:4, col:30>
|   `-TextComment 0xca0eff0 <col:4, col:30> Text=" - size: 32bit: 4, 64bit: 8"
|-CXXRecordDecl 0xc9ea350 <line:95:1, col:7> col:7 implicit referenced class hkBaseObject
|-AccessSpecDecl 0xc9ea400 <line:96:4, col:10> col:4 public
|-CXXDestructorDecl 0xc9ea518 <line:97:5, col:30> col:13 used ~hkBaseObject 'void () noexcept' virtual implicit-inline
| `-CompoundStmt 0xc9ea8c0 <col:29, col:30>
|-CXXMethodDecl 0xc9ea6e8 <line:95:7> col:7 implicit constexpr operator= 'hkBaseObject &(const hkBaseObject &)' inline default noexcept-unevaluated 0xc9ea6e8
| `-ParmVarDecl 0xc9ea818 <col:7> col:7 'const hkBaseObject &'
|-CXXConstructorDecl 0xc9ef808 <col:7> col:7 implicit constexpr hkBaseObject 'void (const hkBaseObject &)' inline default noexcept-unevaluated 0xc9ef808
| `-ParmVarDecl 0xc9ef948 <col:7> col:7 'const hkBaseObject &'
`-CXXConstructorDecl 0xc9fc7f0 <col:7> col:7 implicit used constexpr hkBaseObject 'void () noexcept' inline default
  `-CompoundStmt 0xc9fd700 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkBaseObject = type { ptr }
  NonVirtualBaseLLVMType:%class.hkBaseObject = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9ecba8 prev 0xc9e7748 <<source>:112:1, line:134:1> line:112:7 referenced class hkReferencedObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param needs_overload_resolution implicit_has_const_param
| |-MoveConstructor exists simple non_trivial needs_overload_resolution
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial needs_overload_resolution
| `-Destructor simple non_trivial constexpr needs_overload_resolution
|-private 'hkBaseObject'
|-FullComment 0xca0f250 <line:105:4, line:111:18>
| `-ParagraphComment 0xca0f220 <line:105:4, line:111:18>
|   |-TextComment 0xca0f100 <line:105:4, col:43> Text=" Stores memory size and reference count."
|   |-TextComment 0xca0f120 <line:106:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xca0f140 <line:107:4, col:36> Text=" -      size: 32bit: 8, 64bit: 12"
|   |-TextComment 0xca0f160 <line:108:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xca0f180 <line:109:4, col:44> Text=" -    parent: `hkBaseObject`/`0xe0708a00`"
|   |-TextComment 0xca0f1a0 <line:110:4, col:29> Text=" - signature: `0x3b1c1113`"
|   `-TextComment 0xca0f1c0 <line:111:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xc9ecd40 <line:112:1, col:7> col:7 implicit class hkReferencedObject
|-FieldDecl 0xc9ece38 <line:119:5, col:14> col:14 memSizeAndFlags 'hkUint16':'unsigned short'
| `-FullComment 0xca0f340 <line:117:8, line:118:48>
|   `-ParagraphComment 0xca0f310 <line:117:8, line:118:48>
|     |-TextComment 0xca0f2c0 <line:117:8, col:37> Text=" - offset: 32bit: 4, 64bit:  8"
|     `-TextComment 0xca0f2e0 <line:118:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xc9eced0 <line:122:5, col:14> col:14 referenceCount 'hkUint16':'unsigned short'
| `-FullComment 0xca0f430 <line:120:8, line:121:48>
|   `-ParagraphComment 0xca0f400 <line:120:8, line:121:48>
|     |-TextComment 0xca0f3b0 <line:120:8, col:37> Text=" - offset: 32bit: 6, 64bit: 10"
|     `-TextComment 0xca0f3d0 <line:121:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xc9ef538 <line:132:5, col:17> col:10 _pad0 'char[4]'
|-CXXConstructorDecl 0xc9ef640 <line:112:7> col:7 implicit constexpr hkReferencedObject 'void (const hkReferencedObject &)' inline default noexcept-unevaluated 0xc9ef640
| `-ParmVarDecl 0xc9ef778 <col:7> col:7 'const hkReferencedObject &'
|-CXXConstructorDecl 0xc9efa28 <col:7> col:7 implicit constexpr hkReferencedObject 'void (hkReferencedObject &&)' inline default noexcept-unevaluated 0xc9efa28
| `-ParmVarDecl 0xc9efb68 <col:7> col:7 'hkReferencedObject &&'
|-CXXMethodDecl 0xc9efc70 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(const hkReferencedObject &)' inline default noexcept-unevaluated 0xc9efc70
| `-ParmVarDecl 0xc9efda8 <col:7> col:7 'const hkReferencedObject &'
|-CXXMethodDecl 0xc9efe20 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(hkReferencedObject &&)' inline default noexcept-unevaluated 0xc9efe20
| `-ParmVarDecl 0xc9eff58 <col:7> col:7 'hkReferencedObject &&'
|-CXXDestructorDecl 0xc9effe0 <col:7> col:7 implicit used constexpr ~hkReferencedObject 'void () noexcept' inline default
| |-Overrides: [ 0xc9ea518 hkBaseObject::~hkBaseObject 'void () noexcept' ]
| `-CompoundStmt 0xc9f01c8 <col:7>
`-CXXConstructorDecl 0xc9fc700 <col:7> col:7 implicit used constexpr hkReferencedObject 'void () noexcept' inline default
  |-CXXCtorInitializer 'hkBaseObject'
  | `-CXXConstructExpr 0xc9fd710 <col:7> 'hkBaseObject' 'void () noexcept'
  `-CompoundStmt 0xc9fd770 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [4 x i8] }
  NonVirtualBaseLLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [4 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9e2d58 <<source>:34:1, line:37:1> line:34:8 referenced struct hkVector4 definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable pod trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xc9e2e90 <col:1, col:8> col:8 implicit struct hkVector4
|-FieldDecl 0xc9e2fa8 <line:35:5, col:12> col:12 x 'hkReal':'float'
| `-AlignedAttr 0xc9e3018 <col:29, col:39> aligned
|   `-ConstantExpr 0xc9e2ff8 <col:37> 'int'
|     |-value: Int 16
|     `-IntegerLiteral 0xc9e2f70 <col:37> 'int' 16
|-FieldDecl 0xc9e30a0 <line:36:5, col:11> col:11 y 'float'
|-FieldDecl 0xc9e3110 <col:5, col:14> col:14 z 'float'
|-FieldDecl 0xc9e3180 <col:5, col:17> col:17 w 'float'
|-CXXMethodDecl 0xc9f1f38 <line:34:8> col:8 implicit constexpr operator= 'hkVector4 &(const hkVector4 &)' inline default trivial noexcept-unevaluated 0xc9f1f38
| `-ParmVarDecl 0xc9f2068 <col:8> col:8 'const hkVector4 &'
|-CXXMethodDecl 0xc9f30d8 <col:8> col:8 implicit constexpr operator= 'hkVector4 &(hkVector4 &&)' inline default trivial noexcept-unevaluated 0xc9f30d8
| `-ParmVarDecl 0xc9f3208 <col:8> col:8 'hkVector4 &&'
|-CXXDestructorDecl 0xc9f3cf8 <col:8> col:8 implicit constexpr ~hkVector4 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9fc918 <col:8> col:8 implicit used constexpr hkVector4 'void () noexcept' inline default trivial
| `-CompoundStmt 0xc9fd7d8 <col:8>
|-CXXConstructorDecl 0xc9fccd0 <col:8> col:8 implicit constexpr hkVector4 'void (const hkVector4 &)' inline default trivial noexcept-unevaluated 0xc9fccd0
| `-ParmVarDecl 0xc9fce08 <col:8> col:8 'const hkVector4 &'
`-CXXConstructorDecl 0xc9fce90 <col:8> col:8 implicit constexpr hkVector4 'void (hkVector4 &&)' inline default trivial noexcept-unevaluated 0xc9fce90
  `-ParmVarDecl 0xc9fcfc8 <col:8> col:8 'hkVector4 &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkVector4 = type { float, float, float, float }
  NonVirtualBaseLLVMType:%struct.hkVector4 = type { float, float, float, float }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xc9f1628 <<source>:12:1, line:16:1> line:13:7 class hkEnum definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'EventMode'
| `-EnumType 0xc9e32d0 'EventMode'
|   `-Enum 0xc9e3210 'EventMode'
|-TemplateArgument type 'char'
| `-BuiltinType 0xc96a5f0 'char'
|-CXXRecordDecl 0xc9f18a0 <col:1, col:7> col:7 implicit class hkEnum
|-AccessSpecDecl 0xc9f1950 <line:14:4, col:11> col:4 private
|-FieldDecl 0xc9f19c8 <line:15:5, col:7> col:7 storage 'char'
|-CXXMethodDecl 0xc9f3388 <line:13:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xc9f3388
| `-ParmVarDecl 0xc9f34b8 <col:7> col:7 'const hkEnum<EventMode, char> &'
|-CXXMethodDecl 0xc9f3598 <col:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xc9f3598
| `-ParmVarDecl 0xc9f36c8 <col:7> col:7 'hkEnum<EventMode, char> &&'
|-CXXDestructorDecl 0xc9f3df0 <col:7> col:7 implicit constexpr ~hkEnum 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9fca28 <col:7> col:7 implicit used constexpr hkEnum 'void () noexcept' inline default trivial
| `-CompoundStmt 0xc9fd898 <col:7>
|-CXXConstructorDecl 0xc9fd050 <col:7> col:7 implicit constexpr hkEnum 'void (const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xc9fd050
| `-ParmVarDecl 0xc9fd1a8 <col:7> col:7 'const hkEnum<EventMode, char> &'
`-CXXConstructorDecl 0xc9fd230 <col:7> col:7 implicit constexpr hkEnum 'void (hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xc9fd230
  `-ParmVarDecl 0xc9fd368 <col:7> col:7 'hkEnum<EventMode, char> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkEnum = type { i8 }
  NonVirtualBaseLLVMType:%class.hkEnum = type { i8 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9f0398 <<source>:149:1, line:197:1> line:149:8 referenced struct hkbProjectData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xca0f7a0 <line:143:4, line:148:18>
| `-ParagraphComment 0xca0f770 <line:143:4, line:148:18>
|   |-TextComment 0xca0f680 <line:143:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xca0f6a0 <line:144:4, col:26> Text=" -      size: 32bit: 48"
|   |-TextComment 0xca0f6c0 <line:145:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xca0f6e0 <line:146:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xca0f700 <line:147:4, col:29> Text=" - signature: `0x13a39ba7`"
|   `-TextComment 0xca0f720 <line:148:4, col:18> Text=" -   version: 2"
|-CXXRecordDecl 0xc9f1210 <line:149:1, col:8> col:8 implicit struct hkbProjectData
|-FieldDecl 0xc9f1308 <line:177:5, col:15> col:15 worldUpWS 'hkVector4'
| `-FullComment 0xca0f930 <line:171:8, line:176:30>
|   |-ParagraphComment 0xca0f890 <line:171:8, line:173:22>
|   | |-TextComment 0xca0f810 <line:171:8, col:71> Text=" The offset here is 16 bytes because Vector4 performs a 16-bytes"
|   | |-TextComment 0xca0f830 <line:172:8, col:68> Text=" alignment to account for SIMD operations using f32 * 4 = 128"
|   | `-TextComment 0xca0f850 <line:173:8, col:22> Text=" XMM registers."
|   `-ParagraphComment 0xca0f900 <line:175:8, line:176:30>
|     |-TextComment 0xca0f8b0 <line:175:8, col:38> Text=" - offset: 32bit: 16, 64bit: 16"
|     `-TextComment 0xca0f8d0 <line:176:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-CXXRecordDecl 0xc9f1390 parent 0xc96a548 <line:181:5, col:12> col:12 struct hkbProjectStringData
|-FieldDecl 0xc9f1540 <col:5, col:34> col:34 stringData 'struct hkbProjectStringData *'
| `-FullComment 0xca0fa50 <line:178:8, line:180:38>
|   `-ParagraphComment 0xca0fa20 <line:178:8, line:180:38>
|     |-TextComment 0xca0f9a0 <line:178:8, col:38> Text=" - offset: 32bit: 32, 64bit: 32"
|     |-TextComment 0xca0f9c0 <line:179:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xca0f9e0 <line:180:8, col:38> Text=" -   size: 32bit:  4, 64bit:  8"
|-FieldDecl 0xc9f1a38 <line:186:5, col:29> col:29 defaultEventMode 'hkEnum<EventMode, char>'
| `-FullComment 0xca0fb90 <line:182:8, line:185:38>
|   `-ParagraphComment 0xca0fb60 <line:182:8, line:185:38>
|     |-TextComment 0xca0fac0 <line:182:8, col:20> Text=" - offset: 36"
|     |-TextComment 0xca0fae0 <line:183:8, col:38> Text=" - offset: 32bit: 36, 64bit: 40"
|     |-TextComment 0xca0fb00 <line:184:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xca0fb20 <line:185:8, col:38> Text=" -   size: 32bit:  1, 64bit:  1"
|-FieldDecl 0xc9f1b58 <line:195:5, col:17> col:10 _pad0 'char[7]'
|-CXXMethodDecl 0xc9f1cc0 <line:149:8> col:8 implicit constexpr operator= 'hkbProjectData &(const hkbProjectData &)' inline default noexcept-unevaluated 0xc9f1cc0
| `-ParmVarDecl 0xc9f1df8 <col:8> col:8 'const hkbProjectData &'
|-CXXMethodDecl 0xc9f37b8 <col:8> col:8 implicit constexpr operator= 'hkbProjectData &(hkbProjectData &&)' inline default noexcept-unevaluated 0xc9f37b8
| `-ParmVarDecl 0xc9f38e8 <col:8> col:8 'hkbProjectData &&'
|-CXXDestructorDecl 0xc9f3970 <col:8> col:8 implicit used constexpr ~hkbProjectData 'void () noexcept' inline default
| |-Overrides: [ 0xc9effe0 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xc9f3f30 <col:8>
|-CXXConstructorDecl 0xc9fc608 <col:8> col:8 implicit used constexpr hkbProjectData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xc9fd780 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f1308 'worldUpWS' 'hkVector4'
| | `-CXXConstructExpr 0xc9fd850 <col:8> 'hkVector4' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f1a38 'defaultEventMode' 'hkEnum<EventMode, char>'
| | `-CXXConstructExpr 0xc9fd910 <col:8> 'hkEnum<EventMode, char>' 'void () noexcept'
| `-CompoundStmt 0xc9fd970 <col:8>
|-CXXConstructorDecl 0xc9fcb20 <col:8> col:8 implicit constexpr hkbProjectData 'void (const hkbProjectData &)' inline default noexcept-unevaluated 0xc9fcb20
| `-ParmVarDecl 0xc9fcc58 <col:8> col:8 'const hkbProjectData &'
`-CXXConstructorDecl 0xc9fd400 <col:8> col:8 implicit constexpr hkbProjectData 'void (hkbProjectData &&)' inline default noexcept-unevaluated 0xc9fd400
  `-ParmVarDecl 0xc9fd538 <col:8> col:8 'hkbProjectData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, %struct.hkVector4, ptr, %class.hkEnum, [7 x i8] }
  NonVirtualBaseLLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, %struct.hkVector4, ptr, %class.hkEnum, [7 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xc9f4370 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkStringPtr'
| `-RecordType 0xc9c20b0 'hkStringPtr'
|   `-CXXRecord 0xc9c2008 'hkStringPtr'
|-CXXRecordDecl 0xc9f45b0 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xc9f4660 <line:23:4, col:11> col:4 private
|-FieldDecl 0xc9f4738 <line:24:5, col:8> col:8 m_data 'hkStringPtr *'
|-FieldDecl 0xc9f4790 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xca0fcf0 <line:25:8, col:78>
|   `-ParagraphComment 0xca0fcc0 <col:8, col:78>
|     `-TextComment 0xca0fc90 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xc9f47e8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xca0fde0 <line:28:8, line:29:22>
|   `-ParagraphComment 0xca0fdb0 <line:28:8, line:29:22>
|     |-TextComment 0xca0fd60 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xca0fd80 <line:29:7, col:22> Text=" for the array. "
|-CXXMethodDecl 0xc9f6408 <line:22:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xc9f6408
| `-ParmVarDecl 0xc9f6538 <col:7> col:7 'const hkArray<hkStringPtr> &'
|-CXXMethodDecl 0xc9f6618 <col:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xc9f6618
| `-ParmVarDecl 0xc9f6748 <col:7> col:7 'hkArray<hkStringPtr> &&'
|-CXXDestructorDecl 0xc9f8db0 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc9fdd58 <col:7> col:7 implicit used constexpr hkArray 'void () noexcept' inline default trivial
| `-CompoundStmt 0xca00068 <col:7>
|-CXXConstructorDecl 0xc9fe000 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xc9fe000
| `-ParmVarDecl 0xc9fe138 <col:7> col:7 'const hkArray<hkStringPtr> &'
`-CXXConstructorDecl 0xc9ffc20 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xc9ffc20
  `-ParmVarDecl 0xc9ffd58 <col:7> col:7 'hkArray<hkStringPtr> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc9f40e0 prev 0xc9f1390 <<source>:215:1, line:268:1> line:215:8 referenced struct hkbProjectStringData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xca152b0 <line:204:4, line:214:18>
| |-ParagraphComment 0xca150f0 <line:204:4, col:26>
| | `-TextComment 0xca150c0 <col:4, col:26> Text=" `hkbProjectStringData`"
| |-ParagraphComment 0xca15160 <line:206:4, line:207:69>
| | |-TextComment 0xca15110 <line:206:4, col:63> Text=" - In C++, it represents the name of one field in the class."
| | `-TextComment 0xca15130 <line:207:4, col:69> Text=" - In XML, the value of the `name` attribute of the `hkparam` tag."
| `-ParagraphComment 0xca15270 <line:209:4, line:214:18>
|   |-TextComment 0xca15180 <line:209:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xca151a0 <line:210:4, col:19> Text=" -      size: 76"
|   |-TextComment 0xca151c0 <line:211:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xca151e0 <line:212:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xca15200 <line:213:4, col:28> Text=" - signature: `0x76ad60a`"
|   `-TextComment 0xca15220 <line:214:4, col:18> Text=" -   version: 1"
|-CXXRecordDecl 0xc9f42b0 <line:215:1, col:8> col:8 implicit struct hkbProjectStringData
|-FieldDecl 0xc9f4858 <line:241:5, col:26> col:26 animation_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xca153a0 <line:239:8, line:240:30>
|   `-ParagraphComment 0xca15370 <line:239:8, line:240:30>
|     |-TextComment 0xca15320 <line:239:8, col:38> Text=" - offset: 32bit:  8, 64bit: 16"
|     `-TextComment 0xca15340 <line:240:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f49b8 <line:245:5, col:26> col:26 behavior_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xca15490 <line:243:8, line:244:30>
|   `-ParagraphComment 0xca15460 <line:243:8, line:244:30>
|     |-TextComment 0xca15410 <line:243:8, col:38> Text=" - offset: 32bit: 20, 64bit: 32"
|     `-TextComment 0xca15430 <line:244:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4b18 <line:248:5, col:26> col:26 character_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xca15580 <line:246:8, line:247:30>
|   `-ParagraphComment 0xca15550 <line:246:8, line:247:30>
|     |-TextComment 0xca15500 <line:246:8, col:38> Text=" - offset: 32bit: 32, 64bit: 48"
|     `-TextComment 0xca15520 <line:247:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4c78 <line:251:5, col:26> col:26 event_names 'hkArray<hkStringPtr>'
| `-FullComment 0xca15670 <line:249:8, line:250:30>
|   `-ParagraphComment 0xca15640 <line:249:8, line:250:30>
|     |-TextComment 0xca155f0 <line:249:8, col:38> Text=" - offset: 32bit: 44, 64bit: 64"
|     `-TextComment 0xca15610 <line:250:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4d10 <line:255:5, col:17> col:17 animation_path 'hkStringPtr'
| `-FullComment 0xca15760 <line:253:8, line:254:30>
|   `-ParagraphComment 0xca15730 <line:253:8, line:254:30>
|     |-TextComment 0xca156e0 <line:253:8, col:38> Text=" - offset: 32bit: 56, 64bit: 80"
|     `-TextComment 0xca15700 <line:254:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4da0 <line:258:5, col:17> col:17 behavior_path 'hkStringPtr'
| `-FullComment 0xca15850 <line:256:8, line:257:30>
|   `-ParagraphComment 0xca15820 <line:256:8, line:257:30>
|     |-TextComment 0xca157d0 <line:256:8, col:38> Text=" - offset: 32bit: 60, 64bit: 88"
|     `-TextComment 0xca157f0 <line:257:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4e30 <line:261:5, col:17> col:17 character_path 'hkStringPtr'
| `-FullComment 0xca15940 <line:259:8, line:260:30>
|   `-ParagraphComment 0xca15910 <line:259:8, line:260:30>
|     |-TextComment 0xca158c0 <line:259:8, col:38> Text=" - offset: 32bit: 64, 64bit: 96"
|     `-TextComment 0xca158e0 <line:260:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4ec0 <line:264:5, col:17> col:17 full_path_to_source 'hkStringPtr'
| `-FullComment 0xca15a30 <line:262:8, line:263:30>
|   `-ParagraphComment 0xca15a00 <line:262:8, line:263:30>
|     |-TextComment 0xca159b0 <line:262:8, col:39> Text=" - offset: 32bit: 68, 64bit: 104"
|     `-TextComment 0xca159d0 <line:263:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc9f4f50 <line:267:5, col:17> col:17 root_path 'hkStringPtr'
| `-FullComment 0xca15b20 <line:265:8, line:266:48>
|   `-ParagraphComment 0xca15af0 <line:265:8, line:266:48>
|     |-TextComment 0xca15aa0 <line:265:8, col:39> Text=" - offset: 32bit: 72, 64bit: 112"
|     `-TextComment 0xca15ac0 <line:266:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-CXXMethodDecl 0xc9f6160 <line:215:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(const hkbProjectStringData &)' inline default noexcept-unevaluated 0xc9f6160
| `-ParmVarDecl 0xc9f6298 <col:8> col:8 'const hkbProjectStringData &'
|-CXXMethodDecl 0xc9f6cc8 <col:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(hkbProjectStringData &&)' inline default noexcept-unevaluated 0xc9f6cc8
| `-ParmVarDecl 0xc9f6df8 <col:8> col:8 'hkbProjectStringData &&'
|-CXXDestructorDecl 0xc9f6e80 <col:8> col:8 implicit used constexpr ~hkbProjectStringData 'void () noexcept' inline default
| |-Overrides: [ 0xc9effe0 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xc9f8ff0 <col:8>
|-CXXConstructorDecl 0xc9fdc50 <col:8> col:8 implicit used constexpr hkbProjectStringData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xca00010 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4858 'animation_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xca000e0 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f49b8 'behavior_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xca00160 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4b18 'character_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xca001e0 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4c78 'event_names' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xca00260 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4d10 'animation_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xca002f0 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4da0 'behavior_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xca00338 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4e30 'character_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xca00380 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4ec0 'full_path_to_source' 'hkStringPtr'
| | `-CXXConstructExpr 0xca003c8 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc9f4f50 'root_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xca00410 <col:8> 'hkStringPtr' 'void () noexcept'
| `-CompoundStmt 0xca004a8 <col:8>
|-CXXConstructorDecl 0xc9fde50 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (const hkbProjectStringData &)' inline default noexcept-unevaluated 0xc9fde50
| `-ParmVarDecl 0xc9fdf88 <col:8> col:8 'const hkbProjectStringData &'
`-CXXConstructorDecl 0xc9ffdf0 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (hkbProjectStringData &&)' inline default noexcept-unevaluated 0xc9ffdf0
  `-ParmVarDecl 0xc9fff28 <col:8> col:8 'hkbProjectStringData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectStringData = type { %class.hkReferencedObject, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr }
  NonVirtualBaseLLVMType:%struct.hkbProjectStringData = type { %class.hkReferencedObject, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr }
  IsZeroInitializable:1
  BitFields:[
]>
ASM generation compiler returned: 0

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

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xc0ece78 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkRootLevelContainerNamedVariant'
| `-RecordType 0xc0e9700 'hkRootLevelContainerNamedVariant'
|   `-CXXRecord 0xc0ed6d8 'hkRootLevelContainerNamedVariant'
|-CXXRecordDecl 0xc0ed0c0 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xc0ed170 <line:23:4, col:11> col:4 private
|-FieldDecl 0xc0ed248 <line:24:5, col:8> col:8 m_data 'hkRootLevelContainerNamedVariant *'
|-FieldDecl 0xc0ed2a0 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xc1065b0 <line:25:8, col:78>
|   `-ParagraphComment 0xc106580 <col:8, col:78>
|     `-TextComment 0xc106550 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xc0ed2f8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xc1066a0 <line:28:8, line:29:22>
|   `-ParagraphComment 0xc106670 <line:28:8, line:29:22>
|     |-TextComment 0xc106620 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xc106640 <line:29:7, col:22> Text=" for the array. "
|-CXXConstructorDecl 0xc0ff2d8 <line:22:7> col:7 implicit constexpr hkArray 'void () noexcept' inline default trivial
|-CXXDestructorDecl 0xc0ff3d0 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc0ff788 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkRootLevelContainerNamedVariant> &)' inline default trivial noexcept-unevaluated 0xc0ff788
| `-ParmVarDecl 0xc0ff8c8 <col:7> col:7 'const hkArray<hkRootLevelContainerNamedVariant> &'
`-CXXConstructorDecl 0xc0ff9b8 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkRootLevelContainerNamedVariant> &&)' inline default trivial noexcept-unevaluated 0xc0ff9b8
  `-ParmVarDecl 0xc0fff58 <col:7> col:7 'hkArray<hkRootLevelContainerNamedVariant> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0e9460 <<source>:53:1, line:57:1> line:53:8 referenced struct hkRootLevelContainer definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-FullComment 0xc106840 <line:49:4, line:52:18>
| `-ParagraphComment 0xc106810 <line:49:4, line:52:18>
|   |-TextComment 0xc106770 <line:49:4, col:35> Text=" -      size: 16(if 32bit -> 12)"
|   |-TextComment 0xc106790 <line:50:4, col:22> Text=" -    vtable: false"
|   |-TextComment 0xc1067b0 <line:51:4, col:29> Text=" - signature: `0x2772c11e`"
|   `-TextComment 0xc1067d0 <line:52:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xc0e95a0 <line:53:1, col:8> col:8 implicit struct hkRootLevelContainer
|-CXXRecordDecl 0xc0e9650 parent 0xc06ff78 <line:56:13, col:20> col:20 struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xc0ed368 <col:5, col:54> col:54 namedVariants 'hkArray<struct hkRootLevelContainerNamedVariant>':'hkArray<hkRootLevelContainerNamedVariant>'
|-CXXConstructorDecl 0xc0ff1c8 <line:53:8> col:8 implicit referenced constexpr hkRootLevelContainer 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc0ff538 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (const hkRootLevelContainer &)' inline default trivial noexcept-unevaluated 0xc0ff538
| `-ParmVarDecl 0xc0ff678 <col:8> col:8 'const hkRootLevelContainer &'
|-CXXConstructorDecl 0xc100058 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (hkRootLevelContainer &&)' inline default trivial noexcept-unevaluated 0xc100058
| `-ParmVarDecl 0xc100198 <col:8> col:8 'hkRootLevelContainer &&'
`-CXXDestructorDecl 0xc100318 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainer 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0c7e38 <<source>:4:1, line:7:1> line:4:7 referenced class hkStringPtr definition
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xc0c7f70 <col:1, col:7> col:7 implicit class hkStringPtr
|-AccessSpecDecl 0xc0c8020 <line:5:4, col:11> col:4 private
|-FieldDecl 0xc0c8068 <line:6:5, col:17> col:17 m_stringAndFlag 'const char *'
|-CXXMethodDecl 0xc0fc668 <line:4:7> col:7 implicit constexpr operator= 'hkStringPtr &(const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xc0fc668
| `-ParmVarDecl 0xc0fc798 <col:7> col:7 'const hkStringPtr &'
|-CXXMethodDecl 0xc0fc878 <col:7> col:7 implicit constexpr operator= 'hkStringPtr &(hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xc0fc878
| `-ParmVarDecl 0xc0fc9a8 <col:7> col:7 'hkStringPtr &&'
|-CXXDestructorDecl 0xc0fec80 <col:7> col:7 implicit constexpr ~hkStringPtr 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc100718 <col:7> col:7 implicit used constexpr hkStringPtr 'void () noexcept' inline default trivial
| `-CompoundStmt 0xc106078 <col:7>
|-CXXConstructorDecl 0xc100a30 <col:7> col:7 implicit constexpr hkStringPtr 'void (const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xc100a30
| `-ParmVarDecl 0xc100b68 <col:7> col:7 'const hkStringPtr &'
`-CXXConstructorDecl 0xc100bf0 <col:7> col:7 implicit constexpr hkStringPtr 'void (hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xc100bf0
  `-ParmVarDecl 0xc100d28 <col:7> col:7 'hkStringPtr &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkStringPtr = type { ptr }
  NonVirtualBaseLLVMType:%class.hkStringPtr = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0ed6d8 prev 0xc0e9650 <<source>:70:1, line:80:1> line:70:8 referenced struct hkRootLevelContainerNamedVariant definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xc0ed7d8 <col:1, col:8> col:8 implicit struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xc0ed8c8 <line:73:5, col:17> col:17 name 'hkStringPtr'
|-FieldDecl 0xc0ed930 <line:76:5, col:17> col:17 className 'hkStringPtr'
|-CXXRecordDecl 0xc0ed988 parent 0xc06ff78 <line:79:5, col:12> col:12 struct hkReferencedObject
|-FieldDecl 0xc0edb30 <col:5, col:32> col:32 variant 'struct hkReferencedObject *'
|-CXXConstructorDecl 0xc100608 <line:70:8> col:8 implicit referenced constexpr hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc100878 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (const hkRootLevelContainerNamedVariant &)' inline default trivial noexcept-unevaluated 0xc100878
| `-ParmVarDecl 0xc1009b8 <col:8> col:8 'const hkRootLevelContainerNamedVariant &'
|-CXXConstructorDecl 0xc100e28 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (hkRootLevelContainerNamedVariant &&)' inline default trivial noexcept-unevaluated 0xc100e28
| `-ParmVarDecl 0xc101fa8 <col:8> col:8 'hkRootLevelContainerNamedVariant &&'
`-CXXDestructorDecl 0xc1020f8 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0f0368 <<source>:95:1, line:98:1> line:95:7 referenced class hkBaseObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor can_const_default_init
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment
| `-Destructor non_trivial user_declared
|-FullComment 0xc114d40 <line:89:4, line:94:30>
| |-ParagraphComment 0xc114cc0 <line:89:4, line:92:40>
| | |-TextComment 0xc114c20 <line:89:4, col:35> Text=" The class size is pointer size."
| | |-TextComment 0xc114c40 <line:90:4, col:80> Text=" The SDK description says that the `hkBaseObject`, a virtual function without"
| | |-TextComment 0xc114c60 <line:91:4, col:75> Text=" a field, is the source of inheritance for all Havok Classes so that the"
| | `-TextComment 0xc114c80 <line:92:4, col:40> Text=" vtable does not come after the data."
| `-ParagraphComment 0xc114d10 <line:94:4, col:30>
|   `-TextComment 0xc114ce0 <col:4, col:30> Text=" - size: 32bit: 4, 64bit: 8"
|-CXXRecordDecl 0xc0f04a0 <line:95:1, col:7> col:7 implicit referenced class hkBaseObject
|-AccessSpecDecl 0xc0f0550 <line:96:4, col:10> col:4 public
|-CXXDestructorDecl 0xc0f0668 <line:97:5, col:30> col:13 used ~hkBaseObject 'void () noexcept' virtual implicit-inline
| `-CompoundStmt 0xc0f0a10 <col:29, col:30>
|-CXXMethodDecl 0xc0f0838 <line:95:7> col:7 implicit constexpr operator= 'hkBaseObject &(const hkBaseObject &)' inline default noexcept-unevaluated 0xc0f0838
| `-ParmVarDecl 0xc0f0968 <col:7> col:7 'const hkBaseObject &'
|-CXXConstructorDecl 0xc0f5618 <col:7> col:7 implicit constexpr hkBaseObject 'void (const hkBaseObject &)' inline default noexcept-unevaluated 0xc0f5618
| `-ParmVarDecl 0xc0f5758 <col:7> col:7 'const hkBaseObject &'
`-CXXConstructorDecl 0xc1025c0 <col:7> col:7 implicit used constexpr hkBaseObject 'void () noexcept' inline default
  `-CompoundStmt 0xc1034d0 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkBaseObject = type { ptr }
  NonVirtualBaseLLVMType:%class.hkBaseObject = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0f2c98 prev 0xc0ed988 <<source>:112:1, line:134:1> line:112:7 referenced class hkReferencedObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param needs_overload_resolution implicit_has_const_param
| |-MoveConstructor exists simple non_trivial needs_overload_resolution
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial needs_overload_resolution
| `-Destructor simple non_trivial constexpr needs_overload_resolution
|-private 'hkBaseObject'
|-FullComment 0xc114f40 <line:105:4, line:111:18>
| `-ParagraphComment 0xc114f10 <line:105:4, line:111:18>
|   |-TextComment 0xc114df0 <line:105:4, col:43> Text=" Stores memory size and reference count."
|   |-TextComment 0xc114e10 <line:106:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xc114e30 <line:107:4, col:36> Text=" -      size: 32bit: 8, 64bit: 12"
|   |-TextComment 0xc114e50 <line:108:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xc114e70 <line:109:4, col:44> Text=" -    parent: `hkBaseObject`/`0xe0708a00`"
|   |-TextComment 0xc114e90 <line:110:4, col:29> Text=" - signature: `0x3b1c1113`"
|   `-TextComment 0xc114eb0 <line:111:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xc0f2e30 <line:112:1, col:7> col:7 implicit class hkReferencedObject
|-FieldDecl 0xc0f2f28 <line:119:5, col:14> col:14 memSizeAndFlags 'hkUint16':'unsigned short'
| `-FullComment 0xc115030 <line:117:8, line:118:48>
|   `-ParagraphComment 0xc115000 <line:117:8, line:118:48>
|     |-TextComment 0xc114fb0 <line:117:8, col:37> Text=" - offset: 32bit: 4, 64bit:  8"
|     `-TextComment 0xc114fd0 <line:118:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xc0f2fc0 <line:122:5, col:14> col:14 referenceCount 'hkUint16':'unsigned short'
| `-FullComment 0xc115120 <line:120:8, line:121:48>
|   `-ParagraphComment 0xc1150f0 <line:120:8, line:121:48>
|     |-TextComment 0xc1150a0 <line:120:8, col:37> Text=" - offset: 32bit: 6, 64bit: 10"
|     `-TextComment 0xc1150c0 <line:121:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xc0f5348 <line:132:5, col:17> col:10 _pad0 'char[4]'
|-CXXConstructorDecl 0xc0f5450 <line:112:7> col:7 implicit constexpr hkReferencedObject 'void (const hkReferencedObject &)' inline default noexcept-unevaluated 0xc0f5450
| `-ParmVarDecl 0xc0f5588 <col:7> col:7 'const hkReferencedObject &'
|-CXXConstructorDecl 0xc0f5838 <col:7> col:7 implicit constexpr hkReferencedObject 'void (hkReferencedObject &&)' inline default noexcept-unevaluated 0xc0f5838
| `-ParmVarDecl 0xc0f5978 <col:7> col:7 'hkReferencedObject &&'
|-CXXMethodDecl 0xc0f5a80 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(const hkReferencedObject &)' inline default noexcept-unevaluated 0xc0f5a80
| `-ParmVarDecl 0xc0f5bb8 <col:7> col:7 'const hkReferencedObject &'
|-CXXMethodDecl 0xc0f5c30 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(hkReferencedObject &&)' inline default noexcept-unevaluated 0xc0f5c30
| `-ParmVarDecl 0xc0f5d68 <col:7> col:7 'hkReferencedObject &&'
|-CXXDestructorDecl 0xc0f5df0 <col:7> col:7 implicit used constexpr ~hkReferencedObject 'void () noexcept' inline default
| |-Overrides: [ 0xc0f0668 hkBaseObject::~hkBaseObject 'void () noexcept' ]
| `-CompoundStmt 0xc0f5fd8 <col:7>
`-CXXConstructorDecl 0xc1024d0 <col:7> col:7 implicit used constexpr hkReferencedObject 'void () noexcept' inline default
  |-CXXCtorInitializer 'hkBaseObject'
  | `-CXXConstructExpr 0xc1034e0 <col:7> 'hkBaseObject' 'void () noexcept'
  `-CompoundStmt 0xc103540 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [4 x i8] }
  NonVirtualBaseLLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [4 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0e8b88 <<source>:34:1, line:37:1> line:34:8 referenced struct hkVector4 definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable pod trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xc0e8cc0 <col:1, col:8> col:8 implicit struct hkVector4
|-FieldDecl 0xc0e8dd8 <line:35:5, col:12> col:12 x 'hkReal':'float'
| `-AlignedAttr 0xc0e8e48 <col:29, col:39> aligned
|   `-ConstantExpr 0xc0e8e28 <col:37> 'int'
|     |-value: Int 16
|     `-IntegerLiteral 0xc0e8da0 <col:37> 'int' 16
|-FieldDecl 0xc0e8ed0 <line:36:5, col:11> col:11 y 'float'
|-FieldDecl 0xc0e8f40 <col:5, col:14> col:14 z 'float'
|-FieldDecl 0xc0e8fb0 <col:5, col:17> col:17 w 'float'
|-CXXMethodDecl 0xc0f7d08 <line:34:8> col:8 implicit constexpr operator= 'hkVector4 &(const hkVector4 &)' inline default trivial noexcept-unevaluated 0xc0f7d08
| `-ParmVarDecl 0xc0f7e38 <col:8> col:8 'const hkVector4 &'
|-CXXMethodDecl 0xc0f8ea8 <col:8> col:8 implicit constexpr operator= 'hkVector4 &(hkVector4 &&)' inline default trivial noexcept-unevaluated 0xc0f8ea8
| `-ParmVarDecl 0xc0f8fd8 <col:8> col:8 'hkVector4 &&'
|-CXXDestructorDecl 0xc0f9ac8 <col:8> col:8 implicit constexpr ~hkVector4 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc1026e8 <col:8> col:8 implicit used constexpr hkVector4 'void () noexcept' inline default trivial
| `-CompoundStmt 0xc1035a8 <col:8>
|-CXXConstructorDecl 0xc102aa0 <col:8> col:8 implicit constexpr hkVector4 'void (const hkVector4 &)' inline default trivial noexcept-unevaluated 0xc102aa0
| `-ParmVarDecl 0xc102bd8 <col:8> col:8 'const hkVector4 &'
`-CXXConstructorDecl 0xc102c60 <col:8> col:8 implicit constexpr hkVector4 'void (hkVector4 &&)' inline default trivial noexcept-unevaluated 0xc102c60
  `-ParmVarDecl 0xc102d98 <col:8> col:8 'hkVector4 &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkVector4 = type { float, float, float, float }
  NonVirtualBaseLLVMType:%struct.hkVector4 = type { float, float, float, float }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xc0f73f8 <<source>:12:1, line:16:1> line:13:7 class hkEnum definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'EventMode'
| `-EnumType 0xc0e9100 'EventMode'
|   `-Enum 0xc0e9040 'EventMode'
|-TemplateArgument type 'char'
| `-BuiltinType 0xc070020 'char'
|-CXXRecordDecl 0xc0f7670 <col:1, col:7> col:7 implicit class hkEnum
|-AccessSpecDecl 0xc0f7720 <line:14:4, col:11> col:4 private
|-FieldDecl 0xc0f7798 <line:15:5, col:7> col:7 storage 'char'
|-CXXMethodDecl 0xc0f9158 <line:13:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xc0f9158
| `-ParmVarDecl 0xc0f9288 <col:7> col:7 'const hkEnum<EventMode, char> &'
|-CXXMethodDecl 0xc0f9368 <col:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xc0f9368
| `-ParmVarDecl 0xc0f9498 <col:7> col:7 'hkEnum<EventMode, char> &&'
|-CXXDestructorDecl 0xc0f9bc0 <col:7> col:7 implicit constexpr ~hkEnum 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc1027f8 <col:7> col:7 implicit used constexpr hkEnum 'void () noexcept' inline default trivial
| `-CompoundStmt 0xc103668 <col:7>
|-CXXConstructorDecl 0xc102e20 <col:7> col:7 implicit constexpr hkEnum 'void (const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xc102e20
| `-ParmVarDecl 0xc102f78 <col:7> col:7 'const hkEnum<EventMode, char> &'
`-CXXConstructorDecl 0xc103000 <col:7> col:7 implicit constexpr hkEnum 'void (hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xc103000
  `-ParmVarDecl 0xc103138 <col:7> col:7 'hkEnum<EventMode, char> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkEnum = type { i8 }
  NonVirtualBaseLLVMType:%class.hkEnum = type { i8 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0f61a8 <<source>:149:1, line:197:1> line:149:8 referenced struct hkbProjectData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xc115490 <line:143:4, line:148:18>
| `-ParagraphComment 0xc115460 <line:143:4, line:148:18>
|   |-TextComment 0xc115370 <line:143:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xc115390 <line:144:4, col:26> Text=" -      size: 32bit: 48"
|   |-TextComment 0xc1153b0 <line:145:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xc1153d0 <line:146:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xc1153f0 <line:147:4, col:29> Text=" - signature: `0x13a39ba7`"
|   `-TextComment 0xc115410 <line:148:4, col:18> Text=" -   version: 2"
|-CXXRecordDecl 0xc0f6fe0 <line:149:1, col:8> col:8 implicit struct hkbProjectData
|-FieldDecl 0xc0f70d8 <line:177:5, col:15> col:15 worldUpWS 'hkVector4'
| `-FullComment 0xc115620 <line:171:8, line:176:30>
|   |-ParagraphComment 0xc115580 <line:171:8, line:173:22>
|   | |-TextComment 0xc115500 <line:171:8, col:71> Text=" The offset here is 16 bytes because Vector4 performs a 16-bytes"
|   | |-TextComment 0xc115520 <line:172:8, col:68> Text=" alignment to account for SIMD operations using f32 * 4 = 128"
|   | `-TextComment 0xc115540 <line:173:8, col:22> Text=" XMM registers."
|   `-ParagraphComment 0xc1155f0 <line:175:8, line:176:30>
|     |-TextComment 0xc1155a0 <line:175:8, col:38> Text=" - offset: 32bit: 16, 64bit: 16"
|     `-TextComment 0xc1155c0 <line:176:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-CXXRecordDecl 0xc0f7160 parent 0xc06ff78 <line:181:5, col:12> col:12 struct hkbProjectStringData
|-FieldDecl 0xc0f7310 <col:5, col:34> col:34 stringData 'struct hkbProjectStringData *'
| `-FullComment 0xc115740 <line:178:8, line:180:38>
|   `-ParagraphComment 0xc115710 <line:178:8, line:180:38>
|     |-TextComment 0xc115690 <line:178:8, col:38> Text=" - offset: 32bit: 32, 64bit: 32"
|     |-TextComment 0xc1156b0 <line:179:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xc1156d0 <line:180:8, col:38> Text=" -   size: 32bit:  4, 64bit:  8"
|-FieldDecl 0xc0f7808 <line:186:5, col:29> col:29 defaultEventMode 'hkEnum<EventMode, char>'
| `-FullComment 0xc115880 <line:182:8, line:185:38>
|   `-ParagraphComment 0xc115850 <line:182:8, line:185:38>
|     |-TextComment 0xc1157b0 <line:182:8, col:20> Text=" - offset: 36"
|     |-TextComment 0xc1157d0 <line:183:8, col:38> Text=" - offset: 32bit: 36, 64bit: 40"
|     |-TextComment 0xc1157f0 <line:184:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xc115810 <line:185:8, col:38> Text=" -   size: 32bit:  1, 64bit:  1"
|-FieldDecl 0xc0f7928 <line:195:5, col:17> col:10 _pad0 'char[7]'
|-CXXMethodDecl 0xc0f7a90 <line:149:8> col:8 implicit constexpr operator= 'hkbProjectData &(const hkbProjectData &)' inline default noexcept-unevaluated 0xc0f7a90
| `-ParmVarDecl 0xc0f7bc8 <col:8> col:8 'const hkbProjectData &'
|-CXXMethodDecl 0xc0f9588 <col:8> col:8 implicit constexpr operator= 'hkbProjectData &(hkbProjectData &&)' inline default noexcept-unevaluated 0xc0f9588
| `-ParmVarDecl 0xc0f96b8 <col:8> col:8 'hkbProjectData &&'
|-CXXDestructorDecl 0xc0f9740 <col:8> col:8 implicit used constexpr ~hkbProjectData 'void () noexcept' inline default
| |-Overrides: [ 0xc0f5df0 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xc0f9d00 <col:8>
|-CXXConstructorDecl 0xc1023d8 <col:8> col:8 implicit used constexpr hkbProjectData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xc103550 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0f70d8 'worldUpWS' 'hkVector4'
| | `-CXXConstructExpr 0xc103620 <col:8> 'hkVector4' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0f7808 'defaultEventMode' 'hkEnum<EventMode, char>'
| | `-CXXConstructExpr 0xc1036e0 <col:8> 'hkEnum<EventMode, char>' 'void () noexcept'
| `-CompoundStmt 0xc103740 <col:8>
|-CXXConstructorDecl 0xc1028f0 <col:8> col:8 implicit constexpr hkbProjectData 'void (const hkbProjectData &)' inline default noexcept-unevaluated 0xc1028f0
| `-ParmVarDecl 0xc102a28 <col:8> col:8 'const hkbProjectData &'
`-CXXConstructorDecl 0xc1031d0 <col:8> col:8 implicit constexpr hkbProjectData 'void (hkbProjectData &&)' inline default noexcept-unevaluated 0xc1031d0
  `-ParmVarDecl 0xc103308 <col:8> col:8 'hkbProjectData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, %struct.hkVector4, ptr, %class.hkEnum, [7 x i8] }
  NonVirtualBaseLLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, %struct.hkVector4, ptr, %class.hkEnum, [7 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xc0fa140 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkStringPtr'
| `-RecordType 0xc0c7ee0 'hkStringPtr'
|   `-CXXRecord 0xc0c7e38 'hkStringPtr'
|-CXXRecordDecl 0xc0fa380 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xc0fa430 <line:23:4, col:11> col:4 private
|-FieldDecl 0xc0fa508 <line:24:5, col:8> col:8 m_data 'hkStringPtr *'
|-FieldDecl 0xc0fa560 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xc1159e0 <line:25:8, col:78>
|   `-ParagraphComment 0xc1159b0 <col:8, col:78>
|     `-TextComment 0xc115980 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xc0fa5b8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xc115ad0 <line:28:8, line:29:22>
|   `-ParagraphComment 0xc115aa0 <line:28:8, line:29:22>
|     |-TextComment 0xc115a50 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xc115a70 <line:29:7, col:22> Text=" for the array. "
|-CXXMethodDecl 0xc0fc1d8 <line:22:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xc0fc1d8
| `-ParmVarDecl 0xc0fc308 <col:7> col:7 'const hkArray<hkStringPtr> &'
|-CXXMethodDecl 0xc0fc3e8 <col:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xc0fc3e8
| `-ParmVarDecl 0xc0fc518 <col:7> col:7 'hkArray<hkStringPtr> &&'
|-CXXDestructorDecl 0xc0feb80 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xc103b28 <col:7> col:7 implicit used constexpr hkArray 'void () noexcept' inline default trivial
| `-CompoundStmt 0xc105e38 <col:7>
|-CXXConstructorDecl 0xc103dd0 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xc103dd0
| `-ParmVarDecl 0xc103f08 <col:7> col:7 'const hkArray<hkStringPtr> &'
`-CXXConstructorDecl 0xc1059f0 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xc1059f0
  `-ParmVarDecl 0xc105b28 <col:7> col:7 'hkArray<hkStringPtr> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xc0f9eb0 prev 0xc0f7160 <<source>:215:1, line:268:1> line:215:8 referenced struct hkbProjectStringData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xc11b0a0 <line:204:4, line:214:18>
| |-ParagraphComment 0xc11aee0 <line:204:4, col:26>
| | `-TextComment 0xc11aeb0 <col:4, col:26> Text=" `hkbProjectStringData`"
| |-ParagraphComment 0xc11af50 <line:206:4, line:207:69>
| | |-TextComment 0xc11af00 <line:206:4, col:63> Text=" - In C++, it represents the name of one field in the class."
| | `-TextComment 0xc11af20 <line:207:4, col:69> Text=" - In XML, the value of the `name` attribute of the `hkparam` tag."
| `-ParagraphComment 0xc11b060 <line:209:4, line:214:18>
|   |-TextComment 0xc11af70 <line:209:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xc11af90 <line:210:4, col:19> Text=" -      size: 76"
|   |-TextComment 0xc11afb0 <line:211:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xc11afd0 <line:212:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xc11aff0 <line:213:4, col:28> Text=" - signature: `0x76ad60a`"
|   `-TextComment 0xc11b010 <line:214:4, col:18> Text=" -   version: 1"
|-CXXRecordDecl 0xc0fa080 <line:215:1, col:8> col:8 implicit struct hkbProjectStringData
|-FieldDecl 0xc0fa628 <line:241:5, col:26> col:26 animation_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xc11b190 <line:239:8, line:240:30>
|   `-ParagraphComment 0xc11b160 <line:239:8, line:240:30>
|     |-TextComment 0xc11b110 <line:239:8, col:38> Text=" - offset: 32bit:  8, 64bit: 16"
|     `-TextComment 0xc11b130 <line:240:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0fa788 <line:245:5, col:26> col:26 behavior_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xc11b280 <line:243:8, line:244:30>
|   `-ParagraphComment 0xc11b250 <line:243:8, line:244:30>
|     |-TextComment 0xc11b200 <line:243:8, col:38> Text=" - offset: 32bit: 20, 64bit: 32"
|     `-TextComment 0xc11b220 <line:244:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0fa8e8 <line:248:5, col:26> col:26 character_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xc11b370 <line:246:8, line:247:30>
|   `-ParagraphComment 0xc11b340 <line:246:8, line:247:30>
|     |-TextComment 0xc11b2f0 <line:246:8, col:38> Text=" - offset: 32bit: 32, 64bit: 48"
|     `-TextComment 0xc11b310 <line:247:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0faa48 <line:251:5, col:26> col:26 event_names 'hkArray<hkStringPtr>'
| `-FullComment 0xc11b460 <line:249:8, line:250:30>
|   `-ParagraphComment 0xc11b430 <line:249:8, line:250:30>
|     |-TextComment 0xc11b3e0 <line:249:8, col:38> Text=" - offset: 32bit: 44, 64bit: 64"
|     `-TextComment 0xc11b400 <line:250:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0faae0 <line:255:5, col:17> col:17 animation_path 'hkStringPtr'
| `-FullComment 0xc11b550 <line:253:8, line:254:30>
|   `-ParagraphComment 0xc11b520 <line:253:8, line:254:30>
|     |-TextComment 0xc11b4d0 <line:253:8, col:38> Text=" - offset: 32bit: 56, 64bit: 80"
|     `-TextComment 0xc11b4f0 <line:254:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0fab70 <line:258:5, col:17> col:17 behavior_path 'hkStringPtr'
| `-FullComment 0xc11b640 <line:256:8, line:257:30>
|   `-ParagraphComment 0xc11b610 <line:256:8, line:257:30>
|     |-TextComment 0xc11b5c0 <line:256:8, col:38> Text=" - offset: 32bit: 60, 64bit: 88"
|     `-TextComment 0xc11b5e0 <line:257:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0fac00 <line:261:5, col:17> col:17 character_path 'hkStringPtr'
| `-FullComment 0xc11b730 <line:259:8, line:260:30>
|   `-ParagraphComment 0xc11b700 <line:259:8, line:260:30>
|     |-TextComment 0xc11b6b0 <line:259:8, col:38> Text=" - offset: 32bit: 64, 64bit: 96"
|     `-TextComment 0xc11b6d0 <line:260:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0fac90 <line:264:5, col:17> col:17 full_path_to_source 'hkStringPtr'
| `-FullComment 0xc11b820 <line:262:8, line:263:30>
|   `-ParagraphComment 0xc11b7f0 <line:262:8, line:263:30>
|     |-TextComment 0xc11b7a0 <line:262:8, col:39> Text=" - offset: 32bit: 68, 64bit: 104"
|     `-TextComment 0xc11b7c0 <line:263:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xc0fad20 <line:267:5, col:17> col:17 root_path 'hkStringPtr'
| `-FullComment 0xc11b910 <line:265:8, line:266:48>
|   `-ParagraphComment 0xc11b8e0 <line:265:8, line:266:48>
|     |-TextComment 0xc11b890 <line:265:8, col:39> Text=" - offset: 32bit: 72, 64bit: 112"
|     `-TextComment 0xc11b8b0 <line:266:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-CXXMethodDecl 0xc0fbf30 <line:215:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(const hkbProjectStringData &)' inline default noexcept-unevaluated 0xc0fbf30
| `-ParmVarDecl 0xc0fc068 <col:8> col:8 'const hkbProjectStringData &'
|-CXXMethodDecl 0xc0fca98 <col:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(hkbProjectStringData &&)' inline default noexcept-unevaluated 0xc0fca98
| `-ParmVarDecl 0xc0fcbc8 <col:8> col:8 'hkbProjectStringData &&'
|-CXXDestructorDecl 0xc0fcc50 <col:8> col:8 implicit used constexpr ~hkbProjectStringData 'void () noexcept' inline default
| |-Overrides: [ 0xc0f5df0 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xc0fedc0 <col:8>
|-CXXConstructorDecl 0xc103a20 <col:8> col:8 implicit used constexpr hkbProjectStringData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xc105de0 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fa628 'animation_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xc105eb0 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fa788 'behavior_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xc105f30 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fa8e8 'character_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xc105fb0 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0faa48 'event_names' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xc106030 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0faae0 'animation_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xc1060c0 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fab70 'behavior_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xc106108 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fac00 'character_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xc106150 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fac90 'full_path_to_source' 'hkStringPtr'
| | `-CXXConstructExpr 0xc106198 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xc0fad20 'root_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xc1061e0 <col:8> 'hkStringPtr' 'void () noexcept'
| `-CompoundStmt 0xc106278 <col:8>
|-CXXConstructorDecl 0xc103c20 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (const hkbProjectStringData &)' inline default noexcept-unevaluated 0xc103c20
| `-ParmVarDecl 0xc103d58 <col:8> col:8 'const hkbProjectStringData &'
`-CXXConstructorDecl 0xc105bc0 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (hkbProjectStringData &&)' inline default noexcept-unevaluated 0xc105bc0
  `-ParmVarDecl 0xc105cf8 <col:8> col:8 'hkbProjectStringData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectStringData = type { %class.hkReferencedObject, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr }
  NonVirtualBaseLLVMType:%struct.hkbProjectStringData = type { %class.hkReferencedObject, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr }
  IsZeroInitializable:1
  BitFields:[
]>
```
