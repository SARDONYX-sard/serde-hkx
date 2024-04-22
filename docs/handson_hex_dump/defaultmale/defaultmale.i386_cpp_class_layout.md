# defaultmale x86 C++ Class Layout

[See MemoryLayout](https://godbolt.org/z/xqYs7hYs5)

```log
*** Dumping AST Record Layout
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

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xcdea270 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkRootLevelContainerNamedVariant'
| `-RecordType 0xcdea1e0 'hkRootLevelContainerNamedVariant'
|   `-CXXRecord 0xcded7d8 'hkRootLevelContainerNamedVariant'
|-CXXRecordDecl 0xcdea4b0 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xcdea560 <line:23:4, col:11> col:4 private
|-FieldDecl 0xcdea638 <line:24:5, col:8> col:8 m_data 'hkRootLevelContainerNamedVariant *'
|-FieldDecl 0xcdea690 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xce06880 <line:25:8, col:78>
|   `-ParagraphComment 0xce06850 <col:8, col:78>
|     `-TextComment 0xce06820 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xcdea6e8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xce06970 <line:28:8, line:29:22>
|   `-ParagraphComment 0xce06940 <line:28:8, line:29:22>
|     |-TextComment 0xce068f0 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xce06910 <line:29:7, col:22> Text=" for the array. "
|-CXXConstructorDecl 0xcdfd878 <line:22:7> col:7 implicit constexpr hkArray 'void () noexcept' inline default trivial
|-CXXDestructorDecl 0xcdfd970 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xcdfdd28 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkRootLevelContainerNamedVariant> &)' inline default trivial noexcept-unevaluated 0xcdfdd28
| `-ParmVarDecl 0xcdfde68 <col:7> col:7 'const hkArray<hkRootLevelContainerNamedVariant> &'
`-CXXConstructorDecl 0xcdfdf58 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkRootLevelContainerNamedVariant> &&)' inline default trivial noexcept-unevaluated 0xcdfdf58
  `-ParmVarDecl 0xcdfe098 <col:7> col:7 'hkArray<hkRootLevelContainerNamedVariant> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcde9f40 <<source>:53:1, line:57:1> line:53:8 referenced struct hkRootLevelContainer definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-FullComment 0xce06b10 <line:49:4, line:52:18>
| `-ParagraphComment 0xce06ae0 <line:49:4, line:52:18>
|   |-TextComment 0xce06a40 <line:49:4, col:35> Text=" -      size: 16(if 32bit -> 12)"
|   |-TextComment 0xce06a60 <line:50:4, col:22> Text=" -    vtable: false"
|   |-TextComment 0xce06a80 <line:51:4, col:29> Text=" - signature: `0x2772c11e`"
|   `-TextComment 0xce06aa0 <line:52:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xcdea080 <line:53:1, col:8> col:8 implicit struct hkRootLevelContainer
|-CXXRecordDecl 0xcdea130 parent 0xcd723e8 <line:56:13, col:20> col:20 struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xcded468 <col:5, col:54> col:54 namedVariants 'hkArray<struct hkRootLevelContainerNamedVariant>':'hkArray<hkRootLevelContainerNamedVariant>'
|-CXXConstructorDecl 0xcdfd768 <line:53:8> col:8 implicit referenced constexpr hkRootLevelContainer 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xcdfdad8 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (const hkRootLevelContainer &)' inline default trivial noexcept-unevaluated 0xcdfdad8
| `-ParmVarDecl 0xcdfdc18 <col:8> col:8 'const hkRootLevelContainer &'
|-CXXConstructorDecl 0xcdfe198 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (hkRootLevelContainer &&)' inline default trivial noexcept-unevaluated 0xcdfe198
| `-ParmVarDecl 0xcdfe2d8 <col:8> col:8 'hkRootLevelContainer &&'
`-CXXDestructorDecl 0xcdfe458 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainer 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcd730d0 <<source>:4:1, line:7:1> line:4:7 referenced class hkStringPtr definition
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xcd73210 <col:1, col:7> col:7 implicit class hkStringPtr
|-AccessSpecDecl 0xcd732c0 <line:5:4, col:11> col:4 private
|-FieldDecl 0xcd73308 <line:6:5, col:17> col:17 m_stringAndFlag 'const char *'
|-CXXMethodDecl 0xcdfc3c8 <line:4:7> col:7 implicit constexpr operator= 'hkStringPtr &(const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xcdfc3c8
| `-ParmVarDecl 0xcdfc4f8 <col:7> col:7 'const hkStringPtr &'
|-CXXMethodDecl 0xcdfc5d8 <col:7> col:7 implicit constexpr operator= 'hkStringPtr &(hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xcdfc5d8
| `-ParmVarDecl 0xcdfc708 <col:7> col:7 'hkStringPtr &&'
|-CXXDestructorDecl 0xcdfcd90 <col:7> col:7 implicit constexpr ~hkStringPtr 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xce014c8 <col:7> col:7 implicit used constexpr hkStringPtr 'void () noexcept' inline default trivial
| `-CompoundStmt 0xce06348 <col:7>
|-CXXConstructorDecl 0xce017e0 <col:7> col:7 implicit constexpr hkStringPtr 'void (const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xce017e0
| `-ParmVarDecl 0xce01918 <col:7> col:7 'const hkStringPtr &'
`-CXXConstructorDecl 0xce019a0 <col:7> col:7 implicit constexpr hkStringPtr 'void (hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xce019a0
  `-ParmVarDecl 0xce01ad8 <col:7> col:7 'hkStringPtr &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkStringPtr = type { ptr }
  NonVirtualBaseLLVMType:%class.hkStringPtr = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcded7d8 prev 0xcdea130 <<source>:70:1, line:80:1> line:70:8 referenced struct hkRootLevelContainerNamedVariant definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xcded8d8 <col:1, col:8> col:8 implicit struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xcded9c8 <line:73:5, col:17> col:17 name 'hkStringPtr'
|-FieldDecl 0xcdeda30 <line:76:5, col:17> col:17 className 'hkStringPtr'
|-CXXRecordDecl 0xcdeda88 parent 0xcd723e8 <line:79:5, col:12> col:12 struct hkReferencedObject
|-FieldDecl 0xcdedc30 <col:5, col:32> col:32 variant 'struct hkReferencedObject *'
|-CXXConstructorDecl 0xce013b8 <line:70:8> col:8 implicit referenced constexpr hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xce01628 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (const hkRootLevelContainerNamedVariant &)' inline default trivial noexcept-unevaluated 0xce01628
| `-ParmVarDecl 0xce01768 <col:8> col:8 'const hkRootLevelContainerNamedVariant &'
|-CXXConstructorDecl 0xce01bd8 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (hkRootLevelContainerNamedVariant &&)' inline default trivial noexcept-unevaluated 0xce01bd8
| `-ParmVarDecl 0xce01d18 <col:8> col:8 'hkRootLevelContainerNamedVariant &&'
`-CXXDestructorDecl 0xce01e68 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcdedfe8 <<source>:95:1, line:98:1> line:95:7 referenced class hkBaseObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor can_const_default_init
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment
| `-Destructor non_trivial user_declared
|-FullComment 0xce06e20 <line:89:4, line:94:30>
| |-ParagraphComment 0xce06da0 <line:89:4, line:92:40>
| | |-TextComment 0xce06d00 <line:89:4, col:35> Text=" The class size is pointer size."
| | |-TextComment 0xce06d20 <line:90:4, col:80> Text=" The SDK description says that the `hkBaseObject`, a virtual function without"
| | |-TextComment 0xce06d40 <line:91:4, col:75> Text=" a field, is the source of inheritance for all Havok Classes so that the"
| | `-TextComment 0xce06d60 <line:92:4, col:40> Text=" vtable does not come after the data."
| `-ParagraphComment 0xce06df0 <line:94:4, col:30>
|   `-TextComment 0xce06dc0 <col:4, col:30> Text=" - size: 32bit: 4, 64bit: 8"
|-CXXRecordDecl 0xcdee120 <line:95:1, col:7> col:7 implicit referenced class hkBaseObject
|-AccessSpecDecl 0xcdee1d0 <line:96:4, col:10> col:4 public
|-CXXDestructorDecl 0xcdee2e8 <line:97:5, col:30> col:13 used ~hkBaseObject 'void () noexcept' virtual implicit-inline
| `-CompoundStmt 0xcdf03e0 <col:29, col:30>
|-CXXMethodDecl 0xcdf0208 <line:95:7> col:7 implicit constexpr operator= 'hkBaseObject &(const hkBaseObject &)' inline default noexcept-unevaluated 0xcdf0208
| `-ParmVarDecl 0xcdf0338 <col:7> col:7 'const hkBaseObject &'
|-CXXConstructorDecl 0xcdf2d08 <col:7> col:7 implicit constexpr hkBaseObject 'void (const hkBaseObject &)' inline default noexcept-unevaluated 0xcdf2d08
| `-ParmVarDecl 0xcdf2e48 <col:7> col:7 'const hkBaseObject &'
`-CXXConstructorDecl 0xce02380 <col:7> col:7 implicit used constexpr hkBaseObject 'void () noexcept' inline default
  `-CompoundStmt 0xce03280 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkBaseObject = type { ptr }
  NonVirtualBaseLLVMType:%class.hkBaseObject = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcdf25f8 prev 0xcdeda88 <<source>:112:1, line:134:1> line:112:7 referenced class hkReferencedObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param needs_overload_resolution implicit_has_const_param
