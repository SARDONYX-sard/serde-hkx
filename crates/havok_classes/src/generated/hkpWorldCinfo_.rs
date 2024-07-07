use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWorldCinfo`
/// -         version: `11`
/// -       signature: `0xa5255445`
/// -          size: 240(x86)/256(x86_64)
/// -          vtable: true
///
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
    /// -          name: `gravity`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_gravity: Vector4,
    /// # C++ Info
    /// -          name: `broadPhaseQuerySize`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_broadPhaseQuerySize: i32,
    /// # C++ Info
    /// -          name: `contactRestingVelocity`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contactRestingVelocity: f32,
    /// # C++ Info
    /// -          name: `broadPhaseBorderBehaviour`(ctype: `enum BroadPhaseBorderBehaviour`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_broadPhaseBorderBehaviour: BroadPhaseBorderBehaviour,
    /// # C++ Info
    /// -          name: `mtPostponeAndSortBroadPhaseBorderCallbacks`(ctype: `hkBool`)
    /// -        offset:  41(x86)/ 41(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_mtPostponeAndSortBroadPhaseBorderCallbacks: bool,
    /// # C++ Info
    /// -          name: `broadPhaseWorldAabb`(ctype: `struct hkAabb`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_broadPhaseWorldAabb: hkAabb,
    /// # C++ Info
    /// -          name: `useKdTree`(ctype: `hkBool`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useKdTree: bool,
    /// # C++ Info
    /// -          name: `useMultipleTree`(ctype: `hkBool`)
    /// -        offset:  81(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useMultipleTree: bool,
    /// # C++ Info
    /// -          name: `treeUpdateType`(ctype: `enum TreeUpdateType`)
    /// -        offset:  82(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_treeUpdateType: TreeUpdateType,
    /// # C++ Info
    /// -          name: `autoUpdateKdTree`(ctype: `hkBool`)
    /// -        offset:  83(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_autoUpdateKdTree: bool,
    /// # C++ Info
    /// -          name: `collisionTolerance`(ctype: `hkReal`)
    /// -        offset:  84(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionTolerance: f32,
    /// # C++ Info
    /// -          name: `collisionFilter`(ctype: `struct hkpCollisionFilter*`)
    /// -        offset:  88(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_collisionFilter: Pointer,
    /// # C++ Info
    /// -          name: `convexListFilter`(ctype: `struct hkpConvexListFilter*`)
    /// -        offset:  92(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_convexListFilter: Pointer,
    /// # C++ Info
    /// -          name: `expectedMaxLinearVelocity`(ctype: `hkReal`)
    /// -        offset:  96(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_expectedMaxLinearVelocity: f32,
    /// # C++ Info
    /// -          name: `sizeOfToiEventQueue`(ctype: `hkInt32`)
    /// -        offset: 100(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sizeOfToiEventQueue: i32,
    /// # C++ Info
    /// -          name: `expectedMinPsiDeltaTime`(ctype: `hkReal`)
    /// -        offset: 104(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_expectedMinPsiDeltaTime: f32,
    /// # C++ Info
    /// -          name: `memoryWatchDog`(ctype: `struct hkWorldMemoryAvailableWatchDog*`)
    /// -        offset: 108(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_memoryWatchDog: Pointer,
    /// # C++ Info
    /// -          name: `broadPhaseNumMarkers`(ctype: `hkInt32`)
    /// -        offset: 112(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_broadPhaseNumMarkers: i32,
    /// # C++ Info
    /// -          name: `contactPointGeneration`(ctype: `enum ContactPointGeneration`)
    /// -        offset: 116(x86)/132(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_contactPointGeneration: ContactPointGeneration,
    /// # C++ Info
    /// -          name: `allowToSkipConfirmedCallbacks`(ctype: `hkBool`)
    /// -        offset: 117(x86)/133(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_allowToSkipConfirmedCallbacks: bool,
    /// # C++ Info
    /// -          name: `useHybridBroadphase`(ctype: `hkBool`)
    /// -        offset: 118(x86)/134(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useHybridBroadphase: bool,
    /// # C++ Info
    /// -          name: `solverTau`(ctype: `hkReal`)
    /// -        offset: 120(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_solverTau: f32,
    /// # C++ Info
    /// -          name: `solverDamp`(ctype: `hkReal`)
    /// -        offset: 124(x86)/140(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_solverDamp: f32,
    /// # C++ Info
    /// -          name: `solverIterations`(ctype: `hkInt32`)
    /// -        offset: 128(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_solverIterations: i32,
    /// # C++ Info
    /// -          name: `solverMicrosteps`(ctype: `hkInt32`)
    /// -        offset: 132(x86)/148(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_solverMicrosteps: i32,
    /// # C++ Info
    /// -          name: `maxConstraintViolation`(ctype: `hkReal`)
    /// -        offset: 136(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxConstraintViolation: f32,
    /// # C++ Info
    /// -          name: `forceCoherentConstraintOrderingInSolver`(ctype: `hkBool`)
    /// -        offset: 140(x86)/156(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_forceCoherentConstraintOrderingInSolver: bool,
    /// # C++ Info
    /// -          name: `snapCollisionToConvexEdgeThreshold`(ctype: `hkReal`)
    /// -        offset: 144(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapCollisionToConvexEdgeThreshold: f32,
    /// # C++ Info
    /// -          name: `snapCollisionToConcaveEdgeThreshold`(ctype: `hkReal`)
    /// -        offset: 148(x86)/164(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapCollisionToConcaveEdgeThreshold: f32,
    /// # C++ Info
    /// -          name: `enableToiWeldRejection`(ctype: `hkBool`)
    /// -        offset: 152(x86)/168(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableToiWeldRejection: bool,
    /// # C++ Info
    /// -          name: `enableDeprecatedWelding`(ctype: `hkBool`)
    /// -        offset: 153(x86)/169(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableDeprecatedWelding: bool,
    /// # C++ Info
    /// -          name: `iterativeLinearCastEarlyOutDistance`(ctype: `hkReal`)
    /// -        offset: 156(x86)/172(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_iterativeLinearCastEarlyOutDistance: f32,
    /// # C++ Info
    /// -          name: `iterativeLinearCastMaxIterations`(ctype: `hkInt32`)
    /// -        offset: 160(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_iterativeLinearCastMaxIterations: i32,
    /// # C++ Info
    /// -          name: `deactivationNumInactiveFramesSelectFlag0`(ctype: `hkUint8`)
    /// -        offset: 164(x86)/180(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_deactivationNumInactiveFramesSelectFlag0: u8,
    /// # C++ Info
    /// -          name: `deactivationNumInactiveFramesSelectFlag1`(ctype: `hkUint8`)
    /// -        offset: 165(x86)/181(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_deactivationNumInactiveFramesSelectFlag1: u8,
    /// # C++ Info
    /// -          name: `deactivationIntegrateCounter`(ctype: `hkUint8`)
    /// -        offset: 166(x86)/182(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_deactivationIntegrateCounter: u8,
    /// # C++ Info
    /// -          name: `shouldActivateOnRigidBodyTransformChange`(ctype: `hkBool`)
    /// -        offset: 167(x86)/183(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_shouldActivateOnRigidBodyTransformChange: bool,
    /// # C++ Info
    /// -          name: `deactivationReferenceDistance`(ctype: `hkReal`)
    /// -        offset: 168(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deactivationReferenceDistance: f32,
    /// # C++ Info
    /// -          name: `toiCollisionResponseRotateNormal`(ctype: `hkReal`)
    /// -        offset: 172(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toiCollisionResponseRotateNormal: f32,
    /// # C++ Info
    /// -          name: `maxSectorsPerMidphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 176(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSectorsPerMidphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `maxSectorsPerNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 180(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSectorsPerNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `processToisMultithreaded`(ctype: `hkBool`)
    /// -        offset: 184(x86)/200(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_processToisMultithreaded: bool,
    /// # C++ Info
    /// -          name: `maxEntriesPerToiMidphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 188(x86)/204(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxEntriesPerToiMidphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `maxEntriesPerToiNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 192(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxEntriesPerToiNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `maxNumToiCollisionPairsSinglethreaded`(ctype: `hkInt32`)
    /// -        offset: 196(x86)/212(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxNumToiCollisionPairsSinglethreaded: i32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationSimplifiedToi`(ctype: `hkReal`)
    /// -        offset: 200(x86)/216(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationSimplifiedToi: f32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationToi`(ctype: `hkReal`)
    /// -        offset: 204(x86)/220(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationToi: f32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationToiHigher`(ctype: `hkReal`)
    /// -        offset: 208(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationToiHigher: f32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationToiForced`(ctype: `hkReal`)
    /// -        offset: 212(x86)/228(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationToiForced: f32,
    /// # C++ Info
    /// -          name: `enableDeactivation`(ctype: `hkBool`)
    /// -        offset: 216(x86)/232(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableDeactivation: bool,
    /// # C++ Info
    /// -          name: `simulationType`(ctype: `enum SimulationType`)
    /// -        offset: 217(x86)/233(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_simulationType: SimulationType,
    /// # C++ Info
    /// -          name: `enableSimulationIslands`(ctype: `hkBool`)
    /// -        offset: 218(x86)/234(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableSimulationIslands: bool,
    /// # C++ Info
    /// -          name: `minDesiredIslandSize`(ctype: `hkUint32`)
    /// -        offset: 220(x86)/236(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minDesiredIslandSize: u32,
    /// # C++ Info
    /// -          name: `processActionsInSingleThread`(ctype: `hkBool`)
    /// -        offset: 224(x86)/240(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_processActionsInSingleThread: bool,
    /// # C++ Info
    /// -          name: `allowIntegrationOfIslandsWithoutConstraintsInASeparateJob`(ctype: `hkBool`)
    /// -        offset: 225(x86)/241(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: bool,
    /// # C++ Info
    /// -          name: `frameMarkerPsiSnap`(ctype: `hkReal`)
    /// -        offset: 228(x86)/244(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frameMarkerPsiSnap: f32,
    /// # C++ Info
    /// -          name: `fireCollisionCallbacks`(ctype: `hkBool`)
    /// -        offset: 232(x86)/248(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
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
