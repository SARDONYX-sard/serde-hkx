use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWorld`
/// -         version: `0`
/// -       signature: `0xaadcec37`
/// -          size: 864(x86)/1072(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWorld {
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
    /// -          name: `simulation`(ctype: `struct hkpSimulation*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_simulation: Pointer,
    /// # C++ Info
    /// -          name: `gravity`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_gravity: Vector4,
    /// # C++ Info
    /// -          name: `fixedIsland`(ctype: `void*`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fixedIsland: Pointer,
    /// # C++ Info
    /// -          name: `fixedRigidBody`(ctype: `struct hkpRigidBody*`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_fixedRigidBody: Pointer,
    /// # C++ Info
    /// -          name: `activeSimulationIslands`(ctype: `hkArray<void*>`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activeSimulationIslands: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `inactiveSimulationIslands`(ctype: `hkArray<void*>`)
    /// -        offset:  52(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_inactiveSimulationIslands: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `dirtySimulationIslands`(ctype: `hkArray<void*>`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_dirtySimulationIslands: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `maintenanceMgr`(ctype: `void*`)
    /// -        offset:  76(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_maintenanceMgr: Pointer,
    /// # C++ Info
    /// -          name: `memoryWatchDog`(ctype: `void*`)
    /// -        offset:  80(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_memoryWatchDog: Pointer,
    /// # C++ Info
    /// -          name: `assertOnRunningOutOfSolverMemory`(ctype: `hkBool`)
    /// -        offset:  84(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_assertOnRunningOutOfSolverMemory: bool,
    /// # C++ Info
    /// -          name: `broadPhase`(ctype: `void*`)
    /// -        offset:  88(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_broadPhase: Pointer,
    /// # C++ Info
    /// -          name: `kdTreeManager`(ctype: `void*`)
    /// -        offset:  92(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_kdTreeManager: Pointer,
    /// # C++ Info
    /// -          name: `autoUpdateTree`(ctype: `hkBool`)
    /// -        offset:  96(x86)/152(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_autoUpdateTree: bool,
    /// # C++ Info
    /// -          name: `broadPhaseDispatcher`(ctype: `void*`)
    /// -        offset: 100(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_broadPhaseDispatcher: Pointer,
    /// # C++ Info
    /// -          name: `phantomBroadPhaseListener`(ctype: `void*`)
    /// -        offset: 104(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_phantomBroadPhaseListener: Pointer,
    /// # C++ Info
    /// -          name: `entityEntityBroadPhaseListener`(ctype: `void*`)
    /// -        offset: 108(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_entityEntityBroadPhaseListener: Pointer,
    /// # C++ Info
    /// -          name: `broadPhaseBorderListener`(ctype: `void*`)
    /// -        offset: 112(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_broadPhaseBorderListener: Pointer,
    /// # C++ Info
    /// -          name: `multithreadedSimulationJobData`(ctype: `void*`)
    /// -        offset: 116(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_multithreadedSimulationJobData: Pointer,
    /// # C++ Info
    /// -          name: `collisionInput`(ctype: `void*`)
    /// -        offset: 120(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionInput: Pointer,
    /// # C++ Info
    /// -          name: `collisionFilter`(ctype: `void*`)
    /// -        offset: 124(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionFilter: Pointer,
    /// # C++ Info
    /// -          name: `collisionDispatcher`(ctype: `void*`)
    /// -        offset: 128(x86)/216(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionDispatcher: Pointer,
    /// # C++ Info
    /// -          name: `convexListFilter`(ctype: `void*`)
    /// -        offset: 132(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_convexListFilter: Pointer,
    /// # C++ Info
    /// -          name: `pendingOperations`(ctype: `void*`)
    /// -        offset: 136(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pendingOperations: Pointer,
    /// # C++ Info
    /// -          name: `pendingOperationsCount`(ctype: `hkInt32`)
    /// -        offset: 140(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pendingOperationsCount: i32,
    /// # C++ Info
    /// -          name: `pendingBodyOperationsCount`(ctype: `hkInt32`)
    /// -        offset: 144(x86)/244(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pendingBodyOperationsCount: i32,
    /// # C++ Info
    /// -          name: `criticalOperationsLockCount`(ctype: `hkInt32`)
    /// -        offset: 148(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_criticalOperationsLockCount: i32,
    /// # C++ Info
    /// -          name: `criticalOperationsLockCountForPhantoms`(ctype: `hkInt32`)
    /// -        offset: 152(x86)/252(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_criticalOperationsLockCountForPhantoms: i32,
    /// # C++ Info
    /// -          name: `blockExecutingPendingOperations`(ctype: `hkBool`)
    /// -        offset: 156(x86)/256(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_blockExecutingPendingOperations: bool,
    /// # C++ Info
    /// -          name: `criticalOperationsAllowed`(ctype: `hkBool`)
    /// -        offset: 157(x86)/257(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_criticalOperationsAllowed: bool,
    /// # C++ Info
    /// -          name: `pendingOperationQueues`(ctype: `void*`)
    /// -        offset: 160(x86)/264(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pendingOperationQueues: Pointer,
    /// # C++ Info
    /// -          name: `pendingOperationQueueCount`(ctype: `hkInt32`)
    /// -        offset: 164(x86)/272(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pendingOperationQueueCount: i32,
    /// # C++ Info
    /// -          name: `multiThreadCheck`(ctype: `struct hkMultiThreadCheck`)
    /// -        offset: 168(x86)/276(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_multiThreadCheck: hkMultiThreadCheck,
    /// # C++ Info
    /// -          name: `processActionsInSingleThread`(ctype: `hkBool`)
    /// -        offset: 180(x86)/288(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_processActionsInSingleThread: bool,
    /// # C++ Info
    /// -          name: `allowIntegrationOfIslandsWithoutConstraintsInASeparateJob`(ctype: `hkBool`)
    /// -        offset: 181(x86)/289(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: bool,
    /// # C++ Info
    /// -          name: `minDesiredIslandSize`(ctype: `hkUint32`)
    /// -        offset: 184(x86)/292(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minDesiredIslandSize: u32,
    /// # C++ Info
    /// -          name: `modifyConstraintCriticalSection`(ctype: `void*`)
    /// -        offset: 188(x86)/296(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_modifyConstraintCriticalSection: Pointer,
    /// # C++ Info
    /// -          name: `isLocked`(ctype: `hkInt32`)
    /// -        offset: 192(x86)/304(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_isLocked: i32,
    /// # C++ Info
    /// -          name: `islandDirtyListCriticalSection`(ctype: `void*`)
    /// -        offset: 196(x86)/312(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_islandDirtyListCriticalSection: Pointer,
    /// # C++ Info
    /// -          name: `propertyMasterLock`(ctype: `void*`)
    /// -        offset: 200(x86)/320(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_propertyMasterLock: Pointer,
    /// # C++ Info
    /// -          name: `wantSimulationIslands`(ctype: `hkBool`)
    /// -        offset: 204(x86)/328(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wantSimulationIslands: bool,
    /// # C++ Info
    /// -          name: `useHybridBroadphase`(ctype: `hkBool`)
    /// -        offset: 205(x86)/329(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_useHybridBroadphase: bool,
    /// # C++ Info
    /// -          name: `snapCollisionToConvexEdgeThreshold`(ctype: `hkReal`)
    /// -        offset: 208(x86)/332(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapCollisionToConvexEdgeThreshold: f32,
    /// # C++ Info
    /// -          name: `snapCollisionToConcaveEdgeThreshold`(ctype: `hkReal`)
    /// -        offset: 212(x86)/336(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapCollisionToConcaveEdgeThreshold: f32,
    /// # C++ Info
    /// -          name: `enableToiWeldRejection`(ctype: `hkBool`)
    /// -        offset: 216(x86)/340(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableToiWeldRejection: bool,
    /// # C++ Info
    /// -          name: `wantDeactivation`(ctype: `hkBool`)
    /// -        offset: 217(x86)/341(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wantDeactivation: bool,
    /// # C++ Info
    /// -          name: `shouldActivateOnRigidBodyTransformChange`(ctype: `hkBool`)
    /// -        offset: 218(x86)/342(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_shouldActivateOnRigidBodyTransformChange: bool,
    /// # C++ Info
    /// -          name: `deactivationReferenceDistance`(ctype: `hkReal`)
    /// -        offset: 220(x86)/344(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deactivationReferenceDistance: f32,
    /// # C++ Info
    /// -          name: `toiCollisionResponseRotateNormal`(ctype: `hkReal`)
    /// -        offset: 224(x86)/348(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toiCollisionResponseRotateNormal: f32,
    /// # C++ Info
    /// -          name: `maxSectorsPerMidphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 228(x86)/352(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSectorsPerMidphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `maxSectorsPerNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 232(x86)/356(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSectorsPerNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `processToisMultithreaded`(ctype: `hkBool`)
    /// -        offset: 236(x86)/360(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_processToisMultithreaded: bool,
    /// # C++ Info
    /// -          name: `maxEntriesPerToiMidphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 240(x86)/364(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxEntriesPerToiMidphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `maxEntriesPerToiNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// -        offset: 244(x86)/368(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxEntriesPerToiNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// -          name: `maxNumToiCollisionPairsSinglethreaded`(ctype: `hkInt32`)
    /// -        offset: 248(x86)/372(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxNumToiCollisionPairsSinglethreaded: i32,
    /// # C++ Info
    /// -          name: `simulationType`(ctype: `enum unknown`)
    /// -        offset: 252(x86)/376(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_simulationType: i32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationSimplifiedToi`(ctype: `hkReal`)
    /// -        offset: 256(x86)/380(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationSimplifiedToi: f32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationToi`(ctype: `hkReal`)
    /// -        offset: 260(x86)/384(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationToi: f32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationToiHigher`(ctype: `hkReal`)
    /// -        offset: 264(x86)/388(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationToiHigher: f32,
    /// # C++ Info
    /// -          name: `numToisTillAllowedPenetrationToiForced`(ctype: `hkReal`)
    /// -        offset: 268(x86)/392(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numToisTillAllowedPenetrationToiForced: f32,
    /// # C++ Info
    /// -          name: `lastEntityUid`(ctype: `hkUint32`)
    /// -        offset: 272(x86)/396(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lastEntityUid: u32,
    /// # C++ Info
    /// -          name: `lastIslandUid`(ctype: `hkUint32`)
    /// -        offset: 276(x86)/400(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lastIslandUid: u32,
    /// # C++ Info
    /// -          name: `lastConstraintUid`(ctype: `hkUint32`)
    /// -        offset: 280(x86)/404(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lastConstraintUid: u32,
    /// # C++ Info
    /// -          name: `phantoms`(ctype: `hkArray<hkpPhantom*>`)
    /// -        offset: 284(x86)/408(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_phantoms: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `actionListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 296(x86)/424(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_actionListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `entityListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 308(x86)/440(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_entityListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `phantomListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 320(x86)/456(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_phantomListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `constraintListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 332(x86)/472(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_constraintListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `worldDeletionListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 344(x86)/488(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_worldDeletionListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `islandActivationListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 356(x86)/504(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_islandActivationListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `worldPostSimulationListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 368(x86)/520(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_worldPostSimulationListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `worldPostIntegrateListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 380(x86)/536(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_worldPostIntegrateListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `worldPostCollideListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 392(x86)/552(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_worldPostCollideListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `islandPostIntegrateListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 404(x86)/568(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_islandPostIntegrateListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `islandPostCollideListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 416(x86)/584(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_islandPostCollideListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `contactListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 428(x86)/600(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_contactListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `contactImpulseLimitBreachedListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 440(x86)/616(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_contactImpulseLimitBreachedListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `worldExtensions`(ctype: `hkArray<void*>`)
    /// -        offset: 452(x86)/632(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_worldExtensions: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `violatedConstraintArray`(ctype: `void*`)
    /// -        offset: 464(x86)/648(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_violatedConstraintArray: Pointer,
    /// # C++ Info
    /// -          name: `broadPhaseBorder`(ctype: `void*`)
    /// -        offset: 468(x86)/656(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_broadPhaseBorder: Pointer,
    /// # C++ Info
    /// -          name: `destructionWorld`(ctype: `void*`)
    /// -        offset: 472(x86)/664(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_destructionWorld: Pointer,
    /// # C++ Info
    /// -          name: `npWorld`(ctype: `void*`)
    /// -        offset: 476(x86)/672(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_npWorld: Pointer,
    /// # C++ Info
    /// -          name: `broadPhaseExtents`(ctype: `hkVector4[2]`)
    /// -        offset: 800(x86)/1008(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_broadPhaseExtents: [Vector4; 2usize],
    /// # C++ Info
    /// -          name: `broadPhaseNumMarkers`(ctype: `hkInt32`)
    /// -        offset: 832(x86)/1040(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_broadPhaseNumMarkers: i32,
    /// # C++ Info
    /// -          name: `sizeOfToiEventQueue`(ctype: `hkInt32`)
    /// -        offset: 836(x86)/1044(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sizeOfToiEventQueue: i32,
    /// # C++ Info
    /// -          name: `broadPhaseQuerySize`(ctype: `hkInt32`)
    /// -        offset: 840(x86)/1048(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_broadPhaseQuerySize: i32,
    /// # C++ Info
    /// -          name: `broadPhaseUpdateSize`(ctype: `hkInt32`)
    /// -        offset: 844(x86)/1052(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_broadPhaseUpdateSize: i32,
    /// # C++ Info
    /// -          name: `contactPointGeneration`(ctype: `enum unknown`)
    /// -        offset: 848(x86)/1056(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_contactPointGeneration: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpWorld {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWorld"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xaadcec37)
        }
    }
    impl _serde::Serialize for hkpWorld {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xaadcec37)));
            let mut serializer = __serializer.serialize_struct("hkpWorld", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("simulation", &self.m_simulation)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer.skip_field("fixedIsland", &self.m_fixedIsland)?;
            serializer.serialize_field("fixedRigidBody", &self.m_fixedRigidBody)?;
            serializer
                .skip_array_meta_field(
                    "activeSimulationIslands",
                    &self.m_activeSimulationIslands,
                )?;
            serializer
                .skip_array_meta_field(
                    "inactiveSimulationIslands",
                    &self.m_inactiveSimulationIslands,
                )?;
            serializer
                .skip_array_meta_field(
                    "dirtySimulationIslands",
                    &self.m_dirtySimulationIslands,
                )?;
            serializer.skip_field("maintenanceMgr", &self.m_maintenanceMgr)?;
            serializer.skip_field("memoryWatchDog", &self.m_memoryWatchDog)?;
            serializer
                .skip_field(
                    "assertOnRunningOutOfSolverMemory",
                    &self.m_assertOnRunningOutOfSolverMemory,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("broadPhase", &self.m_broadPhase)?;
            serializer.skip_field("kdTreeManager", &self.m_kdTreeManager)?;
            serializer.serialize_field("autoUpdateTree", &self.m_autoUpdateTree)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("broadPhaseDispatcher", &self.m_broadPhaseDispatcher)?;
            serializer
                .skip_field(
                    "phantomBroadPhaseListener",
                    &self.m_phantomBroadPhaseListener,
                )?;
            serializer
                .skip_field(
                    "entityEntityBroadPhaseListener",
                    &self.m_entityEntityBroadPhaseListener,
                )?;
            serializer
                .skip_field(
                    "broadPhaseBorderListener",
                    &self.m_broadPhaseBorderListener,
                )?;
            serializer
                .skip_field(
                    "multithreadedSimulationJobData",
                    &self.m_multithreadedSimulationJobData,
                )?;
            serializer.skip_field("collisionInput", &self.m_collisionInput)?;
            serializer.skip_field("collisionFilter", &self.m_collisionFilter)?;
            serializer.skip_field("collisionDispatcher", &self.m_collisionDispatcher)?;
            serializer.skip_field("convexListFilter", &self.m_convexListFilter)?;
            serializer.skip_field("pendingOperations", &self.m_pendingOperations)?;
            serializer
                .serialize_field(
                    "pendingOperationsCount",
                    &self.m_pendingOperationsCount,
                )?;
            serializer
                .skip_field(
                    "pendingBodyOperationsCount",
                    &self.m_pendingBodyOperationsCount,
                )?;
            serializer
                .serialize_field(
                    "criticalOperationsLockCount",
                    &self.m_criticalOperationsLockCount,
                )?;
            serializer
                .serialize_field(
                    "criticalOperationsLockCountForPhantoms",
                    &self.m_criticalOperationsLockCountForPhantoms,
                )?;
            serializer
                .serialize_field(
                    "blockExecutingPendingOperations",
                    &self.m_blockExecutingPendingOperations,
                )?;
            serializer
                .serialize_field(
                    "criticalOperationsAllowed",
                    &self.m_criticalOperationsAllowed,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .skip_field("pendingOperationQueues", &self.m_pendingOperationQueues)?;
            serializer
                .serialize_field(
                    "pendingOperationQueueCount",
                    &self.m_pendingOperationQueueCount,
                )?;
            serializer.skip_field("multiThreadCheck", &self.m_multiThreadCheck)?;
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
                .serialize_field("minDesiredIslandSize", &self.m_minDesiredIslandSize)?;
            serializer
                .skip_field(
                    "modifyConstraintCriticalSection",
                    &self.m_modifyConstraintCriticalSection,
                )?;
            serializer.serialize_field("isLocked", &self.m_isLocked)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "islandDirtyListCriticalSection",
                    &self.m_islandDirtyListCriticalSection,
                )?;
            serializer.skip_field("propertyMasterLock", &self.m_propertyMasterLock)?;
            serializer
                .serialize_field(
                    "wantSimulationIslands",
                    &self.m_wantSimulationIslands,
                )?;
            serializer.skip_field("useHybridBroadphase", &self.m_useHybridBroadphase)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
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
            serializer.serialize_field("wantDeactivation", &self.m_wantDeactivation)?;
            serializer
                .serialize_field(
                    "shouldActivateOnRigidBodyTransformChange",
                    &self.m_shouldActivateOnRigidBodyTransformChange,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
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
            serializer.skip_field("simulationType", &self.m_simulationType)?;
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
            serializer.serialize_field("lastEntityUid", &self.m_lastEntityUid)?;
            serializer.serialize_field("lastIslandUid", &self.m_lastIslandUid)?;
            serializer.serialize_field("lastConstraintUid", &self.m_lastConstraintUid)?;
            serializer.serialize_array_meta_field("phantoms", &self.m_phantoms)?;
            serializer
                .skip_array_meta_field("actionListeners", &self.m_actionListeners)?;
            serializer
                .skip_array_meta_field("entityListeners", &self.m_entityListeners)?;
            serializer
                .skip_array_meta_field("phantomListeners", &self.m_phantomListeners)?;
            serializer
                .skip_array_meta_field(
                    "constraintListeners",
                    &self.m_constraintListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "worldDeletionListeners",
                    &self.m_worldDeletionListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "islandActivationListeners",
                    &self.m_islandActivationListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "worldPostSimulationListeners",
                    &self.m_worldPostSimulationListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "worldPostIntegrateListeners",
                    &self.m_worldPostIntegrateListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "worldPostCollideListeners",
                    &self.m_worldPostCollideListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "islandPostIntegrateListeners",
                    &self.m_islandPostIntegrateListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "islandPostCollideListeners",
                    &self.m_islandPostCollideListeners,
                )?;
            serializer
                .skip_array_meta_field("contactListeners", &self.m_contactListeners)?;
            serializer
                .skip_array_meta_field(
                    "contactImpulseLimitBreachedListeners",
                    &self.m_contactImpulseLimitBreachedListeners,
                )?;
            serializer
                .skip_array_meta_field("worldExtensions", &self.m_worldExtensions)?;
            serializer
                .skip_field("violatedConstraintArray", &self.m_violatedConstraintArray)?;
            serializer.skip_field("broadPhaseBorder", &self.m_broadPhaseBorder)?;
            serializer.skip_field("destructionWorld", &self.m_destructionWorld)?;
            serializer.skip_field("npWorld", &self.m_npWorld)?;
            serializer
                .pad_field([0u8; 320usize].as_slice(), [0u8; 328usize].as_slice())?;
            serializer
                .serialize_field(
                    "broadPhaseExtents",
                    &self.m_broadPhaseExtents.as_slice(),
                )?;
            serializer
                .serialize_field("broadPhaseNumMarkers", &self.m_broadPhaseNumMarkers)?;
            serializer
                .serialize_field("sizeOfToiEventQueue", &self.m_sizeOfToiEventQueue)?;
            serializer
                .serialize_field("broadPhaseQuerySize", &self.m_broadPhaseQuerySize)?;
            serializer
                .serialize_field("broadPhaseUpdateSize", &self.m_broadPhaseUpdateSize)?;
            serializer
                .skip_field("contactPointGeneration", &self.m_contactPointGeneration)?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "activeSimulationIslands",
                    &self.m_activeSimulationIslands,
                )?;
            serializer
                .serialize_array_field(
                    "inactiveSimulationIslands",
                    &self.m_inactiveSimulationIslands,
                )?;
            serializer
                .serialize_array_field(
                    "dirtySimulationIslands",
                    &self.m_dirtySimulationIslands,
                )?;
            serializer.serialize_array_field("phantoms", &self.m_phantoms)?;
            serializer
                .serialize_array_field("actionListeners", &self.m_actionListeners)?;
            serializer
                .serialize_array_field("entityListeners", &self.m_entityListeners)?;
            serializer
                .serialize_array_field("phantomListeners", &self.m_phantomListeners)?;
            serializer
                .serialize_array_field(
                    "constraintListeners",
                    &self.m_constraintListeners,
                )?;
            serializer
                .serialize_array_field(
                    "worldDeletionListeners",
                    &self.m_worldDeletionListeners,
                )?;
            serializer
                .serialize_array_field(
                    "islandActivationListeners",
                    &self.m_islandActivationListeners,
                )?;
            serializer
                .serialize_array_field(
                    "worldPostSimulationListeners",
                    &self.m_worldPostSimulationListeners,
                )?;
            serializer
                .serialize_array_field(
                    "worldPostIntegrateListeners",
                    &self.m_worldPostIntegrateListeners,
                )?;
            serializer
                .serialize_array_field(
                    "worldPostCollideListeners",
                    &self.m_worldPostCollideListeners,
                )?;
            serializer
                .serialize_array_field(
                    "islandPostIntegrateListeners",
                    &self.m_islandPostIntegrateListeners,
                )?;
            serializer
                .serialize_array_field(
                    "islandPostCollideListeners",
                    &self.m_islandPostCollideListeners,
                )?;
            serializer
                .serialize_array_field("contactListeners", &self.m_contactListeners)?;
            serializer
                .serialize_array_field(
                    "contactImpulseLimitBreachedListeners",
                    &self.m_contactImpulseLimitBreachedListeners,
                )?;
            serializer
                .serialize_array_field("worldExtensions", &self.m_worldExtensions)?;
            serializer.end()
        }
    }
};