| |-MoveConstructor exists simple non_trivial needs_overload_resolution
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial needs_overload_resolution
| `-Destructor simple non_trivial constexpr needs_overload_resolution
|-private 'hkBaseObject'
|-FullComment 0xce07020 <line:105:4, line:111:18>
| `-ParagraphComment 0xce06ff0 <line:105:4, line:111:18>
|   |-TextComment 0xce06ed0 <line:105:4, col:43> Text=" Stores memory size and reference count."
|   |-TextComment 0xce06ef0 <line:106:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xce06f10 <line:107:4, col:36> Text=" -      size: 32bit: 8, 64bit: 12"
|   |-TextComment 0xce06f30 <line:108:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xce06f50 <line:109:4, col:44> Text=" -    parent: `hkBaseObject`/`0xe0708a00`"
|   |-TextComment 0xce06f70 <line:110:4, col:29> Text=" - signature: `0x3b1c1113`"
|   `-TextComment 0xce06f90 <line:111:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xcdf2790 <line:112:1, col:7> col:7 implicit class hkReferencedObject
|-FieldDecl 0xcdf2888 <line:119:5, col:14> col:14 memSizeAndFlags 'hkUint16':'unsigned short'
| `-FullComment 0xce161c0 <line:117:8, line:118:48>
|   `-ParagraphComment 0xce070e0 <line:117:8, line:118:48>
|     |-TextComment 0xce07090 <line:117:8, col:37> Text=" - offset: 32bit: 4, 64bit:  8"
|     `-TextComment 0xce070b0 <line:118:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xcdf2920 <line:122:5, col:14> col:14 referenceCount 'hkUint16':'unsigned short'
| `-FullComment 0xce162b0 <line:120:8, line:121:48>
|   `-ParagraphComment 0xce16280 <line:120:8, line:121:48>
|     |-TextComment 0xce16230 <line:120:8, col:37> Text=" - offset: 32bit: 6, 64bit: 10"
|     `-TextComment 0xce16250 <line:121:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xcdf2a38 <line:129:5, col:17> col:10 _pad0 'char[0]'
|-CXXConstructorDecl 0xcdf2b40 <line:112:7> col:7 implicit constexpr hkReferencedObject 'void (const hkReferencedObject &)' inline default noexcept-unevaluated 0xcdf2b40
| `-ParmVarDecl 0xcdf2c78 <col:7> col:7 'const hkReferencedObject &'
|-CXXConstructorDecl 0xcdf5c98 <col:7> col:7 implicit constexpr hkReferencedObject 'void (hkReferencedObject &&)' inline default noexcept-unevaluated 0xcdf5c98
| `-ParmVarDecl 0xcdf5dd8 <col:7> col:7 'hkReferencedObject &&'
|-CXXMethodDecl 0xcdf5ee0 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(const hkReferencedObject &)' inline default noexcept-unevaluated 0xcdf5ee0
| `-ParmVarDecl 0xcdf6018 <col:7> col:7 'const hkReferencedObject &'
|-CXXMethodDecl 0xcdf6090 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(hkReferencedObject &&)' inline default noexcept-unevaluated 0xcdf6090
| `-ParmVarDecl 0xcdf61c8 <col:7> col:7 'hkReferencedObject &&'
|-CXXDestructorDecl 0xcdf6250 <col:7> col:7 implicit used constexpr ~hkReferencedObject 'void () noexcept' inline default
| |-Overrides: [ 0xcdee2e8 hkBaseObject::~hkBaseObject 'void () noexcept' ]
| `-CompoundStmt 0xcdf6438 <col:7>
`-CXXConstructorDecl 0xce02290 <col:7> col:7 implicit used constexpr hkReferencedObject 'void () noexcept' inline default
  |-CXXCtorInitializer 'hkBaseObject'
  | `-CXXConstructExpr 0xce03290 <col:7> 'hkBaseObject' 'void () noexcept'
  `-CompoundStmt 0xce032f0 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [0 x i8] }
  NonVirtualBaseLLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [0 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcde8818 <<source>:34:1, line:37:1> line:34:8 referenced struct hkVector4 definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable pod trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xcde97a8 <col:1, col:8> col:8 implicit struct hkVector4
|-FieldDecl 0xcde98b8 <line:35:5, col:12> col:12 x 'hkReal':'float'
| `-AlignedAttr 0xcde9928 <col:29, col:39> aligned
|   `-ConstantExpr 0xcde9908 <col:37> 'int'
|     |-value: Int 16
|     `-IntegerLiteral 0xcde9880 <col:37> 'int' 16
|-FieldDecl 0xcde99b0 <line:36:5, col:11> col:11 y 'float'
|-FieldDecl 0xcde9a20 <col:5, col:14> col:14 z 'float'
|-FieldDecl 0xcde9a90 <col:5, col:17> col:17 w 'float'
|-CXXMethodDecl 0xcdf95d8 <line:34:8> col:8 implicit constexpr operator= 'hkVector4 &(const hkVector4 &)' inline default trivial noexcept-unevaluated 0xcdf95d8
| `-ParmVarDecl 0xcdf9708 <col:8> col:8 'const hkVector4 &'
|-CXXMethodDecl 0xcdf97e8 <col:8> col:8 implicit constexpr operator= 'hkVector4 &(hkVector4 &&)' inline default trivial noexcept-unevaluated 0xcdf97e8
| `-ParmVarDecl 0xcdf9918 <col:8> col:8 'hkVector4 &&'
|-CXXDestructorDecl 0xcdfa438 <col:8> col:8 implicit constexpr ~hkVector4 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xce024a8 <col:8> col:8 implicit used constexpr hkVector4 'void () noexcept' inline default trivial
| `-CompoundStmt 0xce03358 <col:8>
|-CXXConstructorDecl 0xce02860 <col:8> col:8 implicit constexpr hkVector4 'void (const hkVector4 &)' inline default trivial noexcept-unevaluated 0xce02860
| `-ParmVarDecl 0xce02998 <col:8> col:8 'const hkVector4 &'
`-CXXConstructorDecl 0xce02a20 <col:8> col:8 implicit constexpr hkVector4 'void (hkVector4 &&)' inline default trivial noexcept-unevaluated 0xce02a20
  `-ParmVarDecl 0xce02b58 <col:8> col:8 'hkVector4 &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkVector4 = type { float, float, float, float }
  NonVirtualBaseLLVMType:%struct.hkVector4 = type { float, float, float, float }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xcdf8cc8 <<source>:12:1, line:16:1> line:13:7 class hkEnum definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'EventMode'
