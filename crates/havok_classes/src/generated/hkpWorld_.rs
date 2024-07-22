use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpWorld`
/// - version: `0`
/// - signature: `0xaadcec37`
/// - size: `864`(x86)/`1072`(x86_64)
/// -  vtable: `true`
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
    /// - name: `simulation`(ctype: `struct hkpSimulation*`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_simulation: Pointer,
    /// # C++ Info
    /// - name: `gravity`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_gravity: Vector4,
    /// # C++ Info
    /// - name: `fixedIsland`(ctype: `void*`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_fixedIsland: Pointer,
    /// # C++ Info
    /// - name: `fixedRigidBody`(ctype: `struct hkpRigidBody*`)
    /// - offset: ` 36`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_fixedRigidBody: Pointer,
    /// # C++ Info
    /// - name: `activeSimulationIslands`(ctype: `hkArray<void*>`)
    /// - offset: ` 40`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_activeSimulationIslands: Vec<Pointer>,
    /// # C++ Info
    /// - name: `inactiveSimulationIslands`(ctype: `hkArray<void*>`)
    /// - offset: ` 52`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_inactiveSimulationIslands: Vec<Pointer>,
    /// # C++ Info
    /// - name: `dirtySimulationIslands`(ctype: `hkArray<void*>`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_dirtySimulationIslands: Vec<Pointer>,
    /// # C++ Info
    /// - name: `maintenanceMgr`(ctype: `void*`)
    /// - offset: ` 76`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_maintenanceMgr: Pointer,
    /// # C++ Info
    /// - name: `memoryWatchDog`(ctype: `void*`)
    /// - offset: ` 80`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_memoryWatchDog: Pointer,
    /// # C++ Info
    /// - name: `assertOnRunningOutOfSolverMemory`(ctype: `hkBool`)
    /// - offset: ` 84`(x86)/`128`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_assertOnRunningOutOfSolverMemory: bool,
    /// # C++ Info
    /// - name: `broadPhase`(ctype: `void*`)
    /// - offset: ` 88`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_broadPhase: Pointer,
    /// # C++ Info
    /// - name: `kdTreeManager`(ctype: `void*`)
    /// - offset: ` 92`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_kdTreeManager: Pointer,
    /// # C++ Info
    /// - name: `autoUpdateTree`(ctype: `hkBool`)
    /// - offset: ` 96`(x86)/`152`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_autoUpdateTree: bool,
    /// # C++ Info
    /// - name: `broadPhaseDispatcher`(ctype: `void*`)
    /// - offset: `100`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_broadPhaseDispatcher: Pointer,
    /// # C++ Info
    /// - name: `phantomBroadPhaseListener`(ctype: `void*`)
    /// - offset: `104`(x86)/`168`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_phantomBroadPhaseListener: Pointer,
    /// # C++ Info
    /// - name: `entityEntityBroadPhaseListener`(ctype: `void*`)
    /// - offset: `108`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_entityEntityBroadPhaseListener: Pointer,
    /// # C++ Info
    /// - name: `broadPhaseBorderListener`(ctype: `void*`)
    /// - offset: `112`(x86)/`184`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_broadPhaseBorderListener: Pointer,
    /// # C++ Info
    /// - name: `multithreadedSimulationJobData`(ctype: `void*`)
    /// - offset: `116`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_multithreadedSimulationJobData: Pointer,
    /// # C++ Info
    /// - name: `collisionInput`(ctype: `void*`)
    /// - offset: `120`(x86)/`200`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_collisionInput: Pointer,
    /// # C++ Info
    /// - name: `collisionFilter`(ctype: `void*`)
    /// - offset: `124`(x86)/`208`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_collisionFilter: Pointer,
    /// # C++ Info
    /// - name: `collisionDispatcher`(ctype: `void*`)
    /// - offset: `128`(x86)/`216`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_collisionDispatcher: Pointer,
    /// # C++ Info
    /// - name: `convexListFilter`(ctype: `void*`)
    /// - offset: `132`(x86)/`224`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_convexListFilter: Pointer,
    /// # C++ Info
    /// - name: `pendingOperations`(ctype: `void*`)
    /// - offset: `136`(x86)/`232`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_pendingOperations: Pointer,
    /// # C++ Info
    /// - name: `pendingOperationsCount`(ctype: `hkInt32`)
    /// - offset: `140`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pendingOperationsCount: i32,
    /// # C++ Info
    /// - name: `pendingBodyOperationsCount`(ctype: `hkInt32`)
    /// - offset: `144`(x86)/`244`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_pendingBodyOperationsCount: i32,
    /// # C++ Info
    /// - name: `criticalOperationsLockCount`(ctype: `hkInt32`)
    /// - offset: `148`(x86)/`248`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_criticalOperationsLockCount: i32,
    /// # C++ Info
    /// - name: `criticalOperationsLockCountForPhantoms`(ctype: `hkInt32`)
    /// - offset: `152`(x86)/`252`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_criticalOperationsLockCountForPhantoms: i32,
    /// # C++ Info
    /// - name: `blockExecutingPendingOperations`(ctype: `hkBool`)
    /// - offset: `156`(x86)/`256`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_blockExecutingPendingOperations: bool,
    /// # C++ Info
    /// - name: `criticalOperationsAllowed`(ctype: `hkBool`)
    /// - offset: `157`(x86)/`257`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_criticalOperationsAllowed: bool,
    /// # C++ Info
    /// - name: `pendingOperationQueues`(ctype: `void*`)
    /// - offset: `160`(x86)/`264`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_pendingOperationQueues: Pointer,
    /// # C++ Info
    /// - name: `pendingOperationQueueCount`(ctype: `hkInt32`)
    /// - offset: `164`(x86)/`272`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pendingOperationQueueCount: i32,
    /// # C++ Info
    /// - name: `multiThreadCheck`(ctype: `struct hkMultiThreadCheck`)
    /// - offset: `168`(x86)/`276`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_multiThreadCheck: hkMultiThreadCheck,
    /// # C++ Info
    /// - name: `processActionsInSingleThread`(ctype: `hkBool`)
    /// - offset: `180`(x86)/`288`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_processActionsInSingleThread: bool,
    /// # C++ Info
    /// - name: `allowIntegrationOfIslandsWithoutConstraintsInASeparateJob`(ctype: `hkBool`)
    /// - offset: `181`(x86)/`289`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: bool,
    /// # C++ Info
    /// - name: `minDesiredIslandSize`(ctype: `hkUint32`)
    /// - offset: `184`(x86)/`292`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minDesiredIslandSize: u32,
    /// # C++ Info
    /// - name: `modifyConstraintCriticalSection`(ctype: `void*`)
    /// - offset: `188`(x86)/`296`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_modifyConstraintCriticalSection: Pointer,
    /// # C++ Info
    /// - name: `isLocked`(ctype: `hkInt32`)
    /// - offset: `192`(x86)/`304`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_isLocked: i32,
    /// # C++ Info
    /// - name: `islandDirtyListCriticalSection`(ctype: `void*`)
    /// - offset: `196`(x86)/`312`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_islandDirtyListCriticalSection: Pointer,
    /// # C++ Info
    /// - name: `propertyMasterLock`(ctype: `void*`)
    /// - offset: `200`(x86)/`320`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_propertyMasterLock: Pointer,
    /// # C++ Info
    /// - name: `wantSimulationIslands`(ctype: `hkBool`)
    /// - offset: `204`(x86)/`328`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_wantSimulationIslands: bool,
    /// # C++ Info
    /// - name: `useHybridBroadphase`(ctype: `hkBool`)
    /// - offset: `205`(x86)/`329`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_useHybridBroadphase: bool,
    /// # C++ Info
    /// - name: `snapCollisionToConvexEdgeThreshold`(ctype: `hkReal`)
    /// - offset: `208`(x86)/`332`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapCollisionToConvexEdgeThreshold: f32,
    /// # C++ Info
    /// - name: `snapCollisionToConcaveEdgeThreshold`(ctype: `hkReal`)
    /// - offset: `212`(x86)/`336`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapCollisionToConcaveEdgeThreshold: f32,
    /// # C++ Info
    /// - name: `enableToiWeldRejection`(ctype: `hkBool`)
    /// - offset: `216`(x86)/`340`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_enableToiWeldRejection: bool,
    /// # C++ Info
    /// - name: `wantDeactivation`(ctype: `hkBool`)
    /// - offset: `217`(x86)/`341`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_wantDeactivation: bool,
    /// # C++ Info
    /// - name: `shouldActivateOnRigidBodyTransformChange`(ctype: `hkBool`)
    /// - offset: `218`(x86)/`342`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_shouldActivateOnRigidBodyTransformChange: bool,
    /// # C++ Info
    /// - name: `deactivationReferenceDistance`(ctype: `hkReal`)
    /// - offset: `220`(x86)/`344`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_deactivationReferenceDistance: f32,
    /// # C++ Info
    /// - name: `toiCollisionResponseRotateNormal`(ctype: `hkReal`)
    /// - offset: `224`(x86)/`348`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_toiCollisionResponseRotateNormal: f32,
    /// # C++ Info
    /// - name: `maxSectorsPerMidphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `228`(x86)/`352`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxSectorsPerMidphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `maxSectorsPerNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `232`(x86)/`356`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxSectorsPerNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `processToisMultithreaded`(ctype: `hkBool`)
    /// - offset: `236`(x86)/`360`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_processToisMultithreaded: bool,
    /// # C++ Info
    /// - name: `maxEntriesPerToiMidphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `240`(x86)/`364`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxEntriesPerToiMidphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `maxEntriesPerToiNarrowphaseCollideTask`(ctype: `hkInt32`)
    /// - offset: `244`(x86)/`368`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxEntriesPerToiNarrowphaseCollideTask: i32,
    /// # C++ Info
    /// - name: `maxNumToiCollisionPairsSinglethreaded`(ctype: `hkInt32`)
    /// - offset: `248`(x86)/`372`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxNumToiCollisionPairsSinglethreaded: i32,
    /// # C++ Info
    /// - name: `simulationType`(ctype: `enum unknown`)
    /// - offset: `252`(x86)/`376`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_simulationType: i32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationSimplifiedToi`(ctype: `hkReal`)
    /// - offset: `256`(x86)/`380`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationSimplifiedToi: f32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationToi`(ctype: `hkReal`)
    /// - offset: `260`(x86)/`384`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationToi: f32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationToiHigher`(ctype: `hkReal`)
    /// - offset: `264`(x86)/`388`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationToiHigher: f32,
    /// # C++ Info
    /// - name: `numToisTillAllowedPenetrationToiForced`(ctype: `hkReal`)
    /// - offset: `268`(x86)/`392`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numToisTillAllowedPenetrationToiForced: f32,
    /// # C++ Info
    /// - name: `lastEntityUid`(ctype: `hkUint32`)
    /// - offset: `272`(x86)/`396`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lastEntityUid: u32,
    /// # C++ Info
    /// - name: `lastIslandUid`(ctype: `hkUint32`)
    /// - offset: `276`(x86)/`400`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lastIslandUid: u32,
    /// # C++ Info
    /// - name: `lastConstraintUid`(ctype: `hkUint32`)
    /// - offset: `280`(x86)/`404`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lastConstraintUid: u32,
    /// # C++ Info
    /// - name: `phantoms`(ctype: `hkArray<hkpPhantom*>`)
    /// - offset: `284`(x86)/`408`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_phantoms: Vec<Pointer>,
    /// # C++ Info
    /// - name: `actionListeners`(ctype: `hkArray<void*>`)
    /// - offset: `296`(x86)/`424`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_actionListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `entityListeners`(ctype: `hkArray<void*>`)
    /// - offset: `308`(x86)/`440`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_entityListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `phantomListeners`(ctype: `hkArray<void*>`)
    /// - offset: `320`(x86)/`456`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_phantomListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `constraintListeners`(ctype: `hkArray<void*>`)
    /// - offset: `332`(x86)/`472`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_constraintListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `worldDeletionListeners`(ctype: `hkArray<void*>`)
    /// - offset: `344`(x86)/`488`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_worldDeletionListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `islandActivationListeners`(ctype: `hkArray<void*>`)
    /// - offset: `356`(x86)/`504`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_islandActivationListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `worldPostSimulationListeners`(ctype: `hkArray<void*>`)
    /// - offset: `368`(x86)/`520`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_worldPostSimulationListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `worldPostIntegrateListeners`(ctype: `hkArray<void*>`)
    /// - offset: `380`(x86)/`536`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_worldPostIntegrateListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `worldPostCollideListeners`(ctype: `hkArray<void*>`)
    /// - offset: `392`(x86)/`552`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_worldPostCollideListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `islandPostIntegrateListeners`(ctype: `hkArray<void*>`)
    /// - offset: `404`(x86)/`568`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_islandPostIntegrateListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `islandPostCollideListeners`(ctype: `hkArray<void*>`)
    /// - offset: `416`(x86)/`584`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_islandPostCollideListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `contactListeners`(ctype: `hkArray<void*>`)
    /// - offset: `428`(x86)/`600`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_contactListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `contactImpulseLimitBreachedListeners`(ctype: `hkArray<void*>`)
    /// - offset: `440`(x86)/`616`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_contactImpulseLimitBreachedListeners: Vec<Pointer>,
    /// # C++ Info
    /// - name: `worldExtensions`(ctype: `hkArray<void*>`)
    /// - offset: `452`(x86)/`632`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_worldExtensions: Vec<Pointer>,
    /// # C++ Info
    /// - name: `violatedConstraintArray`(ctype: `void*`)
    /// - offset: `464`(x86)/`648`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_violatedConstraintArray: Pointer,
    /// # C++ Info
    /// - name: `broadPhaseBorder`(ctype: `void*`)
    /// - offset: `468`(x86)/`656`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_broadPhaseBorder: Pointer,
    /// # C++ Info
    /// - name: `destructionWorld`(ctype: `void*`)
    /// - offset: `472`(x86)/`664`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_destructionWorld: Pointer,
    /// # C++ Info
    /// - name: `npWorld`(ctype: `void*`)
    /// - offset: `476`(x86)/`672`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_npWorld: Pointer,
    /// # C++ Info
    /// - name: `broadPhaseExtents`(ctype: `hkVector4[2]`)
    /// - offset: `800`(x86)/`1008`(x86_64)
    /// - type_size: ` 32`(x86)/` 32`(x86_64)
    pub m_broadPhaseExtents: [Vector4; 2usize],
    /// # C++ Info
    /// - name: `broadPhaseNumMarkers`(ctype: `hkInt32`)
    /// - offset: `832`(x86)/`1040`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_broadPhaseNumMarkers: i32,
    /// # C++ Info
    /// - name: `sizeOfToiEventQueue`(ctype: `hkInt32`)
    /// - offset: `836`(x86)/`1044`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_sizeOfToiEventQueue: i32,
    /// # C++ Info
    /// - name: `broadPhaseQuerySize`(ctype: `hkInt32`)
    /// - offset: `840`(x86)/`1048`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_broadPhaseQuerySize: i32,
    /// # C++ Info
    /// - name: `broadPhaseUpdateSize`(ctype: `hkInt32`)
    /// - offset: `844`(x86)/`1052`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_broadPhaseUpdateSize: i32,
    /// # C++ Info
    /// - name: `contactPointGeneration`(ctype: `enum unknown`)
    /// - offset: `848`(x86)/`1056`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
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
                .serialize_fixed_array_field(
                    "broadPhaseExtents",
                    self.m_broadPhaseExtents.as_slice(),
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpWorld {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_broadPhaseUpdateSize,
                m_broadPhaseQuerySize,
                m_sizeOfToiEventQueue,
                m_broadPhaseNumMarkers,
                m_broadPhaseExtents,
                m_phantoms,
                m_lastConstraintUid,
                m_lastIslandUid,
                m_lastEntityUid,
                m_numToisTillAllowedPenetrationToiForced,
                m_numToisTillAllowedPenetrationToiHigher,
                m_numToisTillAllowedPenetrationToi,
                m_numToisTillAllowedPenetrationSimplifiedToi,
                m_maxNumToiCollisionPairsSinglethreaded,
                m_maxEntriesPerToiNarrowphaseCollideTask,
                m_maxEntriesPerToiMidphaseCollideTask,
                m_processToisMultithreaded,
                m_maxSectorsPerNarrowphaseCollideTask,
                m_maxSectorsPerMidphaseCollideTask,
                m_toiCollisionResponseRotateNormal,
                m_deactivationReferenceDistance,
                m_shouldActivateOnRigidBodyTransformChange,
                m_wantDeactivation,
                m_enableToiWeldRejection,
                m_snapCollisionToConcaveEdgeThreshold,
                m_snapCollisionToConvexEdgeThreshold,
                m_wantSimulationIslands,
                m_isLocked,
                m_minDesiredIslandSize,
                m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                m_processActionsInSingleThread,
                m_pendingOperationQueueCount,
                m_criticalOperationsAllowed,
                m_blockExecutingPendingOperations,
                m_criticalOperationsLockCountForPhantoms,
                m_criticalOperationsLockCount,
                m_pendingOperationsCount,
                m_autoUpdateTree,
                m_fixedRigidBody,
                m_gravity,
                m_simulation,
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
                        "broadPhaseUpdateSize" => Ok(__Field::m_broadPhaseUpdateSize),
                        "broadPhaseQuerySize" => Ok(__Field::m_broadPhaseQuerySize),
                        "sizeOfToiEventQueue" => Ok(__Field::m_sizeOfToiEventQueue),
                        "broadPhaseNumMarkers" => Ok(__Field::m_broadPhaseNumMarkers),
                        "broadPhaseExtents" => Ok(__Field::m_broadPhaseExtents),
                        "phantoms" => Ok(__Field::m_phantoms),
                        "lastConstraintUid" => Ok(__Field::m_lastConstraintUid),
                        "lastIslandUid" => Ok(__Field::m_lastIslandUid),
                        "lastEntityUid" => Ok(__Field::m_lastEntityUid),
                        "numToisTillAllowedPenetrationToiForced" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationToiForced)
                        }
                        "numToisTillAllowedPenetrationToiHigher" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationToiHigher)
                        }
                        "numToisTillAllowedPenetrationToi" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationToi)
                        }
                        "numToisTillAllowedPenetrationSimplifiedToi" => {
                            Ok(__Field::m_numToisTillAllowedPenetrationSimplifiedToi)
                        }
                        "maxNumToiCollisionPairsSinglethreaded" => {
                            Ok(__Field::m_maxNumToiCollisionPairsSinglethreaded)
                        }
                        "maxEntriesPerToiNarrowphaseCollideTask" => {
                            Ok(__Field::m_maxEntriesPerToiNarrowphaseCollideTask)
                        }
                        "maxEntriesPerToiMidphaseCollideTask" => {
                            Ok(__Field::m_maxEntriesPerToiMidphaseCollideTask)
                        }
                        "processToisMultithreaded" => {
                            Ok(__Field::m_processToisMultithreaded)
                        }
                        "maxSectorsPerNarrowphaseCollideTask" => {
                            Ok(__Field::m_maxSectorsPerNarrowphaseCollideTask)
                        }
                        "maxSectorsPerMidphaseCollideTask" => {
                            Ok(__Field::m_maxSectorsPerMidphaseCollideTask)
                        }
                        "toiCollisionResponseRotateNormal" => {
                            Ok(__Field::m_toiCollisionResponseRotateNormal)
                        }
                        "deactivationReferenceDistance" => {
                            Ok(__Field::m_deactivationReferenceDistance)
                        }
                        "shouldActivateOnRigidBodyTransformChange" => {
                            Ok(__Field::m_shouldActivateOnRigidBodyTransformChange)
                        }
                        "wantDeactivation" => Ok(__Field::m_wantDeactivation),
                        "enableToiWeldRejection" => Ok(__Field::m_enableToiWeldRejection),
                        "snapCollisionToConcaveEdgeThreshold" => {
                            Ok(__Field::m_snapCollisionToConcaveEdgeThreshold)
                        }
                        "snapCollisionToConvexEdgeThreshold" => {
                            Ok(__Field::m_snapCollisionToConvexEdgeThreshold)
                        }
                        "wantSimulationIslands" => Ok(__Field::m_wantSimulationIslands),
                        "isLocked" => Ok(__Field::m_isLocked),
                        "minDesiredIslandSize" => Ok(__Field::m_minDesiredIslandSize),
                        "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob" => {
                            Ok(
                                __Field::m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                            )
                        }
                        "processActionsInSingleThread" => {
                            Ok(__Field::m_processActionsInSingleThread)
                        }
                        "pendingOperationQueueCount" => {
                            Ok(__Field::m_pendingOperationQueueCount)
                        }
                        "criticalOperationsAllowed" => {
                            Ok(__Field::m_criticalOperationsAllowed)
                        }
                        "blockExecutingPendingOperations" => {
                            Ok(__Field::m_blockExecutingPendingOperations)
                        }
                        "criticalOperationsLockCountForPhantoms" => {
                            Ok(__Field::m_criticalOperationsLockCountForPhantoms)
                        }
                        "criticalOperationsLockCount" => {
                            Ok(__Field::m_criticalOperationsLockCount)
                        }
                        "pendingOperationsCount" => Ok(__Field::m_pendingOperationsCount),
                        "autoUpdateTree" => Ok(__Field::m_autoUpdateTree),
                        "fixedRigidBody" => Ok(__Field::m_fixedRigidBody),
                        "gravity" => Ok(__Field::m_gravity),
                        "simulation" => Ok(__Field::m_simulation),
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
            struct __hkpWorldVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpWorld>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpWorldVisitor<'de> {
                type Value = hkpWorld;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkpWorld")
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
                    let mut m_simulation: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_fixedIsland: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_fixedRigidBody: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_activeSimulationIslands: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_inactiveSimulationIslands: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_dirtySimulationIslands: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_maintenanceMgr: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_memoryWatchDog: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_assertOnRunningOutOfSolverMemory: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_broadPhase: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_kdTreeManager: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_autoUpdateTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_broadPhaseDispatcher: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_phantomBroadPhaseListener: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_entityEntityBroadPhaseListener: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_broadPhaseBorderListener: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_multithreadedSimulationJobData: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_collisionInput: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_collisionFilter: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_collisionDispatcher: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_convexListFilter: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_pendingOperations: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_pendingOperationsCount: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_pendingBodyOperationsCount: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_criticalOperationsLockCount: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_criticalOperationsLockCountForPhantoms: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_blockExecutingPendingOperations: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_criticalOperationsAllowed: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_pendingOperationQueues: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_pendingOperationQueueCount: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_multiThreadCheck: _serde::__private::Option<
                        hkMultiThreadCheck,
                    > = _serde::__private::None;
                    let mut m_processActionsInSingleThread: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_minDesiredIslandSize: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_modifyConstraintCriticalSection: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_isLocked: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_islandDirtyListCriticalSection: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_propertyMasterLock: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_wantSimulationIslands: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useHybridBroadphase: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_snapCollisionToConvexEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_snapCollisionToConcaveEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_enableToiWeldRejection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_wantDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
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
                    let mut m_simulationType: _serde::__private::Option<i32> = _serde::__private::None;
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
                    let mut m_lastEntityUid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_lastIslandUid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_lastConstraintUid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_phantoms: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_actionListeners: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_entityListeners: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_phantomListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_constraintListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_worldDeletionListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_islandActivationListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_worldPostSimulationListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_worldPostIntegrateListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_worldPostCollideListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_islandPostIntegrateListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_islandPostCollideListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_contactListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_contactImpulseLimitBreachedListeners: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_worldExtensions: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_violatedConstraintArray: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_broadPhaseBorder: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_destructionWorld: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_npWorld: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_broadPhaseExtents: _serde::__private::Option<
                        [Vector4; 2usize],
                    > = _serde::__private::None;
                    let mut m_broadPhaseNumMarkers: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_sizeOfToiEventQueue: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_broadPhaseQuerySize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_broadPhaseUpdateSize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_contactPointGeneration: _serde::__private::Option<i8> = _serde::__private::None;
                    for i in 0..87usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_simulation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulation",
                                        ),
                                    );
                                }
                                m_simulation = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_gravity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravity",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 8usize)?;
                                m_gravity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_fixedIsland) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedIsland",
                                        ),
                                    );
                                }
                                m_fixedIsland = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_fixedRigidBody) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedRigidBody",
                                        ),
                                    );
                                }
                                m_fixedRigidBody = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_activeSimulationIslands,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeSimulationIslands",
                                        ),
                                    );
                                }
                                m_activeSimulationIslands = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_inactiveSimulationIslands,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inactiveSimulationIslands",
                                        ),
                                    );
                                }
                                m_inactiveSimulationIslands = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_dirtySimulationIslands,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dirtySimulationIslands",
                                        ),
                                    );
                                }
                                m_dirtySimulationIslands = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_maintenanceMgr) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maintenanceMgr",
                                        ),
                                    );
                                }
                                m_maintenanceMgr = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_memoryWatchDog) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_assertOnRunningOutOfSolverMemory,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "assertOnRunningOutOfSolverMemory",
                                        ),
                                    );
                                }
                                m_assertOnRunningOutOfSolverMemory = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_broadPhase) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhase",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_broadPhase = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_kdTreeManager) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kdTreeManager",
                                        ),
                                    );
                                }
                                m_kdTreeManager = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_autoUpdateTree) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "autoUpdateTree",
                                        ),
                                    );
                                }
                                m_autoUpdateTree = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseDispatcher,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseDispatcher",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_broadPhaseDispatcher = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_phantomBroadPhaseListener,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "phantomBroadPhaseListener",
                                        ),
                                    );
                                }
                                m_phantomBroadPhaseListener = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_entityEntityBroadPhaseListener,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "entityEntityBroadPhaseListener",
                                        ),
                                    );
                                }
                                m_entityEntityBroadPhaseListener = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseBorderListener,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseBorderListener",
                                        ),
                                    );
                                }
                                m_broadPhaseBorderListener = _serde::__private::Some(
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
                                    &m_multithreadedSimulationJobData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "multithreadedSimulationJobData",
                                        ),
                                    );
                                }
                                m_multithreadedSimulationJobData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_collisionInput) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionInput",
                                        ),
                                    );
                                }
                                m_collisionInput = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
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
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionDispatcher,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionDispatcher",
                                        ),
                                    );
                                }
                                m_collisionDispatcher = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
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
                            22usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pendingOperations,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingOperations",
                                        ),
                                    );
                                }
                                m_pendingOperations = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pendingOperationsCount,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingOperationsCount",
                                        ),
                                    );
                                }
                                m_pendingOperationsCount = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pendingBodyOperationsCount,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingBodyOperationsCount",
                                        ),
                                    );
                                }
                                m_pendingBodyOperationsCount = _serde::__private::Some(
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
                                    &m_criticalOperationsLockCount,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalOperationsLockCount",
                                        ),
                                    );
                                }
                                m_criticalOperationsLockCount = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(
                                    &m_criticalOperationsLockCountForPhantoms,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalOperationsLockCountForPhantoms",
                                        ),
                                    );
                                }
                                m_criticalOperationsLockCountForPhantoms = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_blockExecutingPendingOperations,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blockExecutingPendingOperations",
                                        ),
                                    );
                                }
                                m_blockExecutingPendingOperations = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_criticalOperationsAllowed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalOperationsAllowed",
                                        ),
                                    );
                                }
                                m_criticalOperationsAllowed = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pendingOperationQueues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingOperationQueues",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 6usize)?;
                                m_pendingOperationQueues = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pendingOperationQueueCount,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingOperationQueueCount",
                                        ),
                                    );
                                }
                                m_pendingOperationQueueCount = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(&m_multiThreadCheck) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "multiThreadCheck",
                                        ),
                                    );
                                }
                                m_multiThreadCheck = _serde::__private::Some(
                                    match __A::next_value::<hkMultiThreadCheck>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
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
                            33usize => {
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
                            34usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minDesiredIslandSize,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minDesiredIslandSize",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_minDesiredIslandSize = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(
                                    &m_modifyConstraintCriticalSection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modifyConstraintCriticalSection",
                                        ),
                                    );
                                }
                                m_modifyConstraintCriticalSection = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            36usize => {
                                if _serde::__private::Option::is_some(&m_isLocked) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isLocked",
                                        ),
                                    );
                                }
                                m_isLocked = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            37usize => {
                                if _serde::__private::Option::is_some(
                                    &m_islandDirtyListCriticalSection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "islandDirtyListCriticalSection",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_islandDirtyListCriticalSection = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            38usize => {
                                if _serde::__private::Option::is_some(
                                    &m_propertyMasterLock,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "propertyMasterLock",
                                        ),
                                    );
                                }
                                m_propertyMasterLock = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            39usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wantSimulationIslands,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wantSimulationIslands",
                                        ),
                                    );
                                }
                                m_wantSimulationIslands = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            40usize => {
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
                            41usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapCollisionToConvexEdgeThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapCollisionToConvexEdgeThreshold",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_snapCollisionToConvexEdgeThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            42usize => {
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
                            43usize => {
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
                            44usize => {
                                if _serde::__private::Option::is_some(&m_wantDeactivation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wantDeactivation",
                                        ),
                                    );
                                }
                                m_wantDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            45usize => {
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
                            46usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationReferenceDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationReferenceDistance",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 1usize)?;
                                m_deactivationReferenceDistance = _serde::__private::Some(
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
                            48usize => {
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
                            49usize => {
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
                            50usize => {
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
                            51usize => {
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
                            52usize => {
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
                            53usize => {
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
                            54usize => {
                                if _serde::__private::Option::is_some(&m_simulationType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationType",
                                        ),
                                    );
                                }
                                m_simulationType = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            55usize => {
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
                            56usize => {
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
                            57usize => {
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
                            58usize => {
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
                            59usize => {
                                if _serde::__private::Option::is_some(&m_lastEntityUid) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastEntityUid",
                                        ),
                                    );
                                }
                                m_lastEntityUid = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            60usize => {
                                if _serde::__private::Option::is_some(&m_lastIslandUid) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastIslandUid",
                                        ),
                                    );
                                }
                                m_lastIslandUid = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            61usize => {
                                if _serde::__private::Option::is_some(
                                    &m_lastConstraintUid,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastConstraintUid",
                                        ),
                                    );
                                }
                                m_lastConstraintUid = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            62usize => {
                                if _serde::__private::Option::is_some(&m_phantoms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "phantoms",
                                        ),
                                    );
                                }
                                m_phantoms = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            63usize => {
                                if _serde::__private::Option::is_some(&m_actionListeners) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "actionListeners",
                                        ),
                                    );
                                }
                                m_actionListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            64usize => {
                                if _serde::__private::Option::is_some(&m_entityListeners) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "entityListeners",
                                        ),
                                    );
                                }
                                m_entityListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            65usize => {
                                if _serde::__private::Option::is_some(&m_phantomListeners) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "phantomListeners",
                                        ),
                                    );
                                }
                                m_phantomListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            66usize => {
                                if _serde::__private::Option::is_some(
                                    &m_constraintListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintListeners",
                                        ),
                                    );
                                }
                                m_constraintListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            67usize => {
                                if _serde::__private::Option::is_some(
                                    &m_worldDeletionListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldDeletionListeners",
                                        ),
                                    );
                                }
                                m_worldDeletionListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            68usize => {
                                if _serde::__private::Option::is_some(
                                    &m_islandActivationListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "islandActivationListeners",
                                        ),
                                    );
                                }
                                m_islandActivationListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            69usize => {
                                if _serde::__private::Option::is_some(
                                    &m_worldPostSimulationListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldPostSimulationListeners",
                                        ),
                                    );
                                }
                                m_worldPostSimulationListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            70usize => {
                                if _serde::__private::Option::is_some(
                                    &m_worldPostIntegrateListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldPostIntegrateListeners",
                                        ),
                                    );
                                }
                                m_worldPostIntegrateListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            71usize => {
                                if _serde::__private::Option::is_some(
                                    &m_worldPostCollideListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldPostCollideListeners",
                                        ),
                                    );
                                }
                                m_worldPostCollideListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            72usize => {
                                if _serde::__private::Option::is_some(
                                    &m_islandPostIntegrateListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "islandPostIntegrateListeners",
                                        ),
                                    );
                                }
                                m_islandPostIntegrateListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            73usize => {
                                if _serde::__private::Option::is_some(
                                    &m_islandPostCollideListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "islandPostCollideListeners",
                                        ),
                                    );
                                }
                                m_islandPostCollideListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            74usize => {
                                if _serde::__private::Option::is_some(&m_contactListeners) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactListeners",
                                        ),
                                    );
                                }
                                m_contactListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            75usize => {
                                if _serde::__private::Option::is_some(
                                    &m_contactImpulseLimitBreachedListeners,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactImpulseLimitBreachedListeners",
                                        ),
                                    );
                                }
                                m_contactImpulseLimitBreachedListeners = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            76usize => {
                                if _serde::__private::Option::is_some(&m_worldExtensions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldExtensions",
                                        ),
                                    );
                                }
                                m_worldExtensions = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            77usize => {
                                if _serde::__private::Option::is_some(
                                    &m_violatedConstraintArray,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "violatedConstraintArray",
                                        ),
                                    );
                                }
                                m_violatedConstraintArray = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            78usize => {
                                if _serde::__private::Option::is_some(&m_broadPhaseBorder) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseBorder",
                                        ),
                                    );
                                }
                                m_broadPhaseBorder = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            79usize => {
                                if _serde::__private::Option::is_some(&m_destructionWorld) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "destructionWorld",
                                        ),
                                    );
                                }
                                m_destructionWorld = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            80usize => {
                                if _serde::__private::Option::is_some(&m_npWorld) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "npWorld",
                                        ),
                                    );
                                }
                                m_npWorld = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            81usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseExtents,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseExtents",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 320usize, 328usize)?;
                                m_broadPhaseExtents = _serde::__private::Some(
                                    match __A::next_value::<[Vector4; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            82usize => {
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
                            83usize => {
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
                            84usize => {
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
                            85usize => {
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseUpdateSize,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseUpdateSize",
                                        ),
                                    );
                                }
                                m_broadPhaseUpdateSize = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            86usize => {
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
                                    match __A::next_value::<i8>(&mut __map) {
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
                    __A::pad(&mut __map, 15usize, 15usize)?;
                    let m_simulation = match m_simulation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulation",
                                ),
                            );
                        }
                    };
                    let m_gravity = match m_gravity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gravity"),
                            );
                        }
                    };
                    let m_fixedIsland = match m_fixedIsland {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedIsland",
                                ),
                            );
                        }
                    };
                    let m_fixedRigidBody = match m_fixedRigidBody {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedRigidBody",
                                ),
                            );
                        }
                    };
                    let m_activeSimulationIslands = match m_activeSimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeSimulationIslands",
                                ),
                            );
                        }
                    };
                    let m_inactiveSimulationIslands = match m_inactiveSimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inactiveSimulationIslands",
                                ),
                            );
                        }
                    };
                    let m_dirtySimulationIslands = match m_dirtySimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dirtySimulationIslands",
                                ),
                            );
                        }
                    };
                    let m_maintenanceMgr = match m_maintenanceMgr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maintenanceMgr",
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
                    let m_assertOnRunningOutOfSolverMemory = match m_assertOnRunningOutOfSolverMemory {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "assertOnRunningOutOfSolverMemory",
                                ),
                            );
                        }
                    };
                    let m_broadPhase = match m_broadPhase {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhase",
                                ),
                            );
                        }
                    };
                    let m_kdTreeManager = match m_kdTreeManager {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kdTreeManager",
                                ),
                            );
                        }
                    };
                    let m_autoUpdateTree = match m_autoUpdateTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "autoUpdateTree",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseDispatcher = match m_broadPhaseDispatcher {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseDispatcher",
                                ),
                            );
                        }
                    };
                    let m_phantomBroadPhaseListener = match m_phantomBroadPhaseListener {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "phantomBroadPhaseListener",
                                ),
                            );
                        }
                    };
                    let m_entityEntityBroadPhaseListener = match m_entityEntityBroadPhaseListener {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "entityEntityBroadPhaseListener",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseBorderListener = match m_broadPhaseBorderListener {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseBorderListener",
                                ),
                            );
                        }
                    };
                    let m_multithreadedSimulationJobData = match m_multithreadedSimulationJobData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "multithreadedSimulationJobData",
                                ),
                            );
                        }
                    };
                    let m_collisionInput = match m_collisionInput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionInput",
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
                    let m_collisionDispatcher = match m_collisionDispatcher {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionDispatcher",
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
                    let m_pendingOperations = match m_pendingOperations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingOperations",
                                ),
                            );
                        }
                    };
                    let m_pendingOperationsCount = match m_pendingOperationsCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingOperationsCount",
                                ),
                            );
                        }
                    };
                    let m_pendingBodyOperationsCount = match m_pendingBodyOperationsCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingBodyOperationsCount",
                                ),
                            );
                        }
                    };
                    let m_criticalOperationsLockCount = match m_criticalOperationsLockCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalOperationsLockCount",
                                ),
                            );
                        }
                    };
                    let m_criticalOperationsLockCountForPhantoms = match m_criticalOperationsLockCountForPhantoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalOperationsLockCountForPhantoms",
                                ),
                            );
                        }
                    };
                    let m_blockExecutingPendingOperations = match m_blockExecutingPendingOperations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blockExecutingPendingOperations",
                                ),
                            );
                        }
                    };
                    let m_criticalOperationsAllowed = match m_criticalOperationsAllowed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalOperationsAllowed",
                                ),
                            );
                        }
                    };
                    let m_pendingOperationQueues = match m_pendingOperationQueues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingOperationQueues",
                                ),
                            );
                        }
                    };
                    let m_pendingOperationQueueCount = match m_pendingOperationQueueCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingOperationQueueCount",
                                ),
                            );
                        }
                    };
                    let m_multiThreadCheck = match m_multiThreadCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "multiThreadCheck",
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
                    let m_modifyConstraintCriticalSection = match m_modifyConstraintCriticalSection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modifyConstraintCriticalSection",
                                ),
                            );
                        }
                    };
                    let m_isLocked = match m_isLocked {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isLocked"),
                            );
                        }
                    };
                    let m_islandDirtyListCriticalSection = match m_islandDirtyListCriticalSection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "islandDirtyListCriticalSection",
                                ),
                            );
                        }
                    };
                    let m_propertyMasterLock = match m_propertyMasterLock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "propertyMasterLock",
                                ),
                            );
                        }
                    };
                    let m_wantSimulationIslands = match m_wantSimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wantSimulationIslands",
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
                    let m_wantDeactivation = match m_wantDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wantDeactivation",
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
                    let m_lastEntityUid = match m_lastEntityUid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastEntityUid",
                                ),
                            );
                        }
                    };
                    let m_lastIslandUid = match m_lastIslandUid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastIslandUid",
                                ),
                            );
                        }
                    };
                    let m_lastConstraintUid = match m_lastConstraintUid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastConstraintUid",
                                ),
                            );
                        }
                    };
                    let m_phantoms = match m_phantoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("phantoms"),
                            );
                        }
                    };
                    let m_actionListeners = match m_actionListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "actionListeners",
                                ),
                            );
                        }
                    };
                    let m_entityListeners = match m_entityListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "entityListeners",
                                ),
                            );
                        }
                    };
                    let m_phantomListeners = match m_phantomListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "phantomListeners",
                                ),
                            );
                        }
                    };
                    let m_constraintListeners = match m_constraintListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintListeners",
                                ),
                            );
                        }
                    };
                    let m_worldDeletionListeners = match m_worldDeletionListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldDeletionListeners",
                                ),
                            );
                        }
                    };
                    let m_islandActivationListeners = match m_islandActivationListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "islandActivationListeners",
                                ),
                            );
                        }
                    };
                    let m_worldPostSimulationListeners = match m_worldPostSimulationListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldPostSimulationListeners",
                                ),
                            );
                        }
                    };
                    let m_worldPostIntegrateListeners = match m_worldPostIntegrateListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldPostIntegrateListeners",
                                ),
                            );
                        }
                    };
                    let m_worldPostCollideListeners = match m_worldPostCollideListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldPostCollideListeners",
                                ),
                            );
                        }
                    };
                    let m_islandPostIntegrateListeners = match m_islandPostIntegrateListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "islandPostIntegrateListeners",
                                ),
                            );
                        }
                    };
                    let m_islandPostCollideListeners = match m_islandPostCollideListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "islandPostCollideListeners",
                                ),
                            );
                        }
                    };
                    let m_contactListeners = match m_contactListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactListeners",
                                ),
                            );
                        }
                    };
                    let m_contactImpulseLimitBreachedListeners = match m_contactImpulseLimitBreachedListeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactImpulseLimitBreachedListeners",
                                ),
                            );
                        }
                    };
                    let m_worldExtensions = match m_worldExtensions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldExtensions",
                                ),
                            );
                        }
                    };
                    let m_violatedConstraintArray = match m_violatedConstraintArray {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "violatedConstraintArray",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseBorder = match m_broadPhaseBorder {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseBorder",
                                ),
                            );
                        }
                    };
                    let m_destructionWorld = match m_destructionWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "destructionWorld",
                                ),
                            );
                        }
                    };
                    let m_npWorld = match m_npWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("npWorld"),
                            );
                        }
                    };
                    let m_broadPhaseExtents = match m_broadPhaseExtents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseExtents",
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
                    let m_broadPhaseUpdateSize = match m_broadPhaseUpdateSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseUpdateSize",
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
                    _serde::__private::Ok(hkpWorld {
                        __ptr,
                        parent,
                        m_simulation,
                        m_gravity,
                        m_fixedIsland,
                        m_fixedRigidBody,
                        m_activeSimulationIslands,
                        m_inactiveSimulationIslands,
                        m_dirtySimulationIslands,
                        m_maintenanceMgr,
                        m_memoryWatchDog,
                        m_assertOnRunningOutOfSolverMemory,
                        m_broadPhase,
                        m_kdTreeManager,
                        m_autoUpdateTree,
                        m_broadPhaseDispatcher,
                        m_phantomBroadPhaseListener,
                        m_entityEntityBroadPhaseListener,
                        m_broadPhaseBorderListener,
                        m_multithreadedSimulationJobData,
                        m_collisionInput,
                        m_collisionFilter,
                        m_collisionDispatcher,
                        m_convexListFilter,
                        m_pendingOperations,
                        m_pendingOperationsCount,
                        m_pendingBodyOperationsCount,
                        m_criticalOperationsLockCount,
                        m_criticalOperationsLockCountForPhantoms,
                        m_blockExecutingPendingOperations,
                        m_criticalOperationsAllowed,
                        m_pendingOperationQueues,
                        m_pendingOperationQueueCount,
                        m_multiThreadCheck,
                        m_processActionsInSingleThread,
                        m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                        m_minDesiredIslandSize,
                        m_modifyConstraintCriticalSection,
                        m_isLocked,
                        m_islandDirtyListCriticalSection,
                        m_propertyMasterLock,
                        m_wantSimulationIslands,
                        m_useHybridBroadphase,
                        m_snapCollisionToConvexEdgeThreshold,
                        m_snapCollisionToConcaveEdgeThreshold,
                        m_enableToiWeldRejection,
                        m_wantDeactivation,
                        m_shouldActivateOnRigidBodyTransformChange,
                        m_deactivationReferenceDistance,
                        m_toiCollisionResponseRotateNormal,
                        m_maxSectorsPerMidphaseCollideTask,
                        m_maxSectorsPerNarrowphaseCollideTask,
                        m_processToisMultithreaded,
                        m_maxEntriesPerToiMidphaseCollideTask,
                        m_maxEntriesPerToiNarrowphaseCollideTask,
                        m_maxNumToiCollisionPairsSinglethreaded,
                        m_simulationType,
                        m_numToisTillAllowedPenetrationSimplifiedToi,
                        m_numToisTillAllowedPenetrationToi,
                        m_numToisTillAllowedPenetrationToiHigher,
                        m_numToisTillAllowedPenetrationToiForced,
                        m_lastEntityUid,
                        m_lastIslandUid,
                        m_lastConstraintUid,
                        m_phantoms,
                        m_actionListeners,
                        m_entityListeners,
                        m_phantomListeners,
                        m_constraintListeners,
                        m_worldDeletionListeners,
                        m_islandActivationListeners,
                        m_worldPostSimulationListeners,
                        m_worldPostIntegrateListeners,
                        m_worldPostCollideListeners,
                        m_islandPostIntegrateListeners,
                        m_islandPostCollideListeners,
                        m_contactListeners,
                        m_contactImpulseLimitBreachedListeners,
                        m_worldExtensions,
                        m_violatedConstraintArray,
                        m_broadPhaseBorder,
                        m_destructionWorld,
                        m_npWorld,
                        m_broadPhaseExtents,
                        m_broadPhaseNumMarkers,
                        m_sizeOfToiEventQueue,
                        m_broadPhaseQuerySize,
                        m_broadPhaseUpdateSize,
                        m_contactPointGeneration,
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
                    let mut m_broadPhaseUpdateSize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_broadPhaseQuerySize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_sizeOfToiEventQueue: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_broadPhaseNumMarkers: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_broadPhaseExtents: _serde::__private::Option<
                        [Vector4; 2usize],
                    > = _serde::__private::None;
                    let mut m_phantoms: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_lastConstraintUid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_lastIslandUid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_lastEntityUid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToiForced: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToiHigher: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationToi: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_numToisTillAllowedPenetrationSimplifiedToi: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxNumToiCollisionPairsSinglethreaded: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxEntriesPerToiNarrowphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxEntriesPerToiMidphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_processToisMultithreaded: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_maxSectorsPerNarrowphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_maxSectorsPerMidphaseCollideTask: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_toiCollisionResponseRotateNormal: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_deactivationReferenceDistance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_shouldActivateOnRigidBodyTransformChange: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_wantDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_enableToiWeldRejection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_snapCollisionToConcaveEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_snapCollisionToConvexEdgeThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_wantSimulationIslands: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isLocked: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_minDesiredIslandSize: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_processActionsInSingleThread: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_pendingOperationQueueCount: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_criticalOperationsAllowed: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_blockExecutingPendingOperations: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_criticalOperationsLockCountForPhantoms: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_criticalOperationsLockCount: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_pendingOperationsCount: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_autoUpdateTree: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_fixedRigidBody: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_simulation: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_broadPhaseUpdateSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseUpdateSize,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseUpdateSize",
                                        ),
                                    );
                                }
                                m_broadPhaseUpdateSize = _serde::__private::Some(
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
                            __Field::m_broadPhaseExtents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_broadPhaseExtents,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseExtents",
                                        ),
                                    );
                                }
                                m_broadPhaseExtents = _serde::__private::Some(
                                    match __A::next_value::<[Vector4; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_phantoms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_phantoms) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "phantoms",
                                        ),
                                    );
                                }
                                m_phantoms = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_lastConstraintUid => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_lastConstraintUid,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastConstraintUid",
                                        ),
                                    );
                                }
                                m_lastConstraintUid = _serde::__private::Some(
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
                            __Field::m_lastIslandUid => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lastIslandUid) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastIslandUid",
                                        ),
                                    );
                                }
                                m_lastIslandUid = _serde::__private::Some(
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
                            __Field::m_lastEntityUid => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lastEntityUid) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastEntityUid",
                                        ),
                                    );
                                }
                                m_lastEntityUid = _serde::__private::Some(
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
                            __Field::m_wantDeactivation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wantDeactivation) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wantDeactivation",
                                        ),
                                    );
                                }
                                m_wantDeactivation = _serde::__private::Some(
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
                            __Field::m_wantSimulationIslands => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wantSimulationIslands,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wantSimulationIslands",
                                        ),
                                    );
                                }
                                m_wantSimulationIslands = _serde::__private::Some(
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
                            __Field::m_isLocked => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isLocked) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isLocked",
                                        ),
                                    );
                                }
                                m_isLocked = _serde::__private::Some(
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
                            __Field::m_pendingOperationQueueCount => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pendingOperationQueueCount,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingOperationQueueCount",
                                        ),
                                    );
                                }
                                m_pendingOperationQueueCount = _serde::__private::Some(
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
                            __Field::m_criticalOperationsAllowed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_criticalOperationsAllowed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalOperationsAllowed",
                                        ),
                                    );
                                }
                                m_criticalOperationsAllowed = _serde::__private::Some(
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
                            __Field::m_blockExecutingPendingOperations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_blockExecutingPendingOperations,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blockExecutingPendingOperations",
                                        ),
                                    );
                                }
                                m_blockExecutingPendingOperations = _serde::__private::Some(
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
                            __Field::m_criticalOperationsLockCountForPhantoms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_criticalOperationsLockCountForPhantoms,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalOperationsLockCountForPhantoms",
                                        ),
                                    );
                                }
                                m_criticalOperationsLockCountForPhantoms = _serde::__private::Some(
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
                            __Field::m_criticalOperationsLockCount => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_criticalOperationsLockCount,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalOperationsLockCount",
                                        ),
                                    );
                                }
                                m_criticalOperationsLockCount = _serde::__private::Some(
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
                            __Field::m_pendingOperationsCount => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pendingOperationsCount,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pendingOperationsCount",
                                        ),
                                    );
                                }
                                m_pendingOperationsCount = _serde::__private::Some(
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
                            __Field::m_autoUpdateTree => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_autoUpdateTree) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "autoUpdateTree",
                                        ),
                                    );
                                }
                                m_autoUpdateTree = _serde::__private::Some(
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
                            __Field::m_fixedRigidBody => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fixedRigidBody) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedRigidBody",
                                        ),
                                    );
                                }
                                m_fixedRigidBody = _serde::__private::Some(
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
                            __Field::m_simulation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simulation) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulation",
                                        ),
                                    );
                                }
                                m_simulation = _serde::__private::Some(
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
                            _ => {}
                        }
                    }
                    let m_broadPhaseUpdateSize = match m_broadPhaseUpdateSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseUpdateSize",
                                ),
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
                    let m_broadPhaseExtents = match m_broadPhaseExtents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseExtents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_phantoms = match m_phantoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("phantoms"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastConstraintUid = match m_lastConstraintUid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastConstraintUid",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastIslandUid = match m_lastIslandUid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastIslandUid",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastEntityUid = match m_lastEntityUid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastEntityUid",
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
                    let m_wantDeactivation = match m_wantDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wantDeactivation",
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
                    let m_wantSimulationIslands = match m_wantSimulationIslands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wantSimulationIslands",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isLocked = match m_isLocked {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isLocked"),
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
                    let m_pendingOperationQueueCount = match m_pendingOperationQueueCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingOperationQueueCount",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_criticalOperationsAllowed = match m_criticalOperationsAllowed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalOperationsAllowed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blockExecutingPendingOperations = match m_blockExecutingPendingOperations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blockExecutingPendingOperations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_criticalOperationsLockCountForPhantoms = match m_criticalOperationsLockCountForPhantoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalOperationsLockCountForPhantoms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_criticalOperationsLockCount = match m_criticalOperationsLockCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalOperationsLockCount",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pendingOperationsCount = match m_pendingOperationsCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pendingOperationsCount",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_autoUpdateTree = match m_autoUpdateTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "autoUpdateTree",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fixedRigidBody = match m_fixedRigidBody {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedRigidBody",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let m_simulation = match m_simulation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulation",
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
                    _serde::__private::Ok(hkpWorld {
                        __ptr,
                        parent,
                        m_simulation,
                        m_gravity,
                        m_fixedRigidBody,
                        m_autoUpdateTree,
                        m_pendingOperationsCount,
                        m_criticalOperationsLockCount,
                        m_criticalOperationsLockCountForPhantoms,
                        m_blockExecutingPendingOperations,
                        m_criticalOperationsAllowed,
                        m_pendingOperationQueueCount,
                        m_processActionsInSingleThread,
                        m_allowIntegrationOfIslandsWithoutConstraintsInASeparateJob,
                        m_minDesiredIslandSize,
                        m_isLocked,
                        m_wantSimulationIslands,
                        m_snapCollisionToConvexEdgeThreshold,
                        m_snapCollisionToConcaveEdgeThreshold,
                        m_enableToiWeldRejection,
                        m_wantDeactivation,
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
                        m_lastEntityUid,
                        m_lastIslandUid,
                        m_lastConstraintUid,
                        m_phantoms,
                        m_broadPhaseExtents,
                        m_broadPhaseNumMarkers,
                        m_sizeOfToiEventQueue,
                        m_broadPhaseQuerySize,
                        m_broadPhaseUpdateSize,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "simulation",
                "gravity",
                "fixedIsland",
                "fixedRigidBody",
                "activeSimulationIslands",
                "inactiveSimulationIslands",
                "dirtySimulationIslands",
                "maintenanceMgr",
                "memoryWatchDog",
                "assertOnRunningOutOfSolverMemory",
                "broadPhase",
                "kdTreeManager",
                "autoUpdateTree",
                "broadPhaseDispatcher",
                "phantomBroadPhaseListener",
                "entityEntityBroadPhaseListener",
                "broadPhaseBorderListener",
                "multithreadedSimulationJobData",
                "collisionInput",
                "collisionFilter",
                "collisionDispatcher",
                "convexListFilter",
                "pendingOperations",
                "pendingOperationsCount",
                "pendingBodyOperationsCount",
                "criticalOperationsLockCount",
                "criticalOperationsLockCountForPhantoms",
                "blockExecutingPendingOperations",
                "criticalOperationsAllowed",
                "pendingOperationQueues",
                "pendingOperationQueueCount",
                "multiThreadCheck",
                "processActionsInSingleThread",
                "allowIntegrationOfIslandsWithoutConstraintsInASeparateJob",
                "minDesiredIslandSize",
                "modifyConstraintCriticalSection",
                "isLocked",
                "islandDirtyListCriticalSection",
                "propertyMasterLock",
                "wantSimulationIslands",
                "useHybridBroadphase",
                "snapCollisionToConvexEdgeThreshold",
                "snapCollisionToConcaveEdgeThreshold",
                "enableToiWeldRejection",
                "wantDeactivation",
                "shouldActivateOnRigidBodyTransformChange",
                "deactivationReferenceDistance",
                "toiCollisionResponseRotateNormal",
                "maxSectorsPerMidphaseCollideTask",
                "maxSectorsPerNarrowphaseCollideTask",
                "processToisMultithreaded",
                "maxEntriesPerToiMidphaseCollideTask",
                "maxEntriesPerToiNarrowphaseCollideTask",
                "maxNumToiCollisionPairsSinglethreaded",
                "simulationType",
                "numToisTillAllowedPenetrationSimplifiedToi",
                "numToisTillAllowedPenetrationToi",
                "numToisTillAllowedPenetrationToiHigher",
                "numToisTillAllowedPenetrationToiForced",
                "lastEntityUid",
                "lastIslandUid",
                "lastConstraintUid",
                "phantoms",
                "actionListeners",
                "entityListeners",
                "phantomListeners",
                "constraintListeners",
                "worldDeletionListeners",
                "islandActivationListeners",
                "worldPostSimulationListeners",
                "worldPostIntegrateListeners",
                "worldPostCollideListeners",
                "islandPostIntegrateListeners",
                "islandPostCollideListeners",
                "contactListeners",
                "contactImpulseLimitBreachedListeners",
                "worldExtensions",
                "violatedConstraintArray",
                "broadPhaseBorder",
                "destructionWorld",
                "npWorld",
                "broadPhaseExtents",
                "broadPhaseNumMarkers",
                "sizeOfToiEventQueue",
                "broadPhaseQuerySize",
                "broadPhaseUpdateSize",
                "contactPointGeneration",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpWorld",
                FIELDS,
                __hkpWorldVisitor {
                    marker: _serde::__private::PhantomData::<hkpWorld>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
