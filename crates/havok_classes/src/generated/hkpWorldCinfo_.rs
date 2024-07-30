use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpWorldCinfo`
/// - version: `11`
/// - signature: `0xa5255445`
/// - size: `240`(x86)/`256`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWorldCinfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `gravity`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_gravity: Vector4,
    /// # C++ Info
    /// - name: `broadPhaseQuerySize`(ctype: `hkInt32`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_broadPhaseQuerySize: i32,
    /// # C++ Info
    /// - name: `contactRestingVelocity`(ctype: `hkReal`)
    /// - offset: ` 36`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_contactRestingVelocity: f32,
    /// # C++ Info
    /// - name: `broadPhaseBorderBehaviour`(ctype: `enum BroadPhaseBorderBehaviour`)
    /// - offset: ` 40`(x86)/` 40`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_broadPhaseBorderBehaviour: BroadPhaseBorderBehaviour,
    /// # C++ Info
    /// - name: `mtPostponeAndSortBroadPhaseBorderCallbacks`(ctype: `hkBool`)
    /// - offset: ` 41`(x86)/` 41`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_mtPostponeAndSortBroadPhaseBorderCallbacks: bool,
    /// # C++ Info
    /// - name: `broadPhaseWorldAabb`(ctype: `struct hkAabb`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 32`(x86)/` 32`(x86_64)
    pub m_broadPhaseWorldAabb: hkAabb,
    /// # C++ Info
    /// - name: `useKdTree`(ctype: `hkBool`)
    /// - offset: ` 80`(x86)/` 80`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useKdTree: bool,
    /// # C++ Info
    /// - name: `useMultipleTree`(ctype: `hkBool`)
    /// - offset: ` 81`(x86)/` 81`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useMultipleTree: bool,
    /// # C++ Info
    /// - name: `treeUpdateType`(ctype: `enum TreeUpdateType`)
    /// - offset: ` 82`(x86)/` 82`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_treeUpdateType: TreeUpdateType,
    /// # C++ Info
    /// - name: `autoUpdateKdTree`(ctype: `hkBool`)
    /// - offset: ` 83`(x86)/` 83`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_autoUpdateKdTree: bool,
    /// # C++ Info
    /// - name: `collisionTolerance`(ctype: `hkReal`)
    /// - offset: ` 84`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_collisionTolerance: f32,
    /// # C++ Info
    /// - name: `collisionFilter`(ctype: `struct hkpCollisionFilter*`)
    /// - offset: ` 88`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_collisionFilter: Pointer,
    /// # C++ Info
    /// - name: `convexListFilter`(ctype: `struct hkpConvexListFilter*`)
    /// - offset: ` 92`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_convexListFilter: Pointer,
    /// # C++ Info
    /// - name: `expectedMaxLinearVelocity`(ctype: `hkReal`)
    /// - offset: ` 96`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_expectedMaxLinearVelocity: f32,
    /// # C++ Info
    /// - name: `sizeOfToiEventQueue`(ctype: `hkInt32`)
    /// - offset: `100`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_sizeOfToiEventQueue: i32,
    /// # C++ Info
    /// - name: `expectedMinPsiDeltaTime`(ctype: `hkReal`)
    /// - offset: `104`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_expectedMinPsiDeltaTime: f32,
    /// # C++ Info
    /// - name: `memoryWatchDog`(ctype: `struct hkWorldMemoryAvailableWatchDog*`)
    /// - offset: `108`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_memoryWatchDog: Pointer,
    /// # C++ Info
    /// - name: `broadPhaseNumMarkers`(ctype: `hkInt32`)
    /// - offset: `112`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_broadPhaseNumMarkers: i32,
    /// # C++ Info
    /// - name: `contactPointGeneration`(ctype: `enum ContactPointGeneration`)
    /// - offset: `116`(x86)/`132`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_contactPointGeneration: ContactPointGeneration,
    /// # C++ Info
    /// - name: `allowToSkipConfirmedCallbacks`(ctype: `hkBool`)
    /// - offset: `117`(x86)/`133`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_allowToSkipConfirmedCallbacks: bool,
    /// # C++ Info
    /// - name: `useHybridBroadphase`(ctype: `hkBool`)
    /// - offset: `118`(x86)/`134`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useHybridBroadphase: bool,
    /// # C++ Info
    /// - name: `solverTau`(ctype: `hkReal`)
    /// - offset: `120`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_solverTau: f32,
    /// # C++ Info
    /// - name: `solverDamp`(ctype: `hkReal`)
    /// - offset: `124`(x86)/`140`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_solverDamp: f32,
    /// # C++ Info
    /// - name: `solverIterations`(ctype: `hkInt32`)
    /// - offset: `128`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_solverIterations: i32,
    /// # C++ Info
    /// - name: `solverMicrosteps`(ctype: `hkInt32`)
    /// - offset: `132`(x86)/`148`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_solverMicrosteps: i32,
    /// # C++ Info
    /// - name: `maxConstraintViolation`(ctype: `hkReal`)
    /// - offset: `136`(x86)/`152`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxConstraintViolation: f32,
    /// # C++ Info
    /// - name: `forceCoherentConstraintOrderingInSolver`(ctype: `hkBool`)
    /// - offset: `140`(x86)/`156`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_forceCoherentConstraintOrderingInSolver: bool,
    /// # C++ Info
    /// - name: `snapCollisionToConvexEdgeThreshold`(ctype: `hkReal`)
    /// - offset: `144`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapCollisionToConvexEdgeThreshold: f32,
    /// # C++ Info
    /// - name: `snapCollisionToConcaveEdgeThreshold`(ctype: `hkReal`)
    /// - offset: `148`(x86)/`164`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapCollisionToConcaveEdgeThreshold: f32,
    /// # C++ Info
    /// - name: `enableToiWeldRejection`(ctype: `hkBool`)
    /// - offset: `152`(x86)/`168`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_enableToiWeldRejection: bool,
    /// # C++ Info
    /// - name: `enableDeprecatedWelding`(ctype: `hkBool`)
    /// - offset: `153`(x86)/`169`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_enableDeprecatedWelding: bool,
    /// # C++ Info
    /// - name: `iterativeLinearCastEarlyOutDistance`(ctype: `hkReal`)
    /// - offset: `156`(x86)/`172`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_iterativeLinearCastEarlyOutDistance: f32,
    /// # C++ Info
    /// - name: `iterativeLinearCastMaxIterations`(ctype: `hkInt32`)
    /// - offset: `160`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_iterativeLinearCastMaxIterations: i32,
    /// # C++ Info
    /// - name: `deactivationNumInactiveFramesSelectFlag0`(ctype: `hkUint8`)
    /// - offset: `164`(x86)/`180`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_deactivationNumInactiveFramesSelectFlag0: u8,
    /// # C++ Info
    /// - name: `deactivationNumInactiveFramesSelectFlag1`(ctype: `hkUint8`)
    /// - offset: `165`(x86)/`181`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_deactivationNumInactiveFramesSelectFlag1: u8,
    /// # C++ Info
    /// - name: `deactivationIntegrateCounter`(ctype: `hkUint8`)
    /// - offset: `166`(x86)/`182`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_deactivationIntegrateCounter: u8,
    /// # C++ Info
    /// - name: `shouldActivateOnRigidBodyTransformChange`(ctype: `hkBool`)
    /// - offset: `167`(x86)/`183`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_shouldActivateOnRigidBodyTransformChange: bool,
    /// # C++ Info
    /// - name: `deactivationReferenceDistance`(ctype: `hkReal`)
    /// - offset: `168`(x86)/`184`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_deactivationReferenceDistance: f32,
    /// # C++ Info
    /// - name: `toiCollisionResponseRotateNormal`(ctype: `hkReal`)
    /// - offset: `172`(x86)/`188`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_toiCollisionResponseRotateNormal: f32,
    /// # C++ Info
    /// - name: `maxSectorsPerMidphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `176`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxSectorsPerMidphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `maxSectorsPerNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `180`(x86)/`196`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxSectorsPerNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `processToisMultithreaded`(ctype: `hkBool`)
    /// - offset: `184`(x86)/`200`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_processToisMultithreaded: bool,
    /// # C++ Info
    /// - name: `maxEntriesPerToiMidphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `188`(x86)/`204`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxEntriesPerToiMidphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `maxEntriesPerToiNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `192`(x86)/`208`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxEntriesPerToiNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `maxNumToiCollisionPairsSinglethreaded`(ctype: `hkInt32`)
    /// - offset: `196`(x86)/`212`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxNumToiCollisionPairsSinglethreaded: i32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationSimplifiedToi`(ctype: `hkReal`)
    /// - offset: `200`(x86)/`216`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationSimplifiedToi: f32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationToi`(ctype: `hkReal`)
    /// - offset: `204`(x86)/`220`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationToi: f32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationToiHigher`(ctype: `hkReal`)
    /// - offset: `208`(x86)/`224`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationToiHigher: f32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationToiForced`(ctype: `hkReal`)
    /// - offset: `212`(x86)/`228`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationToiForced: f32,
    /// # C++ Info
    /// - name: `enableDeactivation`(ctype: `hkBool`)
    /// - offset: `216`(x86)/`232`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_enableDeactivation: bool,
    /// # C++ Info
    /// - name: `simulationType`(ctype: `enum SimulationType`)
    /// - offset: `217`(x86)/`233`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_simulationType: SimulationType,
    /// # C++ Info
    /// - name: `enableSimulationIslands`(ctype: `hkBool`)
    /// - offset: `218`(x86)/`234`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_enableSimulationIslands: bool,
    /// # C++ Info
    /// - name: `minDesiredIslandSize`(ctype: `hkUint32`)
    /// - offset: `220`(x86)/`236`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minDesiredIslandSize: u32,
    /// # C++ Info
    /// - name: `processActionsInSingleThread`(ctype: `hkBool`)
    /// - offset: `224`(x86)/`240`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_processActionsInSingleThread: bool,
    /// # C++ Info
    /// - name: `allowIntegrationOfIslandsWithoutConstraintsInASeparateJob`(ctype: `hkBool`)
    /// - offset: `225`(x86)/`241`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: bool,
    /// # C++ Info
    /// - name: `frameMarkerPsiSnap`(ctype: `hkReal`)
    /// - offset: `228`(x86)/`244`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_frameMarkerPsiSnap: f32,
    /// # C++ Info
    /// - name: `fireCollisionCallbacks`(ctype: `hkBool`)
    /// - offset: `232`(x86)/`248`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_fireCollisionCallbacks: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpWorldCinfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWorldCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa5255445)
        }
    }
    impl _serde::Serialize for hkpWorldCinfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa5255445)));
            let mut serializer = __serializer
                .serialize_struct("hkpWorldCinfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer
                .serialize_field("broadPhaseQuerySize", &self.m_broadPhaseQuerySize)?;
            serializer
                .serialize_field(
                    "contactRestingVelocity",
                    &self.m_contactRestingVelocity,
                )?;
            serializer
                .serialize_field(
                    "broadPhaseBorderBehaviour",
                    &self.m_broadPhaseBorderBehaviour,
                )?;
            serializer
                .serialize_field(
                    "mtPostponeAndSortBroadPhaseBorderCallbacks",
                    &self.m_mtPostponeAndSortBroadPhaseBorderCallbacks,
                )?;
            serializer.pad_field([0u8; 6usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field("broadPhaseWorldAabb", &self.m_broadPhaseWorldAabb)?;
            serializer.serialize_field("useKdTree", &self.m_useKdTree)?;
            serializer.serialize_field("useMultipleTree", &self.m_useMultipleTree)?;
            serializer.serialize_field("treeUpdateType", &self.m_treeUpdateType)?;
            serializer.serialize_field("autoUpdateKdTree", &self.m_autoUpdateKdTree)?;
            serializer
                .serialize_field("collisionTolerance", &self.m_collisionTolerance)?;
            serializer.serialize_field("collisionFilter", &self.m_collisionFilter)?;
            serializer.serialize_field("convexListFilter", &self.m_convexListFilter)?;
            serializer
                .serialize_field(
                    "expectedMaxLinearVelocity",
                    &self.m_expectedMaxLinearVelocity,
                )?;
            serializer
                .serialize_field("sizeOfToiEventQueue", &self.m_sizeOfToiEventQueue)?;
            serializer
                .serialize_field(
                    "expectedMinPsiDeltaTime",
                    &self.m_expectedMinPsiDeltaTime,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("memoryWatchDog", &self.m_memoryWatchDog)?;
            serializer
                .serialize_field("broadPhaseNumMarkers", &self.m_broadPhaseNumMarkers)?;
            serializer
                .serialize_field(
                    "contactPointGeneration",
                    &self.m_contactPointGeneration,
                )?;
            serializer
                .serialize_field(
                    "allowToSkipConfirmedCallbacks",
                    &self.m_allowToSkipConfirmedCallbacks,
                )?;
            serializer
                .serialize_field("useHybridBroadphase", &self.m_useHybridBroadphase)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("solverTau", &self.m_solverTau)?;
            serializer.serialize_field("solverDamp", &self.m_solverDamp)?;
            serializer.serialize_field("solverIterations", &self.m_solverIterations)?;
            serializer.serialize_field("solverMicrosteps", &self.m_solverMicrosteps)?;
            serializer
                .serialize_field(
                    "maxConstraintViolation",
                    &self.m_maxConstraintViolation,
                )?;
            serializer
                .serialize_field(
                    "forceCoherentConstraintOrderingInSolver",
                    &self.m_forceCoherentConstraintOrderingInSolver,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "snapCollisionToConvexEdgeThreshold",
                    &self.m_snapCollisionToConvexEdgeThreshold,
                )?;
            serializer
                .serialize_field(
                    "snapCollisionToConcaveEdgeThreshold",
                    &self.m_snapCollisionToConcaveEdgeThreshold,
                )?;
            serializer
                .serialize_field(
                    "enableToiWeldRejection",
                    &self.m_enableToiWeldRejection,
                )?;
            serializer
                .serialize_field(
                    "enableDeprecatedWelding",
                    &self.m_enableDeprecatedWelding,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "iterativeLinearCastEarlyOutDistance",
                    &self.m_iterativeLinearCastEarlyOutDistance,
                )?;
            serializer
                .serialize_field(
                    "iterativeLinearCastMaxIterations",
                    &self.m_iterativeLinearCastMaxIterations,
                )?;
            serializer
                .serialize_field(
                    "deactivationNumInactiveFramesSelectFlag0",
                    &self.m_deactivationNumInactiveFramesSelectFlag0,
                )?;
            serializer
                .serialize_field(
                    "deactivationNumInactiveFramesSelectFlag1",
                    &self.m_deactivationNumInactiveFramesSelectFlag1,
                )?;
            serializer
                .serialize_field(
                    "deactivationIntegrateCounter",
                    &self.m_deactivationIntegrateCounter,
                )?;
            serializer
                .serialize_field(
                    "shouldActivateOnRigidBodyTransformChange",
                    &self.m_shouldActivateOnRigidBodyTransformChange,
                )?;
            serializer
                .serialize_field(
                    "deactivationReferenceDistance",
                    &self.m_deactivationReferenceDistance,
                )?;
            serializer
                .serialize_field(
                    "toiCollisionResponseRotateNormal",
                    &self.m_toiCollisionResponseRotateNormal,
                )?;
            serializer
                .serialize_field(
                    "maxSectorsPerMidphaseCollideTask",
                    &self.m_maxSectorsPerMidphaseCollideTask,
                )?;
            serializer
                .serialize_field(
                    "maxSectorsPerNarrowphaseCollideTask",
                    &self.m_maxSectorsPerNarrowphaseCollideTask,
                )?;
            serializer
                .serialize_field(
                    "processToisMultithreaded",
                    &self.m_processToisMultithreaded,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "maxEntriesPerToiMidphaseCollideTask",
                    &self.m_maxEntriesPerToiMidphaseCollideTask,
                )?;
            serializer
                .serialize_field(
                    "maxEntriesPerToiNarrowphaseCollideTask",
                    &self.m_maxEntriesPerToiNarrowphaseCollideTask,
                )?;
            serializer
                .serialize_field(
                    "maxNumToiCollisionPairsSinglethreaded",
                    &self.m_maxNumToiCollisionPairsSinglethreaded,
                )?;
            serializer
                .serialize_field(
                    "numToisTillAllowedPenetrationSimplifiedToi",
                    &self.m_numToisTillAllowedPenetrationSimplifiedToi,
                )?;
            serializer
                .serialize_field(
                    "numToisTillAllowedPenetrationToi",
                    &self.m_numToisTillAllowedPenetrationToi,
                )?;
            serializer
                .serialize_field(
                    "numToisTillAllowedPenetrationToiHigher",
                    &self.m_numToisTillAllowedPenetrationToiHigher,
                )?;
            serializer
                .serialize_field(
                    "numToisTillAllowedPenetrationToiForced",
                    &self.m_numToisTillAllowedPenetrationToiForced,
                )?;
            serializer
                .serialize_field("enableDeactivation", &self.m_enableDeactivation)?;
            serializer.serialize_field("simulationType", &self.m_simulationType)?;
            serializer
                .serialize_field(
                    "enableSimulationIslands",
                    &self.m_enableSimulationIslands,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("minDesiredIslandSize", &self.m_minDesiredIslandSize)?;
            serializer
                .serialize_field(
                    "processActionsInSingleThread",
                    &self.m_processActionsInSingleThread,
                )?;
            serializer
                .serialize_field(
                    "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                    &self.m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field("frameMarkerPsiSnap", &self.m_frameMarkerPsiSnap)?;
            serializer
                .serialize_field(
                    "fireCollisionCallbacks",
                    &self.m_fireCollisionCallbacks,
                )?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpWorldCinfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_gravity,
                m_broadPhaseQuerySize,
                m_contactRestingVelocity,
                m_broadPhaseBorderBehaviour,
                m_mtPostponeAndSortBroadPhaseBorderCallbacks,
                m_broadPhaseWorldAabb,
                m_useKdTree,
                m_useMultipleTree,
                m_treeUpdateType,
                m_autoUpdateKdTree,
                m_collisionTolerance,
                m_collisionFilter,
                m_convexListFilter,
                m_expectedMaxLinearVelocity,
                m_sizeOfToiEventQueue,
                m_expectedMinPsiDeltaTime,
                m_memoryWatchDog,
                m_broadPhaseNumMarkers,
                m_contactPointGeneration,
                m_allowToSkipConfirmedCallbacks,
                m_useHybridBroadphase,
                m_solverTau,
                m_solverDamp,
                m_solverIterations,
                m_solverMicrosteps,
                m_maxConstraintViolation,
                m_forceCoherentConstraintOrderingInSolver,
                m_snapCollisionToConvexEdgeThreshold,
                m_snapCollisionToConcaveEdgeThreshold,
                m_enableToiWeldRejection,
                m_enableDeprecatedWelding,
                m_iterativeLinearCastEarlyOutDistance,
                m_iterativeLinearCastMaxIterations,
                m_deactivationNumInactiveFramesSelectFlag0,
                m_deactivationNumInactiveFramesSelectFlag1,
                m_deactivationIntegrateCounter,
                m_shouldActivateOnRigidBodyTransformChange,
                m_deactivationReferenceDistance,
                m_toiCollisionResponseRotateNormal,
                m_maxSectorsPerMidphaseCollideTask,
                m_maxSectorsPerNarrowphaseCollideTask,
                m_processToisMultithreaded,
                m_maxEntriesPerToiMidphaseCollideTask,
                m_maxEntriesPerToiNarrowphaseCollideTask,
                m_maxNumToiCollisionPairsSinglethreaded,
                m_numToisTillAllowedPenetrationSimplifiedToi,
                m_numToisTillAllowedPenetrationToi,
                m_numToisTillAllowedPenetrationToiHigher,
                m_numToisTillAllowedPenetrationToiForced,
                m_enableDeactivation,
                m_simulationType,
                m_enableSimulationIslands,
                m_minDesiredIslandSize,
                m_processActionsInSingleThread,
                m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                m_frameMarkerPsiSnap,
                m_fireCollisionCallbacks,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "gravity" => Ok(__Field::m_gravity),
                        "broadPhaseQuerySize" => Ok(__Field::m_broadPhaseQuerySize),
                        "contactRestingVelocity" => Ok(__Field::m_contactRestingVelocity),
                        "broadPhaseBorderBehaviour" => {
                            Ok(__Field::m_broadPhaseBorderBehaviour)
                        }
                        "mtPostponeAndSortBroadPhaseBorderCallbacks" => {
                            Ok(__Field::m_mtPostponeAndSortBroadPhaseBorderCallbacks)
                        }
                        "broadPhaseWorldAabb" => Ok(__Field::m_broadPhaseWorldAabb),
                        "useKdTree" => Ok(__Field::m_useKdTree),
                        "useMultipleTree" => Ok(__Field::m_useMultipleTree),
                        "treeUpdateType" => Ok(__Field::m_treeUpdateType),
                        "autoUpdateKdTree" => Ok(__Field::m_autoUpdateKdTree),
                        "collisionTolerance" => Ok(__Field::m_collisionTolerance),
                        "collisionFilter" => Ok(__Field::m_collisionFilter),
                        "convexListFilter" => Ok(__Field::m_convexListFilter),
                        "expectedMaxLinearVelocity" => {
                            Ok(__Field::m_expectedMaxLinearVelocity)
                        }
                        "sizeOfToiEventQueue" => Ok(__Field::m_sizeOfToiEventQueue),
                        "expectedMinPsiDeltaTime" => {
                            Ok(__Field::m_expectedMinPsiDeltaTime)
                        }
                        "memoryWatchDog" => Ok(__Field::m_memoryWatchDog),
                        "broadPhaseNumMarkers" => Ok(__Field::m_broadPhaseNumMarkers),
                        "contactPointGeneration" => Ok(__Field::m_contactPointGeneration),
                        "allowToSkipConfirmedCallbacks" => {
                            Ok(__Field::m_allowToSkipConfirmedCallbacks)
                        }
                        "useHybridBroadphase" => Ok(__Field::m_useHybridBroadphase),
                        "solverTau" => Ok(__Field::m_solverTau),
                        "solverDamp" => Ok(__Field::m_solverDamp),
                        "solverIterations" => Ok(__Field::m_solverIterations),
                        "solverMicrosteps" => Ok(__Field::m_solverMicrosteps),
                        "maxConstraintViolation" => Ok(__Field::m_maxConstraintViolation),
                        "forceCoherentConstraintOrderingInSolver" => {
                            Ok(__Field::m_forceCoherentConstraintOrderingInSolver)
                        }
                        "snapCollisionToConvexEdgeThreshold" => {
                            Ok(__Field::m_snapCollisionToConvexEdgeThreshold)
                        }
                        "snapCollisionToConcaveEdgeThreshold" => {
                            Ok(__Field::m_snapCollisionToConcaveEdgeThreshold)
                        }
                        "enableToiWeldRejection" => Ok(__Field::m_enableToiWeldRejection),
                        "enableDeprecatedWelding" => {
                            Ok(__Field::m_enableDeprecatedWelding)
                        }
                        "iterativeLinearCastEarlyOutDistance" => {
                            Ok(__Field::m_iterativeLinearCastEarlyOutDistance)
                        }
                        "iterativeLinearCastMaxIterations" => {
                            Ok(__Field::m_iterativeLinearCastMaxIterations)
                        }
                        "deactivationNumInactiveFramesSelectFlag0" => {
                            Ok(__Field::m_deactivationNumInactiveFramesSelectFlag0)
                        }
                        "deactivationNumInactiveFramesSelectFlag1" => {
                            Ok(__Field::m_deactivationNumInactiveFramesSelectFlag1)
                        }
                        "deactivationIntegrateCounter" => {
                            Ok(__Field::m_deactivationIntegrateCounter)
                        }
                        "shouldActivateOnRigidBodyTransformChange" => {
                            Ok(__Field::m_shouldActivateOnRigidBodyTransformChange)
                        }
                        "deactivationReferenceDistance" => {
                            Ok(__Field::m_deactivationReferenceDistance)
                        }
                        "toiCollisionResponseRotateNormal" => {
                            Ok(__Field::m_toiCollisionResponseRotateNormal)
                        }
                        "maxSectorsPerMidphaseCollideTask" => {
                            Ok(__Field::m_maxSectorsPerMidphaseCollideTask)
                        }
                        "maxSectorsPerNarrowphaseCollideTask" => {
                            Ok(__Field::m_maxSectorsPerNarrowphaseCollideTask)
                        }
                        "processToisMultithreaded" => {
                            Ok(__Field::m_processToisMultithreaded)
                        }
                        "maxEntriesPerToiMidphaseCollideTask" => {
                            Ok(__Field::m_maxEntriesPerToiMidphaseCollideTask)
                        }
                        "maxEntriesPerToiNarrowphaseCollideTask" => {
                            Ok(__Field::m_maxEntriesPerToiNarrowphaseCollideTask)
                        }
                        "maxNumToiCollisionPairsSinglethreaded" => {
                            Ok(__Field::m_maxNumToiCollisionPairsSinglethreaded)
                        }
                        "numToisTillAllowedPenetrationSimplifiedToi" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationSimplifiedToi)
                        }
                        "numToisTillAllowedPenetrationToi" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationToi)
                        }
                        "numToisTillAllowedPenetrationToiHigher" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationToiHigher)
                        }
                        "numToisTillAllowedPenetrationToiForced" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationToiForced)
                        }
                        "enableDeactivation" => Ok(__Field::m_enableDeactivation),
                        "simulationType" => Ok(__Field::m_simulationType),
                        "enableSimulationIslands" => {
                            Ok(__Field::m_enableSimulationIslands)
                        }
                        "minDesiredIslandSize" => Ok(__Field::m_minDesiredIslandSize),
                        "processActionsInSingleThread" => {
                            Ok(__Field::m_processActionsInSingleThread)
                        }
                        "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => {
                            Ok(
                                __Field::m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                            )
                        }
                        "frameMarkerPsiSnap" => Ok(__Field::m_frameMarkerPsiSnap),
                        "fireCollisionCallbacks" => Ok(__Field::m_fireCollisionCallbacks),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpWorldCinfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpWorldCinfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpWorldCinfoVisitor<'de> {
                type Value = hkpWorldCinfo;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkpWorldCinfo")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_broadPhaseQuerySize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_contactRestingVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_broadPhaseBorderBehaviour: _serde::__private::Option<
                        BroadPhaseBorderBehaviour,
                    > = _serde::__private::None;
                    let mut m_mtPostponeAndSortBroadPhaseBorderCallbacks: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_broadPhaseWorldAabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_useKdTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useMultipleTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_treeUpdateType: _serde::__private::Option<
                        TreeUpdateType,
                    > = _serde::__private::None;
                    let mut m_autoUpdateKdTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilter: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_convexListFilter: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_expectedMaxLinearVelocity: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_sizeOfToiEventQueue: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_expectedMinPsiDeltaTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_memoryWatchDog: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_broadPhaseNumMarkers: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_contactPointGeneration: _serde::__private::Option<
                        ContactPointGeneration,
                    > = _serde::__private::None;
                    let mut m_allowToSkipConfirmedCallbacks: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_useHybridBroadphase: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_solverTau: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverDamp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverIterations: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_solverMicrosteps: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_maxConstraintViolation: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_forceCoherentConstraintOrderingInSolver: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_snapCollisionToConvexEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_snapCollisionToConcaveEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableToiWeldRejection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_enableDeprecatedWelding: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_iterativeLinearCastEarlyOutDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_iterativeLinearCastMaxIterations: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_deactivationNumInactiveFramesSelectFlag0: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_deactivationNumInactiveFramesSelectFlag1: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_deactivationIntegrateCounter: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_shouldActivateOnRigidBodyTransformChange: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_deactivationReferenceDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_toiCollisionResponseRotateNormal: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxSectorsPerMidphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxSectorsPerNarrowphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_processToisMultithreaded: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_maxEntriesPerToiMidphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxEntriesPerToiNarrowphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxNumToiCollisionPairsSinglethreaded: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationSimplifiedToi: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToi: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToiHigher: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToiForced: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_simulationType: _serde::__private::Option<
                        SimulationType,
                    > = _serde::__private::None;
                    let mut m_enableSimulationIslands: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_minDesiredIslandSize: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_processActionsInSingleThread: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_frameMarkerPsiSnap: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fireCollisionCallbacks: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..57usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_gravity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravity",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 0usize)?;
                                m_gravity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseQuerySize,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseQuerySize",
                                        ),
                                    );
                                }
                                m_broadPhaseQuerySize = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_contactRestingVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactRestingVelocity",
                                        ),
                                    );
                                }
                                m_contactRestingVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseBorderBehaviour,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseBorderBehaviour",
                                        ),
                                    );
                                }
                                m_broadPhaseBorderBehaviour = _serde::__private::Some(
                                    match __A::next_value::<
                                        BroadPhaseBorderBehaviour,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mtPostponeAndSortBroadPhaseBorderCallbacks,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mtPostponeAndSortBroadPhaseBorderCallbacks",
                                        ),
                                    );
                                }
                                m_mtPostponeAndSortBroadPhaseBorderCallbacks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseWorldAabb,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseWorldAabb",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 6usize, 6usize)?;
                                m_broadPhaseWorldAabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_useKdTree) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useKdTree",
                                        ),
                                    );
                                }
                                m_useKdTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_useMultipleTree) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useMultipleTree",
                                        ),
                                    );
                                }
                                m_useMultipleTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_treeUpdateType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "treeUpdateType",
                                        ),
                                    );
                                }
                                m_treeUpdateType = _serde::__private::Some(
                                    match __A::next_value::<TreeUpdateType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_autoUpdateKdTree) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "autoUpdateKdTree",
                                        ),
                                    );
                                }
                                m_autoUpdateKdTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionTolerance",
                                        ),
                                    );
                                }
                                m_collisionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_collisionFilter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilter",
                                        ),
                                    );
                                }
                                m_collisionFilter = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_convexListFilter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexListFilter",
                                        ),
                                    );
                                }
                                m_convexListFilter = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_expectedMaxLinearVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expectedMaxLinearVelocity",
                                        ),
                                    );
                                }
                                m_expectedMaxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sizeOfToiEventQueue,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sizeOfToiEventQueue",
                                        ),
                                    );
                                }
                                m_sizeOfToiEventQueue = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_expectedMinPsiDeltaTime,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expectedMinPsiDeltaTime",
                                        ),
                                    );
                                }
                                m_expectedMinPsiDeltaTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_memoryWatchDog) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memoryWatchDog",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_memoryWatchDog = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseNumMarkers,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseNumMarkers",
                                        ),
                                    );
                                }
                                m_broadPhaseNumMarkers = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_contactPointGeneration,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactPointGeneration",
                                        ),
                                    );
                                }
                                m_contactPointGeneration = _serde::__private::Some(
                                    match __A::next_value::<
                                        ContactPointGeneration,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allowToSkipConfirmedCallbacks,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowToSkipConfirmedCallbacks",
                                        ),
                                    );
                                }
                                m_allowToSkipConfirmedCallbacks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useHybridBroadphase,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useHybridBroadphase",
                                        ),
                                    );
                                }
                                m_useHybridBroadphase = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_solverTau) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverTau",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 1usize)?;
                                m_solverTau = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_solverDamp) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverDamp",
                                        ),
                                    );
                                }
                                m_solverDamp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_solverIterations) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverIterations",
                                        ),
                                    );
                                }
                                m_solverIterations = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(&m_solverMicrosteps) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverMicrosteps",
                                        ),
                                    );
                                }
                                m_solverMicrosteps = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxConstraintViolation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxConstraintViolation",
                                        ),
                                    );
                                }
                                m_maxConstraintViolation = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(
                                    &m_forceCoherentConstraintOrderingInSolver,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forceCoherentConstraintOrderingInSolver",
                                        ),
                                    );
                                }
                                m_forceCoherentConstraintOrderingInSolver = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapCollisionToConvexEdgeThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapCollisionToConvexEdgeThreshold",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_snapCollisionToConvexEdgeThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapCollisionToConcaveEdgeThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapCollisionToConcaveEdgeThreshold",
                                        ),
                                    );
                                }
                                m_snapCollisionToConcaveEdgeThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableToiWeldRejection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableToiWeldRejection",
                                        ),
                                    );
                                }
                                m_enableToiWeldRejection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableDeprecatedWelding,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeprecatedWelding",
                                        ),
                                    );
                                }
                                m_enableDeprecatedWelding = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(
                                    &m_iterativeLinearCastEarlyOutDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "iterativeLinearCastEarlyOutDistance",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_iterativeLinearCastEarlyOutDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(
                                    &m_iterativeLinearCastMaxIterations,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "iterativeLinearCastMaxIterations",
                                        ),
                                    );
                                }
                                m_iterativeLinearCastMaxIterations = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationNumInactiveFramesSelectFlag0,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationNumInactiveFramesSelectFlag0",
                                        ),
                                    );
                                }
                                m_deactivationNumInactiveFramesSelectFlag0 = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            34usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationNumInactiveFramesSelectFlag1,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationNumInactiveFramesSelectFlag1",
                                        ),
                                    );
                                }
                                m_deactivationNumInactiveFramesSelectFlag1 = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationIntegrateCounter,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationIntegrateCounter",
                                        ),
                                    );
                                }
                                m_deactivationIntegrateCounter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            36usize => {
                                if _serde::__private::Option::is_some(
                                    &m_shouldActivateOnRigidBodyTransformChange,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shouldActivateOnRigidBodyTransformChange",
                                        ),
                                    );
                                }
                                m_shouldActivateOnRigidBodyTransformChange = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            37usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationReferenceDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationReferenceDistance",
                                        ),
                                    );
                                }
                                m_deactivationReferenceDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            38usize => {
                                if _serde::__private::Option::is_some(
                                    &m_toiCollisionResponseRotateNormal,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toiCollisionResponseRotateNormal",
                                        ),
                                    );
                                }
                                m_toiCollisionResponseRotateNormal = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            39usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxSectorsPerMidphaseCollideTask,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSectorsPerMidphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxSectorsPerMidphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            40usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxSectorsPerNarrowphaseCollideTask,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSectorsPerNarrowphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxSectorsPerNarrowphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            41usize => {
                                if _serde::__private::Option::is_some(
                                    &m_processToisMultithreaded,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "processToisMultithreaded",
                                        ),
                                    );
                                }
                                m_processToisMultithreaded = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            42usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxEntriesPerToiMidphaseCollideTask,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxEntriesPerToiMidphaseCollideTask",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_maxEntriesPerToiMidphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            43usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxEntriesPerToiNarrowphaseCollideTask,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxEntriesPerToiNarrowphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxEntriesPerToiNarrowphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            44usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxNumToiCollisionPairsSinglethreaded,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxNumToiCollisionPairsSinglethreaded",
                                        ),
                                    );
                                }
                                m_maxNumToiCollisionPairsSinglethreaded = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            45usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationSimplifiedToi,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationSimplifiedToi",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationSimplifiedToi = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            46usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationToi,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationToi",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationToi = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            47usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationToiHigher,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationToiHigher",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationToiHigher = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            48usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationToiForced,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationToiForced",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationToiForced = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            49usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableDeactivation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeactivation",
                                        ),
                                    );
                                }
                                m_enableDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            50usize => {
                                if _serde::__private::Option::is_some(&m_simulationType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationType",
                                        ),
                                    );
                                }
                                m_simulationType = _serde::__private::Some(
                                    match __A::next_value::<SimulationType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            51usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableSimulationIslands,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableSimulationIslands",
                                        ),
                                    );
                                }
                                m_enableSimulationIslands = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            52usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minDesiredIslandSize,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minDesiredIslandSize",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 1usize)?;
                                m_minDesiredIslandSize = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            53usize => {
                                if _serde::__private::Option::is_some(
                                    &m_processActionsInSingleThread,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "processActionsInSingleThread",
                                        ),
                                    );
                                }
                                m_processActionsInSingleThread = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            54usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                                        ),
                                    );
                                }
                                m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            55usize => {
                                if _serde::__private::Option::is_some(
                                    &m_frameMarkerPsiSnap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameMarkerPsiSnap",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_frameMarkerPsiSnap = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            56usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fireCollisionCallbacks,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fireCollisionCallbacks",
                                        ),
                                    );
                                }
                                m_fireCollisionCallbacks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 7usize, 7usize)?;
                    let m_gravity = match m_gravity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gravity"),
                            );
                        }
                    };
                    let m_broadPhaseQuerySize = match m_broadPhaseQuerySize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseQuerySize",
                                ),
                            );
                        }
                    };
                    let m_contactRestingVelocity = match m_contactRestingVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactRestingVelocity",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseBorderBehaviour = match m_broadPhaseBorderBehaviour {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseBorderBehaviour",
                                ),
                            );
                        }
                    };
                    let m_mtPostponeAndSortBroadPhaseBorderCallbacks = match m_mtPostponeAndSortBroadPhaseBorderCallbacks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mtPostponeAndSortBroadPhaseBorderCallbacks",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseWorldAabb = match m_broadPhaseWorldAabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseWorldAabb",
                                ),
                            );
                        }
                    };
                    let m_useKdTree = match m_useKdTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useKdTree",
                                ),
                            );
                        }
                    };
                    let m_useMultipleTree = match m_useMultipleTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useMultipleTree",
                                ),
                            );
                        }
                    };
                    let m_treeUpdateType = match m_treeUpdateType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "treeUpdateType",
                                ),
                            );
                        }
                    };
                    let m_autoUpdateKdTree = match m_autoUpdateKdTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "autoUpdateKdTree",
                                ),
                            );
                        }
                    };
                    let m_collisionTolerance = match m_collisionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionTolerance",
                                ),
                            );
                        }
                    };
                    let m_collisionFilter = match m_collisionFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilter",
                                ),
                            );
                        }
                    };
                    let m_convexListFilter = match m_convexListFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexListFilter",
                                ),
                            );
                        }
                    };
                    let m_expectedMaxLinearVelocity = match m_expectedMaxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expectedMaxLinearVelocity",
                                ),
                            );
                        }
                    };
                    let m_sizeOfToiEventQueue = match m_sizeOfToiEventQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sizeOfToiEventQueue",
                                ),
                            );
                        }
                    };
                    let m_expectedMinPsiDeltaTime = match m_expectedMinPsiDeltaTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expectedMinPsiDeltaTime",
                                ),
                            );
                        }
                    };
                    let m_memoryWatchDog = match m_memoryWatchDog {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memoryWatchDog",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseNumMarkers = match m_broadPhaseNumMarkers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseNumMarkers",
                                ),
                            );
                        }
                    };
                    let m_contactPointGeneration = match m_contactPointGeneration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactPointGeneration",
                                ),
                            );
                        }
                    };
                    let m_allowToSkipConfirmedCallbacks = match m_allowToSkipConfirmedCallbacks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowToSkipConfirmedCallbacks",
                                ),
                            );
                        }
                    };
                    let m_useHybridBroadphase = match m_useHybridBroadphase {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useHybridBroadphase",
                                ),
                            );
                        }
                    };
                    let m_solverTau = match m_solverTau {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverTau",
                                ),
                            );
                        }
                    };
                    let m_solverDamp = match m_solverDamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverDamp",
                                ),
                            );
                        }
                    };
                    let m_solverIterations = match m_solverIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverIterations",
                                ),
                            );
                        }
                    };
                    let m_solverMicrosteps = match m_solverMicrosteps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverMicrosteps",
                                ),
                            );
                        }
                    };
                    let m_maxConstraintViolation = match m_maxConstraintViolation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxConstraintViolation",
                                ),
                            );
                        }
                    };
                    let m_forceCoherentConstraintOrderingInSolver = match m_forceCoherentConstraintOrderingInSolver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forceCoherentConstraintOrderingInSolver",
                                ),
                            );
                        }
                    };
                    let m_snapCollisionToConvexEdgeThreshold = match m_snapCollisionToConvexEdgeThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapCollisionToConvexEdgeThreshold",
                                ),
                            );
                        }
                    };
                    let m_snapCollisionToConcaveEdgeThreshold = match m_snapCollisionToConcaveEdgeThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapCollisionToConcaveEdgeThreshold",
                                ),
                            );
                        }
                    };
                    let m_enableToiWeldRejection = match m_enableToiWeldRejection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableToiWeldRejection",
                                ),
                            );
                        }
                    };
                    let m_enableDeprecatedWelding = match m_enableDeprecatedWelding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeprecatedWelding",
                                ),
                            );
                        }
                    };
                    let m_iterativeLinearCastEarlyOutDistance = match m_iterativeLinearCastEarlyOutDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "iterativeLinearCastEarlyOutDistance",
                                ),
                            );
                        }
                    };
                    let m_iterativeLinearCastMaxIterations = match m_iterativeLinearCastMaxIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "iterativeLinearCastMaxIterations",
                                ),
                            );
                        }
                    };
                    let m_deactivationNumInactiveFramesSelectFlag0 = match m_deactivationNumInactiveFramesSelectFlag0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationNumInactiveFramesSelectFlag0",
                                ),
                            );
                        }
                    };
                    let m_deactivationNumInactiveFramesSelectFlag1 = match m_deactivationNumInactiveFramesSelectFlag1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationNumInactiveFramesSelectFlag1",
                                ),
                            );
                        }
                    };
                    let m_deactivationIntegrateCounter = match m_deactivationIntegrateCounter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationIntegrateCounter",
                                ),
                            );
                        }
                    };
                    let m_shouldActivateOnRigidBodyTransformChange = match m_shouldActivateOnRigidBodyTransformChange {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shouldActivateOnRigidBodyTransformChange",
                                ),
                            );
                        }
                    };
                    let m_deactivationReferenceDistance = match m_deactivationReferenceDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationReferenceDistance",
                                ),
                            );
                        }
                    };
                    let m_toiCollisionResponseRotateNormal = match m_toiCollisionResponseRotateNormal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toiCollisionResponseRotateNormal",
                                ),
                            );
                        }
                    };
                    let m_maxSectorsPerMidphaseCollideTask = match m_maxSectorsPerMidphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxSectorsPerMidphaseCollideTask",
                                ),
                            );
                        }
                    };
                    let m_maxSectorsPerNarrowphaseCollideTask = match m_maxSectorsPerNarrowphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxSectorsPerNarrowphaseCollideTask",
                                ),
                            );
                        }
                    };
                    let m_processToisMultithreaded = match m_processToisMultithreaded {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "processToisMultithreaded",
                                ),
                            );
                        }
                    };
                    let m_maxEntriesPerToiMidphaseCollideTask = match m_maxEntriesPerToiMidphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxEntriesPerToiMidphaseCollideTask",
                                ),
                            );
                        }
                    };
                    let m_maxEntriesPerToiNarrowphaseCollideTask = match m_maxEntriesPerToiNarrowphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxEntriesPerToiNarrowphaseCollideTask",
                                ),
                            );
                        }
                    };
                    let m_maxNumToiCollisionPairsSinglethreaded = match m_maxNumToiCollisionPairsSinglethreaded {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxNumToiCollisionPairsSinglethreaded",
                                ),
                            );
                        }
                    };
                    let m_numToisTillAllowedPenetrationSimplifiedToi = match m_numToisTillAllowedPenetrationSimplifiedToi {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationSimplifiedToi",
                                ),
                            );
                        }
                    };
                    let m_numToisTillAllowedPenetrationToi = match m_numToisTillAllowedPenetrationToi {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationToi",
                                ),
                            );
                        }
                    };
                    let m_numToisTillAllowedPenetrationToiHigher = match m_numToisTillAllowedPenetrationToiHigher {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationToiHigher",
                                ),
                            );
                        }
                    };
                    let m_numToisTillAllowedPenetrationToiForced = match m_numToisTillAllowedPenetrationToiForced {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationToiForced",
                                ),
                            );
                        }
                    };
                    let m_enableDeactivation = match m_enableDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeactivation",
                                ),
                            );
                        }
                    };
                    let m_simulationType = match m_simulationType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationType",
                                ),
                            );
                        }
                    };
                    let m_enableSimulationIslands = match m_enableSimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableSimulationIslands",
                                ),
                            );
                        }
                    };
                    let m_minDesiredIslandSize = match m_minDesiredIslandSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minDesiredIslandSize",
                                ),
                            );
                        }
                    };
                    let m_processActionsInSingleThread = match m_processActionsInSingleThread {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "processActionsInSingleThread",
                                ),
                            );
                        }
                    };
                    let m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob = match m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                                ),
                            );
                        }
                    };
                    let m_frameMarkerPsiSnap = match m_frameMarkerPsiSnap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frameMarkerPsiSnap",
                                ),
                            );
                        }
                    };
                    let m_fireCollisionCallbacks = match m_fireCollisionCallbacks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fireCollisionCallbacks",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpWorldCinfo {
                        __ptr,
                        parent,
                        m_gravity,
                        m_broadPhaseQuerySize,
                        m_contactRestingVelocity,
                        m_broadPhaseBorderBehaviour,
                        m_mtPostponeAndSortBroadPhaseBorderCallbacks,
                        m_broadPhaseWorldAabb,
                        m_useKdTree,
                        m_useMultipleTree,
                        m_treeUpdateType,
                        m_autoUpdateKdTree,
                        m_collisionTolerance,
                        m_collisionFilter,
                        m_convexListFilter,
                        m_expectedMaxLinearVelocity,
                        m_sizeOfToiEventQueue,
                        m_expectedMinPsiDeltaTime,
                        m_memoryWatchDog,
                        m_broadPhaseNumMarkers,
                        m_contactPointGeneration,
                        m_allowToSkipConfirmedCallbacks,
                        m_useHybridBroadphase,
                        m_solverTau,
                        m_solverDamp,
                        m_solverIterations,
                        m_solverMicrosteps,
                        m_maxConstraintViolation,
                        m_forceCoherentConstraintOrderingInSolver,
                        m_snapCollisionToConvexEdgeThreshold,
                        m_snapCollisionToConcaveEdgeThreshold,
                        m_enableToiWeldRejection,
                        m_enableDeprecatedWelding,
                        m_iterativeLinearCastEarlyOutDistance,
                        m_iterativeLinearCastMaxIterations,
                        m_deactivationNumInactiveFramesSelectFlag0,
                        m_deactivationNumInactiveFramesSelectFlag1,
                        m_deactivationIntegrateCounter,
                        m_shouldActivateOnRigidBodyTransformChange,
                        m_deactivationReferenceDistance,
                        m_toiCollisionResponseRotateNormal,
                        m_maxSectorsPerMidphaseCollideTask,
                        m_maxSectorsPerNarrowphaseCollideTask,
                        m_processToisMultithreaded,
                        m_maxEntriesPerToiMidphaseCollideTask,
                        m_maxEntriesPerToiNarrowphaseCollideTask,
                        m_maxNumToiCollisionPairsSinglethreaded,
                        m_numToisTillAllowedPenetrationSimplifiedToi,
                        m_numToisTillAllowedPenetrationToi,
                        m_numToisTillAllowedPenetrationToiHigher,
                        m_numToisTillAllowedPenetrationToiForced,
                        m_enableDeactivation,
                        m_simulationType,
                        m_enableSimulationIslands,
                        m_minDesiredIslandSize,
                        m_processActionsInSingleThread,
                        m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                        m_frameMarkerPsiSnap,
                        m_fireCollisionCallbacks,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_broadPhaseQuerySize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_contactRestingVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_broadPhaseBorderBehaviour: _serde::__private::Option<
                        BroadPhaseBorderBehaviour,
                    > = _serde::__private::None;
                    let mut m_mtPostponeAndSortBroadPhaseBorderCallbacks: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_broadPhaseWorldAabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_useKdTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useMultipleTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_treeUpdateType: _serde::__private::Option<
                        TreeUpdateType,
                    > = _serde::__private::None;
                    let mut m_autoUpdateKdTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_collisionTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilter: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_convexListFilter: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_expectedMaxLinearVelocity: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_sizeOfToiEventQueue: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_expectedMinPsiDeltaTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_memoryWatchDog: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_broadPhaseNumMarkers: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_contactPointGeneration: _serde::__private::Option<
                        ContactPointGeneration,
                    > = _serde::__private::None;
                    let mut m_allowToSkipConfirmedCallbacks: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_useHybridBroadphase: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_solverTau: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverDamp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverIterations: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_solverMicrosteps: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_maxConstraintViolation: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_forceCoherentConstraintOrderingInSolver: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_snapCollisionToConvexEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_snapCollisionToConcaveEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableToiWeldRejection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_enableDeprecatedWelding: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_iterativeLinearCastEarlyOutDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_iterativeLinearCastMaxIterations: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_deactivationNumInactiveFramesSelectFlag0: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_deactivationNumInactiveFramesSelectFlag1: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_deactivationIntegrateCounter: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_shouldActivateOnRigidBodyTransformChange: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_deactivationReferenceDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_toiCollisionResponseRotateNormal: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxSectorsPerMidphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxSectorsPerNarrowphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_processToisMultithreaded: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_maxEntriesPerToiMidphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxEntriesPerToiNarrowphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxNumToiCollisionPairsSinglethreaded: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationSimplifiedToi: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToi: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToiHigher: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToiForced: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_simulationType: _serde::__private::Option<
                        SimulationType,
                    > = _serde::__private::None;
                    let mut m_enableSimulationIslands: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_minDesiredIslandSize: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_processActionsInSingleThread: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_frameMarkerPsiSnap: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fireCollisionCallbacks: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_gravity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gravity) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravity",
                                        ),
                                    );
                                }
                                m_gravity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseQuerySize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseQuerySize,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseQuerySize",
                                        ),
                                    );
                                }
                                m_broadPhaseQuerySize = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_contactRestingVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_contactRestingVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactRestingVelocity",
                                        ),
                                    );
                                }
                                m_contactRestingVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseBorderBehaviour => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseBorderBehaviour,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseBorderBehaviour",
                                        ),
                                    );
                                }
                                m_broadPhaseBorderBehaviour = _serde::__private::Some(
                                    match __A::next_value::<
                                        BroadPhaseBorderBehaviour,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_mtPostponeAndSortBroadPhaseBorderCallbacks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_mtPostponeAndSortBroadPhaseBorderCallbacks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mtPostponeAndSortBroadPhaseBorderCallbacks",
                                        ),
                                    );
                                }
                                m_mtPostponeAndSortBroadPhaseBorderCallbacks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseWorldAabb => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseWorldAabb,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseWorldAabb",
                                        ),
                                    );
                                }
                                m_broadPhaseWorldAabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_useKdTree => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_useKdTree) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useKdTree",
                                        ),
                                    );
                                }
                                m_useKdTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_useMultipleTree => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_useMultipleTree) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useMultipleTree",
                                        ),
                                    );
                                }
                                m_useMultipleTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_treeUpdateType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_treeUpdateType) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "treeUpdateType",
                                        ),
                                    );
                                }
                                m_treeUpdateType = _serde::__private::Some(
                                    match __A::next_value::<TreeUpdateType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_autoUpdateKdTree => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_autoUpdateKdTree) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "autoUpdateKdTree",
                                        ),
                                    );
                                }
                                m_autoUpdateKdTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionTolerance",
                                        ),
                                    );
                                }
                                m_collisionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionFilter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_collisionFilter) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilter",
                                        ),
                                    );
                                }
                                m_collisionFilter = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_convexListFilter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_convexListFilter) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexListFilter",
                                        ),
                                    );
                                }
                                m_convexListFilter = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_expectedMaxLinearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_expectedMaxLinearVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expectedMaxLinearVelocity",
                                        ),
                                    );
                                }
                                m_expectedMaxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_sizeOfToiEventQueue => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sizeOfToiEventQueue,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sizeOfToiEventQueue",
                                        ),
                                    );
                                }
                                m_sizeOfToiEventQueue = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_expectedMinPsiDeltaTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_expectedMinPsiDeltaTime,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expectedMinPsiDeltaTime",
                                        ),
                                    );
                                }
                                m_expectedMinPsiDeltaTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_memoryWatchDog => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_memoryWatchDog) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memoryWatchDog",
                                        ),
                                    );
                                }
                                m_memoryWatchDog = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseNumMarkers => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseNumMarkers,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseNumMarkers",
                                        ),
                                    );
                                }
                                m_broadPhaseNumMarkers = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_contactPointGeneration => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_contactPointGeneration,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactPointGeneration",
                                        ),
                                    );
                                }
                                m_contactPointGeneration = _serde::__private::Some(
                                    match __A::next_value::<
                                        ContactPointGeneration,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_allowToSkipConfirmedCallbacks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowToSkipConfirmedCallbacks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowToSkipConfirmedCallbacks",
                                        ),
                                    );
                                }
                                m_allowToSkipConfirmedCallbacks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_useHybridBroadphase => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useHybridBroadphase,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useHybridBroadphase",
                                        ),
                                    );
                                }
                                m_useHybridBroadphase = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_solverTau => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverTau) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverTau",
                                        ),
                                    );
                                }
                                m_solverTau = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_solverDamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverDamp) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverDamp",
                                        ),
                                    );
                                }
                                m_solverDamp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_solverIterations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverIterations) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverIterations",
                                        ),
                                    );
                                }
                                m_solverIterations = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_solverMicrosteps => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_solverMicrosteps) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverMicrosteps",
                                        ),
                                    );
                                }
                                m_solverMicrosteps = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxConstraintViolation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxConstraintViolation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxConstraintViolation",
                                        ),
                                    );
                                }
                                m_maxConstraintViolation = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_forceCoherentConstraintOrderingInSolver => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_forceCoherentConstraintOrderingInSolver,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forceCoherentConstraintOrderingInSolver",
                                        ),
                                    );
                                }
                                m_forceCoherentConstraintOrderingInSolver = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_snapCollisionToConvexEdgeThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapCollisionToConvexEdgeThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapCollisionToConvexEdgeThreshold",
                                        ),
                                    );
                                }
                                m_snapCollisionToConvexEdgeThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_snapCollisionToConcaveEdgeThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapCollisionToConcaveEdgeThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapCollisionToConcaveEdgeThreshold",
                                        ),
                                    );
                                }
                                m_snapCollisionToConcaveEdgeThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_enableToiWeldRejection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableToiWeldRejection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableToiWeldRejection",
                                        ),
                                    );
                                }
                                m_enableToiWeldRejection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_enableDeprecatedWelding => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableDeprecatedWelding,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeprecatedWelding",
                                        ),
                                    );
                                }
                                m_enableDeprecatedWelding = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_iterativeLinearCastEarlyOutDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_iterativeLinearCastEarlyOutDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "iterativeLinearCastEarlyOutDistance",
                                        ),
                                    );
                                }
                                m_iterativeLinearCastEarlyOutDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_iterativeLinearCastMaxIterations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_iterativeLinearCastMaxIterations,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "iterativeLinearCastMaxIterations",
                                        ),
                                    );
                                }
                                m_iterativeLinearCastMaxIterations = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationNumInactiveFramesSelectFlag0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationNumInactiveFramesSelectFlag0,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationNumInactiveFramesSelectFlag0",
                                        ),
                                    );
                                }
                                m_deactivationNumInactiveFramesSelectFlag0 = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationNumInactiveFramesSelectFlag1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationNumInactiveFramesSelectFlag1,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationNumInactiveFramesSelectFlag1",
                                        ),
                                    );
                                }
                                m_deactivationNumInactiveFramesSelectFlag1 = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationIntegrateCounter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationIntegrateCounter,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationIntegrateCounter",
                                        ),
                                    );
                                }
                                m_deactivationIntegrateCounter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_shouldActivateOnRigidBodyTransformChange => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_shouldActivateOnRigidBodyTransformChange,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shouldActivateOnRigidBodyTransformChange",
                                        ),
                                    );
                                }
                                m_shouldActivateOnRigidBodyTransformChange = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationReferenceDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationReferenceDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationReferenceDistance",
                                        ),
                                    );
                                }
                                m_deactivationReferenceDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_toiCollisionResponseRotateNormal => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_toiCollisionResponseRotateNormal,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toiCollisionResponseRotateNormal",
                                        ),
                                    );
                                }
                                m_toiCollisionResponseRotateNormal = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxSectorsPerMidphaseCollideTask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxSectorsPerMidphaseCollideTask,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSectorsPerMidphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxSectorsPerMidphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxSectorsPerNarrowphaseCollideTask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxSectorsPerNarrowphaseCollideTask,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSectorsPerNarrowphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxSectorsPerNarrowphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_processToisMultithreaded => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_processToisMultithreaded,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "processToisMultithreaded",
                                        ),
                                    );
                                }
                                m_processToisMultithreaded = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxEntriesPerToiMidphaseCollideTask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxEntriesPerToiMidphaseCollideTask,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxEntriesPerToiMidphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxEntriesPerToiMidphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxEntriesPerToiNarrowphaseCollideTask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxEntriesPerToiNarrowphaseCollideTask,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxEntriesPerToiNarrowphaseCollideTask",
                                        ),
                                    );
                                }
                                m_maxEntriesPerToiNarrowphaseCollideTask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxNumToiCollisionPairsSinglethreaded => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxNumToiCollisionPairsSinglethreaded,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxNumToiCollisionPairsSinglethreaded",
                                        ),
                                    );
                                }
                                m_maxNumToiCollisionPairsSinglethreaded = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_numToisTillAllowedPenetrationSimplifiedToi => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationSimplifiedToi,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationSimplifiedToi",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationSimplifiedToi = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_numToisTillAllowedPenetrationToi => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationToi,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationToi",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationToi = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_numToisTillAllowedPenetrationToiHigher => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationToiHigher,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationToiHigher",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationToiHigher = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_numToisTillAllowedPenetrationToiForced => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numToisTillAllowedPenetrationToiForced,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numToisTillAllowedPenetrationToiForced",
                                        ),
                                    );
                                }
                                m_numToisTillAllowedPenetrationToiForced = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_enableDeactivation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableDeactivation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeactivation",
                                        ),
                                    );
                                }
                                m_enableDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_simulationType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simulationType) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationType",
                                        ),
                                    );
                                }
                                m_simulationType = _serde::__private::Some(
                                    match __A::next_value::<SimulationType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_enableSimulationIslands => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableSimulationIslands,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableSimulationIslands",
                                        ),
                                    );
                                }
                                m_enableSimulationIslands = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_minDesiredIslandSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minDesiredIslandSize,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minDesiredIslandSize",
                                        ),
                                    );
                                }
                                m_minDesiredIslandSize = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_processActionsInSingleThread => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_processActionsInSingleThread,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "processActionsInSingleThread",
                                        ),
                                    );
                                }
                                m_processActionsInSingleThread = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                                        ),
                                    );
                                }
                                m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_frameMarkerPsiSnap => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_frameMarkerPsiSnap,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameMarkerPsiSnap",
                                        ),
                                    );
                                }
                                m_frameMarkerPsiSnap = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_fireCollisionCallbacks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fireCollisionCallbacks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fireCollisionCallbacks",
                                        ),
                                    );
                                }
                                m_fireCollisionCallbacks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_gravity = match m_gravity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gravity"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseQuerySize = match m_broadPhaseQuerySize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseQuerySize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_contactRestingVelocity = match m_contactRestingVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactRestingVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseBorderBehaviour = match m_broadPhaseBorderBehaviour {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseBorderBehaviour",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mtPostponeAndSortBroadPhaseBorderCallbacks = match m_mtPostponeAndSortBroadPhaseBorderCallbacks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mtPostponeAndSortBroadPhaseBorderCallbacks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseWorldAabb = match m_broadPhaseWorldAabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseWorldAabb",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useKdTree = match m_useKdTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useKdTree",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useMultipleTree = match m_useMultipleTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useMultipleTree",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_treeUpdateType = match m_treeUpdateType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "treeUpdateType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_autoUpdateKdTree = match m_autoUpdateKdTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "autoUpdateKdTree",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionTolerance = match m_collisionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionFilter = match m_collisionFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_convexListFilter = match m_convexListFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexListFilter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_expectedMaxLinearVelocity = match m_expectedMaxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expectedMaxLinearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sizeOfToiEventQueue = match m_sizeOfToiEventQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sizeOfToiEventQueue",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_expectedMinPsiDeltaTime = match m_expectedMinPsiDeltaTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expectedMinPsiDeltaTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_memoryWatchDog = match m_memoryWatchDog {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memoryWatchDog",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseNumMarkers = match m_broadPhaseNumMarkers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseNumMarkers",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_contactPointGeneration = match m_contactPointGeneration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactPointGeneration",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowToSkipConfirmedCallbacks = match m_allowToSkipConfirmedCallbacks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowToSkipConfirmedCallbacks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useHybridBroadphase = match m_useHybridBroadphase {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useHybridBroadphase",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverTau = match m_solverTau {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverTau",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverDamp = match m_solverDamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverDamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverIterations = match m_solverIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverIterations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverMicrosteps = match m_solverMicrosteps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverMicrosteps",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxConstraintViolation = match m_maxConstraintViolation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxConstraintViolation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_forceCoherentConstraintOrderingInSolver = match m_forceCoherentConstraintOrderingInSolver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forceCoherentConstraintOrderingInSolver",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapCollisionToConvexEdgeThreshold = match m_snapCollisionToConvexEdgeThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapCollisionToConvexEdgeThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapCollisionToConcaveEdgeThreshold = match m_snapCollisionToConcaveEdgeThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapCollisionToConcaveEdgeThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableToiWeldRejection = match m_enableToiWeldRejection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableToiWeldRejection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableDeprecatedWelding = match m_enableDeprecatedWelding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeprecatedWelding",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_iterativeLinearCastEarlyOutDistance = match m_iterativeLinearCastEarlyOutDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "iterativeLinearCastEarlyOutDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_iterativeLinearCastMaxIterations = match m_iterativeLinearCastMaxIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "iterativeLinearCastMaxIterations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationNumInactiveFramesSelectFlag0 = match m_deactivationNumInactiveFramesSelectFlag0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationNumInactiveFramesSelectFlag0",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationNumInactiveFramesSelectFlag1 = match m_deactivationNumInactiveFramesSelectFlag1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationNumInactiveFramesSelectFlag1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationIntegrateCounter = match m_deactivationIntegrateCounter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationIntegrateCounter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shouldActivateOnRigidBodyTransformChange = match m_shouldActivateOnRigidBodyTransformChange {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shouldActivateOnRigidBodyTransformChange",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationReferenceDistance = match m_deactivationReferenceDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationReferenceDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toiCollisionResponseRotateNormal = match m_toiCollisionResponseRotateNormal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toiCollisionResponseRotateNormal",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxSectorsPerMidphaseCollideTask = match m_maxSectorsPerMidphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxSectorsPerMidphaseCollideTask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxSectorsPerNarrowphaseCollideTask = match m_maxSectorsPerNarrowphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxSectorsPerNarrowphaseCollideTask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_processToisMultithreaded = match m_processToisMultithreaded {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "processToisMultithreaded",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxEntriesPerToiMidphaseCollideTask = match m_maxEntriesPerToiMidphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxEntriesPerToiMidphaseCollideTask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxEntriesPerToiNarrowphaseCollideTask = match m_maxEntriesPerToiNarrowphaseCollideTask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxEntriesPerToiNarrowphaseCollideTask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxNumToiCollisionPairsSinglethreaded = match m_maxNumToiCollisionPairsSinglethreaded {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxNumToiCollisionPairsSinglethreaded",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numToisTillAllowedPenetrationSimplifiedToi = match m_numToisTillAllowedPenetrationSimplifiedToi {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationSimplifiedToi",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numToisTillAllowedPenetrationToi = match m_numToisTillAllowedPenetrationToi {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationToi",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numToisTillAllowedPenetrationToiHigher = match m_numToisTillAllowedPenetrationToiHigher {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationToiHigher",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numToisTillAllowedPenetrationToiForced = match m_numToisTillAllowedPenetrationToiForced {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numToisTillAllowedPenetrationToiForced",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableDeactivation = match m_enableDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeactivation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simulationType = match m_simulationType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableSimulationIslands = match m_enableSimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableSimulationIslands",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minDesiredIslandSize = match m_minDesiredIslandSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minDesiredIslandSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_processActionsInSingleThread = match m_processActionsInSingleThread {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "processActionsInSingleThread",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob = match m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_frameMarkerPsiSnap = match m_frameMarkerPsiSnap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frameMarkerPsiSnap",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fireCollisionCallbacks = match m_fireCollisionCallbacks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fireCollisionCallbacks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpWorldCinfo {
                        __ptr,
                        parent,
                        m_gravity,
                        m_broadPhaseQuerySize,
                        m_contactRestingVelocity,
                        m_broadPhaseBorderBehaviour,
                        m_mtPostponeAndSortBroadPhaseBorderCallbacks,
                        m_broadPhaseWorldAabb,
                        m_useKdTree,
                        m_useMultipleTree,
                        m_treeUpdateType,
                        m_autoUpdateKdTree,
                        m_collisionTolerance,
                        m_collisionFilter,
                        m_convexListFilter,
                        m_expectedMaxLinearVelocity,
                        m_sizeOfToiEventQueue,
                        m_expectedMinPsiDeltaTime,
                        m_memoryWatchDog,
                        m_broadPhaseNumMarkers,
                        m_contactPointGeneration,
                        m_allowToSkipConfirmedCallbacks,
                        m_useHybridBroadphase,
                        m_solverTau,
                        m_solverDamp,
                        m_solverIterations,
                        m_solverMicrosteps,
                        m_maxConstraintViolation,
                        m_forceCoherentConstraintOrderingInSolver,
                        m_snapCollisionToConvexEdgeThreshold,
                        m_snapCollisionToConcaveEdgeThreshold,
                        m_enableToiWeldRejection,
                        m_enableDeprecatedWelding,
                        m_iterativeLinearCastEarlyOutDistance,
                        m_iterativeLinearCastMaxIterations,
                        m_deactivationNumInactiveFramesSelectFlag0,
                        m_deactivationNumInactiveFramesSelectFlag1,
                        m_deactivationIntegrateCounter,
                        m_shouldActivateOnRigidBodyTransformChange,
                        m_deactivationReferenceDistance,
                        m_toiCollisionResponseRotateNormal,
                        m_maxSectorsPerMidphaseCollideTask,
                        m_maxSectorsPerNarrowphaseCollideTask,
                        m_processToisMultithreaded,
                        m_maxEntriesPerToiMidphaseCollideTask,
                        m_maxEntriesPerToiNarrowphaseCollideTask,
                        m_maxNumToiCollisionPairsSinglethreaded,
                        m_numToisTillAllowedPenetrationSimplifiedToi,
                        m_numToisTillAllowedPenetrationToi,
                        m_numToisTillAllowedPenetrationToiHigher,
                        m_numToisTillAllowedPenetrationToiForced,
                        m_enableDeactivation,
                        m_simulationType,
                        m_enableSimulationIslands,
                        m_minDesiredIslandSize,
                        m_processActionsInSingleThread,
                        m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                        m_frameMarkerPsiSnap,
                        m_fireCollisionCallbacks,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "gravity",
                "broadPhaseQuerySize",
                "contactRestingVelocity",
                "broadPhaseBorderBehaviour",
                "mtPostponeAndSortBroadPhaseBorderCallbacks",
                "broadPhaseWorldAabb",
                "useKdTree",
                "useMultipleTree",
                "treeUpdateType",
                "autoUpdateKdTree",
                "collisionTolerance",
                "collisionFilter",
                "convexListFilter",
                "expectedMaxLinearVelocity",
                "sizeOfToiEventQueue",
                "expectedMinPsiDeltaTime",
                "memoryWatchDog",
                "broadPhaseNumMarkers",
                "contactPointGeneration",
                "allowToSkipConfirmedCallbacks",
                "useHybridBroadphase",
                "solverTau",
                "solverDamp",
                "solverIterations",
                "solverMicrosteps",
                "maxConstraintViolation",
                "forceCoherentConstraintOrderingInSolver",
                "snapCollisionToConvexEdgeThreshold",
                "snapCollisionToConcaveEdgeThreshold",
                "enableToiWeldRejection",
                "enableDeprecatedWelding",
                "iterativeLinearCastEarlyOutDistance",
                "iterativeLinearCastMaxIterations",
                "deactivationNumInactiveFramesSelectFlag0",
                "deactivationNumInactiveFramesSelectFlag1",
                "deactivationIntegrateCounter",
                "shouldActivateOnRigidBodyTransformChange",
                "deactivationReferenceDistance",
                "toiCollisionResponseRotateNormal",
                "maxSectorsPerMidphaseCollideTask",
                "maxSectorsPerNarrowphaseCollideTask",
                "processToisMultithreaded",
                "maxEntriesPerToiMidphaseCollideTask",
                "maxEntriesPerToiNarrowphaseCollideTask",
                "maxNumToiCollisionPairsSinglethreaded",
                "numToisTillAllowedPenetrationSimplifiedToi",
                "numToisTillAllowedPenetrationToi",
                "numToisTillAllowedPenetrationToiHigher",
                "numToisTillAllowedPenetrationToiForced",
                "enableDeactivation",
                "simulationType",
                "enableSimulationIslands",
                "minDesiredIslandSize",
                "processActionsInSingleThread",
                "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                "frameMarkerPsiSnap",
                "fireCollisionCallbacks",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpWorldCinfo",
                FIELDS,
                __hkpWorldCinfoVisitor {
                    marker: _serde::__private::PhantomData::<hkpWorldCinfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum SimulationType {
    #[default]
    SIMULATION_TYPE_INVALID = 0isize,
    SIMULATION_TYPE_DISCRETE = 1isize,
    SIMULATION_TYPE_CONTINUOUS = 2isize,
    SIMULATION_TYPE_MULTITHREADED = 3isize,
}
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum ContactPointGeneration {
    #[default]
    CONTACT_POINT_ACCEPT_ALWAYS = 0isize,
    CONTACT_POINT_REJECT_DUBIOUS = 1isize,
    CONTACT_POINT_REJECT_MANY = 2isize,
}
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum BroadPhaseBorderBehaviour {
    #[default]
    BROADPHASE_BORDER_ASSERT = 0isize,
    BROADPHASE_BORDER_FIX_ENTITY = 1isize,
    BROADPHASE_BORDER_REMOVE_ENTITY = 2isize,
    BROADPHASE_BORDER_DO_NOTHING = 3isize,
}
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum TreeUpdateType {
    #[default]
    REBUILD_ACTIVE = 0isize,
    REBUILD_ALL = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SimulationType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SIMULATION_TYPE_INVALID => {
                    __serializer.serialize_field("SIMULATION_TYPE_INVALID", &0u64)
                }
                Self::SIMULATION_TYPE_DISCRETE => {
                    __serializer.serialize_field("SIMULATION_TYPE_DISCRETE", &1u64)
                }
                Self::SIMULATION_TYPE_CONTINUOUS => {
                    __serializer.serialize_field("SIMULATION_TYPE_CONTINUOUS", &2u64)
                }
                Self::SIMULATION_TYPE_MULTITHREADED => {
                    __serializer.serialize_field("SIMULATION_TYPE_MULTITHREADED", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SimulationType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ContactPointGeneration {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::CONTACT_POINT_ACCEPT_ALWAYS => {
                    __serializer.serialize_field("CONTACT_POINT_ACCEPT_ALWAYS", &0u64)
                }
                Self::CONTACT_POINT_REJECT_DUBIOUS => {
                    __serializer.serialize_field("CONTACT_POINT_REJECT_DUBIOUS", &1u64)
                }
                Self::CONTACT_POINT_REJECT_MANY => {
                    __serializer.serialize_field("CONTACT_POINT_REJECT_MANY", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum ContactPointGeneration to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BroadPhaseBorderBehaviour {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BROADPHASE_BORDER_ASSERT => {
                    __serializer.serialize_field("BROADPHASE_BORDER_ASSERT", &0u64)
                }
                Self::BROADPHASE_BORDER_FIX_ENTITY => {
                    __serializer.serialize_field("BROADPHASE_BORDER_FIX_ENTITY", &1u64)
                }
                Self::BROADPHASE_BORDER_REMOVE_ENTITY => {
                    __serializer
                        .serialize_field("BROADPHASE_BORDER_REMOVE_ENTITY", &2u64)
                }
                Self::BROADPHASE_BORDER_DO_NOTHING => {
                    __serializer.serialize_field("BROADPHASE_BORDER_DO_NOTHING", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BroadPhaseBorderBehaviour to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TreeUpdateType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::REBUILD_ACTIVE => {
                    __serializer.serialize_field("REBUILD_ACTIVE", &0u64)
                }
                Self::REBUILD_ALL => __serializer.serialize_field("REBUILD_ALL", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum TreeUpdateType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SimulationType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("SIMULATION_TYPE_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("SIMULATION_TYPE_DISCRETE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("SIMULATION_TYPE_CONTINUOUS") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("SIMULATION_TYPE_MULTITHREADED") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SimulationType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimulationType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SimulationType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SimulationType::SIMULATION_TYPE_INVALID,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SimulationType::SIMULATION_TYPE_DISCRETE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SimulationType::SIMULATION_TYPE_CONTINUOUS,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SimulationType::SIMULATION_TYPE_MULTITHREADED,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SIMULATION_TYPE_INVALID",
                "SIMULATION_TYPE_DISCRETE",
                "SIMULATION_TYPE_CONTINUOUS",
                "SIMULATION_TYPE_MULTITHREADED",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SimulationType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimulationType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ContactPointGeneration {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("CONTACT_POINT_ACCEPT_ALWAYS") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("CONTACT_POINT_REJECT_DUBIOUS") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("CONTACT_POINT_REJECT_MANY") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ContactPointGeneration>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ContactPointGeneration;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ContactPointGeneration",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ContactPointGeneration::CONTACT_POINT_ACCEPT_ALWAYS,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ContactPointGeneration::CONTACT_POINT_REJECT_DUBIOUS,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ContactPointGeneration::CONTACT_POINT_REJECT_MANY,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "CONTACT_POINT_ACCEPT_ALWAYS",
                "CONTACT_POINT_REJECT_DUBIOUS",
                "CONTACT_POINT_REJECT_MANY",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ContactPointGeneration",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ContactPointGeneration>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BroadPhaseBorderBehaviour {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("BROADPHASE_BORDER_ASSERT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("BROADPHASE_BORDER_FIX_ENTITY") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("BROADPHASE_BORDER_REMOVE_ENTITY") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("BROADPHASE_BORDER_DO_NOTHING") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BroadPhaseBorderBehaviour>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BroadPhaseBorderBehaviour;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum BroadPhaseBorderBehaviour",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BroadPhaseBorderBehaviour::BROADPHASE_BORDER_ASSERT,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BroadPhaseBorderBehaviour::BROADPHASE_BORDER_FIX_ENTITY,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BroadPhaseBorderBehaviour::BROADPHASE_BORDER_REMOVE_ENTITY,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BroadPhaseBorderBehaviour::BROADPHASE_BORDER_DO_NOTHING,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "BROADPHASE_BORDER_ASSERT",
                "BROADPHASE_BORDER_FIX_ENTITY",
                "BROADPHASE_BORDER_REMOVE_ENTITY",
                "BROADPHASE_BORDER_DO_NOTHING",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "BroadPhaseBorderBehaviour",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BroadPhaseBorderBehaviour>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TreeUpdateType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("REBUILD_ACTIVE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("REBUILD_ALL") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TreeUpdateType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TreeUpdateType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TreeUpdateType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TreeUpdateType::REBUILD_ACTIVE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TreeUpdateType::REBUILD_ALL)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["REBUILD_ACTIVE", "REBUILD_ALL"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TreeUpdateType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TreeUpdateType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