| `-EnumType 0xcde9be0 'EventMode'
|   `-Enum 0xcde9b20 'EventMode'
|-TemplateArgument type 'char'
| `-BuiltinType 0xcd72490 'char'
|-CXXRecordDecl 0xcdf8f40 <col:1, col:7> col:7 implicit class hkEnum
|-AccessSpecDecl 0xcdf8ff0 <line:14:4, col:11> col:4 private
|-FieldDecl 0xcdf9068 <line:15:5, col:7> col:7 storage 'char'
|-CXXMethodDecl 0xcdf9a98 <line:13:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xcdf9a98
| `-ParmVarDecl 0xcdf9bc8 <col:7> col:7 'const hkEnum<EventMode, char> &'
|-CXXMethodDecl 0xcdf9cd8 <col:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xcdf9cd8
| `-ParmVarDecl 0xcdf9e08 <col:7> col:7 'hkEnum<EventMode, char> &&'
|-CXXDestructorDecl 0xcdfa530 <col:7> col:7 implicit constexpr ~hkEnum 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xce025b8 <col:7> col:7 implicit used constexpr hkEnum 'void () noexcept' inline default trivial
| `-CompoundStmt 0xce03418 <col:7>
|-CXXConstructorDecl 0xce02be0 <col:7> col:7 implicit constexpr hkEnum 'void (const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xce02be0
| `-ParmVarDecl 0xce02d18 <col:7> col:7 'const hkEnum<EventMode, char> &'
`-CXXConstructorDecl 0xce02da0 <col:7> col:7 implicit constexpr hkEnum 'void (hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xce02da0
  `-ParmVarDecl 0xce02ed8 <col:7> col:7 'hkEnum<EventMode, char> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkEnum = type { i8 }
  NonVirtualBaseLLVMType:%class.hkEnum = type { i8 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcdf6608 <<source>:149:1, line:197:1> line:149:8 referenced struct hkbProjectData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xce16620 <line:143:4, line:148:18>
| `-ParagraphComment 0xce165f0 <line:143:4, line:148:18>
|   |-TextComment 0xce16500 <line:143:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xce16520 <line:144:4, col:26> Text=" -      size: 32bit: 48"
|   |-TextComment 0xce16540 <line:145:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xce16560 <line:146:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xce16580 <line:147:4, col:29> Text=" - signature: `0x13a39ba7`"
|   `-TextComment 0xce165a0 <line:148:4, col:18> Text=" -   version: 2"
|-CXXRecordDecl 0xcdf6810 <line:149:1, col:8> col:8 implicit struct hkbProjectData
|-FieldDecl 0xcdf6908 <line:177:5, col:15> col:15 worldUpWS 'hkVector4'
| `-FullComment 0xce167b0 <line:171:8, line:176:30>
|   |-ParagraphComment 0xce16710 <line:171:8, line:173:22>
|   | |-TextComment 0xce16690 <line:171:8, col:71> Text=" The offset here is 16 bytes because Vector4 performs a 16-bytes"
|   | |-TextComment 0xce166b0 <line:172:8, col:68> Text=" alignment to account for SIMD operations using f32 * 4 = 128"
|   | `-TextComment 0xce166d0 <line:173:8, col:22> Text=" XMM registers."
|   `-ParagraphComment 0xce16780 <line:175:8, line:176:30>
|     |-TextComment 0xce16730 <line:175:8, col:38> Text=" - offset: 32bit: 16, 64bit: 16"
|     `-TextComment 0xce16750 <line:176:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-CXXRecordDecl 0xcdf6990 parent 0xcd723e8 <line:181:5, col:12> col:12 struct hkbProjectStringData
|-FieldDecl 0xcdf6b40 <col:5, col:34> col:34 stringData 'struct hkbProjectStringData *'
| `-FullComment 0xce168d0 <line:178:8, line:180:38>
|   `-ParagraphComment 0xce168a0 <line:178:8, line:180:38>
|     |-TextComment 0xce16820 <line:178:8, col:38> Text=" - offset: 32bit: 32, 64bit: 32"
|     |-TextComment 0xce16840 <line:179:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xce16860 <line:180:8, col:38> Text=" -   size: 32bit:  4, 64bit:  8"
|-FieldDecl 0xcdf90d8 <line:186:5, col:29> col:29 defaultEventMode 'hkEnum<EventMode, char>'
| `-FullComment 0xce16a10 <line:182:8, line:185:38>
|   `-ParagraphComment 0xce169e0 <line:182:8, line:185:38>
|     |-TextComment 0xce16940 <line:182:8, col:20> Text=" - offset: 36"
|     |-TextComment 0xce16960 <line:183:8, col:38> Text=" - offset: 32bit: 36, 64bit: 40"
|     |-TextComment 0xce16980 <line:184:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xce169a0 <line:185:8, col:38> Text=" -   size: 32bit:  1, 64bit:  1"
|-FieldDecl 0xcdf91f8 <line:193:5, col:18> col:10 _pad0 'char[11]'
|-CXXMethodDecl 0xcdf9360 <line:149:8> col:8 implicit constexpr operator= 'hkbProjectData &(const hkbProjectData &)' inline default noexcept-unevaluated 0xcdf9360
| `-ParmVarDecl 0xcdf9498 <col:8> col:8 'const hkbProjectData &'
|-CXXMethodDecl 0xcdf9ef8 <col:8> col:8 implicit constexpr operator= 'hkbProjectData &(hkbProjectData &&)' inline default noexcept-unevaluated 0xcdf9ef8
| `-ParmVarDecl 0xcdfa028 <col:8> col:8 'hkbProjectData &&'
|-CXXDestructorDecl 0xcdfa0b0 <col:8> col:8 implicit used constexpr ~hkbProjectData 'void () noexcept' inline default
| |-Overrides: [ 0xcdf6250 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xcdfa688 <col:8>
|-CXXConstructorDecl 0xce02198 <col:8> col:8 implicit used constexpr hkbProjectData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xce03300 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdf6908 'worldUpWS' 'hkVector4'
| | `-CXXConstructExpr 0xce033d0 <col:8> 'hkVector4' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdf90d8 'defaultEventMode' 'hkEnum<EventMode, char>'
| | `-CXXConstructExpr 0xce03490 <col:8> 'hkEnum<EventMode, char>' 'void () noexcept'
| `-CompoundStmt 0xce034f0 <col:8>
|-CXXConstructorDecl 0xce026b0 <col:8> col:8 implicit constexpr hkbProjectData 'void (const hkbProjectData &)' inline default noexcept-unevaluated 0xce026b0
| `-ParmVarDecl 0xce027e8 <col:8> col:8 'const hkbProjectData &'
`-CXXConstructorDecl 0xce02f70 <col:8> col:8 implicit constexpr hkbProjectData 'void (hkbProjectData &&)' inline default noexcept-unevaluated 0xce02f70
  `-ParmVarDecl 0xce030a8 <col:8> col:8 'hkbProjectData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, [8 x i8], %struct.hkVector4, ptr, %class.hkEnum, [11 x i8] }
  NonVirtualBaseLLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, [8 x i8], %struct.hkVector4, ptr, %class.hkEnum, [11 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xcdfaab0 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkStringPtr'
| `-RecordType 0xcd73180 'hkStringPtr'
|   `-CXXRecord 0xcd730d0 'hkStringPtr'
|-CXXRecordDecl 0xcdfae70 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xcdfaf20 <line:23:4, col:11> col:4 private
|-FieldDecl 0xcdfaff8 <line:24:5, col:8> col:8 m_data 'hkStringPtr *'
|-FieldDecl 0xcdfb050 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xce16b70 <line:25:8, col:78>
|   `-ParagraphComment 0xce16b40 <col:8, col:78>
|     `-TextComment 0xce16b10 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xcdfb0a8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xce16c60 <line:28:8, line:29:22>
|   `-ParagraphComment 0xce16c30 <line:28:8, line:29:22>
|     |-TextComment 0xce16be0 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xce16c00 <line:29:7, col:22> Text=" for the array. "
|-CXXMethodDecl 0xcdfbc18 <line:22:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xcdfbc18
| `-ParmVarDecl 0xcdfbd48 <col:7> col:7 'const hkArray<hkStringPtr> &'
|-CXXMethodDecl 0xcdfc148 <col:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xcdfc148
| `-ParmVarDecl 0xcdfc278 <col:7> col:7 'hkArray<hkStringPtr> &&'
|-CXXDestructorDecl 0xcdfcc90 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xce038d8 <col:7> col:7 implicit used constexpr hkArray 'void () noexcept' inline default trivial
| `-CompoundStmt 0xce04188 <col:7>
|-CXXConstructorDecl 0xce03b80 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xce03b80
| `-ParmVarDecl 0xce03cb8 <col:7> col:7 'const hkArray<hkStringPtr> &'
`-CXXConstructorDecl 0xce03d40 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xce03d40
  `-ParmVarDecl 0xce03e78 <col:7> col:7 'hkArray<hkStringPtr> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xcdfa828 prev 0xcdf6990 <<source>:215:1, line:268:1> line:215:8 referenced struct hkbProjectStringData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xce16f80 <line:204:4, line:214:18>
| |-ParagraphComment 0xce16dc0 <line:204:4, col:26>
| | `-TextComment 0xce16d90 <col:4, col:26> Text=" `hkbProjectStringData`"
| |-ParagraphComment 0xce16e30 <line:206:4, line:207:69>
| | |-TextComment 0xce16de0 <line:206:4, col:63> Text=" - In C++, it represents the name of one field in the class."
| | `-TextComment 0xce16e00 <line:207:4, col:69> Text=" - In XML, the value of the `name` attribute of the `hkparam` tag."
| `-ParagraphComment 0xce16f40 <line:209:4, line:214:18>
|   |-TextComment 0xce16e50 <line:209:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xce16e70 <line:210:4, col:19> Text=" -      size: 76"
|   |-TextComment 0xce16e90 <line:211:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xce16eb0 <line:212:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xce16ed0 <line:213:4, col:28> Text=" - signature: `0x76ad60a`"
|   `-TextComment 0xce16ef0 <line:214:4, col:18> Text=" -   version: 1"
|-CXXRecordDecl 0xcdfa9f0 <line:215:1, col:8> col:8 implicit struct hkbProjectStringData
|-FieldDecl 0xcdfb118 <line:241:5, col:26> col:26 animation_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xce17070 <line:239:8, line:240:30>
|   `-ParagraphComment 0xce17040 <line:239:8, line:240:30>
|     |-TextComment 0xce16ff0 <line:239:8, col:38> Text=" - offset: 32bit:  8, 64bit: 16"
|     `-TextComment 0xce17010 <line:240:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb278 <line:245:5, col:26> col:26 behavior_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xce17160 <line:243:8, line:244:30>
|   `-ParagraphComment 0xce17130 <line:243:8, line:244:30>
|     |-TextComment 0xce170e0 <line:243:8, col:38> Text=" - offset: 32bit: 20, 64bit: 32"
|     `-TextComment 0xce17100 <line:244:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb3d8 <line:248:5, col:26> col:26 character_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xce1b370 <line:246:8, line:247:30>
|   `-ParagraphComment 0xce1b340 <line:246:8, line:247:30>
|     |-TextComment 0xce1b2f0 <line:246:8, col:38> Text=" - offset: 32bit: 32, 64bit: 48"
|     `-TextComment 0xce1b310 <line:247:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb538 <line:251:5, col:26> col:26 event_names 'hkArray<hkStringPtr>'
| `-FullComment 0xce1b460 <line:249:8, line:250:30>
|   `-ParagraphComment 0xce1b430 <line:249:8, line:250:30>
|     |-TextComment 0xce1b3e0 <line:249:8, col:38> Text=" - offset: 32bit: 44, 64bit: 64"
|     `-TextComment 0xce1b400 <line:250:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb5d0 <line:255:5, col:17> col:17 animation_path 'hkStringPtr'
| `-FullComment 0xce1b550 <line:253:8, line:254:30>
|   `-ParagraphComment 0xce1b520 <line:253:8, line:254:30>
|     |-TextComment 0xce1b4d0 <line:253:8, col:38> Text=" - offset: 32bit: 56, 64bit: 80"
|     `-TextComment 0xce1b4f0 <line:254:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb660 <line:258:5, col:17> col:17 behavior_path 'hkStringPtr'
| `-FullComment 0xce1b640 <line:256:8, line:257:30>
|   `-ParagraphComment 0xce1b610 <line:256:8, line:257:30>
|     |-TextComment 0xce1b5c0 <line:256:8, col:38> Text=" - offset: 32bit: 60, 64bit: 88"
|     `-TextComment 0xce1b5e0 <line:257:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb6f0 <line:261:5, col:17> col:17 character_path 'hkStringPtr'
| `-FullComment 0xce1b730 <line:259:8, line:260:30>
|   `-ParagraphComment 0xce1b700 <line:259:8, line:260:30>
|     |-TextComment 0xce1b6b0 <line:259:8, col:38> Text=" - offset: 32bit: 64, 64bit: 96"
|     `-TextComment 0xce1b6d0 <line:260:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb780 <line:264:5, col:17> col:17 full_path_to_source 'hkStringPtr'
| `-FullComment 0xce1b820 <line:262:8, line:263:30>
|   `-ParagraphComment 0xce1b7f0 <line:262:8, line:263:30>
|     |-TextComment 0xce1b7a0 <line:262:8, col:39> Text=" - offset: 32bit: 68, 64bit: 104"
|     `-TextComment 0xce1b7c0 <line:263:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xcdfb810 <line:267:5, col:17> col:17 root_path 'hkStringPtr'
| `-FullComment 0xce1b910 <line:265:8, line:266:48>
|   `-ParagraphComment 0xce1b8e0 <line:265:8, line:266:48>
|     |-TextComment 0xce1b890 <line:265:8, col:39> Text=" - offset: 32bit: 72, 64bit: 112"
|     `-TextComment 0xce1b8b0 <line:266:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-CXXMethodDecl 0xcdfb970 <line:215:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(const hkbProjectStringData &)' inline default noexcept-unevaluated 0xcdfb970
| `-ParmVarDecl 0xcdfbaa8 <col:8> col:8 'const hkbProjectStringData &'
|-CXXMethodDecl 0xcdfc7f8 <col:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(hkbProjectStringData &&)' inline default noexcept-unevaluated 0xcdfc7f8
| `-ParmVarDecl 0xcdfc928 <col:8> col:8 'hkbProjectStringData &&'
|-CXXDestructorDecl 0xcdfc9b0 <col:8> col:8 implicit used constexpr ~hkbProjectStringData 'void () noexcept' inline default
| |-Overrides: [ 0xcdf6250 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xcdfced0 <col:8>
|-CXXConstructorDecl 0xce037d0 <col:8> col:8 implicit used constexpr hkbProjectStringData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xce04130 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb118 'animation_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xce06180 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb278 'behavior_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xce06200 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb3d8 'character_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xce06280 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb538 'event_names' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xce06300 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb5d0 'animation_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xce06390 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb660 'behavior_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xce063d8 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb6f0 'character_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xce06420 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb780 'full_path_to_source' 'hkStringPtr'
| | `-CXXConstructExpr 0xce06468 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xcdfb810 'root_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xce064b0 <col:8> 'hkStringPtr' 'void () noexcept'
| `-CompoundStmt 0xce06548 <col:8>
|-CXXConstructorDecl 0xce039d0 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (const hkbProjectStringData &)' inline default noexcept-unevaluated 0xce039d0
| `-ParmVarDecl 0xce03b08 <col:8> col:8 'const hkbProjectStringData &'
`-CXXConstructorDecl 0xce03f10 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (hkbProjectStringData &&)' inline default noexcept-unevaluated 0xce03f10
  `-ParmVarDecl 0xce04048 <col:8> col:8 'hkbProjectStringData &&'

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

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xbe5c170 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkRootLevelContainerNamedVariant'
| `-RecordType 0xbe5c0e0 'hkRootLevelContainerNamedVariant'
|   `-CXXRecord 0xbe5f678 'hkRootLevelContainerNamedVariant'
|-CXXRecordDecl 0xbe5c3b0 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xbe5c460 <line:23:4, col:11> col:4 private
|-FieldDecl 0xbe5c538 <line:24:5, col:8> col:8 m_data 'hkRootLevelContainerNamedVariant *'
|-FieldDecl 0xbe5c590 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xbe78310 <line:25:8, col:78>
|   `-ParagraphComment 0xbe782e0 <col:8, col:78>
|     `-TextComment 0xbe782b0 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xbe5c5e8 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xbe78400 <line:28:8, line:29:22>
|   `-ParagraphComment 0xbe783d0 <line:28:8, line:29:22>
|     |-TextComment 0xbe78380 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xbe783a0 <line:29:7, col:22> Text=" for the array. "
|-CXXConstructorDecl 0xbe6f308 <line:22:7> col:7 implicit constexpr hkArray 'void () noexcept' inline default trivial
|-CXXDestructorDecl 0xbe6f400 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe6f7b8 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkRootLevelContainerNamedVariant> &)' inline default trivial noexcept-unevaluated 0xbe6f7b8
| `-ParmVarDecl 0xbe6f8f8 <col:7> col:7 'const hkArray<hkRootLevelContainerNamedVariant> &'
`-CXXConstructorDecl 0xbe6f9e8 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkRootLevelContainerNamedVariant> &&)' inline default trivial noexcept-unevaluated 0xbe6f9e8
  `-ParmVarDecl 0xbe6fb28 <col:7> col:7 'hkArray<hkRootLevelContainerNamedVariant> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe5be40 <<source>:53:1, line:57:1> line:53:8 referenced struct hkRootLevelContainer definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-FullComment 0xbe785a0 <line:49:4, line:52:18>
| `-ParagraphComment 0xbe78570 <line:49:4, line:52:18>
|   |-TextComment 0xbe784d0 <line:49:4, col:35> Text=" -      size: 16(if 32bit -> 12)"
|   |-TextComment 0xbe784f0 <line:50:4, col:22> Text=" -    vtable: false"
|   |-TextComment 0xbe78510 <line:51:4, col:29> Text=" - signature: `0x2772c11e`"
|   `-TextComment 0xbe78530 <line:52:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xbe5bf80 <line:53:1, col:8> col:8 implicit struct hkRootLevelContainer
|-CXXRecordDecl 0xbe5c030 parent 0xbde3a68 <line:56:13, col:20> col:20 struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xbe5f308 <col:5, col:54> col:54 namedVariants 'hkArray<struct hkRootLevelContainerNamedVariant>':'hkArray<hkRootLevelContainerNamedVariant>'
|-CXXConstructorDecl 0xbe6f1f8 <line:53:8> col:8 implicit referenced constexpr hkRootLevelContainer 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe6f568 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (const hkRootLevelContainer &)' inline default trivial noexcept-unevaluated 0xbe6f568
| `-ParmVarDecl 0xbe6f6a8 <col:8> col:8 'const hkRootLevelContainer &'
|-CXXConstructorDecl 0xbe6fc28 <col:8> col:8 implicit constexpr hkRootLevelContainer 'void (hkRootLevelContainer &&)' inline default trivial noexcept-unevaluated 0xbe6fc28
| `-ParmVarDecl 0xbe6fd68 <col:8> col:8 'hkRootLevelContainer &&'
`-CXXDestructorDecl 0xbe6fee8 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainer 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainer = type { %class.hkArray }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbde4750 <<source>:4:1, line:7:1> line:4:7 referenced class hkStringPtr definition
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xbde4890 <col:1, col:7> col:7 implicit class hkStringPtr
|-AccessSpecDecl 0xbde4940 <line:5:4, col:11> col:4 private
|-FieldDecl 0xbde4988 <line:6:5, col:17> col:17 m_stringAndFlag 'const char *'
|-CXXMethodDecl 0xbe6de58 <line:4:7> col:7 implicit constexpr operator= 'hkStringPtr &(const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xbe6de58
| `-ParmVarDecl 0xbe6df88 <col:7> col:7 'const hkStringPtr &'
|-CXXMethodDecl 0xbe6e068 <col:7> col:7 implicit constexpr operator= 'hkStringPtr &(hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xbe6e068
| `-ParmVarDecl 0xbe6e198 <col:7> col:7 'hkStringPtr &&'
|-CXXDestructorDecl 0xbe6e820 <col:7> col:7 implicit constexpr ~hkStringPtr 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe72f58 <col:7> col:7 implicit used constexpr hkStringPtr 'void () noexcept' inline default trivial
| `-CompoundStmt 0xbe77dd8 <col:7>
|-CXXConstructorDecl 0xbe73270 <col:7> col:7 implicit constexpr hkStringPtr 'void (const hkStringPtr &)' inline default trivial noexcept-unevaluated 0xbe73270
| `-ParmVarDecl 0xbe733a8 <col:7> col:7 'const hkStringPtr &'
`-CXXConstructorDecl 0xbe73430 <col:7> col:7 implicit constexpr hkStringPtr 'void (hkStringPtr &&)' inline default trivial noexcept-unevaluated 0xbe73430
  `-ParmVarDecl 0xbe73568 <col:7> col:7 'hkStringPtr &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkStringPtr = type { ptr }
  NonVirtualBaseLLVMType:%class.hkStringPtr = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe5f678 prev 0xbe5c030 <<source>:70:1, line:80:1> line:70:8 referenced struct hkRootLevelContainerNamedVariant definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param needs_implicit implicit_has_const_param
| |-MoveAssignment exists simple trivial needs_implicit
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xbe5f778 <col:1, col:8> col:8 implicit struct hkRootLevelContainerNamedVariant
|-FieldDecl 0xbe5f868 <line:73:5, col:17> col:17 name 'hkStringPtr'
|-FieldDecl 0xbe5f8d0 <line:76:5, col:17> col:17 className 'hkStringPtr'
|-CXXRecordDecl 0xbe5f928 parent 0xbde3a68 <line:79:5, col:12> col:12 struct hkReferencedObject
|-FieldDecl 0xbe5fad0 <col:5, col:32> col:32 variant 'struct hkReferencedObject *'
|-CXXConstructorDecl 0xbe72e48 <line:70:8> col:8 implicit referenced constexpr hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe730b8 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (const hkRootLevelContainerNamedVariant &)' inline default trivial noexcept-unevaluated 0xbe730b8
| `-ParmVarDecl 0xbe731f8 <col:8> col:8 'const hkRootLevelContainerNamedVariant &'
|-CXXConstructorDecl 0xbe73668 <col:8> col:8 implicit constexpr hkRootLevelContainerNamedVariant 'void (hkRootLevelContainerNamedVariant &&)' inline default trivial noexcept-unevaluated 0xbe73668
| `-ParmVarDecl 0xbe737a8 <col:8> col:8 'hkRootLevelContainerNamedVariant &&'
`-CXXDestructorDecl 0xbe738f8 <col:8> col:8 implicit referenced constexpr ~hkRootLevelContainerNamedVariant 'void () noexcept' inline default trivial

Layout: <CGRecordLayout
  LLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  NonVirtualBaseLLVMType:%struct.hkRootLevelContainerNamedVariant = type { %class.hkStringPtr, %class.hkStringPtr, ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe5fe88 <<source>:95:1, line:98:1> line:95:7 referenced class hkBaseObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor can_const_default_init
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment
| `-Destructor non_trivial user_declared
|-FullComment 0xbe788b0 <line:89:4, line:94:30>
| |-ParagraphComment 0xbe78830 <line:89:4, line:92:40>
| | |-TextComment 0xbe78790 <line:89:4, col:35> Text=" The class size is pointer size."
| | |-TextComment 0xbe787b0 <line:90:4, col:80> Text=" The SDK description says that the `hkBaseObject`, a virtual function without"
| | |-TextComment 0xbe787d0 <line:91:4, col:75> Text=" a field, is the source of inheritance for all Havok Classes so that the"
| | `-TextComment 0xbe787f0 <line:92:4, col:40> Text=" vtable does not come after the data."
| `-ParagraphComment 0xbe78880 <line:94:4, col:30>
|   `-TextComment 0xbe78850 <col:4, col:30> Text=" - size: 32bit: 4, 64bit: 8"
|-CXXRecordDecl 0xbe5ffc0 <line:95:1, col:7> col:7 implicit referenced class hkBaseObject
|-AccessSpecDecl 0xbe60070 <line:96:4, col:10> col:4 public
|-CXXDestructorDecl 0xbe60188 <line:97:5, col:30> col:13 used ~hkBaseObject 'void () noexcept' virtual implicit-inline
| `-CompoundStmt 0xbe62190 <col:29, col:30>
|-CXXMethodDecl 0xbe61fb8 <line:95:7> col:7 implicit constexpr operator= 'hkBaseObject &(const hkBaseObject &)' inline default noexcept-unevaluated 0xbe61fb8
| `-ParmVarDecl 0xbe620e8 <col:7> col:7 'const hkBaseObject &'
|-CXXConstructorDecl 0xbe64ab8 <col:7> col:7 implicit constexpr hkBaseObject 'void (const hkBaseObject &)' inline default noexcept-unevaluated 0xbe64ab8
| `-ParmVarDecl 0xbe64bf8 <col:7> col:7 'const hkBaseObject &'
`-CXXConstructorDecl 0xbe73e10 <col:7> col:7 implicit used constexpr hkBaseObject 'void () noexcept' inline default
  `-CompoundStmt 0xbe74d10 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkBaseObject = type { ptr }
  NonVirtualBaseLLVMType:%class.hkBaseObject = type { ptr }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe643a8 prev 0xbe5f928 <<source>:112:1, line:134:1> line:112:7 referenced class hkReferencedObject definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param needs_overload_resolution implicit_has_const_param
| |-MoveConstructor exists simple non_trivial needs_overload_resolution
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial needs_overload_resolution
| `-Destructor simple non_trivial constexpr needs_overload_resolution
|-private 'hkBaseObject'
|-FullComment 0xbe78ab0 <line:105:4, line:111:18>
| `-ParagraphComment 0xbe78a80 <line:105:4, line:111:18>
|   |-TextComment 0xbe78960 <line:105:4, col:43> Text=" Stores memory size and reference count."
|   |-TextComment 0xbe78980 <line:106:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xbe789a0 <line:107:4, col:36> Text=" -      size: 32bit: 8, 64bit: 12"
|   |-TextComment 0xbe789c0 <line:108:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xbe789e0 <line:109:4, col:44> Text=" -    parent: `hkBaseObject`/`0xe0708a00`"
|   |-TextComment 0xbe78a00 <line:110:4, col:29> Text=" - signature: `0x3b1c1113`"
|   `-TextComment 0xbe78a20 <line:111:4, col:18> Text=" -   version: 0"
|-CXXRecordDecl 0xbe64540 <line:112:1, col:7> col:7 implicit class hkReferencedObject
|-FieldDecl 0xbe64638 <line:119:5, col:14> col:14 memSizeAndFlags 'hkUint16':'unsigned short'
| `-FullComment 0xbe87d10 <line:117:8, line:118:48>
|   `-ParagraphComment 0xbe78b70 <line:117:8, line:118:48>
|     |-TextComment 0xbe78b20 <line:117:8, col:37> Text=" - offset: 32bit: 4, 64bit:  8"
|     `-TextComment 0xbe78b40 <line:118:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xbe646d0 <line:122:5, col:14> col:14 referenceCount 'hkUint16':'unsigned short'
| `-FullComment 0xbe87e00 <line:120:8, line:121:48>
|   `-ParagraphComment 0xbe87dd0 <line:120:8, line:121:48>
|     |-TextComment 0xbe87d80 <line:120:8, col:37> Text=" - offset: 32bit: 6, 64bit: 10"
|     `-TextComment 0xbe87da0 <line:121:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-FieldDecl 0xbe647e8 <line:129:5, col:17> col:10 _pad0 'char[0]'
|-CXXConstructorDecl 0xbe648f0 <line:112:7> col:7 implicit constexpr hkReferencedObject 'void (const hkReferencedObject &)' inline default noexcept-unevaluated 0xbe648f0
| `-ParmVarDecl 0xbe64a28 <col:7> col:7 'const hkReferencedObject &'
|-CXXConstructorDecl 0xbe67728 <col:7> col:7 implicit constexpr hkReferencedObject 'void (hkReferencedObject &&)' inline default noexcept-unevaluated 0xbe67728
| `-ParmVarDecl 0xbe67868 <col:7> col:7 'hkReferencedObject &&'
|-CXXMethodDecl 0xbe67970 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(const hkReferencedObject &)' inline default noexcept-unevaluated 0xbe67970
| `-ParmVarDecl 0xbe67aa8 <col:7> col:7 'const hkReferencedObject &'
|-CXXMethodDecl 0xbe67b20 <col:7> col:7 implicit constexpr operator= 'hkReferencedObject &(hkReferencedObject &&)' inline default noexcept-unevaluated 0xbe67b20
| `-ParmVarDecl 0xbe67c58 <col:7> col:7 'hkReferencedObject &&'
|-CXXDestructorDecl 0xbe67ce0 <col:7> col:7 implicit used constexpr ~hkReferencedObject 'void () noexcept' inline default
| |-Overrides: [ 0xbe60188 hkBaseObject::~hkBaseObject 'void () noexcept' ]
| `-CompoundStmt 0xbe67ec8 <col:7>
`-CXXConstructorDecl 0xbe73d20 <col:7> col:7 implicit used constexpr hkReferencedObject 'void () noexcept' inline default
  |-CXXCtorInitializer 'hkBaseObject'
  | `-CXXConstructExpr 0xbe74d20 <col:7> 'hkBaseObject' 'void () noexcept'
  `-CompoundStmt 0xbe74d80 <col:7>

Layout: <CGRecordLayout
  LLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [0 x i8] }
  NonVirtualBaseLLVMType:%class.hkReferencedObject = type { %class.hkBaseObject, i16, i16, [0 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe5a308 <<source>:34:1, line:37:1> line:34:8 referenced struct hkVector4 definition
|-DefinitionData pass_in_registers aggregate standard_layout trivially_copyable pod trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-CXXRecordDecl 0xbe5b6a8 <col:1, col:8> col:8 implicit struct hkVector4
|-FieldDecl 0xbe5b7b8 <line:35:5, col:12> col:12 x 'hkReal':'float'
| `-AlignedAttr 0xbe5b828 <col:29, col:39> aligned
|   `-ConstantExpr 0xbe5b808 <col:37> 'int'
|     |-value: Int 16
|     `-IntegerLiteral 0xbe5b780 <col:37> 'int' 16
|-FieldDecl 0xbe5b8b0 <line:36:5, col:11> col:11 y 'float'
|-FieldDecl 0xbe5b920 <col:5, col:14> col:14 z 'float'
|-FieldDecl 0xbe5b990 <col:5, col:17> col:17 w 'float'
|-CXXMethodDecl 0xbe6b068 <line:34:8> col:8 implicit constexpr operator= 'hkVector4 &(const hkVector4 &)' inline default trivial noexcept-unevaluated 0xbe6b068
| `-ParmVarDecl 0xbe6b198 <col:8> col:8 'const hkVector4 &'
|-CXXMethodDecl 0xbe6b278 <col:8> col:8 implicit constexpr operator= 'hkVector4 &(hkVector4 &&)' inline default trivial noexcept-unevaluated 0xbe6b278
| `-ParmVarDecl 0xbe6b3a8 <col:8> col:8 'hkVector4 &&'
|-CXXDestructorDecl 0xbe6bec8 <col:8> col:8 implicit constexpr ~hkVector4 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe73f38 <col:8> col:8 implicit used constexpr hkVector4 'void () noexcept' inline default trivial
| `-CompoundStmt 0xbe74de8 <col:8>
|-CXXConstructorDecl 0xbe742f0 <col:8> col:8 implicit constexpr hkVector4 'void (const hkVector4 &)' inline default trivial noexcept-unevaluated 0xbe742f0
| `-ParmVarDecl 0xbe74428 <col:8> col:8 'const hkVector4 &'
`-CXXConstructorDecl 0xbe744b0 <col:8> col:8 implicit constexpr hkVector4 'void (hkVector4 &&)' inline default trivial noexcept-unevaluated 0xbe744b0
  `-ParmVarDecl 0xbe745e8 <col:8> col:8 'hkVector4 &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkVector4 = type { float, float, float, float }
  NonVirtualBaseLLVMType:%struct.hkVector4 = type { float, float, float, float }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xbe6a758 <<source>:12:1, line:16:1> line:13:7 class hkEnum definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'EventMode'
| `-EnumType 0xbe5bae0 'EventMode'
|   `-Enum 0xbe5ba20 'EventMode'
|-TemplateArgument type 'char'
| `-BuiltinType 0xbde3b10 'char'
|-CXXRecordDecl 0xbe6a9d0 <col:1, col:7> col:7 implicit class hkEnum
|-AccessSpecDecl 0xbe6aa80 <line:14:4, col:11> col:4 private
|-FieldDecl 0xbe6aaf8 <line:15:5, col:7> col:7 storage 'char'
|-CXXMethodDecl 0xbe6b528 <line:13:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xbe6b528
| `-ParmVarDecl 0xbe6b658 <col:7> col:7 'const hkEnum<EventMode, char> &'
|-CXXMethodDecl 0xbe6b768 <col:7> col:7 implicit constexpr operator= 'hkEnum<EventMode, char> &(hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xbe6b768
| `-ParmVarDecl 0xbe6b898 <col:7> col:7 'hkEnum<EventMode, char> &&'
|-CXXDestructorDecl 0xbe6bfc0 <col:7> col:7 implicit constexpr ~hkEnum 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe74048 <col:7> col:7 implicit used constexpr hkEnum 'void () noexcept' inline default trivial
| `-CompoundStmt 0xbe74ea8 <col:7>
|-CXXConstructorDecl 0xbe74670 <col:7> col:7 implicit constexpr hkEnum 'void (const hkEnum<EventMode, char> &)' inline default trivial noexcept-unevaluated 0xbe74670
| `-ParmVarDecl 0xbe747a8 <col:7> col:7 'const hkEnum<EventMode, char> &'
`-CXXConstructorDecl 0xbe74830 <col:7> col:7 implicit constexpr hkEnum 'void (hkEnum<EventMode, char> &&)' inline default trivial noexcept-unevaluated 0xbe74830
  `-ParmVarDecl 0xbe74968 <col:7> col:7 'hkEnum<EventMode, char> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkEnum = type { i8 }
  NonVirtualBaseLLVMType:%class.hkEnum = type { i8 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe68098 <<source>:149:1, line:197:1> line:149:8 referenced struct hkbProjectData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xbe88170 <line:143:4, line:148:18>
| `-ParagraphComment 0xbe88140 <line:143:4, line:148:18>
|   |-TextComment 0xbe88050 <line:143:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xbe88070 <line:144:4, col:26> Text=" -      size: 32bit: 48"
|   |-TextComment 0xbe88090 <line:145:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xbe880b0 <line:146:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xbe880d0 <line:147:4, col:29> Text=" - signature: `0x13a39ba7`"
|   `-TextComment 0xbe880f0 <line:148:4, col:18> Text=" -   version: 2"
|-CXXRecordDecl 0xbe682a0 <line:149:1, col:8> col:8 implicit struct hkbProjectData
|-FieldDecl 0xbe68398 <line:177:5, col:15> col:15 worldUpWS 'hkVector4'
| `-FullComment 0xbe88300 <line:171:8, line:176:30>
|   |-ParagraphComment 0xbe88260 <line:171:8, line:173:22>
|   | |-TextComment 0xbe881e0 <line:171:8, col:71> Text=" The offset here is 16 bytes because Vector4 performs a 16-bytes"
|   | |-TextComment 0xbe88200 <line:172:8, col:68> Text=" alignment to account for SIMD operations using f32 * 4 = 128"
|   | `-TextComment 0xbe88220 <line:173:8, col:22> Text=" XMM registers."
|   `-ParagraphComment 0xbe882d0 <line:175:8, line:176:30>
|     |-TextComment 0xbe88280 <line:175:8, col:38> Text=" - offset: 32bit: 16, 64bit: 16"
|     `-TextComment 0xbe882a0 <line:176:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-CXXRecordDecl 0xbe68420 parent 0xbde3a68 <line:181:5, col:12> col:12 struct hkbProjectStringData
|-FieldDecl 0xbe685d0 <col:5, col:34> col:34 stringData 'struct hkbProjectStringData *'
| `-FullComment 0xbe88420 <line:178:8, line:180:38>
|   `-ParagraphComment 0xbe883f0 <line:178:8, line:180:38>
|     |-TextComment 0xbe88370 <line:178:8, col:38> Text=" - offset: 32bit: 32, 64bit: 32"
|     |-TextComment 0xbe88390 <line:179:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xbe883b0 <line:180:8, col:38> Text=" -   size: 32bit:  4, 64bit:  8"
|-FieldDecl 0xbe6ab68 <line:186:5, col:29> col:29 defaultEventMode 'hkEnum<EventMode, char>'
| `-FullComment 0xbe88560 <line:182:8, line:185:38>
|   `-ParagraphComment 0xbe88530 <line:182:8, line:185:38>
|     |-TextComment 0xbe88490 <line:182:8, col:20> Text=" - offset: 36"
|     |-TextComment 0xbe884b0 <line:183:8, col:38> Text=" - offset: 32bit: 36, 64bit: 40"
|     |-TextComment 0xbe884d0 <line:184:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|     `-TextComment 0xbe884f0 <line:185:8, col:38> Text=" -   size: 32bit:  1, 64bit:  1"
|-FieldDecl 0xbe6ac88 <line:193:5, col:18> col:10 _pad0 'char[11]'
|-CXXMethodDecl 0xbe6adf0 <line:149:8> col:8 implicit constexpr operator= 'hkbProjectData &(const hkbProjectData &)' inline default noexcept-unevaluated 0xbe6adf0
| `-ParmVarDecl 0xbe6af28 <col:8> col:8 'const hkbProjectData &'
|-CXXMethodDecl 0xbe6b988 <col:8> col:8 implicit constexpr operator= 'hkbProjectData &(hkbProjectData &&)' inline default noexcept-unevaluated 0xbe6b988
| `-ParmVarDecl 0xbe6bab8 <col:8> col:8 'hkbProjectData &&'
|-CXXDestructorDecl 0xbe6bb40 <col:8> col:8 implicit used constexpr ~hkbProjectData 'void () noexcept' inline default
| |-Overrides: [ 0xbe67ce0 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xbe6c118 <col:8>
|-CXXConstructorDecl 0xbe73c28 <col:8> col:8 implicit used constexpr hkbProjectData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xbe74d90 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe68398 'worldUpWS' 'hkVector4'
| | `-CXXConstructExpr 0xbe74e60 <col:8> 'hkVector4' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6ab68 'defaultEventMode' 'hkEnum<EventMode, char>'
| | `-CXXConstructExpr 0xbe74f20 <col:8> 'hkEnum<EventMode, char>' 'void () noexcept'
| `-CompoundStmt 0xbe74f80 <col:8>
|-CXXConstructorDecl 0xbe74140 <col:8> col:8 implicit constexpr hkbProjectData 'void (const hkbProjectData &)' inline default noexcept-unevaluated 0xbe74140
| `-ParmVarDecl 0xbe74278 <col:8> col:8 'const hkbProjectData &'
`-CXXConstructorDecl 0xbe74a00 <col:8> col:8 implicit constexpr hkbProjectData 'void (hkbProjectData &&)' inline default noexcept-unevaluated 0xbe74a00
  `-ParmVarDecl 0xbe74b38 <col:8> col:8 'hkbProjectData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, [8 x i8], %struct.hkVector4, ptr, %class.hkEnum, [11 x i8] }
  NonVirtualBaseLLVMType:%struct.hkbProjectData = type { %class.hkReferencedObject, [8 x i8], %struct.hkVector4, ptr, %class.hkEnum, [11 x i8] }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: ClassTemplateSpecializationDecl 0xbe6c540 <<source>:21:1, line:31:1> line:22:7 class hkArray definition implicit_instantiation
|-DefinitionData pass_in_registers standard_layout trivially_copyable trivial literal has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple trivial
| |-CopyAssignment simple trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple trivial
| `-Destructor simple irrelevant trivial constexpr
|-TemplateArgument type 'hkStringPtr'
| `-RecordType 0xbde4800 'hkStringPtr'
|   `-CXXRecord 0xbde4750 'hkStringPtr'
|-CXXRecordDecl 0xbe6c900 <col:1, col:7> col:7 implicit class hkArray
|-AccessSpecDecl 0xbe6c9b0 <line:23:4, col:11> col:4 private
|-FieldDecl 0xbe6ca88 <line:24:5, col:8> col:8 m_data 'hkStringPtr *'
|-FieldDecl 0xbe6cae0 <line:27:5, col:9> col:9 m_size 'int'
| `-FullComment 0xbe886c0 <line:25:8, col:78>
|   `-ParagraphComment 0xbe88690 <col:8, col:78>
|     `-TextComment 0xbe88660 <col:8, col:78> Text=" This is where it differs from a normal std::vector. len is not size_t."
|-FieldDecl 0xbe6cb38 <line:30:5, col:9> col:9 m_capacityAndFlags 'int'
| `-FullComment 0xbe887b0 <line:28:8, line:29:22>
|   `-ParagraphComment 0xbe88780 <line:28:8, line:29:22>
|     |-TextComment 0xbe88730 <line:28:8, col:79> Text=" highest 2 bits indicate any special considerations about the allocation"
|     `-TextComment 0xbe88750 <line:29:7, col:22> Text=" for the array. "
|-CXXMethodDecl 0xbe6d6a8 <line:22:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xbe6d6a8
| `-ParmVarDecl 0xbe6d7d8 <col:7> col:7 'const hkArray<hkStringPtr> &'
|-CXXMethodDecl 0xbe6dbd8 <col:7> col:7 implicit constexpr operator= 'hkArray<hkStringPtr> &(hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xbe6dbd8
| `-ParmVarDecl 0xbe6dd08 <col:7> col:7 'hkArray<hkStringPtr> &&'
|-CXXDestructorDecl 0xbe6e720 <col:7> col:7 implicit constexpr ~hkArray 'void () noexcept' inline default trivial
|-CXXConstructorDecl 0xbe75368 <col:7> col:7 implicit used constexpr hkArray 'void () noexcept' inline default trivial
| `-CompoundStmt 0xbe75c18 <col:7>
|-CXXConstructorDecl 0xbe75610 <col:7> col:7 implicit constexpr hkArray 'void (const hkArray<hkStringPtr> &)' inline default trivial noexcept-unevaluated 0xbe75610
| `-ParmVarDecl 0xbe75748 <col:7> col:7 'const hkArray<hkStringPtr> &'
`-CXXConstructorDecl 0xbe757d0 <col:7> col:7 implicit constexpr hkArray 'void (hkArray<hkStringPtr> &&)' inline default trivial noexcept-unevaluated 0xbe757d0
  `-ParmVarDecl 0xbe75908 <col:7> col:7 'hkArray<hkStringPtr> &&'

Layout: <CGRecordLayout
  LLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  NonVirtualBaseLLVMType:%class.hkArray.0 = type { ptr, i32, i32 }
  IsZeroInitializable:1
  BitFields:[
]>

*** Dumping IRgen Record Layout
Record: CXXRecordDecl 0xbe6c2b8 prev 0xbe68420 <<source>:215:1, line:268:1> line:215:8 referenced struct hkbProjectStringData definition
|-DefinitionData polymorphic has_constexpr_non_copy_move_ctor
| |-DefaultConstructor exists non_trivial constexpr defaulted_is_constexpr
| |-CopyConstructor simple non_trivial has_const_param implicit_has_const_param
| |-MoveConstructor exists simple non_trivial
| |-CopyAssignment simple non_trivial has_const_param implicit_has_const_param
| |-MoveAssignment exists simple non_trivial
| `-Destructor simple non_trivial constexpr
|-public 'hkReferencedObject'
|-FullComment 0xbe88ad0 <line:204:4, line:214:18>
| |-ParagraphComment 0xbe88910 <line:204:4, col:26>
| | `-TextComment 0xbe888e0 <col:4, col:26> Text=" `hkbProjectStringData`"
| |-ParagraphComment 0xbe88980 <line:206:4, line:207:69>
| | |-TextComment 0xbe88930 <line:206:4, col:63> Text=" - In C++, it represents the name of one field in the class."
| | `-TextComment 0xbe88950 <line:207:4, col:69> Text=" - In XML, the value of the `name` attribute of the `hkparam` tag."
| `-ParagraphComment 0xbe88a90 <line:209:4, line:214:18>
|   |-TextComment 0xbe889a0 <line:209:4, col:20> Text=" # C++ Class Info"
|   |-TextComment 0xbe889c0 <line:210:4, col:19> Text=" -      size: 76"
|   |-TextComment 0xbe889e0 <line:211:4, col:21> Text=" -    vtable: true"
|   |-TextComment 0xbe88a00 <line:212:4, col:50> Text=" -    parent: `hkReferencedObject`/`0x3b1c1113`"
|   |-TextComment 0xbe88a20 <line:213:4, col:28> Text=" - signature: `0x76ad60a`"
|   `-TextComment 0xbe88a40 <line:214:4, col:18> Text=" -   version: 1"
|-CXXRecordDecl 0xbe6c480 <line:215:1, col:8> col:8 implicit struct hkbProjectStringData
|-FieldDecl 0xbe6cba8 <line:241:5, col:26> col:26 animation_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xbe88bc0 <line:239:8, line:240:30>
|   `-ParagraphComment 0xbe88b90 <line:239:8, line:240:30>
|     |-TextComment 0xbe88b40 <line:239:8, col:38> Text=" - offset: 32bit:  8, 64bit: 16"
|     `-TextComment 0xbe88b60 <line:240:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6cd08 <line:245:5, col:26> col:26 behavior_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xbe88cb0 <line:243:8, line:244:30>
|   `-ParagraphComment 0xbe88c80 <line:243:8, line:244:30>
|     |-TextComment 0xbe88c30 <line:243:8, col:38> Text=" - offset: 32bit: 20, 64bit: 32"
|     `-TextComment 0xbe88c50 <line:244:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6ce68 <line:248:5, col:26> col:26 character_filenames 'hkArray<hkStringPtr>'
| `-FullComment 0xbe8cec0 <line:246:8, line:247:30>
|   `-ParagraphComment 0xbe8ce90 <line:246:8, line:247:30>
|     |-TextComment 0xbe8ce40 <line:246:8, col:38> Text=" - offset: 32bit: 32, 64bit: 48"
|     `-TextComment 0xbe8ce60 <line:247:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6cfc8 <line:251:5, col:26> col:26 event_names 'hkArray<hkStringPtr>'
| `-FullComment 0xbe8cfb0 <line:249:8, line:250:30>
|   `-ParagraphComment 0xbe8cf80 <line:249:8, line:250:30>
|     |-TextComment 0xbe8cf30 <line:249:8, col:38> Text=" - offset: 32bit: 44, 64bit: 64"
|     `-TextComment 0xbe8cf50 <line:250:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6d060 <line:255:5, col:17> col:17 animation_path 'hkStringPtr'
| `-FullComment 0xbe8d0a0 <line:253:8, line:254:30>
|   `-ParagraphComment 0xbe8d070 <line:253:8, line:254:30>
|     |-TextComment 0xbe8d020 <line:253:8, col:38> Text=" - offset: 32bit: 56, 64bit: 80"
|     `-TextComment 0xbe8d040 <line:254:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6d0f0 <line:258:5, col:17> col:17 behavior_path 'hkStringPtr'
| `-FullComment 0xbe8d190 <line:256:8, line:257:30>
|   `-ParagraphComment 0xbe8d160 <line:256:8, line:257:30>
|     |-TextComment 0xbe8d110 <line:256:8, col:38> Text=" - offset: 32bit: 60, 64bit: 88"
|     `-TextComment 0xbe8d130 <line:257:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6d180 <line:261:5, col:17> col:17 character_path 'hkStringPtr'
| `-FullComment 0xbe8d280 <line:259:8, line:260:30>
|   `-ParagraphComment 0xbe8d250 <line:259:8, line:260:30>
|     |-TextComment 0xbe8d200 <line:259:8, col:38> Text=" - offset: 32bit: 64, 64bit: 96"
|     `-TextComment 0xbe8d220 <line:260:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6d210 <line:264:5, col:17> col:17 full_path_to_source 'hkStringPtr'
| `-FullComment 0xbe8d370 <line:262:8, line:263:30>
|   `-ParagraphComment 0xbe8d340 <line:262:8, line:263:30>
|     |-TextComment 0xbe8d2f0 <line:262:8, col:39> Text=" - offset: 32bit: 68, 64bit: 104"
|     `-TextComment 0xbe8d310 <line:263:8, col:30> Text=" -  flags: `FLAGS_NONE`"
|-FieldDecl 0xbe6d2a0 <line:267:5, col:17> col:17 root_path 'hkStringPtr'
| `-FullComment 0xbe8d460 <line:265:8, line:266:48>
|   `-ParagraphComment 0xbe8d430 <line:265:8, line:266:48>
|     |-TextComment 0xbe8d3e0 <line:265:8, col:39> Text=" - offset: 32bit: 72, 64bit: 112"
|     `-TextComment 0xbe8d400 <line:266:8, col:48> Text=" -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`"
|-CXXMethodDecl 0xbe6d400 <line:215:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(const hkbProjectStringData &)' inline default noexcept-unevaluated 0xbe6d400
| `-ParmVarDecl 0xbe6d538 <col:8> col:8 'const hkbProjectStringData &'
|-CXXMethodDecl 0xbe6e288 <col:8> col:8 implicit constexpr operator= 'hkbProjectStringData &(hkbProjectStringData &&)' inline default noexcept-unevaluated 0xbe6e288
| `-ParmVarDecl 0xbe6e3b8 <col:8> col:8 'hkbProjectStringData &&'
|-CXXDestructorDecl 0xbe6e440 <col:8> col:8 implicit used constexpr ~hkbProjectStringData 'void () noexcept' inline default
| |-Overrides: [ 0xbe67ce0 hkReferencedObject::~hkReferencedObject 'void () noexcept' ]
| `-CompoundStmt 0xbe6e960 <col:8>
|-CXXConstructorDecl 0xbe75260 <col:8> col:8 implicit used constexpr hkbProjectStringData 'void () noexcept' inline default
| |-CXXCtorInitializer 'hkReferencedObject'
| | `-CXXConstructExpr 0xbe75bc0 <col:8> 'hkReferencedObject' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6cba8 'animation_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xbe77c10 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6cd08 'behavior_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xbe77c90 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6ce68 'character_filenames' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xbe77d10 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6cfc8 'event_names' 'hkArray<hkStringPtr>'
| | `-CXXConstructExpr 0xbe77d90 <col:8> 'hkArray<hkStringPtr>' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6d060 'animation_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xbe77e20 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6d0f0 'behavior_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xbe77e68 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6d180 'character_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xbe77eb0 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6d210 'full_path_to_source' 'hkStringPtr'
| | `-CXXConstructExpr 0xbe77ef8 <col:8> 'hkStringPtr' 'void () noexcept'
| |-CXXCtorInitializer Field 0xbe6d2a0 'root_path' 'hkStringPtr'
| | `-CXXConstructExpr 0xbe77f40 <col:8> 'hkStringPtr' 'void () noexcept'
| `-CompoundStmt 0xbe77fd8 <col:8>
|-CXXConstructorDecl 0xbe75460 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (const hkbProjectStringData &)' inline default noexcept-unevaluated 0xbe75460
| `-ParmVarDecl 0xbe75598 <col:8> col:8 'const hkbProjectStringData &'
`-CXXConstructorDecl 0xbe759a0 <col:8> col:8 implicit constexpr hkbProjectStringData 'void (hkbProjectStringData &&)' inline default noexcept-unevaluated 0xbe759a0
  `-ParmVarDecl 0xbe75ad8 <col:8> col:8 'hkbProjectStringData &&'

Layout: <CGRecordLayout
  LLVMType:%struct.hkbProjectStringData = type { %class.hkReferencedObject, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr }
  NonVirtualBaseLLVMType:%struct.hkbProjectStringData = type { %class.hkReferencedObject, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkArray.0, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr, %class.hkStringPtr }
  IsZeroInitializable:1
  BitFields:[
]>
```
