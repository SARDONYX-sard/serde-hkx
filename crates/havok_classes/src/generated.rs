/// Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
mod class_requires {
    pub use havok_serde::{
        ser::{Error as _, SerializeFlags, SerializeStruct, Serializer, TypeSize},
        de::{self, Error as _, Deserializer},
        HavokClass,
    };
    pub use havok_types::*;
}
pub mod BGSGamebryoSequenceGenerator_;
pub use BGSGamebryoSequenceGenerator_::*;
pub mod BSBoneSwitchGenerator_;
pub use BSBoneSwitchGenerator_::*;
pub mod BSBoneSwitchGeneratorBoneData_;
pub use BSBoneSwitchGeneratorBoneData_::*;
pub mod BSComputeAddBoneAnimModifier_;
pub use BSComputeAddBoneAnimModifier_::*;
pub mod BSCyclicBlendTransitionGenerator_;
pub use BSCyclicBlendTransitionGenerator_::*;
pub mod BSDecomposeVectorModifier_;
pub use BSDecomposeVectorModifier_::*;
pub mod BSDirectAtModifier_;
pub use BSDirectAtModifier_::*;
pub mod BSDistTriggerModifier_;
pub use BSDistTriggerModifier_::*;
pub mod BSEventEveryNEventsModifier_;
pub use BSEventEveryNEventsModifier_::*;
pub mod BSEventOnDeactivateModifier_;
pub use BSEventOnDeactivateModifier_::*;
pub mod BSEventOnFalseToTrueModifier_;
pub use BSEventOnFalseToTrueModifier_::*;
pub mod BSGetTimeStepModifier_;
pub use BSGetTimeStepModifier_::*;
pub mod BSInterpValueModifier_;
pub use BSInterpValueModifier_::*;
pub mod BSIsActiveModifier_;
pub use BSIsActiveModifier_::*;
pub mod BSIStateManagerModifier_;
pub use BSIStateManagerModifier_::*;
pub mod BSIStateManagerModifierBSiStateData_;
pub use BSIStateManagerModifierBSiStateData_::*;
pub mod BSIStateManagerModifierBSIStateManagerStateListener_;
pub use BSIStateManagerModifierBSIStateManagerStateListener_::*;
pub mod BSiStateTaggingGenerator_;
pub use BSiStateTaggingGenerator_::*;
pub mod BSLimbIKModifier_;
pub use BSLimbIKModifier_::*;
pub mod BSLookAtModifier_;
pub use BSLookAtModifier_::*;
pub mod BSLookAtModifierBoneData_;
pub use BSLookAtModifierBoneData_::*;
pub mod BSModifyOnceModifier_;
pub use BSModifyOnceModifier_::*;
pub mod BSOffsetAnimationGenerator_;
pub use BSOffsetAnimationGenerator_::*;
pub mod BSPassByTargetTriggerModifier_;
pub use BSPassByTargetTriggerModifier_::*;
pub mod BSRagdollContactListenerModifier_;
pub use BSRagdollContactListenerModifier_::*;
pub mod BSSpeedSamplerModifier_;
pub use BSSpeedSamplerModifier_::*;
pub mod BSSynchronizedClipGenerator_;
pub use BSSynchronizedClipGenerator_::*;
pub mod BSTimerModifier_;
pub use BSTimerModifier_::*;
pub mod BSTweenerModifier_;
pub use BSTweenerModifier_::*;
pub mod hkAabb_;
pub use hkAabb_::*;
pub mod hkAabbHalf_;
pub use hkAabbHalf_::*;
pub mod hkAabbUint32_;
pub use hkAabbUint32_::*;
pub mod hkaAnimatedReferenceFrame_;
pub use hkaAnimatedReferenceFrame_::*;
pub mod hkaAnimation_;
pub use hkaAnimation_::*;
pub mod hkaAnimationBinding_;
pub use hkaAnimationBinding_::*;
pub mod hkaAnimationContainer_;
pub use hkaAnimationContainer_::*;
pub mod hkaAnimationPreviewColorContainer_;
pub use hkaAnimationPreviewColorContainer_::*;
pub mod hkaAnnotationTrack_;
pub use hkaAnnotationTrack_::*;
pub mod hkaAnnotationTrackAnnotation_;
pub use hkaAnnotationTrackAnnotation_::*;
pub mod hkaBone_;
pub use hkaBone_::*;
pub mod hkaBoneAttachment_;
pub use hkaBoneAttachment_::*;
pub mod hkaDefaultAnimatedReferenceFrame_;
pub use hkaDefaultAnimatedReferenceFrame_::*;
pub mod hkaDeltaCompressedAnimation_;
pub use hkaDeltaCompressedAnimation_::*;
pub mod hkaDeltaCompressedAnimationQuantizationFormat_;
pub use hkaDeltaCompressedAnimationQuantizationFormat_::*;
pub mod hkaFootstepAnalysisInfo_;
pub use hkaFootstepAnalysisInfo_::*;
pub mod hkaFootstepAnalysisInfoContainer_;
pub use hkaFootstepAnalysisInfoContainer_::*;
pub mod hkaInterleavedUncompressedAnimation_;
pub use hkaInterleavedUncompressedAnimation_::*;
pub mod hkaKeyFrameHierarchyUtility_;
pub use hkaKeyFrameHierarchyUtility_::*;
pub mod hkaKeyFrameHierarchyUtilityControlData_;
pub use hkaKeyFrameHierarchyUtilityControlData_::*;
pub mod hkAlignSceneToNodeOptions_;
pub use hkAlignSceneToNodeOptions_::*;
pub mod hkaMeshBinding_;
pub use hkaMeshBinding_::*;
pub mod hkaMeshBindingMapping_;
pub use hkaMeshBindingMapping_::*;
pub mod hkaQuantizedAnimation_;
pub use hkaQuantizedAnimation_::*;
pub mod hkaQuantizedAnimationTrackCompressionParams_;
pub use hkaQuantizedAnimationTrackCompressionParams_::*;
pub mod hkaRagdollInstance_;
pub use hkaRagdollInstance_::*;
pub mod hkArrayTypeAttribute_;
pub use hkArrayTypeAttribute_::*;
pub mod hkaSkeleton_;
pub use hkaSkeleton_::*;
pub mod hkaSkeletonLocalFrameOnBone_;
pub use hkaSkeletonLocalFrameOnBone_::*;
pub mod hkaSkeletonMapper_;
pub use hkaSkeletonMapper_::*;
pub mod hkaSkeletonMapperData_;
pub use hkaSkeletonMapperData_::*;
pub mod hkaSkeletonMapperDataChainMapping_;
pub use hkaSkeletonMapperDataChainMapping_::*;
pub mod hkaSkeletonMapperDataSimpleMapping_;
pub use hkaSkeletonMapperDataSimpleMapping_::*;
pub mod hkaSplineCompressedAnimation_;
pub use hkaSplineCompressedAnimation_::*;
pub mod hkaSplineCompressedAnimationAnimationCompressionParams_;
pub use hkaSplineCompressedAnimationAnimationCompressionParams_::*;
pub mod hkaSplineCompressedAnimationTrackCompressionParams_;
pub use hkaSplineCompressedAnimationTrackCompressionParams_::*;
pub mod hkaWaveletCompressedAnimation_;
pub use hkaWaveletCompressedAnimation_::*;
pub mod hkaWaveletCompressedAnimationCompressionParams_;
pub use hkaWaveletCompressedAnimationCompressionParams_::*;
pub mod hkaWaveletCompressedAnimationQuantizationFormat_;
pub use hkaWaveletCompressedAnimationQuantizationFormat_::*;
pub mod hkBaseObject_;
pub use hkBaseObject_::*;
pub mod hkbAttachmentModifier_;
pub use hkbAttachmentModifier_::*;
pub mod hkbAttachmentSetup_;
pub use hkbAttachmentSetup_::*;
pub mod hkbAttributeModifier_;
pub use hkbAttributeModifier_::*;
pub mod hkbAttributeModifierAssignment_;
pub use hkbAttributeModifierAssignment_::*;
pub mod hkbAuxiliaryNodeInfo_;
pub use hkbAuxiliaryNodeInfo_::*;
pub mod hkbBehaviorEventsInfo_;
pub use hkbBehaviorEventsInfo_::*;
pub mod hkbBehaviorGraph_;
pub use hkbBehaviorGraph_::*;
pub mod hkbBehaviorGraphData_;
pub use hkbBehaviorGraphData_::*;
pub mod hkbBehaviorGraphInternalState_;
pub use hkbBehaviorGraphInternalState_::*;
pub mod hkbBehaviorGraphInternalStateInfo_;
pub use hkbBehaviorGraphInternalStateInfo_::*;
pub mod hkbBehaviorGraphStringData_;
pub use hkbBehaviorGraphStringData_::*;
pub mod hkbBehaviorInfo_;
pub use hkbBehaviorInfo_::*;
pub mod hkbBehaviorInfoIdToNamePair_;
pub use hkbBehaviorInfoIdToNamePair_::*;
pub mod hkbBehaviorReferenceGenerator_;
pub use hkbBehaviorReferenceGenerator_::*;
pub mod hkbBindable_;
pub use hkbBindable_::*;
pub mod hkbBlendCurveUtils_;
pub use hkbBlendCurveUtils_::*;
pub mod hkbBlenderGenerator_;
pub use hkbBlenderGenerator_::*;
pub mod hkbBlenderGeneratorChild_;
pub use hkbBlenderGeneratorChild_::*;
pub mod hkbBlenderGeneratorChildInternalState_;
pub use hkbBlenderGeneratorChildInternalState_::*;
pub mod hkbBlenderGeneratorInternalState_;
pub use hkbBlenderGeneratorInternalState_::*;
pub mod hkbBlendingTransitionEffect_;
pub use hkbBlendingTransitionEffect_::*;
pub mod hkbBlendingTransitionEffectInternalState_;
pub use hkbBlendingTransitionEffectInternalState_::*;
pub mod hkbBoneIndexArray_;
pub use hkbBoneIndexArray_::*;
pub mod hkbBoneWeightArray_;
pub use hkbBoneWeightArray_::*;
pub mod hkbBoolVariableSequencedData_;
pub use hkbBoolVariableSequencedData_::*;
pub mod hkbBoolVariableSequencedDataSample_;
pub use hkbBoolVariableSequencedDataSample_::*;
pub mod hkbCameraShakeEventPayload_;
pub use hkbCameraShakeEventPayload_::*;
pub mod hkbCharacter_;
pub use hkbCharacter_::*;
pub mod hkbCharacterAddedInfo_;
pub use hkbCharacterAddedInfo_::*;
pub mod hkbCharacterControlCommand_;
pub use hkbCharacterControlCommand_::*;
pub mod hkbCharacterControllerControlData_;
pub use hkbCharacterControllerControlData_::*;
pub mod hkbCharacterControllerModifier_;
pub use hkbCharacterControllerModifier_::*;
pub mod hkbCharacterControllerModifierInternalState_;
pub use hkbCharacterControllerModifierInternalState_::*;
pub mod hkbCharacterData_;
pub use hkbCharacterData_::*;
pub mod hkbCharacterDataCharacterControllerInfo_;
pub use hkbCharacterDataCharacterControllerInfo_::*;
pub mod hkbCharacterInfo_;
pub use hkbCharacterInfo_::*;
pub mod hkbCharacterSetup_;
pub use hkbCharacterSetup_::*;
pub mod hkbCharacterSkinInfo_;
pub use hkbCharacterSkinInfo_::*;
pub mod hkbCharacterSteppedInfo_;
pub use hkbCharacterSteppedInfo_::*;
pub mod hkbCharacterStringData_;
pub use hkbCharacterStringData_::*;
pub mod hkbClientCharacterState_;
pub use hkbClientCharacterState_::*;
pub mod hkbClipGenerator_;
pub use hkbClipGenerator_::*;
pub mod hkbClipGeneratorEcho_;
pub use hkbClipGeneratorEcho_::*;
pub mod hkbClipGeneratorInternalState_;
pub use hkbClipGeneratorInternalState_::*;
pub mod hkbClipTrigger_;
pub use hkbClipTrigger_::*;
pub mod hkbClipTriggerArray_;
pub use hkbClipTriggerArray_::*;
pub mod hkbCombineTransformsModifier_;
pub use hkbCombineTransformsModifier_::*;
pub mod hkbCombineTransformsModifierInternalState_;
pub use hkbCombineTransformsModifierInternalState_::*;
pub mod hkbCompiledExpressionSet_;
pub use hkbCompiledExpressionSet_::*;
pub mod hkbCompiledExpressionSetToken_;
pub use hkbCompiledExpressionSetToken_::*;
pub mod hkbComputeDirectionModifier_;
pub use hkbComputeDirectionModifier_::*;
pub mod hkbComputeDirectionModifierInternalState_;
pub use hkbComputeDirectionModifierInternalState_::*;
pub mod hkbComputeRotationFromAxisAngleModifier_;
pub use hkbComputeRotationFromAxisAngleModifier_::*;
pub mod hkbComputeRotationFromAxisAngleModifierInternalState_;
pub use hkbComputeRotationFromAxisAngleModifierInternalState_::*;
pub mod hkbComputeRotationToTargetModifier_;
pub use hkbComputeRotationToTargetModifier_::*;
pub mod hkbComputeRotationToTargetModifierInternalState_;
pub use hkbComputeRotationToTargetModifierInternalState_::*;
pub mod hkbCondition_;
pub use hkbCondition_::*;
pub mod hkbContext_;
pub use hkbContext_::*;
pub mod hkbDampingModifier_;
pub use hkbDampingModifier_::*;
pub mod hkbDampingModifierInternalState_;
pub use hkbDampingModifierInternalState_::*;
pub mod hkbDefaultMessageLog_;
pub use hkbDefaultMessageLog_::*;
pub mod hkbDelayedModifier_;
pub use hkbDelayedModifier_::*;
pub mod hkbDelayedModifierInternalState_;
pub use hkbDelayedModifierInternalState_::*;
pub mod hkbDetectCloseToGroundModifier_;
pub use hkbDetectCloseToGroundModifier_::*;
pub mod hkbDetectCloseToGroundModifierInternalState_;
pub use hkbDetectCloseToGroundModifierInternalState_::*;
pub mod hkbEvaluateExpressionModifier_;
pub use hkbEvaluateExpressionModifier_::*;
pub mod hkbEvaluateExpressionModifierInternalExpressionData_;
pub use hkbEvaluateExpressionModifierInternalExpressionData_::*;
pub mod hkbEvaluateExpressionModifierInternalState_;
pub use hkbEvaluateExpressionModifierInternalState_::*;
pub mod hkbEvaluateHandleModifier_;
pub use hkbEvaluateHandleModifier_::*;
pub mod hkbEvent_;
pub use hkbEvent_::*;
pub mod hkbEventBase_;
pub use hkbEventBase_::*;
pub mod hkbEventDrivenModifier_;
pub use hkbEventDrivenModifier_::*;
pub mod hkbEventDrivenModifierInternalState_;
pub use hkbEventDrivenModifierInternalState_::*;
pub mod hkbEventInfo_;
pub use hkbEventInfo_::*;
pub mod hkbEventPayload_;
pub use hkbEventPayload_::*;
pub mod hkbEventPayloadList_;
pub use hkbEventPayloadList_::*;
pub mod hkbEventProperty_;
pub use hkbEventProperty_::*;
pub mod hkbEventRaisedInfo_;
pub use hkbEventRaisedInfo_::*;
pub mod hkbEventRangeData_;
pub use hkbEventRangeData_::*;
pub mod hkbEventRangeDataArray_;
pub use hkbEventRangeDataArray_::*;
pub mod hkbEventSequencedData_;
pub use hkbEventSequencedData_::*;
pub mod hkbEventSequencedDataSequencedEvent_;
pub use hkbEventSequencedDataSequencedEvent_::*;
pub mod hkbEventsFromRangeModifier_;
pub use hkbEventsFromRangeModifier_::*;
pub mod hkbEventsFromRangeModifierInternalState_;
pub use hkbEventsFromRangeModifierInternalState_::*;
pub mod hkbExpressionCondition_;
pub use hkbExpressionCondition_::*;
pub mod hkbExpressionData_;
pub use hkbExpressionData_::*;
pub mod hkbExpressionDataArray_;
pub use hkbExpressionDataArray_::*;
pub mod hkbExtractRagdollPoseModifier_;
pub use hkbExtractRagdollPoseModifier_::*;
pub mod hkbFootIkControlData_;
pub use hkbFootIkControlData_::*;
pub mod hkbFootIkControlsModifier_;
pub use hkbFootIkControlsModifier_::*;
pub mod hkbFootIkControlsModifierLeg_;
pub use hkbFootIkControlsModifierLeg_::*;
pub mod hkbFootIkDriverInfo_;
pub use hkbFootIkDriverInfo_::*;
pub mod hkbFootIkDriverInfoLeg_;
pub use hkbFootIkDriverInfoLeg_::*;
pub mod hkbFootIkGains_;
pub use hkbFootIkGains_::*;
pub mod hkbFootIkModifier_;
pub use hkbFootIkModifier_::*;
pub mod hkbFootIkModifierInternalLegData_;
pub use hkbFootIkModifierInternalLegData_::*;
pub mod hkbFootIkModifierLeg_;
pub use hkbFootIkModifierLeg_::*;
pub mod hkbGenerator_;
pub use hkbGenerator_::*;
pub mod hkbGeneratorOutputListener_;
pub use hkbGeneratorOutputListener_::*;
pub mod hkbGeneratorSyncInfo_;
pub use hkbGeneratorSyncInfo_::*;
pub mod hkbGeneratorSyncInfoSyncPoint_;
pub use hkbGeneratorSyncInfoSyncPoint_::*;
pub mod hkbGeneratorTransitionEffect_;
pub use hkbGeneratorTransitionEffect_::*;
pub mod hkbGeneratorTransitionEffectInternalState_;
pub use hkbGeneratorTransitionEffectInternalState_::*;
pub mod hkbGetHandleOnBoneModifier_;
pub use hkbGetHandleOnBoneModifier_::*;
pub mod hkbGetUpModifier_;
pub use hkbGetUpModifier_::*;
pub mod hkbGetUpModifierInternalState_;
pub use hkbGetUpModifierInternalState_::*;
pub mod hkbGetWorldFromModelModifier_;
pub use hkbGetWorldFromModelModifier_::*;
pub mod hkbGetWorldFromModelModifierInternalState_;
pub use hkbGetWorldFromModelModifierInternalState_::*;
pub mod hkbHandIkControlData_;
pub use hkbHandIkControlData_::*;
pub mod hkbHandIkControlsModifier_;
pub use hkbHandIkControlsModifier_::*;
pub mod hkbHandIkControlsModifierHand_;
pub use hkbHandIkControlsModifierHand_::*;
pub mod hkbHandIkDriverInfo_;
pub use hkbHandIkDriverInfo_::*;
pub mod hkbHandIkDriverInfoHand_;
pub use hkbHandIkDriverInfoHand_::*;
pub mod hkbHandIkModifier_;
pub use hkbHandIkModifier_::*;
pub mod hkbHandIkModifierHand_;
pub use hkbHandIkModifierHand_::*;
pub mod hkbHandle_;
pub use hkbHandle_::*;
pub mod hkbIntEventPayload_;
pub use hkbIntEventPayload_::*;
pub mod hkbIntVariableSequencedData_;
pub use hkbIntVariableSequencedData_::*;
pub mod hkbIntVariableSequencedDataSample_;
pub use hkbIntVariableSequencedDataSample_::*;
pub mod hkBitField_;
pub use hkBitField_::*;
pub mod hkbKeyframeBonesModifier_;
pub use hkbKeyframeBonesModifier_::*;
pub mod hkbKeyframeBonesModifierKeyframeInfo_;
pub use hkbKeyframeBonesModifierKeyframeInfo_::*;
pub mod hkbLinkedSymbolInfo_;
pub use hkbLinkedSymbolInfo_::*;
pub mod hkbLookAtModifier_;
pub use hkbLookAtModifier_::*;
pub mod hkbLookAtModifierInternalState_;
pub use hkbLookAtModifierInternalState_::*;
pub mod hkbManualSelectorGenerator_;
pub use hkbManualSelectorGenerator_::*;
pub mod hkbManualSelectorGeneratorInternalState_;
pub use hkbManualSelectorGeneratorInternalState_::*;
pub mod hkbMessageLog_;
pub use hkbMessageLog_::*;
pub mod hkbMirroredSkeletonInfo_;
pub use hkbMirroredSkeletonInfo_::*;
pub mod hkbMirrorModifier_;
pub use hkbMirrorModifier_::*;
pub mod hkbModifier_;
pub use hkbModifier_::*;
pub mod hkbModifierGenerator_;
pub use hkbModifierGenerator_::*;
pub mod hkbModifierList_;
pub use hkbModifierList_::*;
pub mod hkbModifierWrapper_;
pub use hkbModifierWrapper_::*;
pub mod hkbMoveCharacterModifier_;
pub use hkbMoveCharacterModifier_::*;
pub mod hkbMoveCharacterModifierInternalState_;
pub use hkbMoveCharacterModifierInternalState_::*;
pub mod hkbNamedEventPayload_;
pub use hkbNamedEventPayload_::*;
pub mod hkbNamedIntEventPayload_;
pub use hkbNamedIntEventPayload_::*;
pub mod hkbNamedRealEventPayload_;
pub use hkbNamedRealEventPayload_::*;
pub mod hkbNamedStringEventPayload_;
pub use hkbNamedStringEventPayload_::*;
pub mod hkbNode_;
pub use hkbNode_::*;
pub mod hkbNodeInternalStateInfo_;
pub use hkbNodeInternalStateInfo_::*;
pub mod hkbParticleSystemEventPayload_;
pub use hkbParticleSystemEventPayload_::*;
pub mod hkbPoseMatchingGenerator_;
pub use hkbPoseMatchingGenerator_::*;
pub mod hkbPoseMatchingGeneratorInternalState_;
pub use hkbPoseMatchingGeneratorInternalState_::*;
pub mod hkbPoweredRagdollControlData_;
pub use hkbPoweredRagdollControlData_::*;
pub mod hkbPoweredRagdollControlsModifier_;
pub use hkbPoweredRagdollControlsModifier_::*;
pub mod hkbProjectData_;
pub use hkbProjectData_::*;
pub mod hkbProjectStringData_;
pub use hkbProjectStringData_::*;
pub mod hkbProxyModifier_;
pub use hkbProxyModifier_::*;
pub mod hkbProxyModifierProxyInfo_;
pub use hkbProxyModifierProxyInfo_::*;
pub mod hkbRaiseEventCommand_;
pub use hkbRaiseEventCommand_::*;
pub mod hkbRealEventPayload_;
pub use hkbRealEventPayload_::*;
pub mod hkbRealVariableSequencedData_;
pub use hkbRealVariableSequencedData_::*;
pub mod hkbRealVariableSequencedDataSample_;
pub use hkbRealVariableSequencedDataSample_::*;
pub mod hkbReferencePoseGenerator_;
pub use hkbReferencePoseGenerator_::*;
pub mod hkbRegisteredGenerator_;
pub use hkbRegisteredGenerator_::*;
pub mod hkbRigidBodyRagdollControlData_;
pub use hkbRigidBodyRagdollControlData_::*;
pub mod hkbRigidBodyRagdollControlsModifier_;
pub use hkbRigidBodyRagdollControlsModifier_::*;
pub mod hkbRoleAttribute_;
pub use hkbRoleAttribute_::*;
pub mod hkbRotateCharacterModifier_;
pub use hkbRotateCharacterModifier_::*;
pub mod hkbRotateCharacterModifierInternalState_;
pub use hkbRotateCharacterModifierInternalState_::*;
pub mod hkbSenseHandleModifier_;
pub use hkbSenseHandleModifier_::*;
pub mod hkbSenseHandleModifierRange_;
pub use hkbSenseHandleModifierRange_::*;
pub mod hkbSequence_;
pub use hkbSequence_::*;
pub mod hkbSequencedData_;
pub use hkbSequencedData_::*;
pub mod hkbSequenceInternalState_;
pub use hkbSequenceInternalState_::*;
pub mod hkbSequenceStringData_;
pub use hkbSequenceStringData_::*;
pub mod hkbSetBehaviorCommand_;
pub use hkbSetBehaviorCommand_::*;
pub mod hkbSetLocalTimeOfClipGeneratorCommand_;
pub use hkbSetLocalTimeOfClipGeneratorCommand_::*;
pub mod hkbSetNodePropertyCommand_;
pub use hkbSetNodePropertyCommand_::*;
pub mod hkbSetWordVariableCommand_;
pub use hkbSetWordVariableCommand_::*;
pub mod hkbSetWorldFromModelModifier_;
pub use hkbSetWorldFromModelModifier_::*;
pub mod hkbSimulationControlCommand_;
pub use hkbSimulationControlCommand_::*;
pub mod hkbSimulationStateInfo_;
pub use hkbSimulationStateInfo_::*;
pub mod hkbStateChooser_;
pub use hkbStateChooser_::*;
pub mod hkbStateListener_;
pub use hkbStateListener_::*;
pub mod hkbStateMachine_;
pub use hkbStateMachine_::*;
pub mod hkbStateMachineActiveTransitionInfo_;
pub use hkbStateMachineActiveTransitionInfo_::*;
pub mod hkbStateMachineDelayedTransitionInfo_;
pub use hkbStateMachineDelayedTransitionInfo_::*;
pub mod hkbStateMachineEventPropertyArray_;
pub use hkbStateMachineEventPropertyArray_::*;
pub mod hkbStateMachineInternalState_;
pub use hkbStateMachineInternalState_::*;
pub mod hkbStateMachineNestedStateMachineData_;
pub use hkbStateMachineNestedStateMachineData_::*;
pub mod hkbStateMachineProspectiveTransitionInfo_;
pub use hkbStateMachineProspectiveTransitionInfo_::*;
pub mod hkbStateMachineStateInfo_;
pub use hkbStateMachineStateInfo_::*;
pub mod hkbStateMachineTimeInterval_;
pub use hkbStateMachineTimeInterval_::*;
pub mod hkbStateMachineTransitionInfo_;
pub use hkbStateMachineTransitionInfo_::*;
pub mod hkbStateMachineTransitionInfoArray_;
pub use hkbStateMachineTransitionInfoArray_::*;
pub mod hkbStateMachineTransitionInfoReference_;
pub use hkbStateMachineTransitionInfoReference_::*;
pub mod hkbStringCondition_;
pub use hkbStringCondition_::*;
pub mod hkbStringEventPayload_;
pub use hkbStringEventPayload_::*;
pub mod hkbTestStateChooser_;
pub use hkbTestStateChooser_::*;
pub mod hkbTimerModifier_;
pub use hkbTimerModifier_::*;
pub mod hkbTimerModifierInternalState_;
pub use hkbTimerModifierInternalState_::*;
pub mod hkbTransformVectorModifier_;
pub use hkbTransformVectorModifier_::*;
pub mod hkbTransformVectorModifierInternalState_;
pub use hkbTransformVectorModifierInternalState_::*;
pub mod hkbTransitionEffect_;
pub use hkbTransitionEffect_::*;
pub mod hkbTwistModifier_;
pub use hkbTwistModifier_::*;
pub mod hkbVariableBindingSet_;
pub use hkbVariableBindingSet_::*;
pub mod hkbVariableBindingSetBinding_;
pub use hkbVariableBindingSetBinding_::*;
pub mod hkbVariableInfo_;
pub use hkbVariableInfo_::*;
pub mod hkbVariableValue_;
pub use hkbVariableValue_::*;
pub mod hkbVariableValueSet_;
pub use hkbVariableValueSet_::*;
pub mod hkbWorldEnums_;
pub use hkbWorldEnums_::*;
pub mod hkbWorldFromModelModeData_;
pub use hkbWorldFromModelModeData_::*;
pub mod hkClass_;
pub use hkClass_::*;
pub mod hkClassEnum_;
pub use hkClassEnum_::*;
pub mod hkClassEnumItem_;
pub use hkClassEnumItem_::*;
pub mod hkClassMember_;
pub use hkClassMember_::*;
pub mod hkColor_;
pub use hkColor_::*;
pub mod hkContactPoint_;
pub use hkContactPoint_::*;
pub mod hkContactPointMaterial_;
pub use hkContactPointMaterial_::*;
pub mod hkCustomAttributes_;
pub use hkCustomAttributes_::*;
pub mod hkCustomAttributesAttribute_;
pub use hkCustomAttributesAttribute_::*;
pub mod hkDataObjectTypeAttribute_;
pub use hkDataObjectTypeAttribute_::*;
pub mod hkDescriptionAttribute_;
pub use hkDescriptionAttribute_::*;
pub mod hkDocumentationAttribute_;
pub use hkDocumentationAttribute_::*;
pub mod hkGeometry_;
pub use hkGeometry_::*;
pub mod hkGeometryTriangle_;
pub use hkGeometryTriangle_::*;
pub mod hkGizmoAttribute_;
pub use hkGizmoAttribute_::*;
pub mod hkHalf8_;
pub use hkHalf8_::*;
pub mod hkIndexedTransformSet_;
pub use hkIndexedTransformSet_::*;
pub mod hkLinkAttribute_;
pub use hkLinkAttribute_::*;
pub mod hkLocalFrame_;
pub use hkLocalFrame_::*;
pub mod hkLocalFrameGroup_;
pub use hkLocalFrameGroup_::*;
pub mod hkMemoryMeshBody_;
pub use hkMemoryMeshBody_::*;
pub mod hkMemoryMeshMaterial_;
pub use hkMemoryMeshMaterial_::*;
pub mod hkMemoryMeshShape_;
pub use hkMemoryMeshShape_::*;
pub mod hkMemoryMeshTexture_;
pub use hkMemoryMeshTexture_::*;
pub mod hkMemoryMeshVertexBuffer_;
pub use hkMemoryMeshVertexBuffer_::*;
pub mod hkMemoryResourceContainer_;
pub use hkMemoryResourceContainer_::*;
pub mod hkMemoryResourceHandle_;
pub use hkMemoryResourceHandle_::*;
pub mod hkMemoryResourceHandleExternalLink_;
pub use hkMemoryResourceHandleExternalLink_::*;
pub mod hkMemoryTrackerAttribute_;
pub use hkMemoryTrackerAttribute_::*;
pub mod hkMeshBody_;
pub use hkMeshBody_::*;
pub mod hkMeshBoneIndexMapping_;
pub use hkMeshBoneIndexMapping_::*;
pub mod hkMeshMaterial_;
pub use hkMeshMaterial_::*;
pub mod hkMeshSection_;
pub use hkMeshSection_::*;
pub mod hkMeshSectionCinfo_;
pub use hkMeshSectionCinfo_::*;
pub mod hkMeshShape_;
pub use hkMeshShape_::*;
pub mod hkMeshTexture_;
pub use hkMeshTexture_::*;
pub mod hkMeshVertexBuffer_;
pub use hkMeshVertexBuffer_::*;
pub mod hkModelerNodeTypeAttribute_;
pub use hkModelerNodeTypeAttribute_::*;
pub mod hkMonitorStreamColorTable_;
pub use hkMonitorStreamColorTable_::*;
pub mod hkMonitorStreamColorTableColorPair_;
pub use hkMonitorStreamColorTableColorPair_::*;
pub mod hkMonitorStreamFrameInfo_;
pub use hkMonitorStreamFrameInfo_::*;
pub mod hkMonitorStreamStringMap_;
pub use hkMonitorStreamStringMap_::*;
pub mod hkMonitorStreamStringMapStringMap_;
pub use hkMonitorStreamStringMapStringMap_::*;
pub mod hkMoppBvTreeShapeBase_;
pub use hkMoppBvTreeShapeBase_::*;
pub mod hkMotionState_;
pub use hkMotionState_::*;
pub mod hkMultipleVertexBuffer_;
pub use hkMultipleVertexBuffer_::*;
pub mod hkMultipleVertexBufferElementInfo_;
pub use hkMultipleVertexBufferElementInfo_::*;
pub mod hkMultipleVertexBufferLockedElement_;
pub use hkMultipleVertexBufferLockedElement_::*;
pub mod hkMultipleVertexBufferVertexBufferInfo_;
pub use hkMultipleVertexBufferVertexBufferInfo_::*;
pub mod hkMultiThreadCheck_;
pub use hkMultiThreadCheck_::*;
pub mod hkp2dAngConstraintAtom_;
pub use hkp2dAngConstraintAtom_::*;
pub mod hkpAabbPhantom_;
pub use hkpAabbPhantom_::*;
pub mod hkPackedVector3_;
pub use hkPackedVector3_::*;
pub mod hkPackfileHeader_;
pub use hkPackfileHeader_::*;
pub mod hkPackfileSectionHeader_;
pub use hkPackfileSectionHeader_::*;
pub mod hkpAction_;
pub use hkpAction_::*;
pub mod hkpAgent1nSector_;
pub use hkpAgent1nSector_::*;
pub mod hkpAngConstraintAtom_;
pub use hkpAngConstraintAtom_::*;
pub mod hkpAngFrictionConstraintAtom_;
pub use hkpAngFrictionConstraintAtom_::*;
pub mod hkpAngLimitConstraintAtom_;
pub use hkpAngLimitConstraintAtom_::*;
pub mod hkpAngMotorConstraintAtom_;
pub use hkpAngMotorConstraintAtom_::*;
pub mod hkpAngularDashpotAction_;
pub use hkpAngularDashpotAction_::*;
pub mod hkpArrayAction_;
pub use hkpArrayAction_::*;
pub mod hkpBallAndSocketConstraintData_;
pub use hkpBallAndSocketConstraintData_::*;
pub mod hkpBallAndSocketConstraintDataAtoms_;
pub use hkpBallAndSocketConstraintDataAtoms_::*;
pub mod hkpBallGun_;
pub use hkpBallGun_::*;
pub mod hkpBallSocketChainData_;
pub use hkpBallSocketChainData_::*;
pub mod hkpBallSocketChainDataConstraintInfo_;
pub use hkpBallSocketChainDataConstraintInfo_::*;
pub mod hkpBallSocketConstraintAtom_;
pub use hkpBallSocketConstraintAtom_::*;
pub mod hkpBinaryAction_;
pub use hkpBinaryAction_::*;
pub mod hkpBoxMotion_;
pub use hkpBoxMotion_::*;
pub mod hkpBoxShape_;
pub use hkpBoxShape_::*;
pub mod hkpBreakableBody_;
pub use hkpBreakableBody_::*;
pub mod hkpBreakableConstraintData_;
pub use hkpBreakableConstraintData_::*;
pub mod hkpBridgeAtoms_;
pub use hkpBridgeAtoms_::*;
pub mod hkpBridgeConstraintAtom_;
pub use hkpBridgeConstraintAtom_::*;
pub mod hkpBroadPhaseHandle_;
pub use hkpBroadPhaseHandle_::*;
pub mod hkpBvShape_;
pub use hkpBvShape_::*;
pub mod hkpBvTreeShape_;
pub use hkpBvTreeShape_::*;
pub mod hkpCachingShapePhantom_;
pub use hkpCachingShapePhantom_::*;
pub mod hkpCallbackConstraintMotor_;
pub use hkpCallbackConstraintMotor_::*;
pub mod hkpCapsuleShape_;
pub use hkpCapsuleShape_::*;
pub mod hkpCdBody_;
pub use hkpCdBody_::*;
pub mod hkpCenterOfMassChangerModifierConstraintAtom_;
pub use hkpCenterOfMassChangerModifierConstraintAtom_::*;
pub mod hkpCharacterControllerCinfo_;
pub use hkpCharacterControllerCinfo_::*;
pub mod hkpCharacterMotion_;
pub use hkpCharacterMotion_::*;
pub mod hkpCharacterProxyCinfo_;
pub use hkpCharacterProxyCinfo_::*;
pub mod hkpCharacterRigidBodyCinfo_;
pub use hkpCharacterRigidBodyCinfo_::*;
pub mod hkpCogWheelConstraintAtom_;
pub use hkpCogWheelConstraintAtom_::*;
pub mod hkpCogWheelConstraintData_;
pub use hkpCogWheelConstraintData_::*;
pub mod hkpCogWheelConstraintDataAtoms_;
pub use hkpCogWheelConstraintDataAtoms_::*;
pub mod hkpCollidable_;
pub use hkpCollidable_::*;
pub mod hkpCollidableBoundingVolumeData_;
pub use hkpCollidableBoundingVolumeData_::*;
pub mod hkpCollidableCollidableFilter_;
pub use hkpCollidableCollidableFilter_::*;
pub mod hkpCollisionFilter_;
pub use hkpCollisionFilter_::*;
pub mod hkpCollisionFilterList_;
pub use hkpCollisionFilterList_::*;
pub mod hkpCompressedMeshShape_;
pub use hkpCompressedMeshShape_::*;
pub mod hkpCompressedMeshShapeBigTriangle_;
pub use hkpCompressedMeshShapeBigTriangle_::*;
pub mod hkpCompressedMeshShapeChunk_;
pub use hkpCompressedMeshShapeChunk_::*;
pub mod hkpCompressedMeshShapeConvexPiece_;
pub use hkpCompressedMeshShapeConvexPiece_::*;
pub mod hkpCompressedSampledHeightFieldShape_;
pub use hkpCompressedSampledHeightFieldShape_::*;
pub mod hkpConeLimitConstraintAtom_;
pub use hkpConeLimitConstraintAtom_::*;
pub mod hkpConstrainedSystemFilter_;
pub use hkpConstrainedSystemFilter_::*;
pub mod hkpConstraintAtom_;
pub use hkpConstraintAtom_::*;
pub mod hkpConstraintChainData_;
pub use hkpConstraintChainData_::*;
pub mod hkpConstraintChainInstance_;
pub use hkpConstraintChainInstance_::*;
pub mod hkpConstraintChainInstanceAction_;
pub use hkpConstraintChainInstanceAction_::*;
pub mod hkpConstraintCollisionFilter_;
pub use hkpConstraintCollisionFilter_::*;
pub mod hkpConstraintData_;
pub use hkpConstraintData_::*;
pub mod hkpConstraintInstance_;
pub use hkpConstraintInstance_::*;
pub mod hkpConstraintInstanceSmallArraySerializeOverrideType_;
pub use hkpConstraintInstanceSmallArraySerializeOverrideType_::*;
pub mod hkpConstraintMotor_;
pub use hkpConstraintMotor_::*;
pub mod hkpConvexListFilter_;
pub use hkpConvexListFilter_::*;
pub mod hkpConvexListShape_;
pub use hkpConvexListShape_::*;
pub mod hkpConvexPieceMeshShape_;
pub use hkpConvexPieceMeshShape_::*;
pub mod hkpConvexPieceStreamData_;
pub use hkpConvexPieceStreamData_::*;
pub mod hkpConvexShape_;
pub use hkpConvexShape_::*;
pub mod hkpConvexTransformShape_;
pub use hkpConvexTransformShape_::*;
pub mod hkpConvexTransformShapeBase_;
pub use hkpConvexTransformShapeBase_::*;
pub mod hkpConvexTranslateShape_;
pub use hkpConvexTranslateShape_::*;
pub mod hkpConvexVerticesConnectivity_;
pub use hkpConvexVerticesConnectivity_::*;
pub mod hkpConvexVerticesShape_;
pub use hkpConvexVerticesShape_::*;
pub mod hkpConvexVerticesShapeFourVectors_;
pub use hkpConvexVerticesShapeFourVectors_::*;
pub mod hkpCylinderShape_;
pub use hkpCylinderShape_::*;
pub mod hkpDashpotAction_;
pub use hkpDashpotAction_::*;
pub mod hkpDefaultConvexListFilter_;
pub use hkpDefaultConvexListFilter_::*;
pub mod hkpDefaultWorldMemoryWatchDog_;
pub use hkpDefaultWorldMemoryWatchDog_::*;
pub mod hkpDisableEntityCollisionFilter_;
pub use hkpDisableEntityCollisionFilter_::*;
pub mod hkpDisplayBindingData_;
pub use hkpDisplayBindingData_::*;
pub mod hkpDisplayBindingDataPhysicsSystem_;
pub use hkpDisplayBindingDataPhysicsSystem_::*;
pub mod hkpDisplayBindingDataRigidBody_;
pub use hkpDisplayBindingDataRigidBody_::*;
pub mod hkpEntity_;
pub use hkpEntity_::*;
pub mod hkpEntityExtendedListeners_;
pub use hkpEntityExtendedListeners_::*;
pub mod hkpEntitySmallArraySerializeOverrideType_;
pub use hkpEntitySmallArraySerializeOverrideType_::*;
pub mod hkpEntitySpuCollisionCallback_;
pub use hkpEntitySpuCollisionCallback_::*;
pub mod hkpExtendedMeshShape_;
pub use hkpExtendedMeshShape_::*;
pub mod hkpExtendedMeshShapeShapesSubpart_;
pub use hkpExtendedMeshShapeShapesSubpart_::*;
pub mod hkpExtendedMeshShapeSubpart_;
pub use hkpExtendedMeshShapeSubpart_::*;
pub mod hkpExtendedMeshShapeTrianglesSubpart_;
pub use hkpExtendedMeshShapeTrianglesSubpart_::*;
pub mod hkpFastMeshShape_;
pub use hkpFastMeshShape_::*;
pub mod hkpFirstPersonGun_;
pub use hkpFirstPersonGun_::*;
pub mod hkpFixedRigidMotion_;
pub use hkpFixedRigidMotion_::*;
pub mod hkpGenericConstraintData_;
pub use hkpGenericConstraintData_::*;
pub mod hkpGenericConstraintDataScheme_;
pub use hkpGenericConstraintDataScheme_::*;
pub mod hkpGenericConstraintDataSchemeConstraintInfo_;
pub use hkpGenericConstraintDataSchemeConstraintInfo_::*;
pub mod hkpGravityGun_;
pub use hkpGravityGun_::*;
pub mod hkpGroupCollisionFilter_;
pub use hkpGroupCollisionFilter_::*;
pub mod hkpGroupFilter_;
pub use hkpGroupFilter_::*;
pub mod hkpHeightFieldShape_;
pub use hkpHeightFieldShape_::*;
pub mod hkpHingeConstraintData_;
pub use hkpHingeConstraintData_::*;
pub mod hkpHingeConstraintDataAtoms_;
pub use hkpHingeConstraintDataAtoms_::*;
pub mod hkpHingeLimitsData_;
pub use hkpHingeLimitsData_::*;
pub mod hkpHingeLimitsDataAtoms_;
pub use hkpHingeLimitsDataAtoms_::*;
pub mod hkpIgnoreModifierConstraintAtom_;
pub use hkpIgnoreModifierConstraintAtom_::*;
pub mod hkpKeyframedRigidMotion_;
pub use hkpKeyframedRigidMotion_::*;
pub mod hkpLimitedForceConstraintMotor_;
pub use hkpLimitedForceConstraintMotor_::*;
pub mod hkpLimitedHingeConstraintData_;
pub use hkpLimitedHingeConstraintData_::*;
pub mod hkpLimitedHingeConstraintDataAtoms_;
pub use hkpLimitedHingeConstraintDataAtoms_::*;
pub mod hkpLinConstraintAtom_;
pub use hkpLinConstraintAtom_::*;
pub mod hkpLinearParametricCurve_;
pub use hkpLinearParametricCurve_::*;
pub mod hkpLinFrictionConstraintAtom_;
pub use hkpLinFrictionConstraintAtom_::*;
pub mod hkpLinkedCollidable_;
pub use hkpLinkedCollidable_::*;
pub mod hkpLinLimitConstraintAtom_;
pub use hkpLinLimitConstraintAtom_::*;
pub mod hkpLinMotorConstraintAtom_;
pub use hkpLinMotorConstraintAtom_::*;
pub mod hkpLinSoftConstraintAtom_;
pub use hkpLinSoftConstraintAtom_::*;
pub mod hkpListShape_;
pub use hkpListShape_::*;
pub mod hkpListShapeChildInfo_;
pub use hkpListShapeChildInfo_::*;
pub mod hkpMalleableConstraintData_;
pub use hkpMalleableConstraintData_::*;
pub mod hkpMassChangerModifierConstraintAtom_;
pub use hkpMassChangerModifierConstraintAtom_::*;
pub mod hkpMassProperties_;
pub use hkpMassProperties_::*;
pub mod hkpMaterial_;
pub use hkpMaterial_::*;
pub mod hkpMaxSizeMotion_;
pub use hkpMaxSizeMotion_::*;
pub mod hkpMeshMaterial_;
pub use hkpMeshMaterial_::*;
pub mod hkpMeshShape_;
pub use hkpMeshShape_::*;
pub mod hkpMeshShapeSubpart_;
pub use hkpMeshShapeSubpart_::*;
pub mod hkpModifierConstraintAtom_;
pub use hkpModifierConstraintAtom_::*;
pub mod hkpMoppBvTreeShape_;
pub use hkpMoppBvTreeShape_::*;
pub mod hkpMoppCode_;
pub use hkpMoppCode_::*;
pub mod hkpMoppCodeCodeInfo_;
pub use hkpMoppCodeCodeInfo_::*;
pub mod hkpMoppCodeReindexedTerminal_;
pub use hkpMoppCodeReindexedTerminal_::*;
pub mod hkpMotion_;
pub use hkpMotion_::*;
pub mod hkpMotorAction_;
pub use hkpMotorAction_::*;
pub mod hkpMountedBallGun_;
pub use hkpMountedBallGun_::*;
pub mod hkpMouseSpringAction_;
pub use hkpMouseSpringAction_::*;
pub mod hkpMovingSurfaceModifierConstraintAtom_;
pub use hkpMovingSurfaceModifierConstraintAtom_::*;
pub mod hkpMultiRayShape_;
pub use hkpMultiRayShape_::*;
pub mod hkpMultiRayShapeRay_;
pub use hkpMultiRayShapeRay_::*;
pub mod hkpMultiSphereShape_;
pub use hkpMultiSphereShape_::*;
pub mod hkpMultithreadedVehicleManager_;
pub use hkpMultithreadedVehicleManager_::*;
pub mod hkpNamedMeshMaterial_;
pub use hkpNamedMeshMaterial_::*;
pub mod hkpNullCollisionFilter_;
pub use hkpNullCollisionFilter_::*;
pub mod hkPostFinishAttribute_;
pub use hkPostFinishAttribute_::*;
pub mod hkpOverwritePivotConstraintAtom_;
pub use hkpOverwritePivotConstraintAtom_::*;
pub mod hkpPairCollisionFilter_;
pub use hkpPairCollisionFilter_::*;
pub mod hkpPairCollisionFilterMapPairFilterKeyOverrideType_;
pub use hkpPairCollisionFilterMapPairFilterKeyOverrideType_::*;
pub mod hkpParametricCurve_;
pub use hkpParametricCurve_::*;
pub mod hkpPhantom_;
pub use hkpPhantom_::*;
pub mod hkpPhantomCallbackShape_;
pub use hkpPhantomCallbackShape_::*;
pub mod hkpPhysicsData_;
pub use hkpPhysicsData_::*;
pub mod hkpPhysicsSystem_;
pub use hkpPhysicsSystem_::*;
pub mod hkpPhysicsSystemWithContacts_;
pub use hkpPhysicsSystemWithContacts_::*;
pub mod hkpPlaneShape_;
pub use hkpPlaneShape_::*;
pub mod hkpPointToPathConstraintData_;
pub use hkpPointToPathConstraintData_::*;
pub mod hkpPointToPlaneConstraintData_;
pub use hkpPointToPlaneConstraintData_::*;
pub mod hkpPointToPlaneConstraintDataAtoms_;
pub use hkpPointToPlaneConstraintDataAtoms_::*;
pub mod hkpPositionConstraintMotor_;
pub use hkpPositionConstraintMotor_::*;
pub mod hkpPoweredChainData_;
pub use hkpPoweredChainData_::*;
pub mod hkpPoweredChainDataConstraintInfo_;
pub use hkpPoweredChainDataConstraintInfo_::*;
pub mod hkpPoweredChainMapper_;
pub use hkpPoweredChainMapper_::*;
pub mod hkpPoweredChainMapperLinkInfo_;
pub use hkpPoweredChainMapperLinkInfo_::*;
pub mod hkpPoweredChainMapperTarget_;
pub use hkpPoweredChainMapperTarget_::*;
pub mod hkpPrismaticConstraintData_;
pub use hkpPrismaticConstraintData_::*;
pub mod hkpPrismaticConstraintDataAtoms_;
pub use hkpPrismaticConstraintDataAtoms_::*;
pub mod hkpProjectileGun_;
pub use hkpProjectileGun_::*;
pub mod hkpProperty_;
pub use hkpProperty_::*;
pub mod hkpPropertyValue_;
pub use hkpPropertyValue_::*;
pub mod hkpPulleyConstraintAtom_;
pub use hkpPulleyConstraintAtom_::*;
pub mod hkpPulleyConstraintData_;
pub use hkpPulleyConstraintData_::*;
pub mod hkpPulleyConstraintDataAtoms_;
pub use hkpPulleyConstraintDataAtoms_::*;
pub mod hkpRackAndPinionConstraintAtom_;
pub use hkpRackAndPinionConstraintAtom_::*;
pub mod hkpRackAndPinionConstraintData_;
pub use hkpRackAndPinionConstraintData_::*;
pub mod hkpRackAndPinionConstraintDataAtoms_;
pub use hkpRackAndPinionConstraintDataAtoms_::*;
pub mod hkpRagdollConstraintData_;
pub use hkpRagdollConstraintData_::*;
pub mod hkpRagdollConstraintDataAtoms_;
pub use hkpRagdollConstraintDataAtoms_::*;
pub mod hkpRagdollLimitsData_;
pub use hkpRagdollLimitsData_::*;
pub mod hkpRagdollLimitsDataAtoms_;
pub use hkpRagdollLimitsDataAtoms_::*;
pub mod hkpRagdollMotorConstraintAtom_;
pub use hkpRagdollMotorConstraintAtom_::*;
pub mod hkpRayCollidableFilter_;
pub use hkpRayCollidableFilter_::*;
pub mod hkpRayShapeCollectionFilter_;
pub use hkpRayShapeCollectionFilter_::*;
pub mod hkpRejectChassisListener_;
pub use hkpRejectChassisListener_::*;
pub mod hkpRemoveTerminalsMoppModifier_;
pub use hkpRemoveTerminalsMoppModifier_::*;
pub mod hkpReorientAction_;
pub use hkpReorientAction_::*;
pub mod hkpRigidBody_;
pub use hkpRigidBody_::*;
pub mod hkpRotationalConstraintData_;
pub use hkpRotationalConstraintData_::*;
pub mod hkpRotationalConstraintDataAtoms_;
pub use hkpRotationalConstraintDataAtoms_::*;
pub mod hkpSampledHeightFieldShape_;
pub use hkpSampledHeightFieldShape_::*;
pub mod hkpSerializedAgentNnEntry_;
pub use hkpSerializedAgentNnEntry_::*;
pub mod hkpSerializedDisplayMarker_;
pub use hkpSerializedDisplayMarker_::*;
pub mod hkpSerializedDisplayMarkerList_;
pub use hkpSerializedDisplayMarkerList_::*;
pub mod hkpSerializedDisplayRbTransforms_;
pub use hkpSerializedDisplayRbTransforms_::*;
pub mod hkpSerializedDisplayRbTransformsDisplayTransformPair_;
pub use hkpSerializedDisplayRbTransformsDisplayTransformPair_::*;
pub mod hkpSerializedSubTrack1nInfo_;
pub use hkpSerializedSubTrack1nInfo_::*;
pub mod hkpSerializedTrack1nInfo_;
pub use hkpSerializedTrack1nInfo_::*;
pub mod hkpSetLocalRotationsConstraintAtom_;
pub use hkpSetLocalRotationsConstraintAtom_::*;
pub mod hkpSetLocalTransformsConstraintAtom_;
pub use hkpSetLocalTransformsConstraintAtom_::*;
pub mod hkpSetLocalTranslationsConstraintAtom_;
pub use hkpSetLocalTranslationsConstraintAtom_::*;
pub mod hkpSetupStabilizationAtom_;
pub use hkpSetupStabilizationAtom_::*;
pub mod hkpShape_;
pub use hkpShape_::*;
pub mod hkpShapeCollection_;
pub use hkpShapeCollection_::*;
pub mod hkpShapeCollectionFilter_;
pub use hkpShapeCollectionFilter_::*;
pub mod hkpShapeContainer_;
pub use hkpShapeContainer_::*;
pub mod hkpShapeInfo_;
pub use hkpShapeInfo_::*;
pub mod hkpShapeModifier_;
pub use hkpShapeModifier_::*;
pub mod hkpShapePhantom_;
pub use hkpShapePhantom_::*;
pub mod hkpSimpleContactConstraintAtom_;
pub use hkpSimpleContactConstraintAtom_::*;
pub mod hkpSimpleContactConstraintDataInfo_;
pub use hkpSimpleContactConstraintDataInfo_::*;
pub mod hkpSimpleMeshShape_;
pub use hkpSimpleMeshShape_::*;
pub mod hkpSimpleMeshShapeTriangle_;
pub use hkpSimpleMeshShapeTriangle_::*;
pub mod hkpSimpleShapePhantom_;
pub use hkpSimpleShapePhantom_::*;
pub mod hkpSimpleShapePhantomCollisionDetail_;
pub use hkpSimpleShapePhantomCollisionDetail_::*;
pub mod hkpSimulation_;
pub use hkpSimulation_::*;
pub mod hkpSingleShapeContainer_;
pub use hkpSingleShapeContainer_::*;
pub mod hkpSoftContactModifierConstraintAtom_;
pub use hkpSoftContactModifierConstraintAtom_::*;
pub mod hkpSphereMotion_;
pub use hkpSphereMotion_::*;
pub mod hkpSphereRepShape_;
pub use hkpSphereRepShape_::*;
pub mod hkpSphereShape_;
pub use hkpSphereShape_::*;
pub mod hkpSpringAction_;
pub use hkpSpringAction_::*;
pub mod hkpSpringDamperConstraintMotor_;
pub use hkpSpringDamperConstraintMotor_::*;
pub mod hkpStiffSpringChainData_;
pub use hkpStiffSpringChainData_::*;
pub mod hkpStiffSpringChainDataConstraintInfo_;
pub use hkpStiffSpringChainDataConstraintInfo_::*;
pub mod hkpStiffSpringConstraintAtom_;
pub use hkpStiffSpringConstraintAtom_::*;
pub mod hkpStiffSpringConstraintData_;
pub use hkpStiffSpringConstraintData_::*;
pub mod hkpStiffSpringConstraintDataAtoms_;
pub use hkpStiffSpringConstraintDataAtoms_::*;
pub mod hkpStorageExtendedMeshShape_;
pub use hkpStorageExtendedMeshShape_::*;
pub mod hkpStorageExtendedMeshShapeMaterial_;
pub use hkpStorageExtendedMeshShapeMaterial_::*;
pub mod hkpStorageExtendedMeshShapeMeshSubpartStorage_;
pub use hkpStorageExtendedMeshShapeMeshSubpartStorage_::*;
pub mod hkpStorageExtendedMeshShapeShapeSubpartStorage_;
pub use hkpStorageExtendedMeshShapeShapeSubpartStorage_::*;
pub mod hkpStorageMeshShape_;
pub use hkpStorageMeshShape_::*;
pub mod hkpStorageMeshShapeSubpartStorage_;
pub use hkpStorageMeshShapeSubpartStorage_::*;
pub mod hkpStorageSampledHeightFieldShape_;
pub use hkpStorageSampledHeightFieldShape_::*;
pub mod hkpThinBoxMotion_;
pub use hkpThinBoxMotion_::*;
pub mod hkpTransformShape_;
pub use hkpTransformShape_::*;
pub mod hkpTriangleShape_;
pub use hkpTriangleShape_::*;
pub mod hkpTriggerVolume_;
pub use hkpTriggerVolume_::*;
pub mod hkpTriggerVolumeEventInfo_;
pub use hkpTriggerVolumeEventInfo_::*;
pub mod hkpTriSampledHeightFieldBvTreeShape_;
pub use hkpTriSampledHeightFieldBvTreeShape_::*;
pub mod hkpTriSampledHeightFieldCollection_;
pub use hkpTriSampledHeightFieldCollection_::*;
pub mod hkpTwistLimitConstraintAtom_;
pub use hkpTwistLimitConstraintAtom_::*;
pub mod hkpTypedBroadPhaseHandle_;
pub use hkpTypedBroadPhaseHandle_::*;
pub mod hkpTyremarkPoint_;
pub use hkpTyremarkPoint_::*;
pub mod hkpTyremarksInfo_;
pub use hkpTyremarksInfo_::*;
pub mod hkpTyremarksWheel_;
pub use hkpTyremarksWheel_::*;
pub mod hkpUnaryAction_;
pub use hkpUnaryAction_::*;
pub mod hkpVehicleAerodynamics_;
pub use hkpVehicleAerodynamics_::*;
pub mod hkpVehicleBrake_;
pub use hkpVehicleBrake_::*;
pub mod hkpVehicleCastBatchingManager_;
pub use hkpVehicleCastBatchingManager_::*;
pub mod hkpVehicleData_;
pub use hkpVehicleData_::*;
pub mod hkpVehicleDataWheelComponentParams_;
pub use hkpVehicleDataWheelComponentParams_::*;
pub mod hkpVehicleDefaultAerodynamics_;
pub use hkpVehicleDefaultAerodynamics_::*;
pub mod hkpVehicleDefaultAnalogDriverInput_;
pub use hkpVehicleDefaultAnalogDriverInput_::*;
pub mod hkpVehicleDefaultBrake_;
pub use hkpVehicleDefaultBrake_::*;
pub mod hkpVehicleDefaultBrakeWheelBrakingProperties_;
pub use hkpVehicleDefaultBrakeWheelBrakingProperties_::*;
pub mod hkpVehicleDefaultEngine_;
pub use hkpVehicleDefaultEngine_::*;
pub mod hkpVehicleDefaultSteering_;
pub use hkpVehicleDefaultSteering_::*;
pub mod hkpVehicleDefaultSuspension_;
pub use hkpVehicleDefaultSuspension_::*;
pub mod hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters_;
pub use hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters_::*;
pub mod hkpVehicleDefaultTransmission_;
pub use hkpVehicleDefaultTransmission_::*;
pub mod hkpVehicleDefaultVelocityDamper_;
pub use hkpVehicleDefaultVelocityDamper_::*;
pub mod hkpVehicleDriverInput_;
pub use hkpVehicleDriverInput_::*;
pub mod hkpVehicleDriverInputAnalogStatus_;
pub use hkpVehicleDriverInputAnalogStatus_::*;
pub mod hkpVehicleDriverInputStatus_;
pub use hkpVehicleDriverInputStatus_::*;
pub mod hkpVehicleEngine_;
pub use hkpVehicleEngine_::*;
pub mod hkpVehicleFrictionDescription_;
pub use hkpVehicleFrictionDescription_::*;
pub mod hkpVehicleFrictionDescriptionAxisDescription_;
pub use hkpVehicleFrictionDescriptionAxisDescription_::*;
pub mod hkpVehicleFrictionStatus_;
pub use hkpVehicleFrictionStatus_::*;
pub mod hkpVehicleFrictionStatusAxisStatus_;
pub use hkpVehicleFrictionStatusAxisStatus_::*;
pub mod hkpVehicleInstance_;
pub use hkpVehicleInstance_::*;
pub mod hkpVehicleInstanceWheelInfo_;
pub use hkpVehicleInstanceWheelInfo_::*;
pub mod hkpVehicleLinearCastBatchingManager_;
pub use hkpVehicleLinearCastBatchingManager_::*;
pub mod hkpVehicleLinearCastWheelCollide_;
pub use hkpVehicleLinearCastWheelCollide_::*;
pub mod hkpVehicleLinearCastWheelCollideWheelState_;
pub use hkpVehicleLinearCastWheelCollideWheelState_::*;
pub mod hkpVehicleManager_;
pub use hkpVehicleManager_::*;
pub mod hkpVehicleRayCastBatchingManager_;
pub use hkpVehicleRayCastBatchingManager_::*;
pub mod hkpVehicleRayCastWheelCollide_;
pub use hkpVehicleRayCastWheelCollide_::*;
pub mod hkpVehicleSteering_;
pub use hkpVehicleSteering_::*;
pub mod hkpVehicleSuspension_;
pub use hkpVehicleSuspension_::*;
pub mod hkpVehicleSuspensionSuspensionWheelParameters_;
pub use hkpVehicleSuspensionSuspensionWheelParameters_::*;
pub mod hkpVehicleTransmission_;
pub use hkpVehicleTransmission_::*;
pub mod hkpVehicleVelocityDamper_;
pub use hkpVehicleVelocityDamper_::*;
pub mod hkpVehicleWheelCollide_;
pub use hkpVehicleWheelCollide_::*;
pub mod hkpVelocityConstraintMotor_;
pub use hkpVelocityConstraintMotor_::*;
pub mod hkpViscousSurfaceModifierConstraintAtom_;
pub use hkpViscousSurfaceModifierConstraintAtom_::*;
pub mod hkpWeldingUtility_;
pub use hkpWeldingUtility_::*;
pub mod hkpWheelConstraintData_;
pub use hkpWheelConstraintData_::*;
pub mod hkpWheelConstraintDataAtoms_;
pub use hkpWheelConstraintDataAtoms_::*;
pub mod hkpWorld_;
pub use hkpWorld_::*;
pub mod hkpWorldCinfo_;
pub use hkpWorldCinfo_::*;
pub mod hkpWorldObject_;
pub use hkpWorldObject_::*;
pub mod hkQTransform_;
pub use hkQTransform_::*;
pub mod hkRangeInt32Attribute_;
pub use hkRangeInt32Attribute_::*;
pub mod hkRangeRealAttribute_;
pub use hkRangeRealAttribute_::*;
pub mod hkReferencedObject_;
pub use hkReferencedObject_::*;
pub mod hkReflectedFileAttribute_;
pub use hkReflectedFileAttribute_::*;
pub mod hkResourceBase_;
pub use hkResourceBase_::*;
pub mod hkResourceContainer_;
pub use hkResourceContainer_::*;
pub mod hkResourceHandle_;
pub use hkResourceHandle_::*;
pub mod hkRootLevelContainer_;
pub use hkRootLevelContainer_::*;
pub mod hkRootLevelContainerNamedVariant_;
pub use hkRootLevelContainerNamedVariant_::*;
pub mod hkSemanticsAttribute_;
pub use hkSemanticsAttribute_::*;
pub mod hkSimpleLocalFrame_;
pub use hkSimpleLocalFrame_::*;
pub mod hkSphere_;
pub use hkSphere_::*;
pub mod hkSweptTransform_;
pub use hkSweptTransform_::*;
pub mod hkTraceStreamTitle_;
pub use hkTraceStreamTitle_::*;
pub mod hkTrackerSerializableScanSnapshot_;
pub use hkTrackerSerializableScanSnapshot_::*;
pub mod hkTrackerSerializableScanSnapshotAllocation_;
pub use hkTrackerSerializableScanSnapshotAllocation_::*;
pub mod hkTrackerSerializableScanSnapshotBlock_;
pub use hkTrackerSerializableScanSnapshotBlock_::*;
pub mod hkUiAttribute_;
pub use hkUiAttribute_::*;
pub mod hkVertexFormat_;
pub use hkVertexFormat_::*;
pub mod hkVertexFormatElement_;
pub use hkVertexFormatElement_::*;
pub mod hkWorldMemoryAvailableWatchDog_;
pub use hkWorldMemoryAvailableWatchDog_::*;
pub mod hkxAnimatedFloat_;
pub use hkxAnimatedFloat_::*;
pub mod hkxAnimatedMatrix_;
pub use hkxAnimatedMatrix_::*;
pub mod hkxAnimatedQuaternion_;
pub use hkxAnimatedQuaternion_::*;
pub mod hkxAnimatedVector_;
pub use hkxAnimatedVector_::*;
pub mod hkxAttribute_;
pub use hkxAttribute_::*;
pub mod hkxAttributeGroup_;
pub use hkxAttributeGroup_::*;
pub mod hkxAttributeHolder_;
pub use hkxAttributeHolder_::*;
pub mod hkxCamera_;
pub use hkxCamera_::*;
pub mod hkxEdgeSelectionChannel_;
pub use hkxEdgeSelectionChannel_::*;
pub mod hkxEnum_;
pub use hkxEnum_::*;
pub mod hkxEnumItem_;
pub use hkxEnumItem_::*;
pub mod hkxEnvironment_;
pub use hkxEnvironment_::*;
pub mod hkxEnvironmentVariable_;
pub use hkxEnvironmentVariable_::*;
pub mod hkxIndexBuffer_;
pub use hkxIndexBuffer_::*;
pub mod hkxLight_;
pub use hkxLight_::*;
pub mod hkxMaterial_;
pub use hkxMaterial_::*;
pub mod hkxMaterialEffect_;
pub use hkxMaterialEffect_::*;
pub mod hkxMaterialProperty_;
pub use hkxMaterialProperty_::*;
pub mod hkxMaterialShader_;
pub use hkxMaterialShader_::*;
pub mod hkxMaterialShaderSet_;
pub use hkxMaterialShaderSet_::*;
pub mod hkxMaterialTextureStage_;
pub use hkxMaterialTextureStage_::*;
pub mod hkxMesh_;
pub use hkxMesh_::*;
pub mod hkxMeshSection_;
pub use hkxMeshSection_::*;
pub mod hkxMeshUserChannelInfo_;
pub use hkxMeshUserChannelInfo_::*;
pub mod hkxNode_;
pub use hkxNode_::*;
pub mod hkxNodeAnnotationData_;
pub use hkxNodeAnnotationData_::*;
pub mod hkxNodeSelectionSet_;
pub use hkxNodeSelectionSet_::*;
pub mod hkxScene_;
pub use hkxScene_::*;
pub mod hkxSkinBinding_;
pub use hkxSkinBinding_::*;
pub mod hkxSparselyAnimatedBool_;
pub use hkxSparselyAnimatedBool_::*;
pub mod hkxSparselyAnimatedEnum_;
pub use hkxSparselyAnimatedEnum_::*;
pub mod hkxSparselyAnimatedInt_;
pub use hkxSparselyAnimatedInt_::*;
pub mod hkxSparselyAnimatedString_;
pub use hkxSparselyAnimatedString_::*;
pub mod hkxTextureFile_;
pub use hkxTextureFile_::*;
pub mod hkxTextureInplace_;
pub use hkxTextureInplace_::*;
pub mod hkxTriangleSelectionChannel_;
pub use hkxTriangleSelectionChannel_::*;
pub mod hkxVertexBuffer_;
pub use hkxVertexBuffer_::*;
pub mod hkxVertexBufferVertexData_;
pub use hkxVertexBufferVertexData_::*;
pub mod hkxVertexDescription_;
pub use hkxVertexDescription_::*;
pub mod hkxVertexDescriptionElementDecl_;
pub use hkxVertexDescriptionElementDecl_::*;
pub mod hkxVertexFloatDataChannel_;
pub use hkxVertexFloatDataChannel_::*;
pub mod hkxVertexIntDataChannel_;
pub use hkxVertexIntDataChannel_::*;
pub mod hkxVertexSelectionChannel_;
pub use hkxVertexSelectionChannel_::*;
pub mod hkxVertexVectorDataChannel_;
pub use hkxVertexVectorDataChannel_::*;
use havok_serde as _serde;
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Classes<'a> {
    /// For binary writing, the youngest pointer index must be first after sorting in reverse order.
    ///
    /// To speed up the process, swap the first and last indexes instead of using shift.
    /// This dummy class exists to reserve space for this purpose.
    #[default]
    SwapDummy,
    BGSGamebryoSequenceGenerator(BGSGamebryoSequenceGenerator<'a>),
    BSBoneSwitchGenerator(BSBoneSwitchGenerator<'a>),
    BSBoneSwitchGeneratorBoneData(BSBoneSwitchGeneratorBoneData),
    BSComputeAddBoneAnimModifier(BSComputeAddBoneAnimModifier<'a>),
    BSCyclicBlendTransitionGenerator(BSCyclicBlendTransitionGenerator<'a>),
    BSDecomposeVectorModifier(BSDecomposeVectorModifier<'a>),
    BSDirectAtModifier(BSDirectAtModifier<'a>),
    BSDistTriggerModifier(BSDistTriggerModifier<'a>),
    BSEventEveryNEventsModifier(BSEventEveryNEventsModifier<'a>),
    BSEventOnDeactivateModifier(BSEventOnDeactivateModifier<'a>),
    BSEventOnFalseToTrueModifier(BSEventOnFalseToTrueModifier<'a>),
    BSGetTimeStepModifier(BSGetTimeStepModifier<'a>),
    BSInterpValueModifier(BSInterpValueModifier<'a>),
    BSIsActiveModifier(BSIsActiveModifier<'a>),
    BSIStateManagerModifier(BSIStateManagerModifier<'a>),
    BSIStateManagerModifierBSiStateData(BSIStateManagerModifierBSiStateData),
    BSIStateManagerModifierBSIStateManagerStateListener(
        BSIStateManagerModifierBSIStateManagerStateListener,
    ),
    BSiStateTaggingGenerator(BSiStateTaggingGenerator<'a>),
    BSLimbIKModifier(BSLimbIKModifier<'a>),
    BSLookAtModifier(BSLookAtModifier<'a>),
    BSLookAtModifierBoneData(BSLookAtModifierBoneData),
    BSModifyOnceModifier(BSModifyOnceModifier<'a>),
    BSOffsetAnimationGenerator(BSOffsetAnimationGenerator<'a>),
    BSPassByTargetTriggerModifier(BSPassByTargetTriggerModifier<'a>),
    BSRagdollContactListenerModifier(BSRagdollContactListenerModifier<'a>),
    BSSpeedSamplerModifier(BSSpeedSamplerModifier<'a>),
    BSSynchronizedClipGenerator(BSSynchronizedClipGenerator<'a>),
    BSTimerModifier(BSTimerModifier<'a>),
    BSTweenerModifier(BSTweenerModifier<'a>),
    hkAabb(hkAabb),
    hkAabbHalf(hkAabbHalf),
    hkAabbUint32(hkAabbUint32),
    hkaAnimatedReferenceFrame(hkaAnimatedReferenceFrame),
    hkaAnimation(hkaAnimation<'a>),
    hkaAnimationBinding(hkaAnimationBinding<'a>),
    hkaAnimationContainer(hkaAnimationContainer),
    hkaAnimationPreviewColorContainer(hkaAnimationPreviewColorContainer),
    hkaAnnotationTrack(hkaAnnotationTrack<'a>),
    hkaAnnotationTrackAnnotation(hkaAnnotationTrackAnnotation<'a>),
    hkaBone(hkaBone<'a>),
    hkaBoneAttachment(hkaBoneAttachment<'a>),
    hkaDefaultAnimatedReferenceFrame(hkaDefaultAnimatedReferenceFrame),
    hkaDeltaCompressedAnimation(hkaDeltaCompressedAnimation<'a>),
    hkaDeltaCompressedAnimationQuantizationFormat(
        hkaDeltaCompressedAnimationQuantizationFormat,
    ),
    hkaFootstepAnalysisInfo(hkaFootstepAnalysisInfo),
    hkaFootstepAnalysisInfoContainer(hkaFootstepAnalysisInfoContainer),
    hkaInterleavedUncompressedAnimation(hkaInterleavedUncompressedAnimation<'a>),
    hkaKeyFrameHierarchyUtility(hkaKeyFrameHierarchyUtility),
    hkaKeyFrameHierarchyUtilityControlData(hkaKeyFrameHierarchyUtilityControlData),
    hkAlignSceneToNodeOptions(hkAlignSceneToNodeOptions<'a>),
    hkaMeshBinding(hkaMeshBinding<'a>),
    hkaMeshBindingMapping(hkaMeshBindingMapping),
    hkaQuantizedAnimation(hkaQuantizedAnimation<'a>),
    hkaQuantizedAnimationTrackCompressionParams(
        hkaQuantizedAnimationTrackCompressionParams,
    ),
    hkaRagdollInstance(hkaRagdollInstance),
    hkArrayTypeAttribute(hkArrayTypeAttribute),
    hkaSkeleton(hkaSkeleton<'a>),
    hkaSkeletonLocalFrameOnBone(hkaSkeletonLocalFrameOnBone),
    hkaSkeletonMapper(hkaSkeletonMapper),
    hkaSkeletonMapperData(hkaSkeletonMapperData),
    hkaSkeletonMapperDataChainMapping(hkaSkeletonMapperDataChainMapping),
    hkaSkeletonMapperDataSimpleMapping(hkaSkeletonMapperDataSimpleMapping),
    hkaSplineCompressedAnimation(hkaSplineCompressedAnimation<'a>),
    hkaSplineCompressedAnimationAnimationCompressionParams(
        hkaSplineCompressedAnimationAnimationCompressionParams,
    ),
    hkaSplineCompressedAnimationTrackCompressionParams(
        hkaSplineCompressedAnimationTrackCompressionParams,
    ),
    hkaWaveletCompressedAnimation(hkaWaveletCompressedAnimation<'a>),
    hkaWaveletCompressedAnimationCompressionParams(
        hkaWaveletCompressedAnimationCompressionParams,
    ),
    hkaWaveletCompressedAnimationQuantizationFormat(
        hkaWaveletCompressedAnimationQuantizationFormat,
    ),
    hkBaseObject(hkBaseObject),
    hkbAttachmentModifier(hkbAttachmentModifier<'a>),
    hkbAttachmentSetup(hkbAttachmentSetup),
    hkbAttributeModifier(hkbAttributeModifier<'a>),
    hkbAttributeModifierAssignment(hkbAttributeModifierAssignment),
    hkbAuxiliaryNodeInfo(hkbAuxiliaryNodeInfo<'a>),
    hkbBehaviorEventsInfo(hkbBehaviorEventsInfo),
    hkbBehaviorGraph(hkbBehaviorGraph<'a>),
    hkbBehaviorGraphData(hkbBehaviorGraphData),
    hkbBehaviorGraphInternalState(hkbBehaviorGraphInternalState),
    hkbBehaviorGraphInternalStateInfo(hkbBehaviorGraphInternalStateInfo),
    hkbBehaviorGraphStringData(hkbBehaviorGraphStringData<'a>),
    hkbBehaviorInfo(hkbBehaviorInfo<'a>),
    hkbBehaviorInfoIdToNamePair(hkbBehaviorInfoIdToNamePair<'a>),
    hkbBehaviorReferenceGenerator(hkbBehaviorReferenceGenerator<'a>),
    hkbBindable(hkbBindable),
    hkbBlendCurveUtils(hkbBlendCurveUtils),
    hkbBlenderGenerator(hkbBlenderGenerator<'a>),
    hkbBlenderGeneratorChild(hkbBlenderGeneratorChild),
    hkbBlenderGeneratorChildInternalState(hkbBlenderGeneratorChildInternalState),
    hkbBlenderGeneratorInternalState(hkbBlenderGeneratorInternalState),
    hkbBlendingTransitionEffect(hkbBlendingTransitionEffect<'a>),
    hkbBlendingTransitionEffectInternalState(hkbBlendingTransitionEffectInternalState),
    hkbBoneIndexArray(hkbBoneIndexArray),
    hkbBoneWeightArray(hkbBoneWeightArray),
    hkbBoolVariableSequencedData(hkbBoolVariableSequencedData),
    hkbBoolVariableSequencedDataSample(hkbBoolVariableSequencedDataSample),
    hkbCameraShakeEventPayload(hkbCameraShakeEventPayload),
    hkbCharacter(hkbCharacter<'a>),
    hkbCharacterAddedInfo(hkbCharacterAddedInfo<'a>),
    hkbCharacterControlCommand(hkbCharacterControlCommand),
    hkbCharacterControllerControlData(hkbCharacterControllerControlData),
    hkbCharacterControllerModifier(hkbCharacterControllerModifier<'a>),
    hkbCharacterControllerModifierInternalState(
        hkbCharacterControllerModifierInternalState,
    ),
    hkbCharacterData(hkbCharacterData),
    hkbCharacterDataCharacterControllerInfo(hkbCharacterDataCharacterControllerInfo),
    hkbCharacterInfo(hkbCharacterInfo),
    hkbCharacterSetup(hkbCharacterSetup),
    hkbCharacterSkinInfo(hkbCharacterSkinInfo),
    hkbCharacterSteppedInfo(hkbCharacterSteppedInfo),
    hkbCharacterStringData(hkbCharacterStringData<'a>),
    hkbClientCharacterState(hkbClientCharacterState<'a>),
    hkbClipGenerator(hkbClipGenerator<'a>),
    hkbClipGeneratorEcho(hkbClipGeneratorEcho),
    hkbClipGeneratorInternalState(hkbClipGeneratorInternalState),
    hkbClipTrigger(hkbClipTrigger),
    hkbClipTriggerArray(hkbClipTriggerArray),
    hkbCombineTransformsModifier(hkbCombineTransformsModifier<'a>),
    hkbCombineTransformsModifierInternalState(hkbCombineTransformsModifierInternalState),
    hkbCompiledExpressionSet(hkbCompiledExpressionSet),
    hkbCompiledExpressionSetToken(hkbCompiledExpressionSetToken),
    hkbComputeDirectionModifier(hkbComputeDirectionModifier<'a>),
    hkbComputeDirectionModifierInternalState(hkbComputeDirectionModifierInternalState),
    hkbComputeRotationFromAxisAngleModifier(hkbComputeRotationFromAxisAngleModifier<'a>),
    hkbComputeRotationFromAxisAngleModifierInternalState(
        hkbComputeRotationFromAxisAngleModifierInternalState,
    ),
    hkbComputeRotationToTargetModifier(hkbComputeRotationToTargetModifier<'a>),
    hkbComputeRotationToTargetModifierInternalState(
        hkbComputeRotationToTargetModifierInternalState,
    ),
    hkbCondition(hkbCondition),
    hkbContext(hkbContext),
    hkbDampingModifier(hkbDampingModifier<'a>),
    hkbDampingModifierInternalState(hkbDampingModifierInternalState),
    hkbDefaultMessageLog(hkbDefaultMessageLog),
    hkbDelayedModifier(hkbDelayedModifier<'a>),
    hkbDelayedModifierInternalState(hkbDelayedModifierInternalState),
    hkbDetectCloseToGroundModifier(hkbDetectCloseToGroundModifier<'a>),
    hkbDetectCloseToGroundModifierInternalState(
        hkbDetectCloseToGroundModifierInternalState,
    ),
    hkbEvaluateExpressionModifier(hkbEvaluateExpressionModifier<'a>),
    hkbEvaluateExpressionModifierInternalExpressionData(
        hkbEvaluateExpressionModifierInternalExpressionData,
    ),
    hkbEvaluateExpressionModifierInternalState(
        hkbEvaluateExpressionModifierInternalState,
    ),
    hkbEvaluateHandleModifier(hkbEvaluateHandleModifier<'a>),
    hkbEvent(hkbEvent),
    hkbEventBase(hkbEventBase),
    hkbEventDrivenModifier(hkbEventDrivenModifier<'a>),
    hkbEventDrivenModifierInternalState(hkbEventDrivenModifierInternalState),
    hkbEventInfo(hkbEventInfo),
    hkbEventPayload(hkbEventPayload),
    hkbEventPayloadList(hkbEventPayloadList),
    hkbEventProperty(hkbEventProperty),
    hkbEventRaisedInfo(hkbEventRaisedInfo<'a>),
    hkbEventRangeData(hkbEventRangeData),
    hkbEventRangeDataArray(hkbEventRangeDataArray),
    hkbEventSequencedData(hkbEventSequencedData),
    hkbEventSequencedDataSequencedEvent(hkbEventSequencedDataSequencedEvent),
    hkbEventsFromRangeModifier(hkbEventsFromRangeModifier<'a>),
    hkbEventsFromRangeModifierInternalState(hkbEventsFromRangeModifierInternalState),
    hkbExpressionCondition(hkbExpressionCondition<'a>),
    hkbExpressionData(hkbExpressionData<'a>),
    hkbExpressionDataArray(hkbExpressionDataArray<'a>),
    hkbExtractRagdollPoseModifier(hkbExtractRagdollPoseModifier<'a>),
    hkbFootIkControlData(hkbFootIkControlData),
    hkbFootIkControlsModifier(hkbFootIkControlsModifier<'a>),
    hkbFootIkControlsModifierLeg(hkbFootIkControlsModifierLeg),
    hkbFootIkDriverInfo(hkbFootIkDriverInfo),
    hkbFootIkDriverInfoLeg(hkbFootIkDriverInfoLeg),
    hkbFootIkGains(hkbFootIkGains),
    hkbFootIkModifier(hkbFootIkModifier<'a>),
    hkbFootIkModifierInternalLegData(hkbFootIkModifierInternalLegData),
    hkbFootIkModifierLeg(hkbFootIkModifierLeg),
    hkbGenerator(hkbGenerator<'a>),
    hkbGeneratorOutputListener(hkbGeneratorOutputListener),
    hkbGeneratorSyncInfo(hkbGeneratorSyncInfo),
    hkbGeneratorSyncInfoSyncPoint(hkbGeneratorSyncInfoSyncPoint),
    hkbGeneratorTransitionEffect(hkbGeneratorTransitionEffect<'a>),
    hkbGeneratorTransitionEffectInternalState(hkbGeneratorTransitionEffectInternalState),
    hkbGetHandleOnBoneModifier(hkbGetHandleOnBoneModifier<'a>),
    hkbGetUpModifier(hkbGetUpModifier<'a>),
    hkbGetUpModifierInternalState(hkbGetUpModifierInternalState),
    hkbGetWorldFromModelModifier(hkbGetWorldFromModelModifier<'a>),
    hkbGetWorldFromModelModifierInternalState(hkbGetWorldFromModelModifierInternalState),
    hkbHandIkControlData(hkbHandIkControlData),
    hkbHandIkControlsModifier(hkbHandIkControlsModifier<'a>),
    hkbHandIkControlsModifierHand(hkbHandIkControlsModifierHand),
    hkbHandIkDriverInfo(hkbHandIkDriverInfo<'a>),
    hkbHandIkDriverInfoHand(hkbHandIkDriverInfoHand<'a>),
    hkbHandIkModifier(hkbHandIkModifier<'a>),
    hkbHandIkModifierHand(hkbHandIkModifierHand<'a>),
    hkbHandle(hkbHandle),
    hkbIntEventPayload(hkbIntEventPayload),
    hkbIntVariableSequencedData(hkbIntVariableSequencedData),
    hkbIntVariableSequencedDataSample(hkbIntVariableSequencedDataSample),
    hkBitField(hkBitField),
    hkbKeyframeBonesModifier(hkbKeyframeBonesModifier<'a>),
    hkbKeyframeBonesModifierKeyframeInfo(hkbKeyframeBonesModifierKeyframeInfo),
    hkbLinkedSymbolInfo(hkbLinkedSymbolInfo<'a>),
    hkbLookAtModifier(hkbLookAtModifier<'a>),
    hkbLookAtModifierInternalState(hkbLookAtModifierInternalState),
    hkbManualSelectorGenerator(hkbManualSelectorGenerator<'a>),
    hkbManualSelectorGeneratorInternalState(hkbManualSelectorGeneratorInternalState),
    hkbMessageLog(hkbMessageLog),
    hkbMirroredSkeletonInfo(hkbMirroredSkeletonInfo),
    hkbMirrorModifier(hkbMirrorModifier<'a>),
    hkbModifier(hkbModifier<'a>),
    hkbModifierGenerator(hkbModifierGenerator<'a>),
    hkbModifierList(hkbModifierList<'a>),
    hkbModifierWrapper(hkbModifierWrapper<'a>),
    hkbMoveCharacterModifier(hkbMoveCharacterModifier<'a>),
    hkbMoveCharacterModifierInternalState(hkbMoveCharacterModifierInternalState),
    hkbNamedEventPayload(hkbNamedEventPayload<'a>),
    hkbNamedIntEventPayload(hkbNamedIntEventPayload<'a>),
    hkbNamedRealEventPayload(hkbNamedRealEventPayload<'a>),
    hkbNamedStringEventPayload(hkbNamedStringEventPayload<'a>),
    hkbNode(hkbNode<'a>),
    hkbNodeInternalStateInfo(hkbNodeInternalStateInfo<'a>),
    hkbParticleSystemEventPayload(hkbParticleSystemEventPayload),
    hkbPoseMatchingGenerator(hkbPoseMatchingGenerator<'a>),
    hkbPoseMatchingGeneratorInternalState(hkbPoseMatchingGeneratorInternalState),
    hkbPoweredRagdollControlData(hkbPoweredRagdollControlData),
    hkbPoweredRagdollControlsModifier(hkbPoweredRagdollControlsModifier<'a>),
    hkbProjectData(hkbProjectData),
    hkbProjectStringData(hkbProjectStringData<'a>),
    hkbProxyModifier(hkbProxyModifier<'a>),
    hkbProxyModifierProxyInfo(hkbProxyModifierProxyInfo),
    hkbRaiseEventCommand(hkbRaiseEventCommand),
    hkbRealEventPayload(hkbRealEventPayload),
    hkbRealVariableSequencedData(hkbRealVariableSequencedData),
    hkbRealVariableSequencedDataSample(hkbRealVariableSequencedDataSample),
    hkbReferencePoseGenerator(hkbReferencePoseGenerator<'a>),
    hkbRegisteredGenerator(hkbRegisteredGenerator),
    hkbRigidBodyRagdollControlData(hkbRigidBodyRagdollControlData),
    hkbRigidBodyRagdollControlsModifier(hkbRigidBodyRagdollControlsModifier<'a>),
    hkbRoleAttribute(hkbRoleAttribute),
    hkbRotateCharacterModifier(hkbRotateCharacterModifier<'a>),
    hkbRotateCharacterModifierInternalState(hkbRotateCharacterModifierInternalState),
    hkbSenseHandleModifier(hkbSenseHandleModifier<'a>),
    hkbSenseHandleModifierRange(hkbSenseHandleModifierRange),
    hkbSequence(hkbSequence<'a>),
    hkbSequencedData(hkbSequencedData),
    hkbSequenceInternalState(hkbSequenceInternalState),
    hkbSequenceStringData(hkbSequenceStringData<'a>),
    hkbSetBehaviorCommand(hkbSetBehaviorCommand),
    hkbSetLocalTimeOfClipGeneratorCommand(hkbSetLocalTimeOfClipGeneratorCommand),
    hkbSetNodePropertyCommand(hkbSetNodePropertyCommand<'a>),
    hkbSetWordVariableCommand(hkbSetWordVariableCommand),
    hkbSetWorldFromModelModifier(hkbSetWorldFromModelModifier<'a>),
    hkbSimulationControlCommand(hkbSimulationControlCommand),
    hkbSimulationStateInfo(hkbSimulationStateInfo),
    hkbStateChooser(hkbStateChooser),
    hkbStateListener(hkbStateListener),
    hkbStateMachine(hkbStateMachine<'a>),
    hkbStateMachineActiveTransitionInfo(hkbStateMachineActiveTransitionInfo),
    hkbStateMachineDelayedTransitionInfo(hkbStateMachineDelayedTransitionInfo),
    hkbStateMachineEventPropertyArray(hkbStateMachineEventPropertyArray),
    hkbStateMachineInternalState(hkbStateMachineInternalState),
    hkbStateMachineNestedStateMachineData(hkbStateMachineNestedStateMachineData),
    hkbStateMachineProspectiveTransitionInfo(hkbStateMachineProspectiveTransitionInfo),
    hkbStateMachineStateInfo(hkbStateMachineStateInfo<'a>),
    hkbStateMachineTimeInterval(hkbStateMachineTimeInterval),
    hkbStateMachineTransitionInfo(hkbStateMachineTransitionInfo),
    hkbStateMachineTransitionInfoArray(hkbStateMachineTransitionInfoArray),
    hkbStateMachineTransitionInfoReference(hkbStateMachineTransitionInfoReference),
    hkbStringCondition(hkbStringCondition<'a>),
    hkbStringEventPayload(hkbStringEventPayload<'a>),
    hkbTestStateChooser(hkbTestStateChooser<'a>),
    hkbTimerModifier(hkbTimerModifier<'a>),
    hkbTimerModifierInternalState(hkbTimerModifierInternalState),
    hkbTransformVectorModifier(hkbTransformVectorModifier<'a>),
    hkbTransformVectorModifierInternalState(hkbTransformVectorModifierInternalState),
    hkbTransitionEffect(hkbTransitionEffect<'a>),
    hkbTwistModifier(hkbTwistModifier<'a>),
    hkbVariableBindingSet(hkbVariableBindingSet<'a>),
    hkbVariableBindingSetBinding(hkbVariableBindingSetBinding<'a>),
    hkbVariableInfo(hkbVariableInfo),
    hkbVariableValue(hkbVariableValue),
    hkbVariableValueSet(hkbVariableValueSet),
    hkbWorldEnums(hkbWorldEnums),
    hkbWorldFromModelModeData(hkbWorldFromModelModeData),
    hkClass(hkClass<'a>),
    hkClassEnum(hkClassEnum<'a>),
    hkClassEnumItem(hkClassEnumItem<'a>),
    hkClassMember(hkClassMember<'a>),
    hkColor(hkColor),
    hkContactPoint(hkContactPoint),
    hkContactPointMaterial(hkContactPointMaterial),
    hkCustomAttributes(hkCustomAttributes<'a>),
    hkCustomAttributesAttribute(hkCustomAttributesAttribute<'a>),
    hkDataObjectTypeAttribute(hkDataObjectTypeAttribute<'a>),
    hkDescriptionAttribute(hkDescriptionAttribute<'a>),
    hkDocumentationAttribute(hkDocumentationAttribute<'a>),
    hkGeometry(hkGeometry),
    hkGeometryTriangle(hkGeometryTriangle),
    hkGizmoAttribute(hkGizmoAttribute<'a>),
    hkHalf8(hkHalf8),
    hkIndexedTransformSet(hkIndexedTransformSet<'a>),
    hkLinkAttribute(hkLinkAttribute),
    hkLocalFrame(hkLocalFrame),
    hkLocalFrameGroup(hkLocalFrameGroup<'a>),
    hkMemoryMeshBody(hkMemoryMeshBody<'a>),
    hkMemoryMeshMaterial(hkMemoryMeshMaterial<'a>),
    hkMemoryMeshShape(hkMemoryMeshShape<'a>),
    hkMemoryMeshTexture(hkMemoryMeshTexture<'a>),
    hkMemoryMeshVertexBuffer(hkMemoryMeshVertexBuffer),
    hkMemoryResourceContainer(hkMemoryResourceContainer<'a>),
    hkMemoryResourceHandle(hkMemoryResourceHandle<'a>),
    hkMemoryResourceHandleExternalLink(hkMemoryResourceHandleExternalLink<'a>),
    hkMemoryTrackerAttribute(hkMemoryTrackerAttribute),
    hkMeshBody(hkMeshBody),
    hkMeshBoneIndexMapping(hkMeshBoneIndexMapping),
    hkMeshMaterial(hkMeshMaterial),
    hkMeshSection(hkMeshSection),
    hkMeshSectionCinfo(hkMeshSectionCinfo),
    hkMeshShape(hkMeshShape),
    hkMeshTexture(hkMeshTexture),
    hkMeshVertexBuffer(hkMeshVertexBuffer),
    hkModelerNodeTypeAttribute(hkModelerNodeTypeAttribute),
    hkMonitorStreamColorTable(hkMonitorStreamColorTable<'a>),
    hkMonitorStreamColorTableColorPair(hkMonitorStreamColorTableColorPair<'a>),
    hkMonitorStreamFrameInfo(hkMonitorStreamFrameInfo<'a>),
    hkMonitorStreamStringMap(hkMonitorStreamStringMap<'a>),
    hkMonitorStreamStringMapStringMap(hkMonitorStreamStringMapStringMap<'a>),
    hkMoppBvTreeShapeBase(hkMoppBvTreeShapeBase),
    hkMotionState(hkMotionState),
    hkMultipleVertexBuffer(hkMultipleVertexBuffer),
    hkMultipleVertexBufferElementInfo(hkMultipleVertexBufferElementInfo),
    hkMultipleVertexBufferLockedElement(hkMultipleVertexBufferLockedElement),
    hkMultipleVertexBufferVertexBufferInfo(hkMultipleVertexBufferVertexBufferInfo),
    hkMultiThreadCheck(hkMultiThreadCheck),
    hkp2dAngConstraintAtom(hkp2dAngConstraintAtom),
    hkpAabbPhantom(hkpAabbPhantom<'a>),
    hkPackedVector3(hkPackedVector3),
    hkPackfileHeader(hkPackfileHeader),
    hkPackfileSectionHeader(hkPackfileSectionHeader),
    hkpAction(hkpAction<'a>),
    hkpAgent1nSector(hkpAgent1nSector),
    hkpAngConstraintAtom(hkpAngConstraintAtom),
    hkpAngFrictionConstraintAtom(hkpAngFrictionConstraintAtom),
    hkpAngLimitConstraintAtom(hkpAngLimitConstraintAtom),
    hkpAngMotorConstraintAtom(hkpAngMotorConstraintAtom),
    hkpAngularDashpotAction(hkpAngularDashpotAction<'a>),
    hkpArrayAction(hkpArrayAction<'a>),
    hkpBallAndSocketConstraintData(hkpBallAndSocketConstraintData),
    hkpBallAndSocketConstraintDataAtoms(hkpBallAndSocketConstraintDataAtoms),
    hkpBallGun(hkpBallGun<'a>),
    hkpBallSocketChainData(hkpBallSocketChainData),
    hkpBallSocketChainDataConstraintInfo(hkpBallSocketChainDataConstraintInfo),
    hkpBallSocketConstraintAtom(hkpBallSocketConstraintAtom),
    hkpBinaryAction(hkpBinaryAction<'a>),
    hkpBoxMotion(hkpBoxMotion),
    hkpBoxShape(hkpBoxShape),
    hkpBreakableBody(hkpBreakableBody),
    hkpBreakableConstraintData(hkpBreakableConstraintData),
    hkpBridgeAtoms(hkpBridgeAtoms),
    hkpBridgeConstraintAtom(hkpBridgeConstraintAtom),
    hkpBroadPhaseHandle(hkpBroadPhaseHandle),
    hkpBvShape(hkpBvShape),
    hkpBvTreeShape(hkpBvTreeShape),
    hkpCachingShapePhantom(hkpCachingShapePhantom<'a>),
    hkpCallbackConstraintMotor(hkpCallbackConstraintMotor),
    hkpCapsuleShape(hkpCapsuleShape),
    hkpCdBody(hkpCdBody),
    hkpCenterOfMassChangerModifierConstraintAtom(
        hkpCenterOfMassChangerModifierConstraintAtom,
    ),
    hkpCharacterControllerCinfo(hkpCharacterControllerCinfo),
    hkpCharacterMotion(hkpCharacterMotion),
    hkpCharacterProxyCinfo(hkpCharacterProxyCinfo),
    hkpCharacterRigidBodyCinfo(hkpCharacterRigidBodyCinfo),
    hkpCogWheelConstraintAtom(hkpCogWheelConstraintAtom),
    hkpCogWheelConstraintData(hkpCogWheelConstraintData),
    hkpCogWheelConstraintDataAtoms(hkpCogWheelConstraintDataAtoms),
    hkpCollidable(hkpCollidable),
    hkpCollidableBoundingVolumeData(hkpCollidableBoundingVolumeData),
    hkpCollidableCollidableFilter(hkpCollidableCollidableFilter),
    hkpCollisionFilter(hkpCollisionFilter),
    hkpCollisionFilterList(hkpCollisionFilterList),
    hkpCompressedMeshShape(hkpCompressedMeshShape<'a>),
    hkpCompressedMeshShapeBigTriangle(hkpCompressedMeshShapeBigTriangle),
    hkpCompressedMeshShapeChunk(hkpCompressedMeshShapeChunk),
    hkpCompressedMeshShapeConvexPiece(hkpCompressedMeshShapeConvexPiece),
    hkpCompressedSampledHeightFieldShape(hkpCompressedSampledHeightFieldShape),
    hkpConeLimitConstraintAtom(hkpConeLimitConstraintAtom),
    hkpConstrainedSystemFilter(hkpConstrainedSystemFilter),
    hkpConstraintAtom(hkpConstraintAtom),
    hkpConstraintChainData(hkpConstraintChainData),
    hkpConstraintChainInstance(hkpConstraintChainInstance<'a>),
    hkpConstraintChainInstanceAction(hkpConstraintChainInstanceAction<'a>),
    hkpConstraintCollisionFilter(hkpConstraintCollisionFilter),
    hkpConstraintData(hkpConstraintData),
    hkpConstraintInstance(hkpConstraintInstance<'a>),
    hkpConstraintInstanceSmallArraySerializeOverrideType(
        hkpConstraintInstanceSmallArraySerializeOverrideType,
    ),
    hkpConstraintMotor(hkpConstraintMotor),
    hkpConvexListFilter(hkpConvexListFilter),
    hkpConvexListShape(hkpConvexListShape),
    hkpConvexPieceMeshShape(hkpConvexPieceMeshShape),
    hkpConvexPieceStreamData(hkpConvexPieceStreamData),
    hkpConvexShape(hkpConvexShape),
    hkpConvexTransformShape(hkpConvexTransformShape),
    hkpConvexTransformShapeBase(hkpConvexTransformShapeBase),
    hkpConvexTranslateShape(hkpConvexTranslateShape),
    hkpConvexVerticesConnectivity(hkpConvexVerticesConnectivity),
    hkpConvexVerticesShape(hkpConvexVerticesShape),
    hkpConvexVerticesShapeFourVectors(hkpConvexVerticesShapeFourVectors),
    hkpCylinderShape(hkpCylinderShape),
    hkpDashpotAction(hkpDashpotAction<'a>),
    hkpDefaultConvexListFilter(hkpDefaultConvexListFilter),
    hkpDefaultWorldMemoryWatchDog(hkpDefaultWorldMemoryWatchDog),
    hkpDisableEntityCollisionFilter(hkpDisableEntityCollisionFilter),
    hkpDisplayBindingData(hkpDisplayBindingData),
    hkpDisplayBindingDataPhysicsSystem(hkpDisplayBindingDataPhysicsSystem),
    hkpDisplayBindingDataRigidBody(hkpDisplayBindingDataRigidBody),
    hkpEntity(hkpEntity<'a>),
    hkpEntityExtendedListeners(hkpEntityExtendedListeners),
    hkpEntitySmallArraySerializeOverrideType(hkpEntitySmallArraySerializeOverrideType),
    hkpEntitySpuCollisionCallback(hkpEntitySpuCollisionCallback),
    hkpExtendedMeshShape(hkpExtendedMeshShape),
    hkpExtendedMeshShapeShapesSubpart(hkpExtendedMeshShapeShapesSubpart),
    hkpExtendedMeshShapeSubpart(hkpExtendedMeshShapeSubpart),
    hkpExtendedMeshShapeTrianglesSubpart(hkpExtendedMeshShapeTrianglesSubpart),
    hkpFastMeshShape(hkpFastMeshShape),
    hkpFirstPersonGun(hkpFirstPersonGun<'a>),
    hkpFixedRigidMotion(hkpFixedRigidMotion),
    hkpGenericConstraintData(hkpGenericConstraintData),
    hkpGenericConstraintDataScheme(hkpGenericConstraintDataScheme),
    hkpGenericConstraintDataSchemeConstraintInfo(
        hkpGenericConstraintDataSchemeConstraintInfo,
    ),
    hkpGravityGun(hkpGravityGun<'a>),
    hkpGroupCollisionFilter(hkpGroupCollisionFilter),
    hkpGroupFilter(hkpGroupFilter),
    hkpHeightFieldShape(hkpHeightFieldShape),
    hkpHingeConstraintData(hkpHingeConstraintData),
    hkpHingeConstraintDataAtoms(hkpHingeConstraintDataAtoms),
    hkpHingeLimitsData(hkpHingeLimitsData),
    hkpHingeLimitsDataAtoms(hkpHingeLimitsDataAtoms),
    hkpIgnoreModifierConstraintAtom(hkpIgnoreModifierConstraintAtom),
    hkpKeyframedRigidMotion(hkpKeyframedRigidMotion),
    hkpLimitedForceConstraintMotor(hkpLimitedForceConstraintMotor),
    hkpLimitedHingeConstraintData(hkpLimitedHingeConstraintData),
    hkpLimitedHingeConstraintDataAtoms(hkpLimitedHingeConstraintDataAtoms),
    hkpLinConstraintAtom(hkpLinConstraintAtom),
    hkpLinearParametricCurve(hkpLinearParametricCurve),
    hkpLinFrictionConstraintAtom(hkpLinFrictionConstraintAtom),
    hkpLinkedCollidable(hkpLinkedCollidable),
    hkpLinLimitConstraintAtom(hkpLinLimitConstraintAtom),
    hkpLinMotorConstraintAtom(hkpLinMotorConstraintAtom),
    hkpLinSoftConstraintAtom(hkpLinSoftConstraintAtom),
    hkpListShape(hkpListShape),
    hkpListShapeChildInfo(hkpListShapeChildInfo),
    hkpMalleableConstraintData(hkpMalleableConstraintData),
    hkpMassChangerModifierConstraintAtom(hkpMassChangerModifierConstraintAtom),
    hkpMassProperties(hkpMassProperties),
    hkpMaterial(hkpMaterial),
    hkpMaxSizeMotion(hkpMaxSizeMotion),
    hkpMeshMaterial(hkpMeshMaterial),
    hkpMeshShape(hkpMeshShape),
    hkpMeshShapeSubpart(hkpMeshShapeSubpart),
    hkpModifierConstraintAtom(hkpModifierConstraintAtom),
    hkpMoppBvTreeShape(hkpMoppBvTreeShape),
    hkpMoppCode(hkpMoppCode),
    hkpMoppCodeCodeInfo(hkpMoppCodeCodeInfo),
    hkpMoppCodeReindexedTerminal(hkpMoppCodeReindexedTerminal),
    hkpMotion(hkpMotion),
    hkpMotorAction(hkpMotorAction<'a>),
    hkpMountedBallGun(hkpMountedBallGun<'a>),
    hkpMouseSpringAction(hkpMouseSpringAction<'a>),
    hkpMovingSurfaceModifierConstraintAtom(hkpMovingSurfaceModifierConstraintAtom),
    hkpMultiRayShape(hkpMultiRayShape),
    hkpMultiRayShapeRay(hkpMultiRayShapeRay),
    hkpMultiSphereShape(hkpMultiSphereShape),
    hkpMultithreadedVehicleManager(hkpMultithreadedVehicleManager),
    hkpNamedMeshMaterial(hkpNamedMeshMaterial<'a>),
    hkpNullCollisionFilter(hkpNullCollisionFilter),
    hkPostFinishAttribute(hkPostFinishAttribute),
    hkpOverwritePivotConstraintAtom(hkpOverwritePivotConstraintAtom),
    hkpPairCollisionFilter(hkpPairCollisionFilter),
    hkpPairCollisionFilterMapPairFilterKeyOverrideType(
        hkpPairCollisionFilterMapPairFilterKeyOverrideType,
    ),
    hkpParametricCurve(hkpParametricCurve),
    hkpPhantom(hkpPhantom<'a>),
    hkpPhantomCallbackShape(hkpPhantomCallbackShape),
    hkpPhysicsData(hkpPhysicsData),
    hkpPhysicsSystem(hkpPhysicsSystem<'a>),
    hkpPhysicsSystemWithContacts(hkpPhysicsSystemWithContacts<'a>),
    hkpPlaneShape(hkpPlaneShape),
    hkpPointToPathConstraintData(hkpPointToPathConstraintData),
    hkpPointToPlaneConstraintData(hkpPointToPlaneConstraintData),
    hkpPointToPlaneConstraintDataAtoms(hkpPointToPlaneConstraintDataAtoms),
    hkpPositionConstraintMotor(hkpPositionConstraintMotor),
    hkpPoweredChainData(hkpPoweredChainData),
    hkpPoweredChainDataConstraintInfo(hkpPoweredChainDataConstraintInfo),
    hkpPoweredChainMapper(hkpPoweredChainMapper),
    hkpPoweredChainMapperLinkInfo(hkpPoweredChainMapperLinkInfo),
    hkpPoweredChainMapperTarget(hkpPoweredChainMapperTarget),
    hkpPrismaticConstraintData(hkpPrismaticConstraintData),
    hkpPrismaticConstraintDataAtoms(hkpPrismaticConstraintDataAtoms),
    hkpProjectileGun(hkpProjectileGun<'a>),
    hkpProperty(hkpProperty),
    hkpPropertyValue(hkpPropertyValue),
    hkpPulleyConstraintAtom(hkpPulleyConstraintAtom),
    hkpPulleyConstraintData(hkpPulleyConstraintData),
    hkpPulleyConstraintDataAtoms(hkpPulleyConstraintDataAtoms),
    hkpRackAndPinionConstraintAtom(hkpRackAndPinionConstraintAtom),
    hkpRackAndPinionConstraintData(hkpRackAndPinionConstraintData),
    hkpRackAndPinionConstraintDataAtoms(hkpRackAndPinionConstraintDataAtoms),
    hkpRagdollConstraintData(hkpRagdollConstraintData),
    hkpRagdollConstraintDataAtoms(hkpRagdollConstraintDataAtoms),
    hkpRagdollLimitsData(hkpRagdollLimitsData),
    hkpRagdollLimitsDataAtoms(hkpRagdollLimitsDataAtoms),
    hkpRagdollMotorConstraintAtom(hkpRagdollMotorConstraintAtom),
    hkpRayCollidableFilter(hkpRayCollidableFilter),
    hkpRayShapeCollectionFilter(hkpRayShapeCollectionFilter),
    hkpRejectChassisListener(hkpRejectChassisListener),
    hkpRemoveTerminalsMoppModifier(hkpRemoveTerminalsMoppModifier),
    hkpReorientAction(hkpReorientAction<'a>),
    hkpRigidBody(hkpRigidBody<'a>),
    hkpRotationalConstraintData(hkpRotationalConstraintData),
    hkpRotationalConstraintDataAtoms(hkpRotationalConstraintDataAtoms),
    hkpSampledHeightFieldShape(hkpSampledHeightFieldShape),
    hkpSerializedAgentNnEntry(hkpSerializedAgentNnEntry),
    hkpSerializedDisplayMarker(hkpSerializedDisplayMarker),
    hkpSerializedDisplayMarkerList(hkpSerializedDisplayMarkerList),
    hkpSerializedDisplayRbTransforms(hkpSerializedDisplayRbTransforms),
    hkpSerializedDisplayRbTransformsDisplayTransformPair(
        hkpSerializedDisplayRbTransformsDisplayTransformPair,
    ),
    hkpSerializedSubTrack1nInfo(hkpSerializedSubTrack1nInfo),
    hkpSerializedTrack1nInfo(hkpSerializedTrack1nInfo),
    hkpSetLocalRotationsConstraintAtom(hkpSetLocalRotationsConstraintAtom),
    hkpSetLocalTransformsConstraintAtom(hkpSetLocalTransformsConstraintAtom),
    hkpSetLocalTranslationsConstraintAtom(hkpSetLocalTranslationsConstraintAtom),
    hkpSetupStabilizationAtom(hkpSetupStabilizationAtom),
    hkpShape(hkpShape),
    hkpShapeCollection(hkpShapeCollection),
    hkpShapeCollectionFilter(hkpShapeCollectionFilter),
    hkpShapeContainer(hkpShapeContainer),
    hkpShapeInfo(hkpShapeInfo<'a>),
    hkpShapeModifier(hkpShapeModifier),
    hkpShapePhantom(hkpShapePhantom<'a>),
    hkpSimpleContactConstraintAtom(hkpSimpleContactConstraintAtom),
    hkpSimpleContactConstraintDataInfo(hkpSimpleContactConstraintDataInfo),
    hkpSimpleMeshShape(hkpSimpleMeshShape),
    hkpSimpleMeshShapeTriangle(hkpSimpleMeshShapeTriangle),
    hkpSimpleShapePhantom(hkpSimpleShapePhantom<'a>),
    hkpSimpleShapePhantomCollisionDetail(hkpSimpleShapePhantomCollisionDetail),
    hkpSimulation(hkpSimulation),
    hkpSingleShapeContainer(hkpSingleShapeContainer),
    hkpSoftContactModifierConstraintAtom(hkpSoftContactModifierConstraintAtom),
    hkpSphereMotion(hkpSphereMotion),
    hkpSphereRepShape(hkpSphereRepShape),
    hkpSphereShape(hkpSphereShape),
    hkpSpringAction(hkpSpringAction<'a>),
    hkpSpringDamperConstraintMotor(hkpSpringDamperConstraintMotor),
    hkpStiffSpringChainData(hkpStiffSpringChainData),
    hkpStiffSpringChainDataConstraintInfo(hkpStiffSpringChainDataConstraintInfo),
    hkpStiffSpringConstraintAtom(hkpStiffSpringConstraintAtom),
    hkpStiffSpringConstraintData(hkpStiffSpringConstraintData),
    hkpStiffSpringConstraintDataAtoms(hkpStiffSpringConstraintDataAtoms),
    hkpStorageExtendedMeshShape(hkpStorageExtendedMeshShape),
    hkpStorageExtendedMeshShapeMaterial(hkpStorageExtendedMeshShapeMaterial),
    hkpStorageExtendedMeshShapeMeshSubpartStorage(
        hkpStorageExtendedMeshShapeMeshSubpartStorage<'a>,
    ),
    hkpStorageExtendedMeshShapeShapeSubpartStorage(
        hkpStorageExtendedMeshShapeShapeSubpartStorage,
    ),
    hkpStorageMeshShape(hkpStorageMeshShape),
    hkpStorageMeshShapeSubpartStorage(hkpStorageMeshShapeSubpartStorage),
    hkpStorageSampledHeightFieldShape(hkpStorageSampledHeightFieldShape),
    hkpThinBoxMotion(hkpThinBoxMotion),
    hkpTransformShape(hkpTransformShape),
    hkpTriangleShape(hkpTriangleShape),
    hkpTriggerVolume(hkpTriggerVolume),
    hkpTriggerVolumeEventInfo(hkpTriggerVolumeEventInfo),
    hkpTriSampledHeightFieldBvTreeShape(hkpTriSampledHeightFieldBvTreeShape),
    hkpTriSampledHeightFieldCollection(hkpTriSampledHeightFieldCollection),
    hkpTwistLimitConstraintAtom(hkpTwistLimitConstraintAtom),
    hkpTypedBroadPhaseHandle(hkpTypedBroadPhaseHandle),
    hkpTyremarkPoint(hkpTyremarkPoint),
    hkpTyremarksInfo(hkpTyremarksInfo),
    hkpTyremarksWheel(hkpTyremarksWheel),
    hkpUnaryAction(hkpUnaryAction<'a>),
    hkpVehicleAerodynamics(hkpVehicleAerodynamics),
    hkpVehicleBrake(hkpVehicleBrake),
    hkpVehicleCastBatchingManager(hkpVehicleCastBatchingManager),
    hkpVehicleData(hkpVehicleData),
    hkpVehicleDataWheelComponentParams(hkpVehicleDataWheelComponentParams),
    hkpVehicleDefaultAerodynamics(hkpVehicleDefaultAerodynamics),
    hkpVehicleDefaultAnalogDriverInput(hkpVehicleDefaultAnalogDriverInput),
    hkpVehicleDefaultBrake(hkpVehicleDefaultBrake),
    hkpVehicleDefaultBrakeWheelBrakingProperties(
        hkpVehicleDefaultBrakeWheelBrakingProperties,
    ),
    hkpVehicleDefaultEngine(hkpVehicleDefaultEngine),
    hkpVehicleDefaultSteering(hkpVehicleDefaultSteering),
    hkpVehicleDefaultSuspension(hkpVehicleDefaultSuspension),
    hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
        hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
    ),
    hkpVehicleDefaultTransmission(hkpVehicleDefaultTransmission),
    hkpVehicleDefaultVelocityDamper(hkpVehicleDefaultVelocityDamper),
    hkpVehicleDriverInput(hkpVehicleDriverInput),
    hkpVehicleDriverInputAnalogStatus(hkpVehicleDriverInputAnalogStatus),
    hkpVehicleDriverInputStatus(hkpVehicleDriverInputStatus),
    hkpVehicleEngine(hkpVehicleEngine),
    hkpVehicleFrictionDescription(hkpVehicleFrictionDescription),
    hkpVehicleFrictionDescriptionAxisDescription(
        hkpVehicleFrictionDescriptionAxisDescription,
    ),
    hkpVehicleFrictionStatus(hkpVehicleFrictionStatus),
    hkpVehicleFrictionStatusAxisStatus(hkpVehicleFrictionStatusAxisStatus),
    hkpVehicleInstance(hkpVehicleInstance<'a>),
    hkpVehicleInstanceWheelInfo(hkpVehicleInstanceWheelInfo),
    hkpVehicleLinearCastBatchingManager(hkpVehicleLinearCastBatchingManager),
    hkpVehicleLinearCastWheelCollide(hkpVehicleLinearCastWheelCollide),
    hkpVehicleLinearCastWheelCollideWheelState(
        hkpVehicleLinearCastWheelCollideWheelState,
    ),
    hkpVehicleManager(hkpVehicleManager),
    hkpVehicleRayCastBatchingManager(hkpVehicleRayCastBatchingManager),
    hkpVehicleRayCastWheelCollide(hkpVehicleRayCastWheelCollide),
    hkpVehicleSteering(hkpVehicleSteering),
    hkpVehicleSuspension(hkpVehicleSuspension),
    hkpVehicleSuspensionSuspensionWheelParameters(
        hkpVehicleSuspensionSuspensionWheelParameters,
    ),
    hkpVehicleTransmission(hkpVehicleTransmission),
    hkpVehicleVelocityDamper(hkpVehicleVelocityDamper),
    hkpVehicleWheelCollide(hkpVehicleWheelCollide),
    hkpVelocityConstraintMotor(hkpVelocityConstraintMotor),
    hkpViscousSurfaceModifierConstraintAtom(hkpViscousSurfaceModifierConstraintAtom),
    hkpWeldingUtility(hkpWeldingUtility),
    hkpWheelConstraintData(hkpWheelConstraintData),
    hkpWheelConstraintDataAtoms(hkpWheelConstraintDataAtoms),
    hkpWorld(hkpWorld),
    hkpWorldCinfo(hkpWorldCinfo),
    hkpWorldObject(hkpWorldObject<'a>),
    hkQTransform(hkQTransform),
    hkRangeInt32Attribute(hkRangeInt32Attribute),
    hkRangeRealAttribute(hkRangeRealAttribute),
    hkReferencedObject(hkReferencedObject),
    hkReflectedFileAttribute(hkReflectedFileAttribute<'a>),
    hkResourceBase(hkResourceBase),
    hkResourceContainer(hkResourceContainer),
    hkResourceHandle(hkResourceHandle),
    hkRootLevelContainer(hkRootLevelContainer<'a>),
    hkRootLevelContainerNamedVariant(hkRootLevelContainerNamedVariant<'a>),
    hkSemanticsAttribute(hkSemanticsAttribute),
    hkSimpleLocalFrame(hkSimpleLocalFrame<'a>),
    hkSphere(hkSphere),
    hkSweptTransform(hkSweptTransform),
    hkTraceStreamTitle(hkTraceStreamTitle),
    hkTrackerSerializableScanSnapshot(hkTrackerSerializableScanSnapshot),
    hkTrackerSerializableScanSnapshotAllocation(
        hkTrackerSerializableScanSnapshotAllocation,
    ),
    hkTrackerSerializableScanSnapshotBlock(hkTrackerSerializableScanSnapshotBlock),
    hkUiAttribute(hkUiAttribute<'a>),
    hkVertexFormat(hkVertexFormat),
    hkVertexFormatElement(hkVertexFormatElement),
    hkWorldMemoryAvailableWatchDog(hkWorldMemoryAvailableWatchDog),
    hkxAnimatedFloat(hkxAnimatedFloat),
    hkxAnimatedMatrix(hkxAnimatedMatrix),
    hkxAnimatedQuaternion(hkxAnimatedQuaternion),
    hkxAnimatedVector(hkxAnimatedVector),
    hkxAttribute(hkxAttribute<'a>),
    hkxAttributeGroup(hkxAttributeGroup<'a>),
    hkxAttributeHolder(hkxAttributeHolder<'a>),
    hkxCamera(hkxCamera),
    hkxEdgeSelectionChannel(hkxEdgeSelectionChannel),
    hkxEnum(hkxEnum<'a>),
    hkxEnumItem(hkxEnumItem<'a>),
    hkxEnvironment(hkxEnvironment<'a>),
    hkxEnvironmentVariable(hkxEnvironmentVariable<'a>),
    hkxIndexBuffer(hkxIndexBuffer),
    hkxLight(hkxLight),
    hkxMaterial(hkxMaterial<'a>),
    hkxMaterialEffect(hkxMaterialEffect<'a>),
    hkxMaterialProperty(hkxMaterialProperty),
    hkxMaterialShader(hkxMaterialShader<'a>),
    hkxMaterialShaderSet(hkxMaterialShaderSet),
    hkxMaterialTextureStage(hkxMaterialTextureStage),
    hkxMesh(hkxMesh),
    hkxMeshSection(hkxMeshSection),
    hkxMeshUserChannelInfo(hkxMeshUserChannelInfo<'a>),
    hkxNode(hkxNode<'a>),
    hkxNodeAnnotationData(hkxNodeAnnotationData<'a>),
    hkxNodeSelectionSet(hkxNodeSelectionSet<'a>),
    hkxScene(hkxScene<'a>),
    hkxSkinBinding(hkxSkinBinding<'a>),
    hkxSparselyAnimatedBool(hkxSparselyAnimatedBool),
    hkxSparselyAnimatedEnum(hkxSparselyAnimatedEnum),
    hkxSparselyAnimatedInt(hkxSparselyAnimatedInt),
    hkxSparselyAnimatedString(hkxSparselyAnimatedString<'a>),
    hkxTextureFile(hkxTextureFile<'a>),
    hkxTextureInplace(hkxTextureInplace<'a>),
    hkxTriangleSelectionChannel(hkxTriangleSelectionChannel),
    hkxVertexBuffer(hkxVertexBuffer),
    hkxVertexBufferVertexData(hkxVertexBufferVertexData),
    hkxVertexDescription(hkxVertexDescription),
    hkxVertexDescriptionElementDecl(hkxVertexDescriptionElementDecl),
    hkxVertexFloatDataChannel(hkxVertexFloatDataChannel),
    hkxVertexIntDataChannel(hkxVertexIntDataChannel),
    hkxVertexSelectionChannel(hkxVertexSelectionChannel),
    hkxVertexVectorDataChannel(hkxVertexVectorDataChannel),
}
impl _serde::HavokClass for Classes<'_> {
    fn name(&self) -> &'static str {
        match &self {
            Classes::SwapDummy => {
                panic!(
                    "The dummy class is used only for sorting, so being called name is not a good use of the API."
                )
            }
            Classes::BGSGamebryoSequenceGenerator(class) => class.name(),
            Classes::BSBoneSwitchGenerator(class) => class.name(),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.name(),
            Classes::BSComputeAddBoneAnimModifier(class) => class.name(),
            Classes::BSCyclicBlendTransitionGenerator(class) => class.name(),
            Classes::BSDecomposeVectorModifier(class) => class.name(),
            Classes::BSDirectAtModifier(class) => class.name(),
            Classes::BSDistTriggerModifier(class) => class.name(),
            Classes::BSEventEveryNEventsModifier(class) => class.name(),
            Classes::BSEventOnDeactivateModifier(class) => class.name(),
            Classes::BSEventOnFalseToTrueModifier(class) => class.name(),
            Classes::BSGetTimeStepModifier(class) => class.name(),
            Classes::BSInterpValueModifier(class) => class.name(),
            Classes::BSIsActiveModifier(class) => class.name(),
            Classes::BSIStateManagerModifier(class) => class.name(),
            Classes::BSIStateManagerModifierBSiStateData(class) => class.name(),
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.name()
            }
            Classes::BSiStateTaggingGenerator(class) => class.name(),
            Classes::BSLimbIKModifier(class) => class.name(),
            Classes::BSLookAtModifier(class) => class.name(),
            Classes::BSLookAtModifierBoneData(class) => class.name(),
            Classes::BSModifyOnceModifier(class) => class.name(),
            Classes::BSOffsetAnimationGenerator(class) => class.name(),
            Classes::BSPassByTargetTriggerModifier(class) => class.name(),
            Classes::BSRagdollContactListenerModifier(class) => class.name(),
            Classes::BSSpeedSamplerModifier(class) => class.name(),
            Classes::BSSynchronizedClipGenerator(class) => class.name(),
            Classes::BSTimerModifier(class) => class.name(),
            Classes::BSTweenerModifier(class) => class.name(),
            Classes::hkAabb(class) => class.name(),
            Classes::hkAabbHalf(class) => class.name(),
            Classes::hkAabbUint32(class) => class.name(),
            Classes::hkaAnimatedReferenceFrame(class) => class.name(),
            Classes::hkaAnimation(class) => class.name(),
            Classes::hkaAnimationBinding(class) => class.name(),
            Classes::hkaAnimationContainer(class) => class.name(),
            Classes::hkaAnimationPreviewColorContainer(class) => class.name(),
            Classes::hkaAnnotationTrack(class) => class.name(),
            Classes::hkaAnnotationTrackAnnotation(class) => class.name(),
            Classes::hkaBone(class) => class.name(),
            Classes::hkaBoneAttachment(class) => class.name(),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => class.name(),
            Classes::hkaDeltaCompressedAnimation(class) => class.name(),
            Classes::hkaDeltaCompressedAnimationQuantizationFormat(class) => class.name(),
            Classes::hkaFootstepAnalysisInfo(class) => class.name(),
            Classes::hkaFootstepAnalysisInfoContainer(class) => class.name(),
            Classes::hkaInterleavedUncompressedAnimation(class) => class.name(),
            Classes::hkaKeyFrameHierarchyUtility(class) => class.name(),
            Classes::hkaKeyFrameHierarchyUtilityControlData(class) => class.name(),
            Classes::hkAlignSceneToNodeOptions(class) => class.name(),
            Classes::hkaMeshBinding(class) => class.name(),
            Classes::hkaMeshBindingMapping(class) => class.name(),
            Classes::hkaQuantizedAnimation(class) => class.name(),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => class.name(),
            Classes::hkaRagdollInstance(class) => class.name(),
            Classes::hkArrayTypeAttribute(class) => class.name(),
            Classes::hkaSkeleton(class) => class.name(),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.name(),
            Classes::hkaSkeletonMapper(class) => class.name(),
            Classes::hkaSkeletonMapperData(class) => class.name(),
            Classes::hkaSkeletonMapperDataChainMapping(class) => class.name(),
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => class.name(),
            Classes::hkaSplineCompressedAnimation(class) => class.name(),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.name()
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.name()
            }
            Classes::hkaWaveletCompressedAnimation(class) => class.name(),
            Classes::hkaWaveletCompressedAnimationCompressionParams(class) => {
                class.name()
            }
            Classes::hkaWaveletCompressedAnimationQuantizationFormat(class) => {
                class.name()
            }
            Classes::hkBaseObject(class) => class.name(),
            Classes::hkbAttachmentModifier(class) => class.name(),
            Classes::hkbAttachmentSetup(class) => class.name(),
            Classes::hkbAttributeModifier(class) => class.name(),
            Classes::hkbAttributeModifierAssignment(class) => class.name(),
            Classes::hkbAuxiliaryNodeInfo(class) => class.name(),
            Classes::hkbBehaviorEventsInfo(class) => class.name(),
            Classes::hkbBehaviorGraph(class) => class.name(),
            Classes::hkbBehaviorGraphData(class) => class.name(),
            Classes::hkbBehaviorGraphInternalState(class) => class.name(),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => class.name(),
            Classes::hkbBehaviorGraphStringData(class) => class.name(),
            Classes::hkbBehaviorInfo(class) => class.name(),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.name(),
            Classes::hkbBehaviorReferenceGenerator(class) => class.name(),
            Classes::hkbBindable(class) => class.name(),
            Classes::hkbBlendCurveUtils(class) => class.name(),
            Classes::hkbBlenderGenerator(class) => class.name(),
            Classes::hkbBlenderGeneratorChild(class) => class.name(),
            Classes::hkbBlenderGeneratorChildInternalState(class) => class.name(),
            Classes::hkbBlenderGeneratorInternalState(class) => class.name(),
            Classes::hkbBlendingTransitionEffect(class) => class.name(),
            Classes::hkbBlendingTransitionEffectInternalState(class) => class.name(),
            Classes::hkbBoneIndexArray(class) => class.name(),
            Classes::hkbBoneWeightArray(class) => class.name(),
            Classes::hkbBoolVariableSequencedData(class) => class.name(),
            Classes::hkbBoolVariableSequencedDataSample(class) => class.name(),
            Classes::hkbCameraShakeEventPayload(class) => class.name(),
            Classes::hkbCharacter(class) => class.name(),
            Classes::hkbCharacterAddedInfo(class) => class.name(),
            Classes::hkbCharacterControlCommand(class) => class.name(),
            Classes::hkbCharacterControllerControlData(class) => class.name(),
            Classes::hkbCharacterControllerModifier(class) => class.name(),
            Classes::hkbCharacterControllerModifierInternalState(class) => class.name(),
            Classes::hkbCharacterData(class) => class.name(),
            Classes::hkbCharacterDataCharacterControllerInfo(class) => class.name(),
            Classes::hkbCharacterInfo(class) => class.name(),
            Classes::hkbCharacterSetup(class) => class.name(),
            Classes::hkbCharacterSkinInfo(class) => class.name(),
            Classes::hkbCharacterSteppedInfo(class) => class.name(),
            Classes::hkbCharacterStringData(class) => class.name(),
            Classes::hkbClientCharacterState(class) => class.name(),
            Classes::hkbClipGenerator(class) => class.name(),
            Classes::hkbClipGeneratorEcho(class) => class.name(),
            Classes::hkbClipGeneratorInternalState(class) => class.name(),
            Classes::hkbClipTrigger(class) => class.name(),
            Classes::hkbClipTriggerArray(class) => class.name(),
            Classes::hkbCombineTransformsModifier(class) => class.name(),
            Classes::hkbCombineTransformsModifierInternalState(class) => class.name(),
            Classes::hkbCompiledExpressionSet(class) => class.name(),
            Classes::hkbCompiledExpressionSetToken(class) => class.name(),
            Classes::hkbComputeDirectionModifier(class) => class.name(),
            Classes::hkbComputeDirectionModifierInternalState(class) => class.name(),
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => class.name(),
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.name()
            }
            Classes::hkbComputeRotationToTargetModifier(class) => class.name(),
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.name()
            }
            Classes::hkbCondition(class) => class.name(),
            Classes::hkbContext(class) => class.name(),
            Classes::hkbDampingModifier(class) => class.name(),
            Classes::hkbDampingModifierInternalState(class) => class.name(),
            Classes::hkbDefaultMessageLog(class) => class.name(),
            Classes::hkbDelayedModifier(class) => class.name(),
            Classes::hkbDelayedModifierInternalState(class) => class.name(),
            Classes::hkbDetectCloseToGroundModifier(class) => class.name(),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => class.name(),
            Classes::hkbEvaluateExpressionModifier(class) => class.name(),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.name()
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => class.name(),
            Classes::hkbEvaluateHandleModifier(class) => class.name(),
            Classes::hkbEvent(class) => class.name(),
            Classes::hkbEventBase(class) => class.name(),
            Classes::hkbEventDrivenModifier(class) => class.name(),
            Classes::hkbEventDrivenModifierInternalState(class) => class.name(),
            Classes::hkbEventInfo(class) => class.name(),
            Classes::hkbEventPayload(class) => class.name(),
            Classes::hkbEventPayloadList(class) => class.name(),
            Classes::hkbEventProperty(class) => class.name(),
            Classes::hkbEventRaisedInfo(class) => class.name(),
            Classes::hkbEventRangeData(class) => class.name(),
            Classes::hkbEventRangeDataArray(class) => class.name(),
            Classes::hkbEventSequencedData(class) => class.name(),
            Classes::hkbEventSequencedDataSequencedEvent(class) => class.name(),
            Classes::hkbEventsFromRangeModifier(class) => class.name(),
            Classes::hkbEventsFromRangeModifierInternalState(class) => class.name(),
            Classes::hkbExpressionCondition(class) => class.name(),
            Classes::hkbExpressionData(class) => class.name(),
            Classes::hkbExpressionDataArray(class) => class.name(),
            Classes::hkbExtractRagdollPoseModifier(class) => class.name(),
            Classes::hkbFootIkControlData(class) => class.name(),
            Classes::hkbFootIkControlsModifier(class) => class.name(),
            Classes::hkbFootIkControlsModifierLeg(class) => class.name(),
            Classes::hkbFootIkDriverInfo(class) => class.name(),
            Classes::hkbFootIkDriverInfoLeg(class) => class.name(),
            Classes::hkbFootIkGains(class) => class.name(),
            Classes::hkbFootIkModifier(class) => class.name(),
            Classes::hkbFootIkModifierInternalLegData(class) => class.name(),
            Classes::hkbFootIkModifierLeg(class) => class.name(),
            Classes::hkbGenerator(class) => class.name(),
            Classes::hkbGeneratorOutputListener(class) => class.name(),
            Classes::hkbGeneratorSyncInfo(class) => class.name(),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.name(),
            Classes::hkbGeneratorTransitionEffect(class) => class.name(),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => class.name(),
            Classes::hkbGetHandleOnBoneModifier(class) => class.name(),
            Classes::hkbGetUpModifier(class) => class.name(),
            Classes::hkbGetUpModifierInternalState(class) => class.name(),
            Classes::hkbGetWorldFromModelModifier(class) => class.name(),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => class.name(),
            Classes::hkbHandIkControlData(class) => class.name(),
            Classes::hkbHandIkControlsModifier(class) => class.name(),
            Classes::hkbHandIkControlsModifierHand(class) => class.name(),
            Classes::hkbHandIkDriverInfo(class) => class.name(),
            Classes::hkbHandIkDriverInfoHand(class) => class.name(),
            Classes::hkbHandIkModifier(class) => class.name(),
            Classes::hkbHandIkModifierHand(class) => class.name(),
            Classes::hkbHandle(class) => class.name(),
            Classes::hkbIntEventPayload(class) => class.name(),
            Classes::hkbIntVariableSequencedData(class) => class.name(),
            Classes::hkbIntVariableSequencedDataSample(class) => class.name(),
            Classes::hkBitField(class) => class.name(),
            Classes::hkbKeyframeBonesModifier(class) => class.name(),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => class.name(),
            Classes::hkbLinkedSymbolInfo(class) => class.name(),
            Classes::hkbLookAtModifier(class) => class.name(),
            Classes::hkbLookAtModifierInternalState(class) => class.name(),
            Classes::hkbManualSelectorGenerator(class) => class.name(),
            Classes::hkbManualSelectorGeneratorInternalState(class) => class.name(),
            Classes::hkbMessageLog(class) => class.name(),
            Classes::hkbMirroredSkeletonInfo(class) => class.name(),
            Classes::hkbMirrorModifier(class) => class.name(),
            Classes::hkbModifier(class) => class.name(),
            Classes::hkbModifierGenerator(class) => class.name(),
            Classes::hkbModifierList(class) => class.name(),
            Classes::hkbModifierWrapper(class) => class.name(),
            Classes::hkbMoveCharacterModifier(class) => class.name(),
            Classes::hkbMoveCharacterModifierInternalState(class) => class.name(),
            Classes::hkbNamedEventPayload(class) => class.name(),
            Classes::hkbNamedIntEventPayload(class) => class.name(),
            Classes::hkbNamedRealEventPayload(class) => class.name(),
            Classes::hkbNamedStringEventPayload(class) => class.name(),
            Classes::hkbNode(class) => class.name(),
            Classes::hkbNodeInternalStateInfo(class) => class.name(),
            Classes::hkbParticleSystemEventPayload(class) => class.name(),
            Classes::hkbPoseMatchingGenerator(class) => class.name(),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => class.name(),
            Classes::hkbPoweredRagdollControlData(class) => class.name(),
            Classes::hkbPoweredRagdollControlsModifier(class) => class.name(),
            Classes::hkbProjectData(class) => class.name(),
            Classes::hkbProjectStringData(class) => class.name(),
            Classes::hkbProxyModifier(class) => class.name(),
            Classes::hkbProxyModifierProxyInfo(class) => class.name(),
            Classes::hkbRaiseEventCommand(class) => class.name(),
            Classes::hkbRealEventPayload(class) => class.name(),
            Classes::hkbRealVariableSequencedData(class) => class.name(),
            Classes::hkbRealVariableSequencedDataSample(class) => class.name(),
            Classes::hkbReferencePoseGenerator(class) => class.name(),
            Classes::hkbRegisteredGenerator(class) => class.name(),
            Classes::hkbRigidBodyRagdollControlData(class) => class.name(),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => class.name(),
            Classes::hkbRoleAttribute(class) => class.name(),
            Classes::hkbRotateCharacterModifier(class) => class.name(),
            Classes::hkbRotateCharacterModifierInternalState(class) => class.name(),
            Classes::hkbSenseHandleModifier(class) => class.name(),
            Classes::hkbSenseHandleModifierRange(class) => class.name(),
            Classes::hkbSequence(class) => class.name(),
            Classes::hkbSequencedData(class) => class.name(),
            Classes::hkbSequenceInternalState(class) => class.name(),
            Classes::hkbSequenceStringData(class) => class.name(),
            Classes::hkbSetBehaviorCommand(class) => class.name(),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => class.name(),
            Classes::hkbSetNodePropertyCommand(class) => class.name(),
            Classes::hkbSetWordVariableCommand(class) => class.name(),
            Classes::hkbSetWorldFromModelModifier(class) => class.name(),
            Classes::hkbSimulationControlCommand(class) => class.name(),
            Classes::hkbSimulationStateInfo(class) => class.name(),
            Classes::hkbStateChooser(class) => class.name(),
            Classes::hkbStateListener(class) => class.name(),
            Classes::hkbStateMachine(class) => class.name(),
            Classes::hkbStateMachineActiveTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineDelayedTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineEventPropertyArray(class) => class.name(),
            Classes::hkbStateMachineInternalState(class) => class.name(),
            Classes::hkbStateMachineNestedStateMachineData(class) => class.name(),
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineStateInfo(class) => class.name(),
            Classes::hkbStateMachineTimeInterval(class) => class.name(),
            Classes::hkbStateMachineTransitionInfo(class) => class.name(),
            Classes::hkbStateMachineTransitionInfoArray(class) => class.name(),
            Classes::hkbStateMachineTransitionInfoReference(class) => class.name(),
            Classes::hkbStringCondition(class) => class.name(),
            Classes::hkbStringEventPayload(class) => class.name(),
            Classes::hkbTestStateChooser(class) => class.name(),
            Classes::hkbTimerModifier(class) => class.name(),
            Classes::hkbTimerModifierInternalState(class) => class.name(),
            Classes::hkbTransformVectorModifier(class) => class.name(),
            Classes::hkbTransformVectorModifierInternalState(class) => class.name(),
            Classes::hkbTransitionEffect(class) => class.name(),
            Classes::hkbTwistModifier(class) => class.name(),
            Classes::hkbVariableBindingSet(class) => class.name(),
            Classes::hkbVariableBindingSetBinding(class) => class.name(),
            Classes::hkbVariableInfo(class) => class.name(),
            Classes::hkbVariableValue(class) => class.name(),
            Classes::hkbVariableValueSet(class) => class.name(),
            Classes::hkbWorldEnums(class) => class.name(),
            Classes::hkbWorldFromModelModeData(class) => class.name(),
            Classes::hkClass(class) => class.name(),
            Classes::hkClassEnum(class) => class.name(),
            Classes::hkClassEnumItem(class) => class.name(),
            Classes::hkClassMember(class) => class.name(),
            Classes::hkColor(class) => class.name(),
            Classes::hkContactPoint(class) => class.name(),
            Classes::hkContactPointMaterial(class) => class.name(),
            Classes::hkCustomAttributes(class) => class.name(),
            Classes::hkCustomAttributesAttribute(class) => class.name(),
            Classes::hkDataObjectTypeAttribute(class) => class.name(),
            Classes::hkDescriptionAttribute(class) => class.name(),
            Classes::hkDocumentationAttribute(class) => class.name(),
            Classes::hkGeometry(class) => class.name(),
            Classes::hkGeometryTriangle(class) => class.name(),
            Classes::hkGizmoAttribute(class) => class.name(),
            Classes::hkHalf8(class) => class.name(),
            Classes::hkIndexedTransformSet(class) => class.name(),
            Classes::hkLinkAttribute(class) => class.name(),
            Classes::hkLocalFrame(class) => class.name(),
            Classes::hkLocalFrameGroup(class) => class.name(),
            Classes::hkMemoryMeshBody(class) => class.name(),
            Classes::hkMemoryMeshMaterial(class) => class.name(),
            Classes::hkMemoryMeshShape(class) => class.name(),
            Classes::hkMemoryMeshTexture(class) => class.name(),
            Classes::hkMemoryMeshVertexBuffer(class) => class.name(),
            Classes::hkMemoryResourceContainer(class) => class.name(),
            Classes::hkMemoryResourceHandle(class) => class.name(),
            Classes::hkMemoryResourceHandleExternalLink(class) => class.name(),
            Classes::hkMemoryTrackerAttribute(class) => class.name(),
            Classes::hkMeshBody(class) => class.name(),
            Classes::hkMeshBoneIndexMapping(class) => class.name(),
            Classes::hkMeshMaterial(class) => class.name(),
            Classes::hkMeshSection(class) => class.name(),
            Classes::hkMeshSectionCinfo(class) => class.name(),
            Classes::hkMeshShape(class) => class.name(),
            Classes::hkMeshTexture(class) => class.name(),
            Classes::hkMeshVertexBuffer(class) => class.name(),
            Classes::hkModelerNodeTypeAttribute(class) => class.name(),
            Classes::hkMonitorStreamColorTable(class) => class.name(),
            Classes::hkMonitorStreamColorTableColorPair(class) => class.name(),
            Classes::hkMonitorStreamFrameInfo(class) => class.name(),
            Classes::hkMonitorStreamStringMap(class) => class.name(),
            Classes::hkMonitorStreamStringMapStringMap(class) => class.name(),
            Classes::hkMoppBvTreeShapeBase(class) => class.name(),
            Classes::hkMotionState(class) => class.name(),
            Classes::hkMultipleVertexBuffer(class) => class.name(),
            Classes::hkMultipleVertexBufferElementInfo(class) => class.name(),
            Classes::hkMultipleVertexBufferLockedElement(class) => class.name(),
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => class.name(),
            Classes::hkMultiThreadCheck(class) => class.name(),
            Classes::hkp2dAngConstraintAtom(class) => class.name(),
            Classes::hkpAabbPhantom(class) => class.name(),
            Classes::hkPackedVector3(class) => class.name(),
            Classes::hkPackfileHeader(class) => class.name(),
            Classes::hkPackfileSectionHeader(class) => class.name(),
            Classes::hkpAction(class) => class.name(),
            Classes::hkpAgent1nSector(class) => class.name(),
            Classes::hkpAngConstraintAtom(class) => class.name(),
            Classes::hkpAngFrictionConstraintAtom(class) => class.name(),
            Classes::hkpAngLimitConstraintAtom(class) => class.name(),
            Classes::hkpAngMotorConstraintAtom(class) => class.name(),
            Classes::hkpAngularDashpotAction(class) => class.name(),
            Classes::hkpArrayAction(class) => class.name(),
            Classes::hkpBallAndSocketConstraintData(class) => class.name(),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => class.name(),
            Classes::hkpBallGun(class) => class.name(),
            Classes::hkpBallSocketChainData(class) => class.name(),
            Classes::hkpBallSocketChainDataConstraintInfo(class) => class.name(),
            Classes::hkpBallSocketConstraintAtom(class) => class.name(),
            Classes::hkpBinaryAction(class) => class.name(),
            Classes::hkpBoxMotion(class) => class.name(),
            Classes::hkpBoxShape(class) => class.name(),
            Classes::hkpBreakableBody(class) => class.name(),
            Classes::hkpBreakableConstraintData(class) => class.name(),
            Classes::hkpBridgeAtoms(class) => class.name(),
            Classes::hkpBridgeConstraintAtom(class) => class.name(),
            Classes::hkpBroadPhaseHandle(class) => class.name(),
            Classes::hkpBvShape(class) => class.name(),
            Classes::hkpBvTreeShape(class) => class.name(),
            Classes::hkpCachingShapePhantom(class) => class.name(),
            Classes::hkpCallbackConstraintMotor(class) => class.name(),
            Classes::hkpCapsuleShape(class) => class.name(),
            Classes::hkpCdBody(class) => class.name(),
            Classes::hkpCenterOfMassChangerModifierConstraintAtom(class) => class.name(),
            Classes::hkpCharacterControllerCinfo(class) => class.name(),
            Classes::hkpCharacterMotion(class) => class.name(),
            Classes::hkpCharacterProxyCinfo(class) => class.name(),
            Classes::hkpCharacterRigidBodyCinfo(class) => class.name(),
            Classes::hkpCogWheelConstraintAtom(class) => class.name(),
            Classes::hkpCogWheelConstraintData(class) => class.name(),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.name(),
            Classes::hkpCollidable(class) => class.name(),
            Classes::hkpCollidableBoundingVolumeData(class) => class.name(),
            Classes::hkpCollidableCollidableFilter(class) => class.name(),
            Classes::hkpCollisionFilter(class) => class.name(),
            Classes::hkpCollisionFilterList(class) => class.name(),
            Classes::hkpCompressedMeshShape(class) => class.name(),
            Classes::hkpCompressedMeshShapeBigTriangle(class) => class.name(),
            Classes::hkpCompressedMeshShapeChunk(class) => class.name(),
            Classes::hkpCompressedMeshShapeConvexPiece(class) => class.name(),
            Classes::hkpCompressedSampledHeightFieldShape(class) => class.name(),
            Classes::hkpConeLimitConstraintAtom(class) => class.name(),
            Classes::hkpConstrainedSystemFilter(class) => class.name(),
            Classes::hkpConstraintAtom(class) => class.name(),
            Classes::hkpConstraintChainData(class) => class.name(),
            Classes::hkpConstraintChainInstance(class) => class.name(),
            Classes::hkpConstraintChainInstanceAction(class) => class.name(),
            Classes::hkpConstraintCollisionFilter(class) => class.name(),
            Classes::hkpConstraintData(class) => class.name(),
            Classes::hkpConstraintInstance(class) => class.name(),
            Classes::hkpConstraintInstanceSmallArraySerializeOverrideType(class) => {
                class.name()
            }
            Classes::hkpConstraintMotor(class) => class.name(),
            Classes::hkpConvexListFilter(class) => class.name(),
            Classes::hkpConvexListShape(class) => class.name(),
            Classes::hkpConvexPieceMeshShape(class) => class.name(),
            Classes::hkpConvexPieceStreamData(class) => class.name(),
            Classes::hkpConvexShape(class) => class.name(),
            Classes::hkpConvexTransformShape(class) => class.name(),
            Classes::hkpConvexTransformShapeBase(class) => class.name(),
            Classes::hkpConvexTranslateShape(class) => class.name(),
            Classes::hkpConvexVerticesConnectivity(class) => class.name(),
            Classes::hkpConvexVerticesShape(class) => class.name(),
            Classes::hkpConvexVerticesShapeFourVectors(class) => class.name(),
            Classes::hkpCylinderShape(class) => class.name(),
            Classes::hkpDashpotAction(class) => class.name(),
            Classes::hkpDefaultConvexListFilter(class) => class.name(),
            Classes::hkpDefaultWorldMemoryWatchDog(class) => class.name(),
            Classes::hkpDisableEntityCollisionFilter(class) => class.name(),
            Classes::hkpDisplayBindingData(class) => class.name(),
            Classes::hkpDisplayBindingDataPhysicsSystem(class) => class.name(),
            Classes::hkpDisplayBindingDataRigidBody(class) => class.name(),
            Classes::hkpEntity(class) => class.name(),
            Classes::hkpEntityExtendedListeners(class) => class.name(),
            Classes::hkpEntitySmallArraySerializeOverrideType(class) => class.name(),
            Classes::hkpEntitySpuCollisionCallback(class) => class.name(),
            Classes::hkpExtendedMeshShape(class) => class.name(),
            Classes::hkpExtendedMeshShapeShapesSubpart(class) => class.name(),
            Classes::hkpExtendedMeshShapeSubpart(class) => class.name(),
            Classes::hkpExtendedMeshShapeTrianglesSubpart(class) => class.name(),
            Classes::hkpFastMeshShape(class) => class.name(),
            Classes::hkpFirstPersonGun(class) => class.name(),
            Classes::hkpFixedRigidMotion(class) => class.name(),
            Classes::hkpGenericConstraintData(class) => class.name(),
            Classes::hkpGenericConstraintDataScheme(class) => class.name(),
            Classes::hkpGenericConstraintDataSchemeConstraintInfo(class) => class.name(),
            Classes::hkpGravityGun(class) => class.name(),
            Classes::hkpGroupCollisionFilter(class) => class.name(),
            Classes::hkpGroupFilter(class) => class.name(),
            Classes::hkpHeightFieldShape(class) => class.name(),
            Classes::hkpHingeConstraintData(class) => class.name(),
            Classes::hkpHingeConstraintDataAtoms(class) => class.name(),
            Classes::hkpHingeLimitsData(class) => class.name(),
            Classes::hkpHingeLimitsDataAtoms(class) => class.name(),
            Classes::hkpIgnoreModifierConstraintAtom(class) => class.name(),
            Classes::hkpKeyframedRigidMotion(class) => class.name(),
            Classes::hkpLimitedForceConstraintMotor(class) => class.name(),
            Classes::hkpLimitedHingeConstraintData(class) => class.name(),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => class.name(),
            Classes::hkpLinConstraintAtom(class) => class.name(),
            Classes::hkpLinearParametricCurve(class) => class.name(),
            Classes::hkpLinFrictionConstraintAtom(class) => class.name(),
            Classes::hkpLinkedCollidable(class) => class.name(),
            Classes::hkpLinLimitConstraintAtom(class) => class.name(),
            Classes::hkpLinMotorConstraintAtom(class) => class.name(),
            Classes::hkpLinSoftConstraintAtom(class) => class.name(),
            Classes::hkpListShape(class) => class.name(),
            Classes::hkpListShapeChildInfo(class) => class.name(),
            Classes::hkpMalleableConstraintData(class) => class.name(),
            Classes::hkpMassChangerModifierConstraintAtom(class) => class.name(),
            Classes::hkpMassProperties(class) => class.name(),
            Classes::hkpMaterial(class) => class.name(),
            Classes::hkpMaxSizeMotion(class) => class.name(),
            Classes::hkpMeshMaterial(class) => class.name(),
            Classes::hkpMeshShape(class) => class.name(),
            Classes::hkpMeshShapeSubpart(class) => class.name(),
            Classes::hkpModifierConstraintAtom(class) => class.name(),
            Classes::hkpMoppBvTreeShape(class) => class.name(),
            Classes::hkpMoppCode(class) => class.name(),
            Classes::hkpMoppCodeCodeInfo(class) => class.name(),
            Classes::hkpMoppCodeReindexedTerminal(class) => class.name(),
            Classes::hkpMotion(class) => class.name(),
            Classes::hkpMotorAction(class) => class.name(),
            Classes::hkpMountedBallGun(class) => class.name(),
            Classes::hkpMouseSpringAction(class) => class.name(),
            Classes::hkpMovingSurfaceModifierConstraintAtom(class) => class.name(),
            Classes::hkpMultiRayShape(class) => class.name(),
            Classes::hkpMultiRayShapeRay(class) => class.name(),
            Classes::hkpMultiSphereShape(class) => class.name(),
            Classes::hkpMultithreadedVehicleManager(class) => class.name(),
            Classes::hkpNamedMeshMaterial(class) => class.name(),
            Classes::hkpNullCollisionFilter(class) => class.name(),
            Classes::hkPostFinishAttribute(class) => class.name(),
            Classes::hkpOverwritePivotConstraintAtom(class) => class.name(),
            Classes::hkpPairCollisionFilter(class) => class.name(),
            Classes::hkpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.name()
            }
            Classes::hkpParametricCurve(class) => class.name(),
            Classes::hkpPhantom(class) => class.name(),
            Classes::hkpPhantomCallbackShape(class) => class.name(),
            Classes::hkpPhysicsData(class) => class.name(),
            Classes::hkpPhysicsSystem(class) => class.name(),
            Classes::hkpPhysicsSystemWithContacts(class) => class.name(),
            Classes::hkpPlaneShape(class) => class.name(),
            Classes::hkpPointToPathConstraintData(class) => class.name(),
            Classes::hkpPointToPlaneConstraintData(class) => class.name(),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => class.name(),
            Classes::hkpPositionConstraintMotor(class) => class.name(),
            Classes::hkpPoweredChainData(class) => class.name(),
            Classes::hkpPoweredChainDataConstraintInfo(class) => class.name(),
            Classes::hkpPoweredChainMapper(class) => class.name(),
            Classes::hkpPoweredChainMapperLinkInfo(class) => class.name(),
            Classes::hkpPoweredChainMapperTarget(class) => class.name(),
            Classes::hkpPrismaticConstraintData(class) => class.name(),
            Classes::hkpPrismaticConstraintDataAtoms(class) => class.name(),
            Classes::hkpProjectileGun(class) => class.name(),
            Classes::hkpProperty(class) => class.name(),
            Classes::hkpPropertyValue(class) => class.name(),
            Classes::hkpPulleyConstraintAtom(class) => class.name(),
            Classes::hkpPulleyConstraintData(class) => class.name(),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.name(),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.name(),
            Classes::hkpRackAndPinionConstraintData(class) => class.name(),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => class.name(),
            Classes::hkpRagdollConstraintData(class) => class.name(),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.name(),
            Classes::hkpRagdollLimitsData(class) => class.name(),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.name(),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.name(),
            Classes::hkpRayCollidableFilter(class) => class.name(),
            Classes::hkpRayShapeCollectionFilter(class) => class.name(),
            Classes::hkpRejectChassisListener(class) => class.name(),
            Classes::hkpRemoveTerminalsMoppModifier(class) => class.name(),
            Classes::hkpReorientAction(class) => class.name(),
            Classes::hkpRigidBody(class) => class.name(),
            Classes::hkpRotationalConstraintData(class) => class.name(),
            Classes::hkpRotationalConstraintDataAtoms(class) => class.name(),
            Classes::hkpSampledHeightFieldShape(class) => class.name(),
            Classes::hkpSerializedAgentNnEntry(class) => class.name(),
            Classes::hkpSerializedDisplayMarker(class) => class.name(),
            Classes::hkpSerializedDisplayMarkerList(class) => class.name(),
            Classes::hkpSerializedDisplayRbTransforms(class) => class.name(),
            Classes::hkpSerializedDisplayRbTransformsDisplayTransformPair(class) => {
                class.name()
            }
            Classes::hkpSerializedSubTrack1nInfo(class) => class.name(),
            Classes::hkpSerializedTrack1nInfo(class) => class.name(),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => class.name(),
            Classes::hkpSetLocalTransformsConstraintAtom(class) => class.name(),
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => class.name(),
            Classes::hkpSetupStabilizationAtom(class) => class.name(),
            Classes::hkpShape(class) => class.name(),
            Classes::hkpShapeCollection(class) => class.name(),
            Classes::hkpShapeCollectionFilter(class) => class.name(),
            Classes::hkpShapeContainer(class) => class.name(),
            Classes::hkpShapeInfo(class) => class.name(),
            Classes::hkpShapeModifier(class) => class.name(),
            Classes::hkpShapePhantom(class) => class.name(),
            Classes::hkpSimpleContactConstraintAtom(class) => class.name(),
            Classes::hkpSimpleContactConstraintDataInfo(class) => class.name(),
            Classes::hkpSimpleMeshShape(class) => class.name(),
            Classes::hkpSimpleMeshShapeTriangle(class) => class.name(),
            Classes::hkpSimpleShapePhantom(class) => class.name(),
            Classes::hkpSimpleShapePhantomCollisionDetail(class) => class.name(),
            Classes::hkpSimulation(class) => class.name(),
            Classes::hkpSingleShapeContainer(class) => class.name(),
            Classes::hkpSoftContactModifierConstraintAtom(class) => class.name(),
            Classes::hkpSphereMotion(class) => class.name(),
            Classes::hkpSphereRepShape(class) => class.name(),
            Classes::hkpSphereShape(class) => class.name(),
            Classes::hkpSpringAction(class) => class.name(),
            Classes::hkpSpringDamperConstraintMotor(class) => class.name(),
            Classes::hkpStiffSpringChainData(class) => class.name(),
            Classes::hkpStiffSpringChainDataConstraintInfo(class) => class.name(),
            Classes::hkpStiffSpringConstraintAtom(class) => class.name(),
            Classes::hkpStiffSpringConstraintData(class) => class.name(),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => class.name(),
            Classes::hkpStorageExtendedMeshShape(class) => class.name(),
            Classes::hkpStorageExtendedMeshShapeMaterial(class) => class.name(),
            Classes::hkpStorageExtendedMeshShapeMeshSubpartStorage(class) => class.name(),
            Classes::hkpStorageExtendedMeshShapeShapeSubpartStorage(class) => {
                class.name()
            }
            Classes::hkpStorageMeshShape(class) => class.name(),
            Classes::hkpStorageMeshShapeSubpartStorage(class) => class.name(),
            Classes::hkpStorageSampledHeightFieldShape(class) => class.name(),
            Classes::hkpThinBoxMotion(class) => class.name(),
            Classes::hkpTransformShape(class) => class.name(),
            Classes::hkpTriangleShape(class) => class.name(),
            Classes::hkpTriggerVolume(class) => class.name(),
            Classes::hkpTriggerVolumeEventInfo(class) => class.name(),
            Classes::hkpTriSampledHeightFieldBvTreeShape(class) => class.name(),
            Classes::hkpTriSampledHeightFieldCollection(class) => class.name(),
            Classes::hkpTwistLimitConstraintAtom(class) => class.name(),
            Classes::hkpTypedBroadPhaseHandle(class) => class.name(),
            Classes::hkpTyremarkPoint(class) => class.name(),
            Classes::hkpTyremarksInfo(class) => class.name(),
            Classes::hkpTyremarksWheel(class) => class.name(),
            Classes::hkpUnaryAction(class) => class.name(),
            Classes::hkpVehicleAerodynamics(class) => class.name(),
            Classes::hkpVehicleBrake(class) => class.name(),
            Classes::hkpVehicleCastBatchingManager(class) => class.name(),
            Classes::hkpVehicleData(class) => class.name(),
            Classes::hkpVehicleDataWheelComponentParams(class) => class.name(),
            Classes::hkpVehicleDefaultAerodynamics(class) => class.name(),
            Classes::hkpVehicleDefaultAnalogDriverInput(class) => class.name(),
            Classes::hkpVehicleDefaultBrake(class) => class.name(),
            Classes::hkpVehicleDefaultBrakeWheelBrakingProperties(class) => class.name(),
            Classes::hkpVehicleDefaultEngine(class) => class.name(),
            Classes::hkpVehicleDefaultSteering(class) => class.name(),
            Classes::hkpVehicleDefaultSuspension(class) => class.name(),
            Classes::hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.name(),
            Classes::hkpVehicleDefaultTransmission(class) => class.name(),
            Classes::hkpVehicleDefaultVelocityDamper(class) => class.name(),
            Classes::hkpVehicleDriverInput(class) => class.name(),
            Classes::hkpVehicleDriverInputAnalogStatus(class) => class.name(),
            Classes::hkpVehicleDriverInputStatus(class) => class.name(),
            Classes::hkpVehicleEngine(class) => class.name(),
            Classes::hkpVehicleFrictionDescription(class) => class.name(),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => class.name(),
            Classes::hkpVehicleFrictionStatus(class) => class.name(),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => class.name(),
            Classes::hkpVehicleInstance(class) => class.name(),
            Classes::hkpVehicleInstanceWheelInfo(class) => class.name(),
            Classes::hkpVehicleLinearCastBatchingManager(class) => class.name(),
            Classes::hkpVehicleLinearCastWheelCollide(class) => class.name(),
            Classes::hkpVehicleLinearCastWheelCollideWheelState(class) => class.name(),
            Classes::hkpVehicleManager(class) => class.name(),
            Classes::hkpVehicleRayCastBatchingManager(class) => class.name(),
            Classes::hkpVehicleRayCastWheelCollide(class) => class.name(),
            Classes::hkpVehicleSteering(class) => class.name(),
            Classes::hkpVehicleSuspension(class) => class.name(),
            Classes::hkpVehicleSuspensionSuspensionWheelParameters(class) => class.name(),
            Classes::hkpVehicleTransmission(class) => class.name(),
            Classes::hkpVehicleVelocityDamper(class) => class.name(),
            Classes::hkpVehicleWheelCollide(class) => class.name(),
            Classes::hkpVelocityConstraintMotor(class) => class.name(),
            Classes::hkpViscousSurfaceModifierConstraintAtom(class) => class.name(),
            Classes::hkpWeldingUtility(class) => class.name(),
            Classes::hkpWheelConstraintData(class) => class.name(),
            Classes::hkpWheelConstraintDataAtoms(class) => class.name(),
            Classes::hkpWorld(class) => class.name(),
            Classes::hkpWorldCinfo(class) => class.name(),
            Classes::hkpWorldObject(class) => class.name(),
            Classes::hkQTransform(class) => class.name(),
            Classes::hkRangeInt32Attribute(class) => class.name(),
            Classes::hkRangeRealAttribute(class) => class.name(),
            Classes::hkReferencedObject(class) => class.name(),
            Classes::hkReflectedFileAttribute(class) => class.name(),
            Classes::hkResourceBase(class) => class.name(),
            Classes::hkResourceContainer(class) => class.name(),
            Classes::hkResourceHandle(class) => class.name(),
            Classes::hkRootLevelContainer(class) => class.name(),
            Classes::hkRootLevelContainerNamedVariant(class) => class.name(),
            Classes::hkSemanticsAttribute(class) => class.name(),
            Classes::hkSimpleLocalFrame(class) => class.name(),
            Classes::hkSphere(class) => class.name(),
            Classes::hkSweptTransform(class) => class.name(),
            Classes::hkTraceStreamTitle(class) => class.name(),
            Classes::hkTrackerSerializableScanSnapshot(class) => class.name(),
            Classes::hkTrackerSerializableScanSnapshotAllocation(class) => class.name(),
            Classes::hkTrackerSerializableScanSnapshotBlock(class) => class.name(),
            Classes::hkUiAttribute(class) => class.name(),
            Classes::hkVertexFormat(class) => class.name(),
            Classes::hkVertexFormatElement(class) => class.name(),
            Classes::hkWorldMemoryAvailableWatchDog(class) => class.name(),
            Classes::hkxAnimatedFloat(class) => class.name(),
            Classes::hkxAnimatedMatrix(class) => class.name(),
            Classes::hkxAnimatedQuaternion(class) => class.name(),
            Classes::hkxAnimatedVector(class) => class.name(),
            Classes::hkxAttribute(class) => class.name(),
            Classes::hkxAttributeGroup(class) => class.name(),
            Classes::hkxAttributeHolder(class) => class.name(),
            Classes::hkxCamera(class) => class.name(),
            Classes::hkxEdgeSelectionChannel(class) => class.name(),
            Classes::hkxEnum(class) => class.name(),
            Classes::hkxEnumItem(class) => class.name(),
            Classes::hkxEnvironment(class) => class.name(),
            Classes::hkxEnvironmentVariable(class) => class.name(),
            Classes::hkxIndexBuffer(class) => class.name(),
            Classes::hkxLight(class) => class.name(),
            Classes::hkxMaterial(class) => class.name(),
            Classes::hkxMaterialEffect(class) => class.name(),
            Classes::hkxMaterialProperty(class) => class.name(),
            Classes::hkxMaterialShader(class) => class.name(),
            Classes::hkxMaterialShaderSet(class) => class.name(),
            Classes::hkxMaterialTextureStage(class) => class.name(),
            Classes::hkxMesh(class) => class.name(),
            Classes::hkxMeshSection(class) => class.name(),
            Classes::hkxMeshUserChannelInfo(class) => class.name(),
            Classes::hkxNode(class) => class.name(),
            Classes::hkxNodeAnnotationData(class) => class.name(),
            Classes::hkxNodeSelectionSet(class) => class.name(),
            Classes::hkxScene(class) => class.name(),
            Classes::hkxSkinBinding(class) => class.name(),
            Classes::hkxSparselyAnimatedBool(class) => class.name(),
            Classes::hkxSparselyAnimatedEnum(class) => class.name(),
            Classes::hkxSparselyAnimatedInt(class) => class.name(),
            Classes::hkxSparselyAnimatedString(class) => class.name(),
            Classes::hkxTextureFile(class) => class.name(),
            Classes::hkxTextureInplace(class) => class.name(),
            Classes::hkxTriangleSelectionChannel(class) => class.name(),
            Classes::hkxVertexBuffer(class) => class.name(),
            Classes::hkxVertexBufferVertexData(class) => class.name(),
            Classes::hkxVertexDescription(class) => class.name(),
            Classes::hkxVertexDescriptionElementDecl(class) => class.name(),
            Classes::hkxVertexFloatDataChannel(class) => class.name(),
            Classes::hkxVertexIntDataChannel(class) => class.name(),
            Classes::hkxVertexSelectionChannel(class) => class.name(),
            Classes::hkxVertexVectorDataChannel(class) => class.name(),
        }
    }
    fn signature(&self) -> _serde::__private::Signature {
        match &self {
            Classes::SwapDummy => {
                panic!(
                    "The dummy class is used only for sorting, so being called name is not a good use of the API."
                )
            }
            Classes::BGSGamebryoSequenceGenerator(class) => class.signature(),
            Classes::BSBoneSwitchGenerator(class) => class.signature(),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.signature(),
            Classes::BSComputeAddBoneAnimModifier(class) => class.signature(),
            Classes::BSCyclicBlendTransitionGenerator(class) => class.signature(),
            Classes::BSDecomposeVectorModifier(class) => class.signature(),
            Classes::BSDirectAtModifier(class) => class.signature(),
            Classes::BSDistTriggerModifier(class) => class.signature(),
            Classes::BSEventEveryNEventsModifier(class) => class.signature(),
            Classes::BSEventOnDeactivateModifier(class) => class.signature(),
            Classes::BSEventOnFalseToTrueModifier(class) => class.signature(),
            Classes::BSGetTimeStepModifier(class) => class.signature(),
            Classes::BSInterpValueModifier(class) => class.signature(),
            Classes::BSIsActiveModifier(class) => class.signature(),
            Classes::BSIStateManagerModifier(class) => class.signature(),
            Classes::BSIStateManagerModifierBSiStateData(class) => class.signature(),
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.signature()
            }
            Classes::BSiStateTaggingGenerator(class) => class.signature(),
            Classes::BSLimbIKModifier(class) => class.signature(),
            Classes::BSLookAtModifier(class) => class.signature(),
            Classes::BSLookAtModifierBoneData(class) => class.signature(),
            Classes::BSModifyOnceModifier(class) => class.signature(),
            Classes::BSOffsetAnimationGenerator(class) => class.signature(),
            Classes::BSPassByTargetTriggerModifier(class) => class.signature(),
            Classes::BSRagdollContactListenerModifier(class) => class.signature(),
            Classes::BSSpeedSamplerModifier(class) => class.signature(),
            Classes::BSSynchronizedClipGenerator(class) => class.signature(),
            Classes::BSTimerModifier(class) => class.signature(),
            Classes::BSTweenerModifier(class) => class.signature(),
            Classes::hkAabb(class) => class.signature(),
            Classes::hkAabbHalf(class) => class.signature(),
            Classes::hkAabbUint32(class) => class.signature(),
            Classes::hkaAnimatedReferenceFrame(class) => class.signature(),
            Classes::hkaAnimation(class) => class.signature(),
            Classes::hkaAnimationBinding(class) => class.signature(),
            Classes::hkaAnimationContainer(class) => class.signature(),
            Classes::hkaAnimationPreviewColorContainer(class) => class.signature(),
            Classes::hkaAnnotationTrack(class) => class.signature(),
            Classes::hkaAnnotationTrackAnnotation(class) => class.signature(),
            Classes::hkaBone(class) => class.signature(),
            Classes::hkaBoneAttachment(class) => class.signature(),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => class.signature(),
            Classes::hkaDeltaCompressedAnimation(class) => class.signature(),
            Classes::hkaDeltaCompressedAnimationQuantizationFormat(class) => {
                class.signature()
            }
            Classes::hkaFootstepAnalysisInfo(class) => class.signature(),
            Classes::hkaFootstepAnalysisInfoContainer(class) => class.signature(),
            Classes::hkaInterleavedUncompressedAnimation(class) => class.signature(),
            Classes::hkaKeyFrameHierarchyUtility(class) => class.signature(),
            Classes::hkaKeyFrameHierarchyUtilityControlData(class) => class.signature(),
            Classes::hkAlignSceneToNodeOptions(class) => class.signature(),
            Classes::hkaMeshBinding(class) => class.signature(),
            Classes::hkaMeshBindingMapping(class) => class.signature(),
            Classes::hkaQuantizedAnimation(class) => class.signature(),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaRagdollInstance(class) => class.signature(),
            Classes::hkArrayTypeAttribute(class) => class.signature(),
            Classes::hkaSkeleton(class) => class.signature(),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.signature(),
            Classes::hkaSkeletonMapper(class) => class.signature(),
            Classes::hkaSkeletonMapperData(class) => class.signature(),
            Classes::hkaSkeletonMapperDataChainMapping(class) => class.signature(),
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => class.signature(),
            Classes::hkaSplineCompressedAnimation(class) => class.signature(),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaWaveletCompressedAnimation(class) => class.signature(),
            Classes::hkaWaveletCompressedAnimationCompressionParams(class) => {
                class.signature()
            }
            Classes::hkaWaveletCompressedAnimationQuantizationFormat(class) => {
                class.signature()
            }
            Classes::hkBaseObject(class) => class.signature(),
            Classes::hkbAttachmentModifier(class) => class.signature(),
            Classes::hkbAttachmentSetup(class) => class.signature(),
            Classes::hkbAttributeModifier(class) => class.signature(),
            Classes::hkbAttributeModifierAssignment(class) => class.signature(),
            Classes::hkbAuxiliaryNodeInfo(class) => class.signature(),
            Classes::hkbBehaviorEventsInfo(class) => class.signature(),
            Classes::hkbBehaviorGraph(class) => class.signature(),
            Classes::hkbBehaviorGraphData(class) => class.signature(),
            Classes::hkbBehaviorGraphInternalState(class) => class.signature(),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => class.signature(),
            Classes::hkbBehaviorGraphStringData(class) => class.signature(),
            Classes::hkbBehaviorInfo(class) => class.signature(),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.signature(),
            Classes::hkbBehaviorReferenceGenerator(class) => class.signature(),
            Classes::hkbBindable(class) => class.signature(),
            Classes::hkbBlendCurveUtils(class) => class.signature(),
            Classes::hkbBlenderGenerator(class) => class.signature(),
            Classes::hkbBlenderGeneratorChild(class) => class.signature(),
            Classes::hkbBlenderGeneratorChildInternalState(class) => class.signature(),
            Classes::hkbBlenderGeneratorInternalState(class) => class.signature(),
            Classes::hkbBlendingTransitionEffect(class) => class.signature(),
            Classes::hkbBlendingTransitionEffectInternalState(class) => class.signature(),
            Classes::hkbBoneIndexArray(class) => class.signature(),
            Classes::hkbBoneWeightArray(class) => class.signature(),
            Classes::hkbBoolVariableSequencedData(class) => class.signature(),
            Classes::hkbBoolVariableSequencedDataSample(class) => class.signature(),
            Classes::hkbCameraShakeEventPayload(class) => class.signature(),
            Classes::hkbCharacter(class) => class.signature(),
            Classes::hkbCharacterAddedInfo(class) => class.signature(),
            Classes::hkbCharacterControlCommand(class) => class.signature(),
            Classes::hkbCharacterControllerControlData(class) => class.signature(),
            Classes::hkbCharacterControllerModifier(class) => class.signature(),
            Classes::hkbCharacterControllerModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbCharacterData(class) => class.signature(),
            Classes::hkbCharacterDataCharacterControllerInfo(class) => class.signature(),
            Classes::hkbCharacterInfo(class) => class.signature(),
            Classes::hkbCharacterSetup(class) => class.signature(),
            Classes::hkbCharacterSkinInfo(class) => class.signature(),
            Classes::hkbCharacterSteppedInfo(class) => class.signature(),
            Classes::hkbCharacterStringData(class) => class.signature(),
            Classes::hkbClientCharacterState(class) => class.signature(),
            Classes::hkbClipGenerator(class) => class.signature(),
            Classes::hkbClipGeneratorEcho(class) => class.signature(),
            Classes::hkbClipGeneratorInternalState(class) => class.signature(),
            Classes::hkbClipTrigger(class) => class.signature(),
            Classes::hkbClipTriggerArray(class) => class.signature(),
            Classes::hkbCombineTransformsModifier(class) => class.signature(),
            Classes::hkbCombineTransformsModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbCompiledExpressionSet(class) => class.signature(),
            Classes::hkbCompiledExpressionSetToken(class) => class.signature(),
            Classes::hkbComputeDirectionModifier(class) => class.signature(),
            Classes::hkbComputeDirectionModifierInternalState(class) => class.signature(),
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => class.signature(),
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbComputeRotationToTargetModifier(class) => class.signature(),
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbCondition(class) => class.signature(),
            Classes::hkbContext(class) => class.signature(),
            Classes::hkbDampingModifier(class) => class.signature(),
            Classes::hkbDampingModifierInternalState(class) => class.signature(),
            Classes::hkbDefaultMessageLog(class) => class.signature(),
            Classes::hkbDelayedModifier(class) => class.signature(),
            Classes::hkbDelayedModifierInternalState(class) => class.signature(),
            Classes::hkbDetectCloseToGroundModifier(class) => class.signature(),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbEvaluateExpressionModifier(class) => class.signature(),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.signature()
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbEvaluateHandleModifier(class) => class.signature(),
            Classes::hkbEvent(class) => class.signature(),
            Classes::hkbEventBase(class) => class.signature(),
            Classes::hkbEventDrivenModifier(class) => class.signature(),
            Classes::hkbEventDrivenModifierInternalState(class) => class.signature(),
            Classes::hkbEventInfo(class) => class.signature(),
            Classes::hkbEventPayload(class) => class.signature(),
            Classes::hkbEventPayloadList(class) => class.signature(),
            Classes::hkbEventProperty(class) => class.signature(),
            Classes::hkbEventRaisedInfo(class) => class.signature(),
            Classes::hkbEventRangeData(class) => class.signature(),
            Classes::hkbEventRangeDataArray(class) => class.signature(),
            Classes::hkbEventSequencedData(class) => class.signature(),
            Classes::hkbEventSequencedDataSequencedEvent(class) => class.signature(),
            Classes::hkbEventsFromRangeModifier(class) => class.signature(),
            Classes::hkbEventsFromRangeModifierInternalState(class) => class.signature(),
            Classes::hkbExpressionCondition(class) => class.signature(),
            Classes::hkbExpressionData(class) => class.signature(),
            Classes::hkbExpressionDataArray(class) => class.signature(),
            Classes::hkbExtractRagdollPoseModifier(class) => class.signature(),
            Classes::hkbFootIkControlData(class) => class.signature(),
            Classes::hkbFootIkControlsModifier(class) => class.signature(),
            Classes::hkbFootIkControlsModifierLeg(class) => class.signature(),
            Classes::hkbFootIkDriverInfo(class) => class.signature(),
            Classes::hkbFootIkDriverInfoLeg(class) => class.signature(),
            Classes::hkbFootIkGains(class) => class.signature(),
            Classes::hkbFootIkModifier(class) => class.signature(),
            Classes::hkbFootIkModifierInternalLegData(class) => class.signature(),
            Classes::hkbFootIkModifierLeg(class) => class.signature(),
            Classes::hkbGenerator(class) => class.signature(),
            Classes::hkbGeneratorOutputListener(class) => class.signature(),
            Classes::hkbGeneratorSyncInfo(class) => class.signature(),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.signature(),
            Classes::hkbGeneratorTransitionEffect(class) => class.signature(),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => {
                class.signature()
            }
            Classes::hkbGetHandleOnBoneModifier(class) => class.signature(),
            Classes::hkbGetUpModifier(class) => class.signature(),
            Classes::hkbGetUpModifierInternalState(class) => class.signature(),
            Classes::hkbGetWorldFromModelModifier(class) => class.signature(),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => {
                class.signature()
            }
            Classes::hkbHandIkControlData(class) => class.signature(),
            Classes::hkbHandIkControlsModifier(class) => class.signature(),
            Classes::hkbHandIkControlsModifierHand(class) => class.signature(),
            Classes::hkbHandIkDriverInfo(class) => class.signature(),
            Classes::hkbHandIkDriverInfoHand(class) => class.signature(),
            Classes::hkbHandIkModifier(class) => class.signature(),
            Classes::hkbHandIkModifierHand(class) => class.signature(),
            Classes::hkbHandle(class) => class.signature(),
            Classes::hkbIntEventPayload(class) => class.signature(),
            Classes::hkbIntVariableSequencedData(class) => class.signature(),
            Classes::hkbIntVariableSequencedDataSample(class) => class.signature(),
            Classes::hkBitField(class) => class.signature(),
            Classes::hkbKeyframeBonesModifier(class) => class.signature(),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => class.signature(),
            Classes::hkbLinkedSymbolInfo(class) => class.signature(),
            Classes::hkbLookAtModifier(class) => class.signature(),
            Classes::hkbLookAtModifierInternalState(class) => class.signature(),
            Classes::hkbManualSelectorGenerator(class) => class.signature(),
            Classes::hkbManualSelectorGeneratorInternalState(class) => class.signature(),
            Classes::hkbMessageLog(class) => class.signature(),
            Classes::hkbMirroredSkeletonInfo(class) => class.signature(),
            Classes::hkbMirrorModifier(class) => class.signature(),
            Classes::hkbModifier(class) => class.signature(),
            Classes::hkbModifierGenerator(class) => class.signature(),
            Classes::hkbModifierList(class) => class.signature(),
            Classes::hkbModifierWrapper(class) => class.signature(),
            Classes::hkbMoveCharacterModifier(class) => class.signature(),
            Classes::hkbMoveCharacterModifierInternalState(class) => class.signature(),
            Classes::hkbNamedEventPayload(class) => class.signature(),
            Classes::hkbNamedIntEventPayload(class) => class.signature(),
            Classes::hkbNamedRealEventPayload(class) => class.signature(),
            Classes::hkbNamedStringEventPayload(class) => class.signature(),
            Classes::hkbNode(class) => class.signature(),
            Classes::hkbNodeInternalStateInfo(class) => class.signature(),
            Classes::hkbParticleSystemEventPayload(class) => class.signature(),
            Classes::hkbPoseMatchingGenerator(class) => class.signature(),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => class.signature(),
            Classes::hkbPoweredRagdollControlData(class) => class.signature(),
            Classes::hkbPoweredRagdollControlsModifier(class) => class.signature(),
            Classes::hkbProjectData(class) => class.signature(),
            Classes::hkbProjectStringData(class) => class.signature(),
            Classes::hkbProxyModifier(class) => class.signature(),
            Classes::hkbProxyModifierProxyInfo(class) => class.signature(),
            Classes::hkbRaiseEventCommand(class) => class.signature(),
            Classes::hkbRealEventPayload(class) => class.signature(),
            Classes::hkbRealVariableSequencedData(class) => class.signature(),
            Classes::hkbRealVariableSequencedDataSample(class) => class.signature(),
            Classes::hkbReferencePoseGenerator(class) => class.signature(),
            Classes::hkbRegisteredGenerator(class) => class.signature(),
            Classes::hkbRigidBodyRagdollControlData(class) => class.signature(),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => class.signature(),
            Classes::hkbRoleAttribute(class) => class.signature(),
            Classes::hkbRotateCharacterModifier(class) => class.signature(),
            Classes::hkbRotateCharacterModifierInternalState(class) => class.signature(),
            Classes::hkbSenseHandleModifier(class) => class.signature(),
            Classes::hkbSenseHandleModifierRange(class) => class.signature(),
            Classes::hkbSequence(class) => class.signature(),
            Classes::hkbSequencedData(class) => class.signature(),
            Classes::hkbSequenceInternalState(class) => class.signature(),
            Classes::hkbSequenceStringData(class) => class.signature(),
            Classes::hkbSetBehaviorCommand(class) => class.signature(),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => class.signature(),
            Classes::hkbSetNodePropertyCommand(class) => class.signature(),
            Classes::hkbSetWordVariableCommand(class) => class.signature(),
            Classes::hkbSetWorldFromModelModifier(class) => class.signature(),
            Classes::hkbSimulationControlCommand(class) => class.signature(),
            Classes::hkbSimulationStateInfo(class) => class.signature(),
            Classes::hkbStateChooser(class) => class.signature(),
            Classes::hkbStateListener(class) => class.signature(),
            Classes::hkbStateMachine(class) => class.signature(),
            Classes::hkbStateMachineActiveTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineDelayedTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineEventPropertyArray(class) => class.signature(),
            Classes::hkbStateMachineInternalState(class) => class.signature(),
            Classes::hkbStateMachineNestedStateMachineData(class) => class.signature(),
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineStateInfo(class) => class.signature(),
            Classes::hkbStateMachineTimeInterval(class) => class.signature(),
            Classes::hkbStateMachineTransitionInfo(class) => class.signature(),
            Classes::hkbStateMachineTransitionInfoArray(class) => class.signature(),
            Classes::hkbStateMachineTransitionInfoReference(class) => class.signature(),
            Classes::hkbStringCondition(class) => class.signature(),
            Classes::hkbStringEventPayload(class) => class.signature(),
            Classes::hkbTestStateChooser(class) => class.signature(),
            Classes::hkbTimerModifier(class) => class.signature(),
            Classes::hkbTimerModifierInternalState(class) => class.signature(),
            Classes::hkbTransformVectorModifier(class) => class.signature(),
            Classes::hkbTransformVectorModifierInternalState(class) => class.signature(),
            Classes::hkbTransitionEffect(class) => class.signature(),
            Classes::hkbTwistModifier(class) => class.signature(),
            Classes::hkbVariableBindingSet(class) => class.signature(),
            Classes::hkbVariableBindingSetBinding(class) => class.signature(),
            Classes::hkbVariableInfo(class) => class.signature(),
            Classes::hkbVariableValue(class) => class.signature(),
            Classes::hkbVariableValueSet(class) => class.signature(),
            Classes::hkbWorldEnums(class) => class.signature(),
            Classes::hkbWorldFromModelModeData(class) => class.signature(),
            Classes::hkClass(class) => class.signature(),
            Classes::hkClassEnum(class) => class.signature(),
            Classes::hkClassEnumItem(class) => class.signature(),
            Classes::hkClassMember(class) => class.signature(),
            Classes::hkColor(class) => class.signature(),
            Classes::hkContactPoint(class) => class.signature(),
            Classes::hkContactPointMaterial(class) => class.signature(),
            Classes::hkCustomAttributes(class) => class.signature(),
            Classes::hkCustomAttributesAttribute(class) => class.signature(),
            Classes::hkDataObjectTypeAttribute(class) => class.signature(),
            Classes::hkDescriptionAttribute(class) => class.signature(),
            Classes::hkDocumentationAttribute(class) => class.signature(),
            Classes::hkGeometry(class) => class.signature(),
            Classes::hkGeometryTriangle(class) => class.signature(),
            Classes::hkGizmoAttribute(class) => class.signature(),
            Classes::hkHalf8(class) => class.signature(),
            Classes::hkIndexedTransformSet(class) => class.signature(),
            Classes::hkLinkAttribute(class) => class.signature(),
            Classes::hkLocalFrame(class) => class.signature(),
            Classes::hkLocalFrameGroup(class) => class.signature(),
            Classes::hkMemoryMeshBody(class) => class.signature(),
            Classes::hkMemoryMeshMaterial(class) => class.signature(),
            Classes::hkMemoryMeshShape(class) => class.signature(),
            Classes::hkMemoryMeshTexture(class) => class.signature(),
            Classes::hkMemoryMeshVertexBuffer(class) => class.signature(),
            Classes::hkMemoryResourceContainer(class) => class.signature(),
            Classes::hkMemoryResourceHandle(class) => class.signature(),
            Classes::hkMemoryResourceHandleExternalLink(class) => class.signature(),
            Classes::hkMemoryTrackerAttribute(class) => class.signature(),
            Classes::hkMeshBody(class) => class.signature(),
            Classes::hkMeshBoneIndexMapping(class) => class.signature(),
            Classes::hkMeshMaterial(class) => class.signature(),
            Classes::hkMeshSection(class) => class.signature(),
            Classes::hkMeshSectionCinfo(class) => class.signature(),
            Classes::hkMeshShape(class) => class.signature(),
            Classes::hkMeshTexture(class) => class.signature(),
            Classes::hkMeshVertexBuffer(class) => class.signature(),
            Classes::hkModelerNodeTypeAttribute(class) => class.signature(),
            Classes::hkMonitorStreamColorTable(class) => class.signature(),
            Classes::hkMonitorStreamColorTableColorPair(class) => class.signature(),
            Classes::hkMonitorStreamFrameInfo(class) => class.signature(),
            Classes::hkMonitorStreamStringMap(class) => class.signature(),
            Classes::hkMonitorStreamStringMapStringMap(class) => class.signature(),
            Classes::hkMoppBvTreeShapeBase(class) => class.signature(),
            Classes::hkMotionState(class) => class.signature(),
            Classes::hkMultipleVertexBuffer(class) => class.signature(),
            Classes::hkMultipleVertexBufferElementInfo(class) => class.signature(),
            Classes::hkMultipleVertexBufferLockedElement(class) => class.signature(),
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => class.signature(),
            Classes::hkMultiThreadCheck(class) => class.signature(),
            Classes::hkp2dAngConstraintAtom(class) => class.signature(),
            Classes::hkpAabbPhantom(class) => class.signature(),
            Classes::hkPackedVector3(class) => class.signature(),
            Classes::hkPackfileHeader(class) => class.signature(),
            Classes::hkPackfileSectionHeader(class) => class.signature(),
            Classes::hkpAction(class) => class.signature(),
            Classes::hkpAgent1nSector(class) => class.signature(),
            Classes::hkpAngConstraintAtom(class) => class.signature(),
            Classes::hkpAngFrictionConstraintAtom(class) => class.signature(),
            Classes::hkpAngLimitConstraintAtom(class) => class.signature(),
            Classes::hkpAngMotorConstraintAtom(class) => class.signature(),
            Classes::hkpAngularDashpotAction(class) => class.signature(),
            Classes::hkpArrayAction(class) => class.signature(),
            Classes::hkpBallAndSocketConstraintData(class) => class.signature(),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => class.signature(),
            Classes::hkpBallGun(class) => class.signature(),
            Classes::hkpBallSocketChainData(class) => class.signature(),
            Classes::hkpBallSocketChainDataConstraintInfo(class) => class.signature(),
            Classes::hkpBallSocketConstraintAtom(class) => class.signature(),
            Classes::hkpBinaryAction(class) => class.signature(),
            Classes::hkpBoxMotion(class) => class.signature(),
            Classes::hkpBoxShape(class) => class.signature(),
            Classes::hkpBreakableBody(class) => class.signature(),
            Classes::hkpBreakableConstraintData(class) => class.signature(),
            Classes::hkpBridgeAtoms(class) => class.signature(),
            Classes::hkpBridgeConstraintAtom(class) => class.signature(),
            Classes::hkpBroadPhaseHandle(class) => class.signature(),
            Classes::hkpBvShape(class) => class.signature(),
            Classes::hkpBvTreeShape(class) => class.signature(),
            Classes::hkpCachingShapePhantom(class) => class.signature(),
            Classes::hkpCallbackConstraintMotor(class) => class.signature(),
            Classes::hkpCapsuleShape(class) => class.signature(),
            Classes::hkpCdBody(class) => class.signature(),
            Classes::hkpCenterOfMassChangerModifierConstraintAtom(class) => {
                class.signature()
            }
            Classes::hkpCharacterControllerCinfo(class) => class.signature(),
            Classes::hkpCharacterMotion(class) => class.signature(),
            Classes::hkpCharacterProxyCinfo(class) => class.signature(),
            Classes::hkpCharacterRigidBodyCinfo(class) => class.signature(),
            Classes::hkpCogWheelConstraintAtom(class) => class.signature(),
            Classes::hkpCogWheelConstraintData(class) => class.signature(),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.signature(),
            Classes::hkpCollidable(class) => class.signature(),
            Classes::hkpCollidableBoundingVolumeData(class) => class.signature(),
            Classes::hkpCollidableCollidableFilter(class) => class.signature(),
            Classes::hkpCollisionFilter(class) => class.signature(),
            Classes::hkpCollisionFilterList(class) => class.signature(),
            Classes::hkpCompressedMeshShape(class) => class.signature(),
            Classes::hkpCompressedMeshShapeBigTriangle(class) => class.signature(),
            Classes::hkpCompressedMeshShapeChunk(class) => class.signature(),
            Classes::hkpCompressedMeshShapeConvexPiece(class) => class.signature(),
            Classes::hkpCompressedSampledHeightFieldShape(class) => class.signature(),
            Classes::hkpConeLimitConstraintAtom(class) => class.signature(),
            Classes::hkpConstrainedSystemFilter(class) => class.signature(),
            Classes::hkpConstraintAtom(class) => class.signature(),
            Classes::hkpConstraintChainData(class) => class.signature(),
            Classes::hkpConstraintChainInstance(class) => class.signature(),
            Classes::hkpConstraintChainInstanceAction(class) => class.signature(),
            Classes::hkpConstraintCollisionFilter(class) => class.signature(),
            Classes::hkpConstraintData(class) => class.signature(),
            Classes::hkpConstraintInstance(class) => class.signature(),
            Classes::hkpConstraintInstanceSmallArraySerializeOverrideType(class) => {
                class.signature()
            }
            Classes::hkpConstraintMotor(class) => class.signature(),
            Classes::hkpConvexListFilter(class) => class.signature(),
            Classes::hkpConvexListShape(class) => class.signature(),
            Classes::hkpConvexPieceMeshShape(class) => class.signature(),
            Classes::hkpConvexPieceStreamData(class) => class.signature(),
            Classes::hkpConvexShape(class) => class.signature(),
            Classes::hkpConvexTransformShape(class) => class.signature(),
            Classes::hkpConvexTransformShapeBase(class) => class.signature(),
            Classes::hkpConvexTranslateShape(class) => class.signature(),
            Classes::hkpConvexVerticesConnectivity(class) => class.signature(),
            Classes::hkpConvexVerticesShape(class) => class.signature(),
            Classes::hkpConvexVerticesShapeFourVectors(class) => class.signature(),
            Classes::hkpCylinderShape(class) => class.signature(),
            Classes::hkpDashpotAction(class) => class.signature(),
            Classes::hkpDefaultConvexListFilter(class) => class.signature(),
            Classes::hkpDefaultWorldMemoryWatchDog(class) => class.signature(),
            Classes::hkpDisableEntityCollisionFilter(class) => class.signature(),
            Classes::hkpDisplayBindingData(class) => class.signature(),
            Classes::hkpDisplayBindingDataPhysicsSystem(class) => class.signature(),
            Classes::hkpDisplayBindingDataRigidBody(class) => class.signature(),
            Classes::hkpEntity(class) => class.signature(),
            Classes::hkpEntityExtendedListeners(class) => class.signature(),
            Classes::hkpEntitySmallArraySerializeOverrideType(class) => class.signature(),
            Classes::hkpEntitySpuCollisionCallback(class) => class.signature(),
            Classes::hkpExtendedMeshShape(class) => class.signature(),
            Classes::hkpExtendedMeshShapeShapesSubpart(class) => class.signature(),
            Classes::hkpExtendedMeshShapeSubpart(class) => class.signature(),
            Classes::hkpExtendedMeshShapeTrianglesSubpart(class) => class.signature(),
            Classes::hkpFastMeshShape(class) => class.signature(),
            Classes::hkpFirstPersonGun(class) => class.signature(),
            Classes::hkpFixedRigidMotion(class) => class.signature(),
            Classes::hkpGenericConstraintData(class) => class.signature(),
            Classes::hkpGenericConstraintDataScheme(class) => class.signature(),
            Classes::hkpGenericConstraintDataSchemeConstraintInfo(class) => {
                class.signature()
            }
            Classes::hkpGravityGun(class) => class.signature(),
            Classes::hkpGroupCollisionFilter(class) => class.signature(),
            Classes::hkpGroupFilter(class) => class.signature(),
            Classes::hkpHeightFieldShape(class) => class.signature(),
            Classes::hkpHingeConstraintData(class) => class.signature(),
            Classes::hkpHingeConstraintDataAtoms(class) => class.signature(),
            Classes::hkpHingeLimitsData(class) => class.signature(),
            Classes::hkpHingeLimitsDataAtoms(class) => class.signature(),
            Classes::hkpIgnoreModifierConstraintAtom(class) => class.signature(),
            Classes::hkpKeyframedRigidMotion(class) => class.signature(),
            Classes::hkpLimitedForceConstraintMotor(class) => class.signature(),
            Classes::hkpLimitedHingeConstraintData(class) => class.signature(),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => class.signature(),
            Classes::hkpLinConstraintAtom(class) => class.signature(),
            Classes::hkpLinearParametricCurve(class) => class.signature(),
            Classes::hkpLinFrictionConstraintAtom(class) => class.signature(),
            Classes::hkpLinkedCollidable(class) => class.signature(),
            Classes::hkpLinLimitConstraintAtom(class) => class.signature(),
            Classes::hkpLinMotorConstraintAtom(class) => class.signature(),
            Classes::hkpLinSoftConstraintAtom(class) => class.signature(),
            Classes::hkpListShape(class) => class.signature(),
            Classes::hkpListShapeChildInfo(class) => class.signature(),
            Classes::hkpMalleableConstraintData(class) => class.signature(),
            Classes::hkpMassChangerModifierConstraintAtom(class) => class.signature(),
            Classes::hkpMassProperties(class) => class.signature(),
            Classes::hkpMaterial(class) => class.signature(),
            Classes::hkpMaxSizeMotion(class) => class.signature(),
            Classes::hkpMeshMaterial(class) => class.signature(),
            Classes::hkpMeshShape(class) => class.signature(),
            Classes::hkpMeshShapeSubpart(class) => class.signature(),
            Classes::hkpModifierConstraintAtom(class) => class.signature(),
            Classes::hkpMoppBvTreeShape(class) => class.signature(),
            Classes::hkpMoppCode(class) => class.signature(),
            Classes::hkpMoppCodeCodeInfo(class) => class.signature(),
            Classes::hkpMoppCodeReindexedTerminal(class) => class.signature(),
            Classes::hkpMotion(class) => class.signature(),
            Classes::hkpMotorAction(class) => class.signature(),
            Classes::hkpMountedBallGun(class) => class.signature(),
            Classes::hkpMouseSpringAction(class) => class.signature(),
            Classes::hkpMovingSurfaceModifierConstraintAtom(class) => class.signature(),
            Classes::hkpMultiRayShape(class) => class.signature(),
            Classes::hkpMultiRayShapeRay(class) => class.signature(),
            Classes::hkpMultiSphereShape(class) => class.signature(),
            Classes::hkpMultithreadedVehicleManager(class) => class.signature(),
            Classes::hkpNamedMeshMaterial(class) => class.signature(),
            Classes::hkpNullCollisionFilter(class) => class.signature(),
            Classes::hkPostFinishAttribute(class) => class.signature(),
            Classes::hkpOverwritePivotConstraintAtom(class) => class.signature(),
            Classes::hkpPairCollisionFilter(class) => class.signature(),
            Classes::hkpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.signature()
            }
            Classes::hkpParametricCurve(class) => class.signature(),
            Classes::hkpPhantom(class) => class.signature(),
            Classes::hkpPhantomCallbackShape(class) => class.signature(),
            Classes::hkpPhysicsData(class) => class.signature(),
            Classes::hkpPhysicsSystem(class) => class.signature(),
            Classes::hkpPhysicsSystemWithContacts(class) => class.signature(),
            Classes::hkpPlaneShape(class) => class.signature(),
            Classes::hkpPointToPathConstraintData(class) => class.signature(),
            Classes::hkpPointToPlaneConstraintData(class) => class.signature(),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => class.signature(),
            Classes::hkpPositionConstraintMotor(class) => class.signature(),
            Classes::hkpPoweredChainData(class) => class.signature(),
            Classes::hkpPoweredChainDataConstraintInfo(class) => class.signature(),
            Classes::hkpPoweredChainMapper(class) => class.signature(),
            Classes::hkpPoweredChainMapperLinkInfo(class) => class.signature(),
            Classes::hkpPoweredChainMapperTarget(class) => class.signature(),
            Classes::hkpPrismaticConstraintData(class) => class.signature(),
            Classes::hkpPrismaticConstraintDataAtoms(class) => class.signature(),
            Classes::hkpProjectileGun(class) => class.signature(),
            Classes::hkpProperty(class) => class.signature(),
            Classes::hkpPropertyValue(class) => class.signature(),
            Classes::hkpPulleyConstraintAtom(class) => class.signature(),
            Classes::hkpPulleyConstraintData(class) => class.signature(),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.signature(),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.signature(),
            Classes::hkpRackAndPinionConstraintData(class) => class.signature(),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => class.signature(),
            Classes::hkpRagdollConstraintData(class) => class.signature(),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.signature(),
            Classes::hkpRagdollLimitsData(class) => class.signature(),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.signature(),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.signature(),
            Classes::hkpRayCollidableFilter(class) => class.signature(),
            Classes::hkpRayShapeCollectionFilter(class) => class.signature(),
            Classes::hkpRejectChassisListener(class) => class.signature(),
            Classes::hkpRemoveTerminalsMoppModifier(class) => class.signature(),
            Classes::hkpReorientAction(class) => class.signature(),
            Classes::hkpRigidBody(class) => class.signature(),
            Classes::hkpRotationalConstraintData(class) => class.signature(),
            Classes::hkpRotationalConstraintDataAtoms(class) => class.signature(),
            Classes::hkpSampledHeightFieldShape(class) => class.signature(),
            Classes::hkpSerializedAgentNnEntry(class) => class.signature(),
            Classes::hkpSerializedDisplayMarker(class) => class.signature(),
            Classes::hkpSerializedDisplayMarkerList(class) => class.signature(),
            Classes::hkpSerializedDisplayRbTransforms(class) => class.signature(),
            Classes::hkpSerializedDisplayRbTransformsDisplayTransformPair(class) => {
                class.signature()
            }
            Classes::hkpSerializedSubTrack1nInfo(class) => class.signature(),
            Classes::hkpSerializedTrack1nInfo(class) => class.signature(),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => class.signature(),
            Classes::hkpSetLocalTransformsConstraintAtom(class) => class.signature(),
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => class.signature(),
            Classes::hkpSetupStabilizationAtom(class) => class.signature(),
            Classes::hkpShape(class) => class.signature(),
            Classes::hkpShapeCollection(class) => class.signature(),
            Classes::hkpShapeCollectionFilter(class) => class.signature(),
            Classes::hkpShapeContainer(class) => class.signature(),
            Classes::hkpShapeInfo(class) => class.signature(),
            Classes::hkpShapeModifier(class) => class.signature(),
            Classes::hkpShapePhantom(class) => class.signature(),
            Classes::hkpSimpleContactConstraintAtom(class) => class.signature(),
            Classes::hkpSimpleContactConstraintDataInfo(class) => class.signature(),
            Classes::hkpSimpleMeshShape(class) => class.signature(),
            Classes::hkpSimpleMeshShapeTriangle(class) => class.signature(),
            Classes::hkpSimpleShapePhantom(class) => class.signature(),
            Classes::hkpSimpleShapePhantomCollisionDetail(class) => class.signature(),
            Classes::hkpSimulation(class) => class.signature(),
            Classes::hkpSingleShapeContainer(class) => class.signature(),
            Classes::hkpSoftContactModifierConstraintAtom(class) => class.signature(),
            Classes::hkpSphereMotion(class) => class.signature(),
            Classes::hkpSphereRepShape(class) => class.signature(),
            Classes::hkpSphereShape(class) => class.signature(),
            Classes::hkpSpringAction(class) => class.signature(),
            Classes::hkpSpringDamperConstraintMotor(class) => class.signature(),
            Classes::hkpStiffSpringChainData(class) => class.signature(),
            Classes::hkpStiffSpringChainDataConstraintInfo(class) => class.signature(),
            Classes::hkpStiffSpringConstraintAtom(class) => class.signature(),
            Classes::hkpStiffSpringConstraintData(class) => class.signature(),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => class.signature(),
            Classes::hkpStorageExtendedMeshShape(class) => class.signature(),
            Classes::hkpStorageExtendedMeshShapeMaterial(class) => class.signature(),
            Classes::hkpStorageExtendedMeshShapeMeshSubpartStorage(class) => {
                class.signature()
            }
            Classes::hkpStorageExtendedMeshShapeShapeSubpartStorage(class) => {
                class.signature()
            }
            Classes::hkpStorageMeshShape(class) => class.signature(),
            Classes::hkpStorageMeshShapeSubpartStorage(class) => class.signature(),
            Classes::hkpStorageSampledHeightFieldShape(class) => class.signature(),
            Classes::hkpThinBoxMotion(class) => class.signature(),
            Classes::hkpTransformShape(class) => class.signature(),
            Classes::hkpTriangleShape(class) => class.signature(),
            Classes::hkpTriggerVolume(class) => class.signature(),
            Classes::hkpTriggerVolumeEventInfo(class) => class.signature(),
            Classes::hkpTriSampledHeightFieldBvTreeShape(class) => class.signature(),
            Classes::hkpTriSampledHeightFieldCollection(class) => class.signature(),
            Classes::hkpTwistLimitConstraintAtom(class) => class.signature(),
            Classes::hkpTypedBroadPhaseHandle(class) => class.signature(),
            Classes::hkpTyremarkPoint(class) => class.signature(),
            Classes::hkpTyremarksInfo(class) => class.signature(),
            Classes::hkpTyremarksWheel(class) => class.signature(),
            Classes::hkpUnaryAction(class) => class.signature(),
            Classes::hkpVehicleAerodynamics(class) => class.signature(),
            Classes::hkpVehicleBrake(class) => class.signature(),
            Classes::hkpVehicleCastBatchingManager(class) => class.signature(),
            Classes::hkpVehicleData(class) => class.signature(),
            Classes::hkpVehicleDataWheelComponentParams(class) => class.signature(),
            Classes::hkpVehicleDefaultAerodynamics(class) => class.signature(),
            Classes::hkpVehicleDefaultAnalogDriverInput(class) => class.signature(),
            Classes::hkpVehicleDefaultBrake(class) => class.signature(),
            Classes::hkpVehicleDefaultBrakeWheelBrakingProperties(class) => {
                class.signature()
            }
            Classes::hkpVehicleDefaultEngine(class) => class.signature(),
            Classes::hkpVehicleDefaultSteering(class) => class.signature(),
            Classes::hkpVehicleDefaultSuspension(class) => class.signature(),
            Classes::hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.signature(),
            Classes::hkpVehicleDefaultTransmission(class) => class.signature(),
            Classes::hkpVehicleDefaultVelocityDamper(class) => class.signature(),
            Classes::hkpVehicleDriverInput(class) => class.signature(),
            Classes::hkpVehicleDriverInputAnalogStatus(class) => class.signature(),
            Classes::hkpVehicleDriverInputStatus(class) => class.signature(),
            Classes::hkpVehicleEngine(class) => class.signature(),
            Classes::hkpVehicleFrictionDescription(class) => class.signature(),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => {
                class.signature()
            }
            Classes::hkpVehicleFrictionStatus(class) => class.signature(),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => class.signature(),
            Classes::hkpVehicleInstance(class) => class.signature(),
            Classes::hkpVehicleInstanceWheelInfo(class) => class.signature(),
            Classes::hkpVehicleLinearCastBatchingManager(class) => class.signature(),
            Classes::hkpVehicleLinearCastWheelCollide(class) => class.signature(),
            Classes::hkpVehicleLinearCastWheelCollideWheelState(class) => {
                class.signature()
            }
            Classes::hkpVehicleManager(class) => class.signature(),
            Classes::hkpVehicleRayCastBatchingManager(class) => class.signature(),
            Classes::hkpVehicleRayCastWheelCollide(class) => class.signature(),
            Classes::hkpVehicleSteering(class) => class.signature(),
            Classes::hkpVehicleSuspension(class) => class.signature(),
            Classes::hkpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.signature()
            }
            Classes::hkpVehicleTransmission(class) => class.signature(),
            Classes::hkpVehicleVelocityDamper(class) => class.signature(),
            Classes::hkpVehicleWheelCollide(class) => class.signature(),
            Classes::hkpVelocityConstraintMotor(class) => class.signature(),
            Classes::hkpViscousSurfaceModifierConstraintAtom(class) => class.signature(),
            Classes::hkpWeldingUtility(class) => class.signature(),
            Classes::hkpWheelConstraintData(class) => class.signature(),
            Classes::hkpWheelConstraintDataAtoms(class) => class.signature(),
            Classes::hkpWorld(class) => class.signature(),
            Classes::hkpWorldCinfo(class) => class.signature(),
            Classes::hkpWorldObject(class) => class.signature(),
            Classes::hkQTransform(class) => class.signature(),
            Classes::hkRangeInt32Attribute(class) => class.signature(),
            Classes::hkRangeRealAttribute(class) => class.signature(),
            Classes::hkReferencedObject(class) => class.signature(),
            Classes::hkReflectedFileAttribute(class) => class.signature(),
            Classes::hkResourceBase(class) => class.signature(),
            Classes::hkResourceContainer(class) => class.signature(),
            Classes::hkResourceHandle(class) => class.signature(),
            Classes::hkRootLevelContainer(class) => class.signature(),
            Classes::hkRootLevelContainerNamedVariant(class) => class.signature(),
            Classes::hkSemanticsAttribute(class) => class.signature(),
            Classes::hkSimpleLocalFrame(class) => class.signature(),
            Classes::hkSphere(class) => class.signature(),
            Classes::hkSweptTransform(class) => class.signature(),
            Classes::hkTraceStreamTitle(class) => class.signature(),
            Classes::hkTrackerSerializableScanSnapshot(class) => class.signature(),
            Classes::hkTrackerSerializableScanSnapshotAllocation(class) => {
                class.signature()
            }
            Classes::hkTrackerSerializableScanSnapshotBlock(class) => class.signature(),
            Classes::hkUiAttribute(class) => class.signature(),
            Classes::hkVertexFormat(class) => class.signature(),
            Classes::hkVertexFormatElement(class) => class.signature(),
            Classes::hkWorldMemoryAvailableWatchDog(class) => class.signature(),
            Classes::hkxAnimatedFloat(class) => class.signature(),
            Classes::hkxAnimatedMatrix(class) => class.signature(),
            Classes::hkxAnimatedQuaternion(class) => class.signature(),
            Classes::hkxAnimatedVector(class) => class.signature(),
            Classes::hkxAttribute(class) => class.signature(),
            Classes::hkxAttributeGroup(class) => class.signature(),
            Classes::hkxAttributeHolder(class) => class.signature(),
            Classes::hkxCamera(class) => class.signature(),
            Classes::hkxEdgeSelectionChannel(class) => class.signature(),
            Classes::hkxEnum(class) => class.signature(),
            Classes::hkxEnumItem(class) => class.signature(),
            Classes::hkxEnvironment(class) => class.signature(),
            Classes::hkxEnvironmentVariable(class) => class.signature(),
            Classes::hkxIndexBuffer(class) => class.signature(),
            Classes::hkxLight(class) => class.signature(),
            Classes::hkxMaterial(class) => class.signature(),
            Classes::hkxMaterialEffect(class) => class.signature(),
            Classes::hkxMaterialProperty(class) => class.signature(),
            Classes::hkxMaterialShader(class) => class.signature(),
            Classes::hkxMaterialShaderSet(class) => class.signature(),
            Classes::hkxMaterialTextureStage(class) => class.signature(),
            Classes::hkxMesh(class) => class.signature(),
            Classes::hkxMeshSection(class) => class.signature(),
            Classes::hkxMeshUserChannelInfo(class) => class.signature(),
            Classes::hkxNode(class) => class.signature(),
            Classes::hkxNodeAnnotationData(class) => class.signature(),
            Classes::hkxNodeSelectionSet(class) => class.signature(),
            Classes::hkxScene(class) => class.signature(),
            Classes::hkxSkinBinding(class) => class.signature(),
            Classes::hkxSparselyAnimatedBool(class) => class.signature(),
            Classes::hkxSparselyAnimatedEnum(class) => class.signature(),
            Classes::hkxSparselyAnimatedInt(class) => class.signature(),
            Classes::hkxSparselyAnimatedString(class) => class.signature(),
            Classes::hkxTextureFile(class) => class.signature(),
            Classes::hkxTextureInplace(class) => class.signature(),
            Classes::hkxTriangleSelectionChannel(class) => class.signature(),
            Classes::hkxVertexBuffer(class) => class.signature(),
            Classes::hkxVertexBufferVertexData(class) => class.signature(),
            Classes::hkxVertexDescription(class) => class.signature(),
            Classes::hkxVertexDescriptionElementDecl(class) => class.signature(),
            Classes::hkxVertexFloatDataChannel(class) => class.signature(),
            Classes::hkxVertexIntDataChannel(class) => class.signature(),
            Classes::hkxVertexSelectionChannel(class) => class.signature(),
            Classes::hkxVertexVectorDataChannel(class) => class.signature(),
        }
    }
    fn deps_indexes(&self) -> Vec<usize> {
        match &self {
            Classes::SwapDummy => {
                panic!(
                    "The dummy class is used only for sorting, so being called name is not a good use of the API."
                )
            }
            Classes::BGSGamebryoSequenceGenerator(class) => class.deps_indexes(),
            Classes::BSBoneSwitchGenerator(class) => class.deps_indexes(),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.deps_indexes(),
            Classes::BSComputeAddBoneAnimModifier(class) => class.deps_indexes(),
            Classes::BSCyclicBlendTransitionGenerator(class) => class.deps_indexes(),
            Classes::BSDecomposeVectorModifier(class) => class.deps_indexes(),
            Classes::BSDirectAtModifier(class) => class.deps_indexes(),
            Classes::BSDistTriggerModifier(class) => class.deps_indexes(),
            Classes::BSEventEveryNEventsModifier(class) => class.deps_indexes(),
            Classes::BSEventOnDeactivateModifier(class) => class.deps_indexes(),
            Classes::BSEventOnFalseToTrueModifier(class) => class.deps_indexes(),
            Classes::BSGetTimeStepModifier(class) => class.deps_indexes(),
            Classes::BSInterpValueModifier(class) => class.deps_indexes(),
            Classes::BSIsActiveModifier(class) => class.deps_indexes(),
            Classes::BSIStateManagerModifier(class) => class.deps_indexes(),
            Classes::BSIStateManagerModifierBSiStateData(class) => class.deps_indexes(),
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.deps_indexes()
            }
            Classes::BSiStateTaggingGenerator(class) => class.deps_indexes(),
            Classes::BSLimbIKModifier(class) => class.deps_indexes(),
            Classes::BSLookAtModifier(class) => class.deps_indexes(),
            Classes::BSLookAtModifierBoneData(class) => class.deps_indexes(),
            Classes::BSModifyOnceModifier(class) => class.deps_indexes(),
            Classes::BSOffsetAnimationGenerator(class) => class.deps_indexes(),
            Classes::BSPassByTargetTriggerModifier(class) => class.deps_indexes(),
            Classes::BSRagdollContactListenerModifier(class) => class.deps_indexes(),
            Classes::BSSpeedSamplerModifier(class) => class.deps_indexes(),
            Classes::BSSynchronizedClipGenerator(class) => class.deps_indexes(),
            Classes::BSTimerModifier(class) => class.deps_indexes(),
            Classes::BSTweenerModifier(class) => class.deps_indexes(),
            Classes::hkAabb(class) => class.deps_indexes(),
            Classes::hkAabbHalf(class) => class.deps_indexes(),
            Classes::hkAabbUint32(class) => class.deps_indexes(),
            Classes::hkaAnimatedReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaAnimation(class) => class.deps_indexes(),
            Classes::hkaAnimationBinding(class) => class.deps_indexes(),
            Classes::hkaAnimationContainer(class) => class.deps_indexes(),
            Classes::hkaAnimationPreviewColorContainer(class) => class.deps_indexes(),
            Classes::hkaAnnotationTrack(class) => class.deps_indexes(),
            Classes::hkaAnnotationTrackAnnotation(class) => class.deps_indexes(),
            Classes::hkaBone(class) => class.deps_indexes(),
            Classes::hkaBoneAttachment(class) => class.deps_indexes(),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => class.deps_indexes(),
            Classes::hkaDeltaCompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaDeltaCompressedAnimationQuantizationFormat(class) => {
                class.deps_indexes()
            }
            Classes::hkaFootstepAnalysisInfo(class) => class.deps_indexes(),
            Classes::hkaFootstepAnalysisInfoContainer(class) => class.deps_indexes(),
            Classes::hkaInterleavedUncompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaKeyFrameHierarchyUtility(class) => class.deps_indexes(),
            Classes::hkaKeyFrameHierarchyUtilityControlData(class) => {
                class.deps_indexes()
            }
            Classes::hkAlignSceneToNodeOptions(class) => class.deps_indexes(),
            Classes::hkaMeshBinding(class) => class.deps_indexes(),
            Classes::hkaMeshBindingMapping(class) => class.deps_indexes(),
            Classes::hkaQuantizedAnimation(class) => class.deps_indexes(),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaRagdollInstance(class) => class.deps_indexes(),
            Classes::hkArrayTypeAttribute(class) => class.deps_indexes(),
            Classes::hkaSkeleton(class) => class.deps_indexes(),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapper(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapperData(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapperDataChainMapping(class) => class.deps_indexes(),
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => class.deps_indexes(),
            Classes::hkaSplineCompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaWaveletCompressedAnimation(class) => class.deps_indexes(),
            Classes::hkaWaveletCompressedAnimationCompressionParams(class) => {
                class.deps_indexes()
            }
            Classes::hkaWaveletCompressedAnimationQuantizationFormat(class) => {
                class.deps_indexes()
            }
            Classes::hkBaseObject(class) => class.deps_indexes(),
            Classes::hkbAttachmentModifier(class) => class.deps_indexes(),
            Classes::hkbAttachmentSetup(class) => class.deps_indexes(),
            Classes::hkbAttributeModifier(class) => class.deps_indexes(),
            Classes::hkbAttributeModifierAssignment(class) => class.deps_indexes(),
            Classes::hkbAuxiliaryNodeInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorEventsInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraph(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphData(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphInternalState(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorGraphStringData(class) => class.deps_indexes(),
            Classes::hkbBehaviorInfo(class) => class.deps_indexes(),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.deps_indexes(),
            Classes::hkbBehaviorReferenceGenerator(class) => class.deps_indexes(),
            Classes::hkbBindable(class) => class.deps_indexes(),
            Classes::hkbBlendCurveUtils(class) => class.deps_indexes(),
            Classes::hkbBlenderGenerator(class) => class.deps_indexes(),
            Classes::hkbBlenderGeneratorChild(class) => class.deps_indexes(),
            Classes::hkbBlenderGeneratorChildInternalState(class) => class.deps_indexes(),
            Classes::hkbBlenderGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbBlendingTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbBlendingTransitionEffectInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbBoneIndexArray(class) => class.deps_indexes(),
            Classes::hkbBoneWeightArray(class) => class.deps_indexes(),
            Classes::hkbBoolVariableSequencedData(class) => class.deps_indexes(),
            Classes::hkbBoolVariableSequencedDataSample(class) => class.deps_indexes(),
            Classes::hkbCameraShakeEventPayload(class) => class.deps_indexes(),
            Classes::hkbCharacter(class) => class.deps_indexes(),
            Classes::hkbCharacterAddedInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterControlCommand(class) => class.deps_indexes(),
            Classes::hkbCharacterControllerControlData(class) => class.deps_indexes(),
            Classes::hkbCharacterControllerModifier(class) => class.deps_indexes(),
            Classes::hkbCharacterControllerModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbCharacterData(class) => class.deps_indexes(),
            Classes::hkbCharacterDataCharacterControllerInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkbCharacterInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterSetup(class) => class.deps_indexes(),
            Classes::hkbCharacterSkinInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterSteppedInfo(class) => class.deps_indexes(),
            Classes::hkbCharacterStringData(class) => class.deps_indexes(),
            Classes::hkbClientCharacterState(class) => class.deps_indexes(),
            Classes::hkbClipGenerator(class) => class.deps_indexes(),
            Classes::hkbClipGeneratorEcho(class) => class.deps_indexes(),
            Classes::hkbClipGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbClipTrigger(class) => class.deps_indexes(),
            Classes::hkbClipTriggerArray(class) => class.deps_indexes(),
            Classes::hkbCombineTransformsModifier(class) => class.deps_indexes(),
            Classes::hkbCombineTransformsModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbCompiledExpressionSet(class) => class.deps_indexes(),
            Classes::hkbCompiledExpressionSetToken(class) => class.deps_indexes(),
            Classes::hkbComputeDirectionModifier(class) => class.deps_indexes(),
            Classes::hkbComputeDirectionModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbComputeRotationToTargetModifier(class) => class.deps_indexes(),
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbCondition(class) => class.deps_indexes(),
            Classes::hkbContext(class) => class.deps_indexes(),
            Classes::hkbDampingModifier(class) => class.deps_indexes(),
            Classes::hkbDampingModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbDefaultMessageLog(class) => class.deps_indexes(),
            Classes::hkbDelayedModifier(class) => class.deps_indexes(),
            Classes::hkbDelayedModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbDetectCloseToGroundModifier(class) => class.deps_indexes(),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbEvaluateExpressionModifier(class) => class.deps_indexes(),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.deps_indexes()
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbEvaluateHandleModifier(class) => class.deps_indexes(),
            Classes::hkbEvent(class) => class.deps_indexes(),
            Classes::hkbEventBase(class) => class.deps_indexes(),
            Classes::hkbEventDrivenModifier(class) => class.deps_indexes(),
            Classes::hkbEventDrivenModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbEventInfo(class) => class.deps_indexes(),
            Classes::hkbEventPayload(class) => class.deps_indexes(),
            Classes::hkbEventPayloadList(class) => class.deps_indexes(),
            Classes::hkbEventProperty(class) => class.deps_indexes(),
            Classes::hkbEventRaisedInfo(class) => class.deps_indexes(),
            Classes::hkbEventRangeData(class) => class.deps_indexes(),
            Classes::hkbEventRangeDataArray(class) => class.deps_indexes(),
            Classes::hkbEventSequencedData(class) => class.deps_indexes(),
            Classes::hkbEventSequencedDataSequencedEvent(class) => class.deps_indexes(),
            Classes::hkbEventsFromRangeModifier(class) => class.deps_indexes(),
            Classes::hkbEventsFromRangeModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbExpressionCondition(class) => class.deps_indexes(),
            Classes::hkbExpressionData(class) => class.deps_indexes(),
            Classes::hkbExpressionDataArray(class) => class.deps_indexes(),
            Classes::hkbExtractRagdollPoseModifier(class) => class.deps_indexes(),
            Classes::hkbFootIkControlData(class) => class.deps_indexes(),
            Classes::hkbFootIkControlsModifier(class) => class.deps_indexes(),
            Classes::hkbFootIkControlsModifierLeg(class) => class.deps_indexes(),
            Classes::hkbFootIkDriverInfo(class) => class.deps_indexes(),
            Classes::hkbFootIkDriverInfoLeg(class) => class.deps_indexes(),
            Classes::hkbFootIkGains(class) => class.deps_indexes(),
            Classes::hkbFootIkModifier(class) => class.deps_indexes(),
            Classes::hkbFootIkModifierInternalLegData(class) => class.deps_indexes(),
            Classes::hkbFootIkModifierLeg(class) => class.deps_indexes(),
            Classes::hkbGenerator(class) => class.deps_indexes(),
            Classes::hkbGeneratorOutputListener(class) => class.deps_indexes(),
            Classes::hkbGeneratorSyncInfo(class) => class.deps_indexes(),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.deps_indexes(),
            Classes::hkbGeneratorTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbGetHandleOnBoneModifier(class) => class.deps_indexes(),
            Classes::hkbGetUpModifier(class) => class.deps_indexes(),
            Classes::hkbGetUpModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbGetWorldFromModelModifier(class) => class.deps_indexes(),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbHandIkControlData(class) => class.deps_indexes(),
            Classes::hkbHandIkControlsModifier(class) => class.deps_indexes(),
            Classes::hkbHandIkControlsModifierHand(class) => class.deps_indexes(),
            Classes::hkbHandIkDriverInfo(class) => class.deps_indexes(),
            Classes::hkbHandIkDriverInfoHand(class) => class.deps_indexes(),
            Classes::hkbHandIkModifier(class) => class.deps_indexes(),
            Classes::hkbHandIkModifierHand(class) => class.deps_indexes(),
            Classes::hkbHandle(class) => class.deps_indexes(),
            Classes::hkbIntEventPayload(class) => class.deps_indexes(),
            Classes::hkbIntVariableSequencedData(class) => class.deps_indexes(),
            Classes::hkbIntVariableSequencedDataSample(class) => class.deps_indexes(),
            Classes::hkBitField(class) => class.deps_indexes(),
            Classes::hkbKeyframeBonesModifier(class) => class.deps_indexes(),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => class.deps_indexes(),
            Classes::hkbLinkedSymbolInfo(class) => class.deps_indexes(),
            Classes::hkbLookAtModifier(class) => class.deps_indexes(),
            Classes::hkbLookAtModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbManualSelectorGenerator(class) => class.deps_indexes(),
            Classes::hkbManualSelectorGeneratorInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbMessageLog(class) => class.deps_indexes(),
            Classes::hkbMirroredSkeletonInfo(class) => class.deps_indexes(),
            Classes::hkbMirrorModifier(class) => class.deps_indexes(),
            Classes::hkbModifier(class) => class.deps_indexes(),
            Classes::hkbModifierGenerator(class) => class.deps_indexes(),
            Classes::hkbModifierList(class) => class.deps_indexes(),
            Classes::hkbModifierWrapper(class) => class.deps_indexes(),
            Classes::hkbMoveCharacterModifier(class) => class.deps_indexes(),
            Classes::hkbMoveCharacterModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbNamedEventPayload(class) => class.deps_indexes(),
            Classes::hkbNamedIntEventPayload(class) => class.deps_indexes(),
            Classes::hkbNamedRealEventPayload(class) => class.deps_indexes(),
            Classes::hkbNamedStringEventPayload(class) => class.deps_indexes(),
            Classes::hkbNode(class) => class.deps_indexes(),
            Classes::hkbNodeInternalStateInfo(class) => class.deps_indexes(),
            Classes::hkbParticleSystemEventPayload(class) => class.deps_indexes(),
            Classes::hkbPoseMatchingGenerator(class) => class.deps_indexes(),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => class.deps_indexes(),
            Classes::hkbPoweredRagdollControlData(class) => class.deps_indexes(),
            Classes::hkbPoweredRagdollControlsModifier(class) => class.deps_indexes(),
            Classes::hkbProjectData(class) => class.deps_indexes(),
            Classes::hkbProjectStringData(class) => class.deps_indexes(),
            Classes::hkbProxyModifier(class) => class.deps_indexes(),
            Classes::hkbProxyModifierProxyInfo(class) => class.deps_indexes(),
            Classes::hkbRaiseEventCommand(class) => class.deps_indexes(),
            Classes::hkbRealEventPayload(class) => class.deps_indexes(),
            Classes::hkbRealVariableSequencedData(class) => class.deps_indexes(),
            Classes::hkbRealVariableSequencedDataSample(class) => class.deps_indexes(),
            Classes::hkbReferencePoseGenerator(class) => class.deps_indexes(),
            Classes::hkbRegisteredGenerator(class) => class.deps_indexes(),
            Classes::hkbRigidBodyRagdollControlData(class) => class.deps_indexes(),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => class.deps_indexes(),
            Classes::hkbRoleAttribute(class) => class.deps_indexes(),
            Classes::hkbRotateCharacterModifier(class) => class.deps_indexes(),
            Classes::hkbRotateCharacterModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbSenseHandleModifier(class) => class.deps_indexes(),
            Classes::hkbSenseHandleModifierRange(class) => class.deps_indexes(),
            Classes::hkbSequence(class) => class.deps_indexes(),
            Classes::hkbSequencedData(class) => class.deps_indexes(),
            Classes::hkbSequenceInternalState(class) => class.deps_indexes(),
            Classes::hkbSequenceStringData(class) => class.deps_indexes(),
            Classes::hkbSetBehaviorCommand(class) => class.deps_indexes(),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => class.deps_indexes(),
            Classes::hkbSetNodePropertyCommand(class) => class.deps_indexes(),
            Classes::hkbSetWordVariableCommand(class) => class.deps_indexes(),
            Classes::hkbSetWorldFromModelModifier(class) => class.deps_indexes(),
            Classes::hkbSimulationControlCommand(class) => class.deps_indexes(),
            Classes::hkbSimulationStateInfo(class) => class.deps_indexes(),
            Classes::hkbStateChooser(class) => class.deps_indexes(),
            Classes::hkbStateListener(class) => class.deps_indexes(),
            Classes::hkbStateMachine(class) => class.deps_indexes(),
            Classes::hkbStateMachineActiveTransitionInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineDelayedTransitionInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineEventPropertyArray(class) => class.deps_indexes(),
            Classes::hkbStateMachineInternalState(class) => class.deps_indexes(),
            Classes::hkbStateMachineNestedStateMachineData(class) => class.deps_indexes(),
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkbStateMachineStateInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineTimeInterval(class) => class.deps_indexes(),
            Classes::hkbStateMachineTransitionInfo(class) => class.deps_indexes(),
            Classes::hkbStateMachineTransitionInfoArray(class) => class.deps_indexes(),
            Classes::hkbStateMachineTransitionInfoReference(class) => {
                class.deps_indexes()
            }
            Classes::hkbStringCondition(class) => class.deps_indexes(),
            Classes::hkbStringEventPayload(class) => class.deps_indexes(),
            Classes::hkbTestStateChooser(class) => class.deps_indexes(),
            Classes::hkbTimerModifier(class) => class.deps_indexes(),
            Classes::hkbTimerModifierInternalState(class) => class.deps_indexes(),
            Classes::hkbTransformVectorModifier(class) => class.deps_indexes(),
            Classes::hkbTransformVectorModifierInternalState(class) => {
                class.deps_indexes()
            }
            Classes::hkbTransitionEffect(class) => class.deps_indexes(),
            Classes::hkbTwistModifier(class) => class.deps_indexes(),
            Classes::hkbVariableBindingSet(class) => class.deps_indexes(),
            Classes::hkbVariableBindingSetBinding(class) => class.deps_indexes(),
            Classes::hkbVariableInfo(class) => class.deps_indexes(),
            Classes::hkbVariableValue(class) => class.deps_indexes(),
            Classes::hkbVariableValueSet(class) => class.deps_indexes(),
            Classes::hkbWorldEnums(class) => class.deps_indexes(),
            Classes::hkbWorldFromModelModeData(class) => class.deps_indexes(),
            Classes::hkClass(class) => class.deps_indexes(),
            Classes::hkClassEnum(class) => class.deps_indexes(),
            Classes::hkClassEnumItem(class) => class.deps_indexes(),
            Classes::hkClassMember(class) => class.deps_indexes(),
            Classes::hkColor(class) => class.deps_indexes(),
            Classes::hkContactPoint(class) => class.deps_indexes(),
            Classes::hkContactPointMaterial(class) => class.deps_indexes(),
            Classes::hkCustomAttributes(class) => class.deps_indexes(),
            Classes::hkCustomAttributesAttribute(class) => class.deps_indexes(),
            Classes::hkDataObjectTypeAttribute(class) => class.deps_indexes(),
            Classes::hkDescriptionAttribute(class) => class.deps_indexes(),
            Classes::hkDocumentationAttribute(class) => class.deps_indexes(),
            Classes::hkGeometry(class) => class.deps_indexes(),
            Classes::hkGeometryTriangle(class) => class.deps_indexes(),
            Classes::hkGizmoAttribute(class) => class.deps_indexes(),
            Classes::hkHalf8(class) => class.deps_indexes(),
            Classes::hkIndexedTransformSet(class) => class.deps_indexes(),
            Classes::hkLinkAttribute(class) => class.deps_indexes(),
            Classes::hkLocalFrame(class) => class.deps_indexes(),
            Classes::hkLocalFrameGroup(class) => class.deps_indexes(),
            Classes::hkMemoryMeshBody(class) => class.deps_indexes(),
            Classes::hkMemoryMeshMaterial(class) => class.deps_indexes(),
            Classes::hkMemoryMeshShape(class) => class.deps_indexes(),
            Classes::hkMemoryMeshTexture(class) => class.deps_indexes(),
            Classes::hkMemoryMeshVertexBuffer(class) => class.deps_indexes(),
            Classes::hkMemoryResourceContainer(class) => class.deps_indexes(),
            Classes::hkMemoryResourceHandle(class) => class.deps_indexes(),
            Classes::hkMemoryResourceHandleExternalLink(class) => class.deps_indexes(),
            Classes::hkMemoryTrackerAttribute(class) => class.deps_indexes(),
            Classes::hkMeshBody(class) => class.deps_indexes(),
            Classes::hkMeshBoneIndexMapping(class) => class.deps_indexes(),
            Classes::hkMeshMaterial(class) => class.deps_indexes(),
            Classes::hkMeshSection(class) => class.deps_indexes(),
            Classes::hkMeshSectionCinfo(class) => class.deps_indexes(),
            Classes::hkMeshShape(class) => class.deps_indexes(),
            Classes::hkMeshTexture(class) => class.deps_indexes(),
            Classes::hkMeshVertexBuffer(class) => class.deps_indexes(),
            Classes::hkModelerNodeTypeAttribute(class) => class.deps_indexes(),
            Classes::hkMonitorStreamColorTable(class) => class.deps_indexes(),
            Classes::hkMonitorStreamColorTableColorPair(class) => class.deps_indexes(),
            Classes::hkMonitorStreamFrameInfo(class) => class.deps_indexes(),
            Classes::hkMonitorStreamStringMap(class) => class.deps_indexes(),
            Classes::hkMonitorStreamStringMapStringMap(class) => class.deps_indexes(),
            Classes::hkMoppBvTreeShapeBase(class) => class.deps_indexes(),
            Classes::hkMotionState(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBuffer(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBufferElementInfo(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBufferLockedElement(class) => class.deps_indexes(),
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkMultiThreadCheck(class) => class.deps_indexes(),
            Classes::hkp2dAngConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAabbPhantom(class) => class.deps_indexes(),
            Classes::hkPackedVector3(class) => class.deps_indexes(),
            Classes::hkPackfileHeader(class) => class.deps_indexes(),
            Classes::hkPackfileSectionHeader(class) => class.deps_indexes(),
            Classes::hkpAction(class) => class.deps_indexes(),
            Classes::hkpAgent1nSector(class) => class.deps_indexes(),
            Classes::hkpAngConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngFrictionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngMotorConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpAngularDashpotAction(class) => class.deps_indexes(),
            Classes::hkpArrayAction(class) => class.deps_indexes(),
            Classes::hkpBallAndSocketConstraintData(class) => class.deps_indexes(),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpBallGun(class) => class.deps_indexes(),
            Classes::hkpBallSocketChainData(class) => class.deps_indexes(),
            Classes::hkpBallSocketChainDataConstraintInfo(class) => class.deps_indexes(),
            Classes::hkpBallSocketConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpBinaryAction(class) => class.deps_indexes(),
            Classes::hkpBoxMotion(class) => class.deps_indexes(),
            Classes::hkpBoxShape(class) => class.deps_indexes(),
            Classes::hkpBreakableBody(class) => class.deps_indexes(),
            Classes::hkpBreakableConstraintData(class) => class.deps_indexes(),
            Classes::hkpBridgeAtoms(class) => class.deps_indexes(),
            Classes::hkpBridgeConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpBroadPhaseHandle(class) => class.deps_indexes(),
            Classes::hkpBvShape(class) => class.deps_indexes(),
            Classes::hkpBvTreeShape(class) => class.deps_indexes(),
            Classes::hkpCachingShapePhantom(class) => class.deps_indexes(),
            Classes::hkpCallbackConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpCapsuleShape(class) => class.deps_indexes(),
            Classes::hkpCdBody(class) => class.deps_indexes(),
            Classes::hkpCenterOfMassChangerModifierConstraintAtom(class) => {
                class.deps_indexes()
            }
            Classes::hkpCharacterControllerCinfo(class) => class.deps_indexes(),
            Classes::hkpCharacterMotion(class) => class.deps_indexes(),
            Classes::hkpCharacterProxyCinfo(class) => class.deps_indexes(),
            Classes::hkpCharacterRigidBodyCinfo(class) => class.deps_indexes(),
            Classes::hkpCogWheelConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpCogWheelConstraintData(class) => class.deps_indexes(),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpCollidable(class) => class.deps_indexes(),
            Classes::hkpCollidableBoundingVolumeData(class) => class.deps_indexes(),
            Classes::hkpCollidableCollidableFilter(class) => class.deps_indexes(),
            Classes::hkpCollisionFilter(class) => class.deps_indexes(),
            Classes::hkpCollisionFilterList(class) => class.deps_indexes(),
            Classes::hkpCompressedMeshShape(class) => class.deps_indexes(),
            Classes::hkpCompressedMeshShapeBigTriangle(class) => class.deps_indexes(),
            Classes::hkpCompressedMeshShapeChunk(class) => class.deps_indexes(),
            Classes::hkpCompressedMeshShapeConvexPiece(class) => class.deps_indexes(),
            Classes::hkpCompressedSampledHeightFieldShape(class) => class.deps_indexes(),
            Classes::hkpConeLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpConstrainedSystemFilter(class) => class.deps_indexes(),
            Classes::hkpConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpConstraintChainData(class) => class.deps_indexes(),
            Classes::hkpConstraintChainInstance(class) => class.deps_indexes(),
            Classes::hkpConstraintChainInstanceAction(class) => class.deps_indexes(),
            Classes::hkpConstraintCollisionFilter(class) => class.deps_indexes(),
            Classes::hkpConstraintData(class) => class.deps_indexes(),
            Classes::hkpConstraintInstance(class) => class.deps_indexes(),
            Classes::hkpConstraintInstanceSmallArraySerializeOverrideType(class) => {
                class.deps_indexes()
            }
            Classes::hkpConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpConvexListFilter(class) => class.deps_indexes(),
            Classes::hkpConvexListShape(class) => class.deps_indexes(),
            Classes::hkpConvexPieceMeshShape(class) => class.deps_indexes(),
            Classes::hkpConvexPieceStreamData(class) => class.deps_indexes(),
            Classes::hkpConvexShape(class) => class.deps_indexes(),
            Classes::hkpConvexTransformShape(class) => class.deps_indexes(),
            Classes::hkpConvexTransformShapeBase(class) => class.deps_indexes(),
            Classes::hkpConvexTranslateShape(class) => class.deps_indexes(),
            Classes::hkpConvexVerticesConnectivity(class) => class.deps_indexes(),
            Classes::hkpConvexVerticesShape(class) => class.deps_indexes(),
            Classes::hkpConvexVerticesShapeFourVectors(class) => class.deps_indexes(),
            Classes::hkpCylinderShape(class) => class.deps_indexes(),
            Classes::hkpDashpotAction(class) => class.deps_indexes(),
            Classes::hkpDefaultConvexListFilter(class) => class.deps_indexes(),
            Classes::hkpDefaultWorldMemoryWatchDog(class) => class.deps_indexes(),
            Classes::hkpDisableEntityCollisionFilter(class) => class.deps_indexes(),
            Classes::hkpDisplayBindingData(class) => class.deps_indexes(),
            Classes::hkpDisplayBindingDataPhysicsSystem(class) => class.deps_indexes(),
            Classes::hkpDisplayBindingDataRigidBody(class) => class.deps_indexes(),
            Classes::hkpEntity(class) => class.deps_indexes(),
            Classes::hkpEntityExtendedListeners(class) => class.deps_indexes(),
            Classes::hkpEntitySmallArraySerializeOverrideType(class) => {
                class.deps_indexes()
            }
            Classes::hkpEntitySpuCollisionCallback(class) => class.deps_indexes(),
            Classes::hkpExtendedMeshShape(class) => class.deps_indexes(),
            Classes::hkpExtendedMeshShapeShapesSubpart(class) => class.deps_indexes(),
            Classes::hkpExtendedMeshShapeSubpart(class) => class.deps_indexes(),
            Classes::hkpExtendedMeshShapeTrianglesSubpart(class) => class.deps_indexes(),
            Classes::hkpFastMeshShape(class) => class.deps_indexes(),
            Classes::hkpFirstPersonGun(class) => class.deps_indexes(),
            Classes::hkpFixedRigidMotion(class) => class.deps_indexes(),
            Classes::hkpGenericConstraintData(class) => class.deps_indexes(),
            Classes::hkpGenericConstraintDataScheme(class) => class.deps_indexes(),
            Classes::hkpGenericConstraintDataSchemeConstraintInfo(class) => {
                class.deps_indexes()
            }
            Classes::hkpGravityGun(class) => class.deps_indexes(),
            Classes::hkpGroupCollisionFilter(class) => class.deps_indexes(),
            Classes::hkpGroupFilter(class) => class.deps_indexes(),
            Classes::hkpHeightFieldShape(class) => class.deps_indexes(),
            Classes::hkpHingeConstraintData(class) => class.deps_indexes(),
            Classes::hkpHingeConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpHingeLimitsData(class) => class.deps_indexes(),
            Classes::hkpHingeLimitsDataAtoms(class) => class.deps_indexes(),
            Classes::hkpIgnoreModifierConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpKeyframedRigidMotion(class) => class.deps_indexes(),
            Classes::hkpLimitedForceConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpLimitedHingeConstraintData(class) => class.deps_indexes(),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpLinConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinearParametricCurve(class) => class.deps_indexes(),
            Classes::hkpLinFrictionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinkedCollidable(class) => class.deps_indexes(),
            Classes::hkpLinLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinMotorConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpLinSoftConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpListShape(class) => class.deps_indexes(),
            Classes::hkpListShapeChildInfo(class) => class.deps_indexes(),
            Classes::hkpMalleableConstraintData(class) => class.deps_indexes(),
            Classes::hkpMassChangerModifierConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpMassProperties(class) => class.deps_indexes(),
            Classes::hkpMaterial(class) => class.deps_indexes(),
            Classes::hkpMaxSizeMotion(class) => class.deps_indexes(),
            Classes::hkpMeshMaterial(class) => class.deps_indexes(),
            Classes::hkpMeshShape(class) => class.deps_indexes(),
            Classes::hkpMeshShapeSubpart(class) => class.deps_indexes(),
            Classes::hkpModifierConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpMoppBvTreeShape(class) => class.deps_indexes(),
            Classes::hkpMoppCode(class) => class.deps_indexes(),
            Classes::hkpMoppCodeCodeInfo(class) => class.deps_indexes(),
            Classes::hkpMoppCodeReindexedTerminal(class) => class.deps_indexes(),
            Classes::hkpMotion(class) => class.deps_indexes(),
            Classes::hkpMotorAction(class) => class.deps_indexes(),
            Classes::hkpMountedBallGun(class) => class.deps_indexes(),
            Classes::hkpMouseSpringAction(class) => class.deps_indexes(),
            Classes::hkpMovingSurfaceModifierConstraintAtom(class) => {
                class.deps_indexes()
            }
            Classes::hkpMultiRayShape(class) => class.deps_indexes(),
            Classes::hkpMultiRayShapeRay(class) => class.deps_indexes(),
            Classes::hkpMultiSphereShape(class) => class.deps_indexes(),
            Classes::hkpMultithreadedVehicleManager(class) => class.deps_indexes(),
            Classes::hkpNamedMeshMaterial(class) => class.deps_indexes(),
            Classes::hkpNullCollisionFilter(class) => class.deps_indexes(),
            Classes::hkPostFinishAttribute(class) => class.deps_indexes(),
            Classes::hkpOverwritePivotConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpPairCollisionFilter(class) => class.deps_indexes(),
            Classes::hkpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.deps_indexes()
            }
            Classes::hkpParametricCurve(class) => class.deps_indexes(),
            Classes::hkpPhantom(class) => class.deps_indexes(),
            Classes::hkpPhantomCallbackShape(class) => class.deps_indexes(),
            Classes::hkpPhysicsData(class) => class.deps_indexes(),
            Classes::hkpPhysicsSystem(class) => class.deps_indexes(),
            Classes::hkpPhysicsSystemWithContacts(class) => class.deps_indexes(),
            Classes::hkpPlaneShape(class) => class.deps_indexes(),
            Classes::hkpPointToPathConstraintData(class) => class.deps_indexes(),
            Classes::hkpPointToPlaneConstraintData(class) => class.deps_indexes(),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpPositionConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpPoweredChainData(class) => class.deps_indexes(),
            Classes::hkpPoweredChainDataConstraintInfo(class) => class.deps_indexes(),
            Classes::hkpPoweredChainMapper(class) => class.deps_indexes(),
            Classes::hkpPoweredChainMapperLinkInfo(class) => class.deps_indexes(),
            Classes::hkpPoweredChainMapperTarget(class) => class.deps_indexes(),
            Classes::hkpPrismaticConstraintData(class) => class.deps_indexes(),
            Classes::hkpPrismaticConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpProjectileGun(class) => class.deps_indexes(),
            Classes::hkpProperty(class) => class.deps_indexes(),
            Classes::hkpPropertyValue(class) => class.deps_indexes(),
            Classes::hkpPulleyConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpPulleyConstraintData(class) => class.deps_indexes(),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpRackAndPinionConstraintData(class) => class.deps_indexes(),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRagdollConstraintData(class) => class.deps_indexes(),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRagdollLimitsData(class) => class.deps_indexes(),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.deps_indexes(),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpRayCollidableFilter(class) => class.deps_indexes(),
            Classes::hkpRayShapeCollectionFilter(class) => class.deps_indexes(),
            Classes::hkpRejectChassisListener(class) => class.deps_indexes(),
            Classes::hkpRemoveTerminalsMoppModifier(class) => class.deps_indexes(),
            Classes::hkpReorientAction(class) => class.deps_indexes(),
            Classes::hkpRigidBody(class) => class.deps_indexes(),
            Classes::hkpRotationalConstraintData(class) => class.deps_indexes(),
            Classes::hkpRotationalConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpSampledHeightFieldShape(class) => class.deps_indexes(),
            Classes::hkpSerializedAgentNnEntry(class) => class.deps_indexes(),
            Classes::hkpSerializedDisplayMarker(class) => class.deps_indexes(),
            Classes::hkpSerializedDisplayMarkerList(class) => class.deps_indexes(),
            Classes::hkpSerializedDisplayRbTransforms(class) => class.deps_indexes(),
            Classes::hkpSerializedDisplayRbTransformsDisplayTransformPair(class) => {
                class.deps_indexes()
            }
            Classes::hkpSerializedSubTrack1nInfo(class) => class.deps_indexes(),
            Classes::hkpSerializedTrack1nInfo(class) => class.deps_indexes(),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSetLocalTransformsConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSetupStabilizationAtom(class) => class.deps_indexes(),
            Classes::hkpShape(class) => class.deps_indexes(),
            Classes::hkpShapeCollection(class) => class.deps_indexes(),
            Classes::hkpShapeCollectionFilter(class) => class.deps_indexes(),
            Classes::hkpShapeContainer(class) => class.deps_indexes(),
            Classes::hkpShapeInfo(class) => class.deps_indexes(),
            Classes::hkpShapeModifier(class) => class.deps_indexes(),
            Classes::hkpShapePhantom(class) => class.deps_indexes(),
            Classes::hkpSimpleContactConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSimpleContactConstraintDataInfo(class) => class.deps_indexes(),
            Classes::hkpSimpleMeshShape(class) => class.deps_indexes(),
            Classes::hkpSimpleMeshShapeTriangle(class) => class.deps_indexes(),
            Classes::hkpSimpleShapePhantom(class) => class.deps_indexes(),
            Classes::hkpSimpleShapePhantomCollisionDetail(class) => class.deps_indexes(),
            Classes::hkpSimulation(class) => class.deps_indexes(),
            Classes::hkpSingleShapeContainer(class) => class.deps_indexes(),
            Classes::hkpSoftContactModifierConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpSphereMotion(class) => class.deps_indexes(),
            Classes::hkpSphereRepShape(class) => class.deps_indexes(),
            Classes::hkpSphereShape(class) => class.deps_indexes(),
            Classes::hkpSpringAction(class) => class.deps_indexes(),
            Classes::hkpSpringDamperConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpStiffSpringChainData(class) => class.deps_indexes(),
            Classes::hkpStiffSpringChainDataConstraintInfo(class) => class.deps_indexes(),
            Classes::hkpStiffSpringConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpStiffSpringConstraintData(class) => class.deps_indexes(),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpStorageExtendedMeshShape(class) => class.deps_indexes(),
            Classes::hkpStorageExtendedMeshShapeMaterial(class) => class.deps_indexes(),
            Classes::hkpStorageExtendedMeshShapeMeshSubpartStorage(class) => {
                class.deps_indexes()
            }
            Classes::hkpStorageExtendedMeshShapeShapeSubpartStorage(class) => {
                class.deps_indexes()
            }
            Classes::hkpStorageMeshShape(class) => class.deps_indexes(),
            Classes::hkpStorageMeshShapeSubpartStorage(class) => class.deps_indexes(),
            Classes::hkpStorageSampledHeightFieldShape(class) => class.deps_indexes(),
            Classes::hkpThinBoxMotion(class) => class.deps_indexes(),
            Classes::hkpTransformShape(class) => class.deps_indexes(),
            Classes::hkpTriangleShape(class) => class.deps_indexes(),
            Classes::hkpTriggerVolume(class) => class.deps_indexes(),
            Classes::hkpTriggerVolumeEventInfo(class) => class.deps_indexes(),
            Classes::hkpTriSampledHeightFieldBvTreeShape(class) => class.deps_indexes(),
            Classes::hkpTriSampledHeightFieldCollection(class) => class.deps_indexes(),
            Classes::hkpTwistLimitConstraintAtom(class) => class.deps_indexes(),
            Classes::hkpTypedBroadPhaseHandle(class) => class.deps_indexes(),
            Classes::hkpTyremarkPoint(class) => class.deps_indexes(),
            Classes::hkpTyremarksInfo(class) => class.deps_indexes(),
            Classes::hkpTyremarksWheel(class) => class.deps_indexes(),
            Classes::hkpUnaryAction(class) => class.deps_indexes(),
            Classes::hkpVehicleAerodynamics(class) => class.deps_indexes(),
            Classes::hkpVehicleBrake(class) => class.deps_indexes(),
            Classes::hkpVehicleCastBatchingManager(class) => class.deps_indexes(),
            Classes::hkpVehicleData(class) => class.deps_indexes(),
            Classes::hkpVehicleDataWheelComponentParams(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultAerodynamics(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultAnalogDriverInput(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultBrake(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultBrakeWheelBrakingProperties(class) => {
                class.deps_indexes()
            }
            Classes::hkpVehicleDefaultEngine(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultSteering(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultSuspension(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.deps_indexes(),
            Classes::hkpVehicleDefaultTransmission(class) => class.deps_indexes(),
            Classes::hkpVehicleDefaultVelocityDamper(class) => class.deps_indexes(),
            Classes::hkpVehicleDriverInput(class) => class.deps_indexes(),
            Classes::hkpVehicleDriverInputAnalogStatus(class) => class.deps_indexes(),
            Classes::hkpVehicleDriverInputStatus(class) => class.deps_indexes(),
            Classes::hkpVehicleEngine(class) => class.deps_indexes(),
            Classes::hkpVehicleFrictionDescription(class) => class.deps_indexes(),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => {
                class.deps_indexes()
            }
            Classes::hkpVehicleFrictionStatus(class) => class.deps_indexes(),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => class.deps_indexes(),
            Classes::hkpVehicleInstance(class) => class.deps_indexes(),
            Classes::hkpVehicleInstanceWheelInfo(class) => class.deps_indexes(),
            Classes::hkpVehicleLinearCastBatchingManager(class) => class.deps_indexes(),
            Classes::hkpVehicleLinearCastWheelCollide(class) => class.deps_indexes(),
            Classes::hkpVehicleLinearCastWheelCollideWheelState(class) => {
                class.deps_indexes()
            }
            Classes::hkpVehicleManager(class) => class.deps_indexes(),
            Classes::hkpVehicleRayCastBatchingManager(class) => class.deps_indexes(),
            Classes::hkpVehicleRayCastWheelCollide(class) => class.deps_indexes(),
            Classes::hkpVehicleSteering(class) => class.deps_indexes(),
            Classes::hkpVehicleSuspension(class) => class.deps_indexes(),
            Classes::hkpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.deps_indexes()
            }
            Classes::hkpVehicleTransmission(class) => class.deps_indexes(),
            Classes::hkpVehicleVelocityDamper(class) => class.deps_indexes(),
            Classes::hkpVehicleWheelCollide(class) => class.deps_indexes(),
            Classes::hkpVelocityConstraintMotor(class) => class.deps_indexes(),
            Classes::hkpViscousSurfaceModifierConstraintAtom(class) => {
                class.deps_indexes()
            }
            Classes::hkpWeldingUtility(class) => class.deps_indexes(),
            Classes::hkpWheelConstraintData(class) => class.deps_indexes(),
            Classes::hkpWheelConstraintDataAtoms(class) => class.deps_indexes(),
            Classes::hkpWorld(class) => class.deps_indexes(),
            Classes::hkpWorldCinfo(class) => class.deps_indexes(),
            Classes::hkpWorldObject(class) => class.deps_indexes(),
            Classes::hkQTransform(class) => class.deps_indexes(),
            Classes::hkRangeInt32Attribute(class) => class.deps_indexes(),
            Classes::hkRangeRealAttribute(class) => class.deps_indexes(),
            Classes::hkReferencedObject(class) => class.deps_indexes(),
            Classes::hkReflectedFileAttribute(class) => class.deps_indexes(),
            Classes::hkResourceBase(class) => class.deps_indexes(),
            Classes::hkResourceContainer(class) => class.deps_indexes(),
            Classes::hkResourceHandle(class) => class.deps_indexes(),
            Classes::hkRootLevelContainer(class) => class.deps_indexes(),
            Classes::hkRootLevelContainerNamedVariant(class) => class.deps_indexes(),
            Classes::hkSemanticsAttribute(class) => class.deps_indexes(),
            Classes::hkSimpleLocalFrame(class) => class.deps_indexes(),
            Classes::hkSphere(class) => class.deps_indexes(),
            Classes::hkSweptTransform(class) => class.deps_indexes(),
            Classes::hkTraceStreamTitle(class) => class.deps_indexes(),
            Classes::hkTrackerSerializableScanSnapshot(class) => class.deps_indexes(),
            Classes::hkTrackerSerializableScanSnapshotAllocation(class) => {
                class.deps_indexes()
            }
            Classes::hkTrackerSerializableScanSnapshotBlock(class) => {
                class.deps_indexes()
            }
            Classes::hkUiAttribute(class) => class.deps_indexes(),
            Classes::hkVertexFormat(class) => class.deps_indexes(),
            Classes::hkVertexFormatElement(class) => class.deps_indexes(),
            Classes::hkWorldMemoryAvailableWatchDog(class) => class.deps_indexes(),
            Classes::hkxAnimatedFloat(class) => class.deps_indexes(),
            Classes::hkxAnimatedMatrix(class) => class.deps_indexes(),
            Classes::hkxAnimatedQuaternion(class) => class.deps_indexes(),
            Classes::hkxAnimatedVector(class) => class.deps_indexes(),
            Classes::hkxAttribute(class) => class.deps_indexes(),
            Classes::hkxAttributeGroup(class) => class.deps_indexes(),
            Classes::hkxAttributeHolder(class) => class.deps_indexes(),
            Classes::hkxCamera(class) => class.deps_indexes(),
            Classes::hkxEdgeSelectionChannel(class) => class.deps_indexes(),
            Classes::hkxEnum(class) => class.deps_indexes(),
            Classes::hkxEnumItem(class) => class.deps_indexes(),
            Classes::hkxEnvironment(class) => class.deps_indexes(),
            Classes::hkxEnvironmentVariable(class) => class.deps_indexes(),
            Classes::hkxIndexBuffer(class) => class.deps_indexes(),
            Classes::hkxLight(class) => class.deps_indexes(),
            Classes::hkxMaterial(class) => class.deps_indexes(),
            Classes::hkxMaterialEffect(class) => class.deps_indexes(),
            Classes::hkxMaterialProperty(class) => class.deps_indexes(),
            Classes::hkxMaterialShader(class) => class.deps_indexes(),
            Classes::hkxMaterialShaderSet(class) => class.deps_indexes(),
            Classes::hkxMaterialTextureStage(class) => class.deps_indexes(),
            Classes::hkxMesh(class) => class.deps_indexes(),
            Classes::hkxMeshSection(class) => class.deps_indexes(),
            Classes::hkxMeshUserChannelInfo(class) => class.deps_indexes(),
            Classes::hkxNode(class) => class.deps_indexes(),
            Classes::hkxNodeAnnotationData(class) => class.deps_indexes(),
            Classes::hkxNodeSelectionSet(class) => class.deps_indexes(),
            Classes::hkxScene(class) => class.deps_indexes(),
            Classes::hkxSkinBinding(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedBool(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedEnum(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedInt(class) => class.deps_indexes(),
            Classes::hkxSparselyAnimatedString(class) => class.deps_indexes(),
            Classes::hkxTextureFile(class) => class.deps_indexes(),
            Classes::hkxTextureInplace(class) => class.deps_indexes(),
            Classes::hkxTriangleSelectionChannel(class) => class.deps_indexes(),
            Classes::hkxVertexBuffer(class) => class.deps_indexes(),
            Classes::hkxVertexBufferVertexData(class) => class.deps_indexes(),
            Classes::hkxVertexDescription(class) => class.deps_indexes(),
            Classes::hkxVertexDescriptionElementDecl(class) => class.deps_indexes(),
            Classes::hkxVertexFloatDataChannel(class) => class.deps_indexes(),
            Classes::hkxVertexIntDataChannel(class) => class.deps_indexes(),
            Classes::hkxVertexSelectionChannel(class) => class.deps_indexes(),
            Classes::hkxVertexVectorDataChannel(class) => class.deps_indexes(),
        }
    }
}
impl<'a> _serde::Serialize for Classes<'a> {
    fn serialize<S: _serde::ser::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            Classes::SwapDummy => {
                panic!(
                    "The dummy class is used only for sorting, so being called name is not a good use of the API."
                )
            }
            Classes::BGSGamebryoSequenceGenerator(class) => class.serialize(serializer),
            Classes::BSBoneSwitchGenerator(class) => class.serialize(serializer),
            Classes::BSBoneSwitchGeneratorBoneData(class) => class.serialize(serializer),
            Classes::BSComputeAddBoneAnimModifier(class) => class.serialize(serializer),
            Classes::BSCyclicBlendTransitionGenerator(class) => {
                class.serialize(serializer)
            }
            Classes::BSDecomposeVectorModifier(class) => class.serialize(serializer),
            Classes::BSDirectAtModifier(class) => class.serialize(serializer),
            Classes::BSDistTriggerModifier(class) => class.serialize(serializer),
            Classes::BSEventEveryNEventsModifier(class) => class.serialize(serializer),
            Classes::BSEventOnDeactivateModifier(class) => class.serialize(serializer),
            Classes::BSEventOnFalseToTrueModifier(class) => class.serialize(serializer),
            Classes::BSGetTimeStepModifier(class) => class.serialize(serializer),
            Classes::BSInterpValueModifier(class) => class.serialize(serializer),
            Classes::BSIsActiveModifier(class) => class.serialize(serializer),
            Classes::BSIStateManagerModifier(class) => class.serialize(serializer),
            Classes::BSIStateManagerModifierBSiStateData(class) => {
                class.serialize(serializer)
            }
            Classes::BSIStateManagerModifierBSIStateManagerStateListener(class) => {
                class.serialize(serializer)
            }
            Classes::BSiStateTaggingGenerator(class) => class.serialize(serializer),
            Classes::BSLimbIKModifier(class) => class.serialize(serializer),
            Classes::BSLookAtModifier(class) => class.serialize(serializer),
            Classes::BSLookAtModifierBoneData(class) => class.serialize(serializer),
            Classes::BSModifyOnceModifier(class) => class.serialize(serializer),
            Classes::BSOffsetAnimationGenerator(class) => class.serialize(serializer),
            Classes::BSPassByTargetTriggerModifier(class) => class.serialize(serializer),
            Classes::BSRagdollContactListenerModifier(class) => {
                class.serialize(serializer)
            }
            Classes::BSSpeedSamplerModifier(class) => class.serialize(serializer),
            Classes::BSSynchronizedClipGenerator(class) => class.serialize(serializer),
            Classes::BSTimerModifier(class) => class.serialize(serializer),
            Classes::BSTweenerModifier(class) => class.serialize(serializer),
            Classes::hkAabb(class) => class.serialize(serializer),
            Classes::hkAabbHalf(class) => class.serialize(serializer),
            Classes::hkAabbUint32(class) => class.serialize(serializer),
            Classes::hkaAnimatedReferenceFrame(class) => class.serialize(serializer),
            Classes::hkaAnimation(class) => class.serialize(serializer),
            Classes::hkaAnimationBinding(class) => class.serialize(serializer),
            Classes::hkaAnimationContainer(class) => class.serialize(serializer),
            Classes::hkaAnimationPreviewColorContainer(class) => {
                class.serialize(serializer)
            }
            Classes::hkaAnnotationTrack(class) => class.serialize(serializer),
            Classes::hkaAnnotationTrackAnnotation(class) => class.serialize(serializer),
            Classes::hkaBone(class) => class.serialize(serializer),
            Classes::hkaBoneAttachment(class) => class.serialize(serializer),
            Classes::hkaDefaultAnimatedReferenceFrame(class) => {
                class.serialize(serializer)
            }
            Classes::hkaDeltaCompressedAnimation(class) => class.serialize(serializer),
            Classes::hkaDeltaCompressedAnimationQuantizationFormat(class) => {
                class.serialize(serializer)
            }
            Classes::hkaFootstepAnalysisInfo(class) => class.serialize(serializer),
            Classes::hkaFootstepAnalysisInfoContainer(class) => {
                class.serialize(serializer)
            }
            Classes::hkaInterleavedUncompressedAnimation(class) => {
                class.serialize(serializer)
            }
            Classes::hkaKeyFrameHierarchyUtility(class) => class.serialize(serializer),
            Classes::hkaKeyFrameHierarchyUtilityControlData(class) => {
                class.serialize(serializer)
            }
            Classes::hkAlignSceneToNodeOptions(class) => class.serialize(serializer),
            Classes::hkaMeshBinding(class) => class.serialize(serializer),
            Classes::hkaMeshBindingMapping(class) => class.serialize(serializer),
            Classes::hkaQuantizedAnimation(class) => class.serialize(serializer),
            Classes::hkaQuantizedAnimationTrackCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaRagdollInstance(class) => class.serialize(serializer),
            Classes::hkArrayTypeAttribute(class) => class.serialize(serializer),
            Classes::hkaSkeleton(class) => class.serialize(serializer),
            Classes::hkaSkeletonLocalFrameOnBone(class) => class.serialize(serializer),
            Classes::hkaSkeletonMapper(class) => class.serialize(serializer),
            Classes::hkaSkeletonMapperData(class) => class.serialize(serializer),
            Classes::hkaSkeletonMapperDataChainMapping(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSkeletonMapperDataSimpleMapping(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSplineCompressedAnimation(class) => class.serialize(serializer),
            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaSplineCompressedAnimationTrackCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaWaveletCompressedAnimation(class) => class.serialize(serializer),
            Classes::hkaWaveletCompressedAnimationCompressionParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkaWaveletCompressedAnimationQuantizationFormat(class) => {
                class.serialize(serializer)
            }
            Classes::hkBaseObject(class) => class.serialize(serializer),
            Classes::hkbAttachmentModifier(class) => class.serialize(serializer),
            Classes::hkbAttachmentSetup(class) => class.serialize(serializer),
            Classes::hkbAttributeModifier(class) => class.serialize(serializer),
            Classes::hkbAttributeModifierAssignment(class) => class.serialize(serializer),
            Classes::hkbAuxiliaryNodeInfo(class) => class.serialize(serializer),
            Classes::hkbBehaviorEventsInfo(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraph(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraphData(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraphInternalState(class) => class.serialize(serializer),
            Classes::hkbBehaviorGraphInternalStateInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBehaviorGraphStringData(class) => class.serialize(serializer),
            Classes::hkbBehaviorInfo(class) => class.serialize(serializer),
            Classes::hkbBehaviorInfoIdToNamePair(class) => class.serialize(serializer),
            Classes::hkbBehaviorReferenceGenerator(class) => class.serialize(serializer),
            Classes::hkbBindable(class) => class.serialize(serializer),
            Classes::hkbBlendCurveUtils(class) => class.serialize(serializer),
            Classes::hkbBlenderGenerator(class) => class.serialize(serializer),
            Classes::hkbBlenderGeneratorChild(class) => class.serialize(serializer),
            Classes::hkbBlenderGeneratorChildInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBlenderGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBlendingTransitionEffect(class) => class.serialize(serializer),
            Classes::hkbBlendingTransitionEffectInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbBoneIndexArray(class) => class.serialize(serializer),
            Classes::hkbBoneWeightArray(class) => class.serialize(serializer),
            Classes::hkbBoolVariableSequencedData(class) => class.serialize(serializer),
            Classes::hkbBoolVariableSequencedDataSample(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCameraShakeEventPayload(class) => class.serialize(serializer),
            Classes::hkbCharacter(class) => class.serialize(serializer),
            Classes::hkbCharacterAddedInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterControlCommand(class) => class.serialize(serializer),
            Classes::hkbCharacterControllerControlData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCharacterControllerModifier(class) => class.serialize(serializer),
            Classes::hkbCharacterControllerModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCharacterData(class) => class.serialize(serializer),
            Classes::hkbCharacterDataCharacterControllerInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCharacterInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterSetup(class) => class.serialize(serializer),
            Classes::hkbCharacterSkinInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterSteppedInfo(class) => class.serialize(serializer),
            Classes::hkbCharacterStringData(class) => class.serialize(serializer),
            Classes::hkbClientCharacterState(class) => class.serialize(serializer),
            Classes::hkbClipGenerator(class) => class.serialize(serializer),
            Classes::hkbClipGeneratorEcho(class) => class.serialize(serializer),
            Classes::hkbClipGeneratorInternalState(class) => class.serialize(serializer),
            Classes::hkbClipTrigger(class) => class.serialize(serializer),
            Classes::hkbClipTriggerArray(class) => class.serialize(serializer),
            Classes::hkbCombineTransformsModifier(class) => class.serialize(serializer),
            Classes::hkbCombineTransformsModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCompiledExpressionSet(class) => class.serialize(serializer),
            Classes::hkbCompiledExpressionSetToken(class) => class.serialize(serializer),
            Classes::hkbComputeDirectionModifier(class) => class.serialize(serializer),
            Classes::hkbComputeDirectionModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationFromAxisAngleModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationToTargetModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbComputeRotationToTargetModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbCondition(class) => class.serialize(serializer),
            Classes::hkbContext(class) => class.serialize(serializer),
            Classes::hkbDampingModifier(class) => class.serialize(serializer),
            Classes::hkbDampingModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbDefaultMessageLog(class) => class.serialize(serializer),
            Classes::hkbDelayedModifier(class) => class.serialize(serializer),
            Classes::hkbDelayedModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbDetectCloseToGroundModifier(class) => class.serialize(serializer),
            Classes::hkbDetectCloseToGroundModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEvaluateExpressionModifier(class) => class.serialize(serializer),
            Classes::hkbEvaluateExpressionModifierInternalExpressionData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEvaluateExpressionModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEvaluateHandleModifier(class) => class.serialize(serializer),
            Classes::hkbEvent(class) => class.serialize(serializer),
            Classes::hkbEventBase(class) => class.serialize(serializer),
            Classes::hkbEventDrivenModifier(class) => class.serialize(serializer),
            Classes::hkbEventDrivenModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEventInfo(class) => class.serialize(serializer),
            Classes::hkbEventPayload(class) => class.serialize(serializer),
            Classes::hkbEventPayloadList(class) => class.serialize(serializer),
            Classes::hkbEventProperty(class) => class.serialize(serializer),
            Classes::hkbEventRaisedInfo(class) => class.serialize(serializer),
            Classes::hkbEventRangeData(class) => class.serialize(serializer),
            Classes::hkbEventRangeDataArray(class) => class.serialize(serializer),
            Classes::hkbEventSequencedData(class) => class.serialize(serializer),
            Classes::hkbEventSequencedDataSequencedEvent(class) => {
                class.serialize(serializer)
            }
            Classes::hkbEventsFromRangeModifier(class) => class.serialize(serializer),
            Classes::hkbEventsFromRangeModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbExpressionCondition(class) => class.serialize(serializer),
            Classes::hkbExpressionData(class) => class.serialize(serializer),
            Classes::hkbExpressionDataArray(class) => class.serialize(serializer),
            Classes::hkbExtractRagdollPoseModifier(class) => class.serialize(serializer),
            Classes::hkbFootIkControlData(class) => class.serialize(serializer),
            Classes::hkbFootIkControlsModifier(class) => class.serialize(serializer),
            Classes::hkbFootIkControlsModifierLeg(class) => class.serialize(serializer),
            Classes::hkbFootIkDriverInfo(class) => class.serialize(serializer),
            Classes::hkbFootIkDriverInfoLeg(class) => class.serialize(serializer),
            Classes::hkbFootIkGains(class) => class.serialize(serializer),
            Classes::hkbFootIkModifier(class) => class.serialize(serializer),
            Classes::hkbFootIkModifierInternalLegData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbFootIkModifierLeg(class) => class.serialize(serializer),
            Classes::hkbGenerator(class) => class.serialize(serializer),
            Classes::hkbGeneratorOutputListener(class) => class.serialize(serializer),
            Classes::hkbGeneratorSyncInfo(class) => class.serialize(serializer),
            Classes::hkbGeneratorSyncInfoSyncPoint(class) => class.serialize(serializer),
            Classes::hkbGeneratorTransitionEffect(class) => class.serialize(serializer),
            Classes::hkbGeneratorTransitionEffectInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbGetHandleOnBoneModifier(class) => class.serialize(serializer),
            Classes::hkbGetUpModifier(class) => class.serialize(serializer),
            Classes::hkbGetUpModifierInternalState(class) => class.serialize(serializer),
            Classes::hkbGetWorldFromModelModifier(class) => class.serialize(serializer),
            Classes::hkbGetWorldFromModelModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbHandIkControlData(class) => class.serialize(serializer),
            Classes::hkbHandIkControlsModifier(class) => class.serialize(serializer),
            Classes::hkbHandIkControlsModifierHand(class) => class.serialize(serializer),
            Classes::hkbHandIkDriverInfo(class) => class.serialize(serializer),
            Classes::hkbHandIkDriverInfoHand(class) => class.serialize(serializer),
            Classes::hkbHandIkModifier(class) => class.serialize(serializer),
            Classes::hkbHandIkModifierHand(class) => class.serialize(serializer),
            Classes::hkbHandle(class) => class.serialize(serializer),
            Classes::hkbIntEventPayload(class) => class.serialize(serializer),
            Classes::hkbIntVariableSequencedData(class) => class.serialize(serializer),
            Classes::hkbIntVariableSequencedDataSample(class) => {
                class.serialize(serializer)
            }
            Classes::hkBitField(class) => class.serialize(serializer),
            Classes::hkbKeyframeBonesModifier(class) => class.serialize(serializer),
            Classes::hkbKeyframeBonesModifierKeyframeInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbLinkedSymbolInfo(class) => class.serialize(serializer),
            Classes::hkbLookAtModifier(class) => class.serialize(serializer),
            Classes::hkbLookAtModifierInternalState(class) => class.serialize(serializer),
            Classes::hkbManualSelectorGenerator(class) => class.serialize(serializer),
            Classes::hkbManualSelectorGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbMessageLog(class) => class.serialize(serializer),
            Classes::hkbMirroredSkeletonInfo(class) => class.serialize(serializer),
            Classes::hkbMirrorModifier(class) => class.serialize(serializer),
            Classes::hkbModifier(class) => class.serialize(serializer),
            Classes::hkbModifierGenerator(class) => class.serialize(serializer),
            Classes::hkbModifierList(class) => class.serialize(serializer),
            Classes::hkbModifierWrapper(class) => class.serialize(serializer),
            Classes::hkbMoveCharacterModifier(class) => class.serialize(serializer),
            Classes::hkbMoveCharacterModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbNamedEventPayload(class) => class.serialize(serializer),
            Classes::hkbNamedIntEventPayload(class) => class.serialize(serializer),
            Classes::hkbNamedRealEventPayload(class) => class.serialize(serializer),
            Classes::hkbNamedStringEventPayload(class) => class.serialize(serializer),
            Classes::hkbNode(class) => class.serialize(serializer),
            Classes::hkbNodeInternalStateInfo(class) => class.serialize(serializer),
            Classes::hkbParticleSystemEventPayload(class) => class.serialize(serializer),
            Classes::hkbPoseMatchingGenerator(class) => class.serialize(serializer),
            Classes::hkbPoseMatchingGeneratorInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbPoweredRagdollControlData(class) => class.serialize(serializer),
            Classes::hkbPoweredRagdollControlsModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbProjectData(class) => class.serialize(serializer),
            Classes::hkbProjectStringData(class) => class.serialize(serializer),
            Classes::hkbProxyModifier(class) => class.serialize(serializer),
            Classes::hkbProxyModifierProxyInfo(class) => class.serialize(serializer),
            Classes::hkbRaiseEventCommand(class) => class.serialize(serializer),
            Classes::hkbRealEventPayload(class) => class.serialize(serializer),
            Classes::hkbRealVariableSequencedData(class) => class.serialize(serializer),
            Classes::hkbRealVariableSequencedDataSample(class) => {
                class.serialize(serializer)
            }
            Classes::hkbReferencePoseGenerator(class) => class.serialize(serializer),
            Classes::hkbRegisteredGenerator(class) => class.serialize(serializer),
            Classes::hkbRigidBodyRagdollControlData(class) => class.serialize(serializer),
            Classes::hkbRigidBodyRagdollControlsModifier(class) => {
                class.serialize(serializer)
            }
            Classes::hkbRoleAttribute(class) => class.serialize(serializer),
            Classes::hkbRotateCharacterModifier(class) => class.serialize(serializer),
            Classes::hkbRotateCharacterModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbSenseHandleModifier(class) => class.serialize(serializer),
            Classes::hkbSenseHandleModifierRange(class) => class.serialize(serializer),
            Classes::hkbSequence(class) => class.serialize(serializer),
            Classes::hkbSequencedData(class) => class.serialize(serializer),
            Classes::hkbSequenceInternalState(class) => class.serialize(serializer),
            Classes::hkbSequenceStringData(class) => class.serialize(serializer),
            Classes::hkbSetBehaviorCommand(class) => class.serialize(serializer),
            Classes::hkbSetLocalTimeOfClipGeneratorCommand(class) => {
                class.serialize(serializer)
            }
            Classes::hkbSetNodePropertyCommand(class) => class.serialize(serializer),
            Classes::hkbSetWordVariableCommand(class) => class.serialize(serializer),
            Classes::hkbSetWorldFromModelModifier(class) => class.serialize(serializer),
            Classes::hkbSimulationControlCommand(class) => class.serialize(serializer),
            Classes::hkbSimulationStateInfo(class) => class.serialize(serializer),
            Classes::hkbStateChooser(class) => class.serialize(serializer),
            Classes::hkbStateListener(class) => class.serialize(serializer),
            Classes::hkbStateMachine(class) => class.serialize(serializer),
            Classes::hkbStateMachineActiveTransitionInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineDelayedTransitionInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineEventPropertyArray(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineInternalState(class) => class.serialize(serializer),
            Classes::hkbStateMachineNestedStateMachineData(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineProspectiveTransitionInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineStateInfo(class) => class.serialize(serializer),
            Classes::hkbStateMachineTimeInterval(class) => class.serialize(serializer),
            Classes::hkbStateMachineTransitionInfo(class) => class.serialize(serializer),
            Classes::hkbStateMachineTransitionInfoArray(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStateMachineTransitionInfoReference(class) => {
                class.serialize(serializer)
            }
            Classes::hkbStringCondition(class) => class.serialize(serializer),
            Classes::hkbStringEventPayload(class) => class.serialize(serializer),
            Classes::hkbTestStateChooser(class) => class.serialize(serializer),
            Classes::hkbTimerModifier(class) => class.serialize(serializer),
            Classes::hkbTimerModifierInternalState(class) => class.serialize(serializer),
            Classes::hkbTransformVectorModifier(class) => class.serialize(serializer),
            Classes::hkbTransformVectorModifierInternalState(class) => {
                class.serialize(serializer)
            }
            Classes::hkbTransitionEffect(class) => class.serialize(serializer),
            Classes::hkbTwistModifier(class) => class.serialize(serializer),
            Classes::hkbVariableBindingSet(class) => class.serialize(serializer),
            Classes::hkbVariableBindingSetBinding(class) => class.serialize(serializer),
            Classes::hkbVariableInfo(class) => class.serialize(serializer),
            Classes::hkbVariableValue(class) => class.serialize(serializer),
            Classes::hkbVariableValueSet(class) => class.serialize(serializer),
            Classes::hkbWorldEnums(class) => class.serialize(serializer),
            Classes::hkbWorldFromModelModeData(class) => class.serialize(serializer),
            Classes::hkClass(class) => class.serialize(serializer),
            Classes::hkClassEnum(class) => class.serialize(serializer),
            Classes::hkClassEnumItem(class) => class.serialize(serializer),
            Classes::hkClassMember(class) => class.serialize(serializer),
            Classes::hkColor(class) => class.serialize(serializer),
            Classes::hkContactPoint(class) => class.serialize(serializer),
            Classes::hkContactPointMaterial(class) => class.serialize(serializer),
            Classes::hkCustomAttributes(class) => class.serialize(serializer),
            Classes::hkCustomAttributesAttribute(class) => class.serialize(serializer),
            Classes::hkDataObjectTypeAttribute(class) => class.serialize(serializer),
            Classes::hkDescriptionAttribute(class) => class.serialize(serializer),
            Classes::hkDocumentationAttribute(class) => class.serialize(serializer),
            Classes::hkGeometry(class) => class.serialize(serializer),
            Classes::hkGeometryTriangle(class) => class.serialize(serializer),
            Classes::hkGizmoAttribute(class) => class.serialize(serializer),
            Classes::hkHalf8(class) => class.serialize(serializer),
            Classes::hkIndexedTransformSet(class) => class.serialize(serializer),
            Classes::hkLinkAttribute(class) => class.serialize(serializer),
            Classes::hkLocalFrame(class) => class.serialize(serializer),
            Classes::hkLocalFrameGroup(class) => class.serialize(serializer),
            Classes::hkMemoryMeshBody(class) => class.serialize(serializer),
            Classes::hkMemoryMeshMaterial(class) => class.serialize(serializer),
            Classes::hkMemoryMeshShape(class) => class.serialize(serializer),
            Classes::hkMemoryMeshTexture(class) => class.serialize(serializer),
            Classes::hkMemoryMeshVertexBuffer(class) => class.serialize(serializer),
            Classes::hkMemoryResourceContainer(class) => class.serialize(serializer),
            Classes::hkMemoryResourceHandle(class) => class.serialize(serializer),
            Classes::hkMemoryResourceHandleExternalLink(class) => {
                class.serialize(serializer)
            }
            Classes::hkMemoryTrackerAttribute(class) => class.serialize(serializer),
            Classes::hkMeshBody(class) => class.serialize(serializer),
            Classes::hkMeshBoneIndexMapping(class) => class.serialize(serializer),
            Classes::hkMeshMaterial(class) => class.serialize(serializer),
            Classes::hkMeshSection(class) => class.serialize(serializer),
            Classes::hkMeshSectionCinfo(class) => class.serialize(serializer),
            Classes::hkMeshShape(class) => class.serialize(serializer),
            Classes::hkMeshTexture(class) => class.serialize(serializer),
            Classes::hkMeshVertexBuffer(class) => class.serialize(serializer),
            Classes::hkModelerNodeTypeAttribute(class) => class.serialize(serializer),
            Classes::hkMonitorStreamColorTable(class) => class.serialize(serializer),
            Classes::hkMonitorStreamColorTableColorPair(class) => {
                class.serialize(serializer)
            }
            Classes::hkMonitorStreamFrameInfo(class) => class.serialize(serializer),
            Classes::hkMonitorStreamStringMap(class) => class.serialize(serializer),
            Classes::hkMonitorStreamStringMapStringMap(class) => {
                class.serialize(serializer)
            }
            Classes::hkMoppBvTreeShapeBase(class) => class.serialize(serializer),
            Classes::hkMotionState(class) => class.serialize(serializer),
            Classes::hkMultipleVertexBuffer(class) => class.serialize(serializer),
            Classes::hkMultipleVertexBufferElementInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkMultipleVertexBufferLockedElement(class) => {
                class.serialize(serializer)
            }
            Classes::hkMultipleVertexBufferVertexBufferInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkMultiThreadCheck(class) => class.serialize(serializer),
            Classes::hkp2dAngConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAabbPhantom(class) => class.serialize(serializer),
            Classes::hkPackedVector3(class) => class.serialize(serializer),
            Classes::hkPackfileHeader(class) => class.serialize(serializer),
            Classes::hkPackfileSectionHeader(class) => class.serialize(serializer),
            Classes::hkpAction(class) => class.serialize(serializer),
            Classes::hkpAgent1nSector(class) => class.serialize(serializer),
            Classes::hkpAngConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngFrictionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngMotorConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpAngularDashpotAction(class) => class.serialize(serializer),
            Classes::hkpArrayAction(class) => class.serialize(serializer),
            Classes::hkpBallAndSocketConstraintData(class) => class.serialize(serializer),
            Classes::hkpBallAndSocketConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpBallGun(class) => class.serialize(serializer),
            Classes::hkpBallSocketChainData(class) => class.serialize(serializer),
            Classes::hkpBallSocketChainDataConstraintInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkpBallSocketConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpBinaryAction(class) => class.serialize(serializer),
            Classes::hkpBoxMotion(class) => class.serialize(serializer),
            Classes::hkpBoxShape(class) => class.serialize(serializer),
            Classes::hkpBreakableBody(class) => class.serialize(serializer),
            Classes::hkpBreakableConstraintData(class) => class.serialize(serializer),
            Classes::hkpBridgeAtoms(class) => class.serialize(serializer),
            Classes::hkpBridgeConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpBroadPhaseHandle(class) => class.serialize(serializer),
            Classes::hkpBvShape(class) => class.serialize(serializer),
            Classes::hkpBvTreeShape(class) => class.serialize(serializer),
            Classes::hkpCachingShapePhantom(class) => class.serialize(serializer),
            Classes::hkpCallbackConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpCapsuleShape(class) => class.serialize(serializer),
            Classes::hkpCdBody(class) => class.serialize(serializer),
            Classes::hkpCenterOfMassChangerModifierConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpCharacterControllerCinfo(class) => class.serialize(serializer),
            Classes::hkpCharacterMotion(class) => class.serialize(serializer),
            Classes::hkpCharacterProxyCinfo(class) => class.serialize(serializer),
            Classes::hkpCharacterRigidBodyCinfo(class) => class.serialize(serializer),
            Classes::hkpCogWheelConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpCogWheelConstraintData(class) => class.serialize(serializer),
            Classes::hkpCogWheelConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpCollidable(class) => class.serialize(serializer),
            Classes::hkpCollidableBoundingVolumeData(class) => {
                class.serialize(serializer)
            }
            Classes::hkpCollidableCollidableFilter(class) => class.serialize(serializer),
            Classes::hkpCollisionFilter(class) => class.serialize(serializer),
            Classes::hkpCollisionFilterList(class) => class.serialize(serializer),
            Classes::hkpCompressedMeshShape(class) => class.serialize(serializer),
            Classes::hkpCompressedMeshShapeBigTriangle(class) => {
                class.serialize(serializer)
            }
            Classes::hkpCompressedMeshShapeChunk(class) => class.serialize(serializer),
            Classes::hkpCompressedMeshShapeConvexPiece(class) => {
                class.serialize(serializer)
            }
            Classes::hkpCompressedSampledHeightFieldShape(class) => {
                class.serialize(serializer)
            }
            Classes::hkpConeLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpConstrainedSystemFilter(class) => class.serialize(serializer),
            Classes::hkpConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpConstraintChainData(class) => class.serialize(serializer),
            Classes::hkpConstraintChainInstance(class) => class.serialize(serializer),
            Classes::hkpConstraintChainInstanceAction(class) => {
                class.serialize(serializer)
            }
            Classes::hkpConstraintCollisionFilter(class) => class.serialize(serializer),
            Classes::hkpConstraintData(class) => class.serialize(serializer),
            Classes::hkpConstraintInstance(class) => class.serialize(serializer),
            Classes::hkpConstraintInstanceSmallArraySerializeOverrideType(class) => {
                class.serialize(serializer)
            }
            Classes::hkpConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpConvexListFilter(class) => class.serialize(serializer),
            Classes::hkpConvexListShape(class) => class.serialize(serializer),
            Classes::hkpConvexPieceMeshShape(class) => class.serialize(serializer),
            Classes::hkpConvexPieceStreamData(class) => class.serialize(serializer),
            Classes::hkpConvexShape(class) => class.serialize(serializer),
            Classes::hkpConvexTransformShape(class) => class.serialize(serializer),
            Classes::hkpConvexTransformShapeBase(class) => class.serialize(serializer),
            Classes::hkpConvexTranslateShape(class) => class.serialize(serializer),
            Classes::hkpConvexVerticesConnectivity(class) => class.serialize(serializer),
            Classes::hkpConvexVerticesShape(class) => class.serialize(serializer),
            Classes::hkpConvexVerticesShapeFourVectors(class) => {
                class.serialize(serializer)
            }
            Classes::hkpCylinderShape(class) => class.serialize(serializer),
            Classes::hkpDashpotAction(class) => class.serialize(serializer),
            Classes::hkpDefaultConvexListFilter(class) => class.serialize(serializer),
            Classes::hkpDefaultWorldMemoryWatchDog(class) => class.serialize(serializer),
            Classes::hkpDisableEntityCollisionFilter(class) => {
                class.serialize(serializer)
            }
            Classes::hkpDisplayBindingData(class) => class.serialize(serializer),
            Classes::hkpDisplayBindingDataPhysicsSystem(class) => {
                class.serialize(serializer)
            }
            Classes::hkpDisplayBindingDataRigidBody(class) => class.serialize(serializer),
            Classes::hkpEntity(class) => class.serialize(serializer),
            Classes::hkpEntityExtendedListeners(class) => class.serialize(serializer),
            Classes::hkpEntitySmallArraySerializeOverrideType(class) => {
                class.serialize(serializer)
            }
            Classes::hkpEntitySpuCollisionCallback(class) => class.serialize(serializer),
            Classes::hkpExtendedMeshShape(class) => class.serialize(serializer),
            Classes::hkpExtendedMeshShapeShapesSubpart(class) => {
                class.serialize(serializer)
            }
            Classes::hkpExtendedMeshShapeSubpart(class) => class.serialize(serializer),
            Classes::hkpExtendedMeshShapeTrianglesSubpart(class) => {
                class.serialize(serializer)
            }
            Classes::hkpFastMeshShape(class) => class.serialize(serializer),
            Classes::hkpFirstPersonGun(class) => class.serialize(serializer),
            Classes::hkpFixedRigidMotion(class) => class.serialize(serializer),
            Classes::hkpGenericConstraintData(class) => class.serialize(serializer),
            Classes::hkpGenericConstraintDataScheme(class) => class.serialize(serializer),
            Classes::hkpGenericConstraintDataSchemeConstraintInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkpGravityGun(class) => class.serialize(serializer),
            Classes::hkpGroupCollisionFilter(class) => class.serialize(serializer),
            Classes::hkpGroupFilter(class) => class.serialize(serializer),
            Classes::hkpHeightFieldShape(class) => class.serialize(serializer),
            Classes::hkpHingeConstraintData(class) => class.serialize(serializer),
            Classes::hkpHingeConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpHingeLimitsData(class) => class.serialize(serializer),
            Classes::hkpHingeLimitsDataAtoms(class) => class.serialize(serializer),
            Classes::hkpIgnoreModifierConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpKeyframedRigidMotion(class) => class.serialize(serializer),
            Classes::hkpLimitedForceConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpLimitedHingeConstraintData(class) => class.serialize(serializer),
            Classes::hkpLimitedHingeConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpLinConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinearParametricCurve(class) => class.serialize(serializer),
            Classes::hkpLinFrictionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinkedCollidable(class) => class.serialize(serializer),
            Classes::hkpLinLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinMotorConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpLinSoftConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpListShape(class) => class.serialize(serializer),
            Classes::hkpListShapeChildInfo(class) => class.serialize(serializer),
            Classes::hkpMalleableConstraintData(class) => class.serialize(serializer),
            Classes::hkpMassChangerModifierConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpMassProperties(class) => class.serialize(serializer),
            Classes::hkpMaterial(class) => class.serialize(serializer),
            Classes::hkpMaxSizeMotion(class) => class.serialize(serializer),
            Classes::hkpMeshMaterial(class) => class.serialize(serializer),
            Classes::hkpMeshShape(class) => class.serialize(serializer),
            Classes::hkpMeshShapeSubpart(class) => class.serialize(serializer),
            Classes::hkpModifierConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpMoppBvTreeShape(class) => class.serialize(serializer),
            Classes::hkpMoppCode(class) => class.serialize(serializer),
            Classes::hkpMoppCodeCodeInfo(class) => class.serialize(serializer),
            Classes::hkpMoppCodeReindexedTerminal(class) => class.serialize(serializer),
            Classes::hkpMotion(class) => class.serialize(serializer),
            Classes::hkpMotorAction(class) => class.serialize(serializer),
            Classes::hkpMountedBallGun(class) => class.serialize(serializer),
            Classes::hkpMouseSpringAction(class) => class.serialize(serializer),
            Classes::hkpMovingSurfaceModifierConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpMultiRayShape(class) => class.serialize(serializer),
            Classes::hkpMultiRayShapeRay(class) => class.serialize(serializer),
            Classes::hkpMultiSphereShape(class) => class.serialize(serializer),
            Classes::hkpMultithreadedVehicleManager(class) => class.serialize(serializer),
            Classes::hkpNamedMeshMaterial(class) => class.serialize(serializer),
            Classes::hkpNullCollisionFilter(class) => class.serialize(serializer),
            Classes::hkPostFinishAttribute(class) => class.serialize(serializer),
            Classes::hkpOverwritePivotConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpPairCollisionFilter(class) => class.serialize(serializer),
            Classes::hkpPairCollisionFilterMapPairFilterKeyOverrideType(class) => {
                class.serialize(serializer)
            }
            Classes::hkpParametricCurve(class) => class.serialize(serializer),
            Classes::hkpPhantom(class) => class.serialize(serializer),
            Classes::hkpPhantomCallbackShape(class) => class.serialize(serializer),
            Classes::hkpPhysicsData(class) => class.serialize(serializer),
            Classes::hkpPhysicsSystem(class) => class.serialize(serializer),
            Classes::hkpPhysicsSystemWithContacts(class) => class.serialize(serializer),
            Classes::hkpPlaneShape(class) => class.serialize(serializer),
            Classes::hkpPointToPathConstraintData(class) => class.serialize(serializer),
            Classes::hkpPointToPlaneConstraintData(class) => class.serialize(serializer),
            Classes::hkpPointToPlaneConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpPositionConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpPoweredChainData(class) => class.serialize(serializer),
            Classes::hkpPoweredChainDataConstraintInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkpPoweredChainMapper(class) => class.serialize(serializer),
            Classes::hkpPoweredChainMapperLinkInfo(class) => class.serialize(serializer),
            Classes::hkpPoweredChainMapperTarget(class) => class.serialize(serializer),
            Classes::hkpPrismaticConstraintData(class) => class.serialize(serializer),
            Classes::hkpPrismaticConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpProjectileGun(class) => class.serialize(serializer),
            Classes::hkpProperty(class) => class.serialize(serializer),
            Classes::hkpPropertyValue(class) => class.serialize(serializer),
            Classes::hkpPulleyConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpPulleyConstraintData(class) => class.serialize(serializer),
            Classes::hkpPulleyConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpRackAndPinionConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpRackAndPinionConstraintData(class) => class.serialize(serializer),
            Classes::hkpRackAndPinionConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpRagdollConstraintData(class) => class.serialize(serializer),
            Classes::hkpRagdollConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpRagdollLimitsData(class) => class.serialize(serializer),
            Classes::hkpRagdollLimitsDataAtoms(class) => class.serialize(serializer),
            Classes::hkpRagdollMotorConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpRayCollidableFilter(class) => class.serialize(serializer),
            Classes::hkpRayShapeCollectionFilter(class) => class.serialize(serializer),
            Classes::hkpRejectChassisListener(class) => class.serialize(serializer),
            Classes::hkpRemoveTerminalsMoppModifier(class) => class.serialize(serializer),
            Classes::hkpReorientAction(class) => class.serialize(serializer),
            Classes::hkpRigidBody(class) => class.serialize(serializer),
            Classes::hkpRotationalConstraintData(class) => class.serialize(serializer),
            Classes::hkpRotationalConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSampledHeightFieldShape(class) => class.serialize(serializer),
            Classes::hkpSerializedAgentNnEntry(class) => class.serialize(serializer),
            Classes::hkpSerializedDisplayMarker(class) => class.serialize(serializer),
            Classes::hkpSerializedDisplayMarkerList(class) => class.serialize(serializer),
            Classes::hkpSerializedDisplayRbTransforms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSerializedDisplayRbTransformsDisplayTransformPair(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSerializedSubTrack1nInfo(class) => class.serialize(serializer),
            Classes::hkpSerializedTrack1nInfo(class) => class.serialize(serializer),
            Classes::hkpSetLocalRotationsConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetLocalTransformsConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetLocalTranslationsConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSetupStabilizationAtom(class) => class.serialize(serializer),
            Classes::hkpShape(class) => class.serialize(serializer),
            Classes::hkpShapeCollection(class) => class.serialize(serializer),
            Classes::hkpShapeCollectionFilter(class) => class.serialize(serializer),
            Classes::hkpShapeContainer(class) => class.serialize(serializer),
            Classes::hkpShapeInfo(class) => class.serialize(serializer),
            Classes::hkpShapeModifier(class) => class.serialize(serializer),
            Classes::hkpShapePhantom(class) => class.serialize(serializer),
            Classes::hkpSimpleContactConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpSimpleContactConstraintDataInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSimpleMeshShape(class) => class.serialize(serializer),
            Classes::hkpSimpleMeshShapeTriangle(class) => class.serialize(serializer),
            Classes::hkpSimpleShapePhantom(class) => class.serialize(serializer),
            Classes::hkpSimpleShapePhantomCollisionDetail(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSimulation(class) => class.serialize(serializer),
            Classes::hkpSingleShapeContainer(class) => class.serialize(serializer),
            Classes::hkpSoftContactModifierConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpSphereMotion(class) => class.serialize(serializer),
            Classes::hkpSphereRepShape(class) => class.serialize(serializer),
            Classes::hkpSphereShape(class) => class.serialize(serializer),
            Classes::hkpSpringAction(class) => class.serialize(serializer),
            Classes::hkpSpringDamperConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpStiffSpringChainData(class) => class.serialize(serializer),
            Classes::hkpStiffSpringChainDataConstraintInfo(class) => {
                class.serialize(serializer)
            }
            Classes::hkpStiffSpringConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpStiffSpringConstraintData(class) => class.serialize(serializer),
            Classes::hkpStiffSpringConstraintDataAtoms(class) => {
                class.serialize(serializer)
            }
            Classes::hkpStorageExtendedMeshShape(class) => class.serialize(serializer),
            Classes::hkpStorageExtendedMeshShapeMaterial(class) => {
                class.serialize(serializer)
            }
            Classes::hkpStorageExtendedMeshShapeMeshSubpartStorage(class) => {
                class.serialize(serializer)
            }
            Classes::hkpStorageExtendedMeshShapeShapeSubpartStorage(class) => {
                class.serialize(serializer)
            }
            Classes::hkpStorageMeshShape(class) => class.serialize(serializer),
            Classes::hkpStorageMeshShapeSubpartStorage(class) => {
                class.serialize(serializer)
            }
            Classes::hkpStorageSampledHeightFieldShape(class) => {
                class.serialize(serializer)
            }
            Classes::hkpThinBoxMotion(class) => class.serialize(serializer),
            Classes::hkpTransformShape(class) => class.serialize(serializer),
            Classes::hkpTriangleShape(class) => class.serialize(serializer),
            Classes::hkpTriggerVolume(class) => class.serialize(serializer),
            Classes::hkpTriggerVolumeEventInfo(class) => class.serialize(serializer),
            Classes::hkpTriSampledHeightFieldBvTreeShape(class) => {
                class.serialize(serializer)
            }
            Classes::hkpTriSampledHeightFieldCollection(class) => {
                class.serialize(serializer)
            }
            Classes::hkpTwistLimitConstraintAtom(class) => class.serialize(serializer),
            Classes::hkpTypedBroadPhaseHandle(class) => class.serialize(serializer),
            Classes::hkpTyremarkPoint(class) => class.serialize(serializer),
            Classes::hkpTyremarksInfo(class) => class.serialize(serializer),
            Classes::hkpTyremarksWheel(class) => class.serialize(serializer),
            Classes::hkpUnaryAction(class) => class.serialize(serializer),
            Classes::hkpVehicleAerodynamics(class) => class.serialize(serializer),
            Classes::hkpVehicleBrake(class) => class.serialize(serializer),
            Classes::hkpVehicleCastBatchingManager(class) => class.serialize(serializer),
            Classes::hkpVehicleData(class) => class.serialize(serializer),
            Classes::hkpVehicleDataWheelComponentParams(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleDefaultAerodynamics(class) => class.serialize(serializer),
            Classes::hkpVehicleDefaultAnalogDriverInput(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleDefaultBrake(class) => class.serialize(serializer),
            Classes::hkpVehicleDefaultBrakeWheelBrakingProperties(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleDefaultEngine(class) => class.serialize(serializer),
            Classes::hkpVehicleDefaultSteering(class) => class.serialize(serializer),
            Classes::hkpVehicleDefaultSuspension(class) => class.serialize(serializer),
            Classes::hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                class,
            ) => class.serialize(serializer),
            Classes::hkpVehicleDefaultTransmission(class) => class.serialize(serializer),
            Classes::hkpVehicleDefaultVelocityDamper(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleDriverInput(class) => class.serialize(serializer),
            Classes::hkpVehicleDriverInputAnalogStatus(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleDriverInputStatus(class) => class.serialize(serializer),
            Classes::hkpVehicleEngine(class) => class.serialize(serializer),
            Classes::hkpVehicleFrictionDescription(class) => class.serialize(serializer),
            Classes::hkpVehicleFrictionDescriptionAxisDescription(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleFrictionStatus(class) => class.serialize(serializer),
            Classes::hkpVehicleFrictionStatusAxisStatus(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleInstance(class) => class.serialize(serializer),
            Classes::hkpVehicleInstanceWheelInfo(class) => class.serialize(serializer),
            Classes::hkpVehicleLinearCastBatchingManager(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleLinearCastWheelCollide(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleLinearCastWheelCollideWheelState(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleManager(class) => class.serialize(serializer),
            Classes::hkpVehicleRayCastBatchingManager(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleRayCastWheelCollide(class) => class.serialize(serializer),
            Classes::hkpVehicleSteering(class) => class.serialize(serializer),
            Classes::hkpVehicleSuspension(class) => class.serialize(serializer),
            Classes::hkpVehicleSuspensionSuspensionWheelParameters(class) => {
                class.serialize(serializer)
            }
            Classes::hkpVehicleTransmission(class) => class.serialize(serializer),
            Classes::hkpVehicleVelocityDamper(class) => class.serialize(serializer),
            Classes::hkpVehicleWheelCollide(class) => class.serialize(serializer),
            Classes::hkpVelocityConstraintMotor(class) => class.serialize(serializer),
            Classes::hkpViscousSurfaceModifierConstraintAtom(class) => {
                class.serialize(serializer)
            }
            Classes::hkpWeldingUtility(class) => class.serialize(serializer),
            Classes::hkpWheelConstraintData(class) => class.serialize(serializer),
            Classes::hkpWheelConstraintDataAtoms(class) => class.serialize(serializer),
            Classes::hkpWorld(class) => class.serialize(serializer),
            Classes::hkpWorldCinfo(class) => class.serialize(serializer),
            Classes::hkpWorldObject(class) => class.serialize(serializer),
            Classes::hkQTransform(class) => class.serialize(serializer),
            Classes::hkRangeInt32Attribute(class) => class.serialize(serializer),
            Classes::hkRangeRealAttribute(class) => class.serialize(serializer),
            Classes::hkReferencedObject(class) => class.serialize(serializer),
            Classes::hkReflectedFileAttribute(class) => class.serialize(serializer),
            Classes::hkResourceBase(class) => class.serialize(serializer),
            Classes::hkResourceContainer(class) => class.serialize(serializer),
            Classes::hkResourceHandle(class) => class.serialize(serializer),
            Classes::hkRootLevelContainer(class) => class.serialize(serializer),
            Classes::hkRootLevelContainerNamedVariant(class) => {
                class.serialize(serializer)
            }
            Classes::hkSemanticsAttribute(class) => class.serialize(serializer),
            Classes::hkSimpleLocalFrame(class) => class.serialize(serializer),
            Classes::hkSphere(class) => class.serialize(serializer),
            Classes::hkSweptTransform(class) => class.serialize(serializer),
            Classes::hkTraceStreamTitle(class) => class.serialize(serializer),
            Classes::hkTrackerSerializableScanSnapshot(class) => {
                class.serialize(serializer)
            }
            Classes::hkTrackerSerializableScanSnapshotAllocation(class) => {
                class.serialize(serializer)
            }
            Classes::hkTrackerSerializableScanSnapshotBlock(class) => {
                class.serialize(serializer)
            }
            Classes::hkUiAttribute(class) => class.serialize(serializer),
            Classes::hkVertexFormat(class) => class.serialize(serializer),
            Classes::hkVertexFormatElement(class) => class.serialize(serializer),
            Classes::hkWorldMemoryAvailableWatchDog(class) => class.serialize(serializer),
            Classes::hkxAnimatedFloat(class) => class.serialize(serializer),
            Classes::hkxAnimatedMatrix(class) => class.serialize(serializer),
            Classes::hkxAnimatedQuaternion(class) => class.serialize(serializer),
            Classes::hkxAnimatedVector(class) => class.serialize(serializer),
            Classes::hkxAttribute(class) => class.serialize(serializer),
            Classes::hkxAttributeGroup(class) => class.serialize(serializer),
            Classes::hkxAttributeHolder(class) => class.serialize(serializer),
            Classes::hkxCamera(class) => class.serialize(serializer),
            Classes::hkxEdgeSelectionChannel(class) => class.serialize(serializer),
            Classes::hkxEnum(class) => class.serialize(serializer),
            Classes::hkxEnumItem(class) => class.serialize(serializer),
            Classes::hkxEnvironment(class) => class.serialize(serializer),
            Classes::hkxEnvironmentVariable(class) => class.serialize(serializer),
            Classes::hkxIndexBuffer(class) => class.serialize(serializer),
            Classes::hkxLight(class) => class.serialize(serializer),
            Classes::hkxMaterial(class) => class.serialize(serializer),
            Classes::hkxMaterialEffect(class) => class.serialize(serializer),
            Classes::hkxMaterialProperty(class) => class.serialize(serializer),
            Classes::hkxMaterialShader(class) => class.serialize(serializer),
            Classes::hkxMaterialShaderSet(class) => class.serialize(serializer),
            Classes::hkxMaterialTextureStage(class) => class.serialize(serializer),
            Classes::hkxMesh(class) => class.serialize(serializer),
            Classes::hkxMeshSection(class) => class.serialize(serializer),
            Classes::hkxMeshUserChannelInfo(class) => class.serialize(serializer),
            Classes::hkxNode(class) => class.serialize(serializer),
            Classes::hkxNodeAnnotationData(class) => class.serialize(serializer),
            Classes::hkxNodeSelectionSet(class) => class.serialize(serializer),
            Classes::hkxScene(class) => class.serialize(serializer),
            Classes::hkxSkinBinding(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedBool(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedEnum(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedInt(class) => class.serialize(serializer),
            Classes::hkxSparselyAnimatedString(class) => class.serialize(serializer),
            Classes::hkxTextureFile(class) => class.serialize(serializer),
            Classes::hkxTextureInplace(class) => class.serialize(serializer),
            Classes::hkxTriangleSelectionChannel(class) => class.serialize(serializer),
            Classes::hkxVertexBuffer(class) => class.serialize(serializer),
            Classes::hkxVertexBufferVertexData(class) => class.serialize(serializer),
            Classes::hkxVertexDescription(class) => class.serialize(serializer),
            Classes::hkxVertexDescriptionElementDecl(class) => {
                class.serialize(serializer)
            }
            Classes::hkxVertexFloatDataChannel(class) => class.serialize(serializer),
            Classes::hkxVertexIntDataChannel(class) => class.serialize(serializer),
            Classes::hkxVertexSelectionChannel(class) => class.serialize(serializer),
            Classes::hkxVertexVectorDataChannel(class) => class.serialize(serializer),
        }
    }
}
impl<'a, 'de: 'a> _serde::Deserialize<'de> for Classes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: _serde::Deserializer<'de>,
    {
        struct ClassesVisitor<'a> {
            marker: core::marker::PhantomData<Classes<'a>>,
        }
        impl<'a, 'de: 'a> _serde::de::Visitor<'de> for ClassesVisitor<'a> {
            type Value = Classes<'a>;
            fn expecting(
                &self,
                formatter: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
                formatter.write_str("a valid class enum")
            }
            fn visit_class_index<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: _serde::de::ClassIndexAccess<'de>,
            {
                let class_name = map.next_key()?;
                match class_name {
                    "BGSGamebryoSequenceGenerator" => {
                        Ok(Classes::BGSGamebryoSequenceGenerator(map.next_value()?))
                    }
                    "BSBoneSwitchGenerator" => {
                        Ok(Classes::BSBoneSwitchGenerator(map.next_value()?))
                    }
                    "BSBoneSwitchGeneratorBoneData" => {
                        Ok(Classes::BSBoneSwitchGeneratorBoneData(map.next_value()?))
                    }
                    "BSComputeAddBoneAnimModifier" => {
                        Ok(Classes::BSComputeAddBoneAnimModifier(map.next_value()?))
                    }
                    "BSCyclicBlendTransitionGenerator" => {
                        Ok(Classes::BSCyclicBlendTransitionGenerator(map.next_value()?))
                    }
                    "BSDecomposeVectorModifier" => {
                        Ok(Classes::BSDecomposeVectorModifier(map.next_value()?))
                    }
                    "BSDirectAtModifier" => {
                        Ok(Classes::BSDirectAtModifier(map.next_value()?))
                    }
                    "BSDistTriggerModifier" => {
                        Ok(Classes::BSDistTriggerModifier(map.next_value()?))
                    }
                    "BSEventEveryNEventsModifier" => {
                        Ok(Classes::BSEventEveryNEventsModifier(map.next_value()?))
                    }
                    "BSEventOnDeactivateModifier" => {
                        Ok(Classes::BSEventOnDeactivateModifier(map.next_value()?))
                    }
                    "BSEventOnFalseToTrueModifier" => {
                        Ok(Classes::BSEventOnFalseToTrueModifier(map.next_value()?))
                    }
                    "BSGetTimeStepModifier" => {
                        Ok(Classes::BSGetTimeStepModifier(map.next_value()?))
                    }
                    "BSInterpValueModifier" => {
                        Ok(Classes::BSInterpValueModifier(map.next_value()?))
                    }
                    "BSIsActiveModifier" => {
                        Ok(Classes::BSIsActiveModifier(map.next_value()?))
                    }
                    "BSIStateManagerModifier" => {
                        Ok(Classes::BSIStateManagerModifier(map.next_value()?))
                    }
                    "BSIStateManagerModifierBSiStateData" => {
                        Ok(
                            Classes::BSIStateManagerModifierBSiStateData(
                                map.next_value()?,
                            ),
                        )
                    }
                    "BSIStateManagerModifierBSIStateManagerStateListener" => {
                        Ok(
                            Classes::BSIStateManagerModifierBSIStateManagerStateListener(
                                map.next_value()?,
                            ),
                        )
                    }
                    "BSiStateTaggingGenerator" => {
                        Ok(Classes::BSiStateTaggingGenerator(map.next_value()?))
                    }
                    "BSLimbIKModifier" => {
                        Ok(Classes::BSLimbIKModifier(map.next_value()?))
                    }
                    "BSLookAtModifier" => {
                        Ok(Classes::BSLookAtModifier(map.next_value()?))
                    }
                    "BSLookAtModifierBoneData" => {
                        Ok(Classes::BSLookAtModifierBoneData(map.next_value()?))
                    }
                    "BSModifyOnceModifier" => {
                        Ok(Classes::BSModifyOnceModifier(map.next_value()?))
                    }
                    "BSOffsetAnimationGenerator" => {
                        Ok(Classes::BSOffsetAnimationGenerator(map.next_value()?))
                    }
                    "BSPassByTargetTriggerModifier" => {
                        Ok(Classes::BSPassByTargetTriggerModifier(map.next_value()?))
                    }
                    "BSRagdollContactListenerModifier" => {
                        Ok(Classes::BSRagdollContactListenerModifier(map.next_value()?))
                    }
                    "BSSpeedSamplerModifier" => {
                        Ok(Classes::BSSpeedSamplerModifier(map.next_value()?))
                    }
                    "BSSynchronizedClipGenerator" => {
                        Ok(Classes::BSSynchronizedClipGenerator(map.next_value()?))
                    }
                    "BSTimerModifier" => Ok(Classes::BSTimerModifier(map.next_value()?)),
                    "BSTweenerModifier" => {
                        Ok(Classes::BSTweenerModifier(map.next_value()?))
                    }
                    "hkAabb" => Ok(Classes::hkAabb(map.next_value()?)),
                    "hkAabbHalf" => Ok(Classes::hkAabbHalf(map.next_value()?)),
                    "hkAabbUint32" => Ok(Classes::hkAabbUint32(map.next_value()?)),
                    "hkaAnimatedReferenceFrame" => {
                        Ok(Classes::hkaAnimatedReferenceFrame(map.next_value()?))
                    }
                    "hkaAnimation" => Ok(Classes::hkaAnimation(map.next_value()?)),
                    "hkaAnimationBinding" => {
                        Ok(Classes::hkaAnimationBinding(map.next_value()?))
                    }
                    "hkaAnimationContainer" => {
                        Ok(Classes::hkaAnimationContainer(map.next_value()?))
                    }
                    "hkaAnimationPreviewColorContainer" => {
                        Ok(Classes::hkaAnimationPreviewColorContainer(map.next_value()?))
                    }
                    "hkaAnnotationTrack" => {
                        Ok(Classes::hkaAnnotationTrack(map.next_value()?))
                    }
                    "hkaAnnotationTrackAnnotation" => {
                        Ok(Classes::hkaAnnotationTrackAnnotation(map.next_value()?))
                    }
                    "hkaBone" => Ok(Classes::hkaBone(map.next_value()?)),
                    "hkaBoneAttachment" => {
                        Ok(Classes::hkaBoneAttachment(map.next_value()?))
                    }
                    "hkaDefaultAnimatedReferenceFrame" => {
                        Ok(Classes::hkaDefaultAnimatedReferenceFrame(map.next_value()?))
                    }
                    "hkaDeltaCompressedAnimation" => {
                        Ok(Classes::hkaDeltaCompressedAnimation(map.next_value()?))
                    }
                    "hkaDeltaCompressedAnimationQuantizationFormat" => {
                        Ok(
                            Classes::hkaDeltaCompressedAnimationQuantizationFormat(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaFootstepAnalysisInfo" => {
                        Ok(Classes::hkaFootstepAnalysisInfo(map.next_value()?))
                    }
                    "hkaFootstepAnalysisInfoContainer" => {
                        Ok(Classes::hkaFootstepAnalysisInfoContainer(map.next_value()?))
                    }
                    "hkaInterleavedUncompressedAnimation" => {
                        Ok(
                            Classes::hkaInterleavedUncompressedAnimation(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaKeyFrameHierarchyUtility" => {
                        Ok(Classes::hkaKeyFrameHierarchyUtility(map.next_value()?))
                    }
                    "hkaKeyFrameHierarchyUtilityControlData" => {
                        Ok(
                            Classes::hkaKeyFrameHierarchyUtilityControlData(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkAlignSceneToNodeOptions" => {
                        Ok(Classes::hkAlignSceneToNodeOptions(map.next_value()?))
                    }
                    "hkaMeshBinding" => Ok(Classes::hkaMeshBinding(map.next_value()?)),
                    "hkaMeshBindingMapping" => {
                        Ok(Classes::hkaMeshBindingMapping(map.next_value()?))
                    }
                    "hkaQuantizedAnimation" => {
                        Ok(Classes::hkaQuantizedAnimation(map.next_value()?))
                    }
                    "hkaQuantizedAnimationTrackCompressionParams" => {
                        Ok(
                            Classes::hkaQuantizedAnimationTrackCompressionParams(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaRagdollInstance" => {
                        Ok(Classes::hkaRagdollInstance(map.next_value()?))
                    }
                    "hkArrayTypeAttribute" => {
                        Ok(Classes::hkArrayTypeAttribute(map.next_value()?))
                    }
                    "hkaSkeleton" => Ok(Classes::hkaSkeleton(map.next_value()?)),
                    "hkaSkeletonLocalFrameOnBone" => {
                        Ok(Classes::hkaSkeletonLocalFrameOnBone(map.next_value()?))
                    }
                    "hkaSkeletonMapper" => {
                        Ok(Classes::hkaSkeletonMapper(map.next_value()?))
                    }
                    "hkaSkeletonMapperData" => {
                        Ok(Classes::hkaSkeletonMapperData(map.next_value()?))
                    }
                    "hkaSkeletonMapperDataChainMapping" => {
                        Ok(Classes::hkaSkeletonMapperDataChainMapping(map.next_value()?))
                    }
                    "hkaSkeletonMapperDataSimpleMapping" => {
                        Ok(
                            Classes::hkaSkeletonMapperDataSimpleMapping(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaSplineCompressedAnimation" => {
                        Ok(Classes::hkaSplineCompressedAnimation(map.next_value()?))
                    }
                    "hkaSplineCompressedAnimationAnimationCompressionParams" => {
                        Ok(
                            Classes::hkaSplineCompressedAnimationAnimationCompressionParams(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaSplineCompressedAnimationTrackCompressionParams" => {
                        Ok(
                            Classes::hkaSplineCompressedAnimationTrackCompressionParams(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaWaveletCompressedAnimation" => {
                        Ok(Classes::hkaWaveletCompressedAnimation(map.next_value()?))
                    }
                    "hkaWaveletCompressedAnimationCompressionParams" => {
                        Ok(
                            Classes::hkaWaveletCompressedAnimationCompressionParams(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkaWaveletCompressedAnimationQuantizationFormat" => {
                        Ok(
                            Classes::hkaWaveletCompressedAnimationQuantizationFormat(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkBaseObject" => Ok(Classes::hkBaseObject(map.next_value()?)),
                    "hkbAttachmentModifier" => {
                        Ok(Classes::hkbAttachmentModifier(map.next_value()?))
                    }
                    "hkbAttachmentSetup" => {
                        Ok(Classes::hkbAttachmentSetup(map.next_value()?))
                    }
                    "hkbAttributeModifier" => {
                        Ok(Classes::hkbAttributeModifier(map.next_value()?))
                    }
                    "hkbAttributeModifierAssignment" => {
                        Ok(Classes::hkbAttributeModifierAssignment(map.next_value()?))
                    }
                    "hkbAuxiliaryNodeInfo" => {
                        Ok(Classes::hkbAuxiliaryNodeInfo(map.next_value()?))
                    }
                    "hkbBehaviorEventsInfo" => {
                        Ok(Classes::hkbBehaviorEventsInfo(map.next_value()?))
                    }
                    "hkbBehaviorGraph" => {
                        Ok(Classes::hkbBehaviorGraph(map.next_value()?))
                    }
                    "hkbBehaviorGraphData" => {
                        Ok(Classes::hkbBehaviorGraphData(map.next_value()?))
                    }
                    "hkbBehaviorGraphInternalState" => {
                        Ok(Classes::hkbBehaviorGraphInternalState(map.next_value()?))
                    }
                    "hkbBehaviorGraphInternalStateInfo" => {
                        Ok(Classes::hkbBehaviorGraphInternalStateInfo(map.next_value()?))
                    }
                    "hkbBehaviorGraphStringData" => {
                        Ok(Classes::hkbBehaviorGraphStringData(map.next_value()?))
                    }
                    "hkbBehaviorInfo" => Ok(Classes::hkbBehaviorInfo(map.next_value()?)),
                    "hkbBehaviorInfoIdToNamePair" => {
                        Ok(Classes::hkbBehaviorInfoIdToNamePair(map.next_value()?))
                    }
                    "hkbBehaviorReferenceGenerator" => {
                        Ok(Classes::hkbBehaviorReferenceGenerator(map.next_value()?))
                    }
                    "hkbBindable" => Ok(Classes::hkbBindable(map.next_value()?)),
                    "hkbBlendCurveUtils" => {
                        Ok(Classes::hkbBlendCurveUtils(map.next_value()?))
                    }
                    "hkbBlenderGenerator" => {
                        Ok(Classes::hkbBlenderGenerator(map.next_value()?))
                    }
                    "hkbBlenderGeneratorChild" => {
                        Ok(Classes::hkbBlenderGeneratorChild(map.next_value()?))
                    }
                    "hkbBlenderGeneratorChildInternalState" => {
                        Ok(
                            Classes::hkbBlenderGeneratorChildInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbBlenderGeneratorInternalState" => {
                        Ok(Classes::hkbBlenderGeneratorInternalState(map.next_value()?))
                    }
                    "hkbBlendingTransitionEffect" => {
                        Ok(Classes::hkbBlendingTransitionEffect(map.next_value()?))
                    }
                    "hkbBlendingTransitionEffectInternalState" => {
                        Ok(
                            Classes::hkbBlendingTransitionEffectInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbBoneIndexArray" => {
                        Ok(Classes::hkbBoneIndexArray(map.next_value()?))
                    }
                    "hkbBoneWeightArray" => {
                        Ok(Classes::hkbBoneWeightArray(map.next_value()?))
                    }
                    "hkbBoolVariableSequencedData" => {
                        Ok(Classes::hkbBoolVariableSequencedData(map.next_value()?))
                    }
                    "hkbBoolVariableSequencedDataSample" => {
                        Ok(
                            Classes::hkbBoolVariableSequencedDataSample(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbCameraShakeEventPayload" => {
                        Ok(Classes::hkbCameraShakeEventPayload(map.next_value()?))
                    }
                    "hkbCharacter" => Ok(Classes::hkbCharacter(map.next_value()?)),
                    "hkbCharacterAddedInfo" => {
                        Ok(Classes::hkbCharacterAddedInfo(map.next_value()?))
                    }
                    "hkbCharacterControlCommand" => {
                        Ok(Classes::hkbCharacterControlCommand(map.next_value()?))
                    }
                    "hkbCharacterControllerControlData" => {
                        Ok(Classes::hkbCharacterControllerControlData(map.next_value()?))
                    }
                    "hkbCharacterControllerModifier" => {
                        Ok(Classes::hkbCharacterControllerModifier(map.next_value()?))
                    }
                    "hkbCharacterControllerModifierInternalState" => {
                        Ok(
                            Classes::hkbCharacterControllerModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbCharacterData" => {
                        Ok(Classes::hkbCharacterData(map.next_value()?))
                    }
                    "hkbCharacterDataCharacterControllerInfo" => {
                        Ok(
                            Classes::hkbCharacterDataCharacterControllerInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbCharacterInfo" => {
                        Ok(Classes::hkbCharacterInfo(map.next_value()?))
                    }
                    "hkbCharacterSetup" => {
                        Ok(Classes::hkbCharacterSetup(map.next_value()?))
                    }
                    "hkbCharacterSkinInfo" => {
                        Ok(Classes::hkbCharacterSkinInfo(map.next_value()?))
                    }
                    "hkbCharacterSteppedInfo" => {
                        Ok(Classes::hkbCharacterSteppedInfo(map.next_value()?))
                    }
                    "hkbCharacterStringData" => {
                        Ok(Classes::hkbCharacterStringData(map.next_value()?))
                    }
                    "hkbClientCharacterState" => {
                        Ok(Classes::hkbClientCharacterState(map.next_value()?))
                    }
                    "hkbClipGenerator" => {
                        Ok(Classes::hkbClipGenerator(map.next_value()?))
                    }
                    "hkbClipGeneratorEcho" => {
                        Ok(Classes::hkbClipGeneratorEcho(map.next_value()?))
                    }
                    "hkbClipGeneratorInternalState" => {
                        Ok(Classes::hkbClipGeneratorInternalState(map.next_value()?))
                    }
                    "hkbClipTrigger" => Ok(Classes::hkbClipTrigger(map.next_value()?)),
                    "hkbClipTriggerArray" => {
                        Ok(Classes::hkbClipTriggerArray(map.next_value()?))
                    }
                    "hkbCombineTransformsModifier" => {
                        Ok(Classes::hkbCombineTransformsModifier(map.next_value()?))
                    }
                    "hkbCombineTransformsModifierInternalState" => {
                        Ok(
                            Classes::hkbCombineTransformsModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbCompiledExpressionSet" => {
                        Ok(Classes::hkbCompiledExpressionSet(map.next_value()?))
                    }
                    "hkbCompiledExpressionSetToken" => {
                        Ok(Classes::hkbCompiledExpressionSetToken(map.next_value()?))
                    }
                    "hkbComputeDirectionModifier" => {
                        Ok(Classes::hkbComputeDirectionModifier(map.next_value()?))
                    }
                    "hkbComputeDirectionModifierInternalState" => {
                        Ok(
                            Classes::hkbComputeDirectionModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbComputeRotationFromAxisAngleModifier" => {
                        Ok(
                            Classes::hkbComputeRotationFromAxisAngleModifier(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbComputeRotationFromAxisAngleModifierInternalState" => {
                        Ok(
                            Classes::hkbComputeRotationFromAxisAngleModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbComputeRotationToTargetModifier" => {
                        Ok(
                            Classes::hkbComputeRotationToTargetModifier(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbComputeRotationToTargetModifierInternalState" => {
                        Ok(
                            Classes::hkbComputeRotationToTargetModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbCondition" => Ok(Classes::hkbCondition(map.next_value()?)),
                    "hkbContext" => Ok(Classes::hkbContext(map.next_value()?)),
                    "hkbDampingModifier" => {
                        Ok(Classes::hkbDampingModifier(map.next_value()?))
                    }
                    "hkbDampingModifierInternalState" => {
                        Ok(Classes::hkbDampingModifierInternalState(map.next_value()?))
                    }
                    "hkbDefaultMessageLog" => {
                        Ok(Classes::hkbDefaultMessageLog(map.next_value()?))
                    }
                    "hkbDelayedModifier" => {
                        Ok(Classes::hkbDelayedModifier(map.next_value()?))
                    }
                    "hkbDelayedModifierInternalState" => {
                        Ok(Classes::hkbDelayedModifierInternalState(map.next_value()?))
                    }
                    "hkbDetectCloseToGroundModifier" => {
                        Ok(Classes::hkbDetectCloseToGroundModifier(map.next_value()?))
                    }
                    "hkbDetectCloseToGroundModifierInternalState" => {
                        Ok(
                            Classes::hkbDetectCloseToGroundModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbEvaluateExpressionModifier" => {
                        Ok(Classes::hkbEvaluateExpressionModifier(map.next_value()?))
                    }
                    "hkbEvaluateExpressionModifierInternalExpressionData" => {
                        Ok(
                            Classes::hkbEvaluateExpressionModifierInternalExpressionData(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbEvaluateExpressionModifierInternalState" => {
                        Ok(
                            Classes::hkbEvaluateExpressionModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbEvaluateHandleModifier" => {
                        Ok(Classes::hkbEvaluateHandleModifier(map.next_value()?))
                    }
                    "hkbEvent" => Ok(Classes::hkbEvent(map.next_value()?)),
                    "hkbEventBase" => Ok(Classes::hkbEventBase(map.next_value()?)),
                    "hkbEventDrivenModifier" => {
                        Ok(Classes::hkbEventDrivenModifier(map.next_value()?))
                    }
                    "hkbEventDrivenModifierInternalState" => {
                        Ok(
                            Classes::hkbEventDrivenModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbEventInfo" => Ok(Classes::hkbEventInfo(map.next_value()?)),
                    "hkbEventPayload" => Ok(Classes::hkbEventPayload(map.next_value()?)),
                    "hkbEventPayloadList" => {
                        Ok(Classes::hkbEventPayloadList(map.next_value()?))
                    }
                    "hkbEventProperty" => {
                        Ok(Classes::hkbEventProperty(map.next_value()?))
                    }
                    "hkbEventRaisedInfo" => {
                        Ok(Classes::hkbEventRaisedInfo(map.next_value()?))
                    }
                    "hkbEventRangeData" => {
                        Ok(Classes::hkbEventRangeData(map.next_value()?))
                    }
                    "hkbEventRangeDataArray" => {
                        Ok(Classes::hkbEventRangeDataArray(map.next_value()?))
                    }
                    "hkbEventSequencedData" => {
                        Ok(Classes::hkbEventSequencedData(map.next_value()?))
                    }
                    "hkbEventSequencedDataSequencedEvent" => {
                        Ok(
                            Classes::hkbEventSequencedDataSequencedEvent(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbEventsFromRangeModifier" => {
                        Ok(Classes::hkbEventsFromRangeModifier(map.next_value()?))
                    }
                    "hkbEventsFromRangeModifierInternalState" => {
                        Ok(
                            Classes::hkbEventsFromRangeModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbExpressionCondition" => {
                        Ok(Classes::hkbExpressionCondition(map.next_value()?))
                    }
                    "hkbExpressionData" => {
                        Ok(Classes::hkbExpressionData(map.next_value()?))
                    }
                    "hkbExpressionDataArray" => {
                        Ok(Classes::hkbExpressionDataArray(map.next_value()?))
                    }
                    "hkbExtractRagdollPoseModifier" => {
                        Ok(Classes::hkbExtractRagdollPoseModifier(map.next_value()?))
                    }
                    "hkbFootIkControlData" => {
                        Ok(Classes::hkbFootIkControlData(map.next_value()?))
                    }
                    "hkbFootIkControlsModifier" => {
                        Ok(Classes::hkbFootIkControlsModifier(map.next_value()?))
                    }
                    "hkbFootIkControlsModifierLeg" => {
                        Ok(Classes::hkbFootIkControlsModifierLeg(map.next_value()?))
                    }
                    "hkbFootIkDriverInfo" => {
                        Ok(Classes::hkbFootIkDriverInfo(map.next_value()?))
                    }
                    "hkbFootIkDriverInfoLeg" => {
                        Ok(Classes::hkbFootIkDriverInfoLeg(map.next_value()?))
                    }
                    "hkbFootIkGains" => Ok(Classes::hkbFootIkGains(map.next_value()?)),
                    "hkbFootIkModifier" => {
                        Ok(Classes::hkbFootIkModifier(map.next_value()?))
                    }
                    "hkbFootIkModifierInternalLegData" => {
                        Ok(Classes::hkbFootIkModifierInternalLegData(map.next_value()?))
                    }
                    "hkbFootIkModifierLeg" => {
                        Ok(Classes::hkbFootIkModifierLeg(map.next_value()?))
                    }
                    "hkbGenerator" => Ok(Classes::hkbGenerator(map.next_value()?)),
                    "hkbGeneratorOutputListener" => {
                        Ok(Classes::hkbGeneratorOutputListener(map.next_value()?))
                    }
                    "hkbGeneratorSyncInfo" => {
                        Ok(Classes::hkbGeneratorSyncInfo(map.next_value()?))
                    }
                    "hkbGeneratorSyncInfoSyncPoint" => {
                        Ok(Classes::hkbGeneratorSyncInfoSyncPoint(map.next_value()?))
                    }
                    "hkbGeneratorTransitionEffect" => {
                        Ok(Classes::hkbGeneratorTransitionEffect(map.next_value()?))
                    }
                    "hkbGeneratorTransitionEffectInternalState" => {
                        Ok(
                            Classes::hkbGeneratorTransitionEffectInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbGetHandleOnBoneModifier" => {
                        Ok(Classes::hkbGetHandleOnBoneModifier(map.next_value()?))
                    }
                    "hkbGetUpModifier" => {
                        Ok(Classes::hkbGetUpModifier(map.next_value()?))
                    }
                    "hkbGetUpModifierInternalState" => {
                        Ok(Classes::hkbGetUpModifierInternalState(map.next_value()?))
                    }
                    "hkbGetWorldFromModelModifier" => {
                        Ok(Classes::hkbGetWorldFromModelModifier(map.next_value()?))
                    }
                    "hkbGetWorldFromModelModifierInternalState" => {
                        Ok(
                            Classes::hkbGetWorldFromModelModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbHandIkControlData" => {
                        Ok(Classes::hkbHandIkControlData(map.next_value()?))
                    }
                    "hkbHandIkControlsModifier" => {
                        Ok(Classes::hkbHandIkControlsModifier(map.next_value()?))
                    }
                    "hkbHandIkControlsModifierHand" => {
                        Ok(Classes::hkbHandIkControlsModifierHand(map.next_value()?))
                    }
                    "hkbHandIkDriverInfo" => {
                        Ok(Classes::hkbHandIkDriverInfo(map.next_value()?))
                    }
                    "hkbHandIkDriverInfoHand" => {
                        Ok(Classes::hkbHandIkDriverInfoHand(map.next_value()?))
                    }
                    "hkbHandIkModifier" => {
                        Ok(Classes::hkbHandIkModifier(map.next_value()?))
                    }
                    "hkbHandIkModifierHand" => {
                        Ok(Classes::hkbHandIkModifierHand(map.next_value()?))
                    }
                    "hkbHandle" => Ok(Classes::hkbHandle(map.next_value()?)),
                    "hkbIntEventPayload" => {
                        Ok(Classes::hkbIntEventPayload(map.next_value()?))
                    }
                    "hkbIntVariableSequencedData" => {
                        Ok(Classes::hkbIntVariableSequencedData(map.next_value()?))
                    }
                    "hkbIntVariableSequencedDataSample" => {
                        Ok(Classes::hkbIntVariableSequencedDataSample(map.next_value()?))
                    }
                    "hkBitField" => Ok(Classes::hkBitField(map.next_value()?)),
                    "hkbKeyframeBonesModifier" => {
                        Ok(Classes::hkbKeyframeBonesModifier(map.next_value()?))
                    }
                    "hkbKeyframeBonesModifierKeyframeInfo" => {
                        Ok(
                            Classes::hkbKeyframeBonesModifierKeyframeInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbLinkedSymbolInfo" => {
                        Ok(Classes::hkbLinkedSymbolInfo(map.next_value()?))
                    }
                    "hkbLookAtModifier" => {
                        Ok(Classes::hkbLookAtModifier(map.next_value()?))
                    }
                    "hkbLookAtModifierInternalState" => {
                        Ok(Classes::hkbLookAtModifierInternalState(map.next_value()?))
                    }
                    "hkbManualSelectorGenerator" => {
                        Ok(Classes::hkbManualSelectorGenerator(map.next_value()?))
                    }
                    "hkbManualSelectorGeneratorInternalState" => {
                        Ok(
                            Classes::hkbManualSelectorGeneratorInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbMessageLog" => Ok(Classes::hkbMessageLog(map.next_value()?)),
                    "hkbMirroredSkeletonInfo" => {
                        Ok(Classes::hkbMirroredSkeletonInfo(map.next_value()?))
                    }
                    "hkbMirrorModifier" => {
                        Ok(Classes::hkbMirrorModifier(map.next_value()?))
                    }
                    "hkbModifier" => Ok(Classes::hkbModifier(map.next_value()?)),
                    "hkbModifierGenerator" => {
                        Ok(Classes::hkbModifierGenerator(map.next_value()?))
                    }
                    "hkbModifierList" => Ok(Classes::hkbModifierList(map.next_value()?)),
                    "hkbModifierWrapper" => {
                        Ok(Classes::hkbModifierWrapper(map.next_value()?))
                    }
                    "hkbMoveCharacterModifier" => {
                        Ok(Classes::hkbMoveCharacterModifier(map.next_value()?))
                    }
                    "hkbMoveCharacterModifierInternalState" => {
                        Ok(
                            Classes::hkbMoveCharacterModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbNamedEventPayload" => {
                        Ok(Classes::hkbNamedEventPayload(map.next_value()?))
                    }
                    "hkbNamedIntEventPayload" => {
                        Ok(Classes::hkbNamedIntEventPayload(map.next_value()?))
                    }
                    "hkbNamedRealEventPayload" => {
                        Ok(Classes::hkbNamedRealEventPayload(map.next_value()?))
                    }
                    "hkbNamedStringEventPayload" => {
                        Ok(Classes::hkbNamedStringEventPayload(map.next_value()?))
                    }
                    "hkbNode" => Ok(Classes::hkbNode(map.next_value()?)),
                    "hkbNodeInternalStateInfo" => {
                        Ok(Classes::hkbNodeInternalStateInfo(map.next_value()?))
                    }
                    "hkbParticleSystemEventPayload" => {
                        Ok(Classes::hkbParticleSystemEventPayload(map.next_value()?))
                    }
                    "hkbPoseMatchingGenerator" => {
                        Ok(Classes::hkbPoseMatchingGenerator(map.next_value()?))
                    }
                    "hkbPoseMatchingGeneratorInternalState" => {
                        Ok(
                            Classes::hkbPoseMatchingGeneratorInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbPoweredRagdollControlData" => {
                        Ok(Classes::hkbPoweredRagdollControlData(map.next_value()?))
                    }
                    "hkbPoweredRagdollControlsModifier" => {
                        Ok(Classes::hkbPoweredRagdollControlsModifier(map.next_value()?))
                    }
                    "hkbProjectData" => Ok(Classes::hkbProjectData(map.next_value()?)),
                    "hkbProjectStringData" => {
                        Ok(Classes::hkbProjectStringData(map.next_value()?))
                    }
                    "hkbProxyModifier" => {
                        Ok(Classes::hkbProxyModifier(map.next_value()?))
                    }
                    "hkbProxyModifierProxyInfo" => {
                        Ok(Classes::hkbProxyModifierProxyInfo(map.next_value()?))
                    }
                    "hkbRaiseEventCommand" => {
                        Ok(Classes::hkbRaiseEventCommand(map.next_value()?))
                    }
                    "hkbRealEventPayload" => {
                        Ok(Classes::hkbRealEventPayload(map.next_value()?))
                    }
                    "hkbRealVariableSequencedData" => {
                        Ok(Classes::hkbRealVariableSequencedData(map.next_value()?))
                    }
                    "hkbRealVariableSequencedDataSample" => {
                        Ok(
                            Classes::hkbRealVariableSequencedDataSample(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbReferencePoseGenerator" => {
                        Ok(Classes::hkbReferencePoseGenerator(map.next_value()?))
                    }
                    "hkbRegisteredGenerator" => {
                        Ok(Classes::hkbRegisteredGenerator(map.next_value()?))
                    }
                    "hkbRigidBodyRagdollControlData" => {
                        Ok(Classes::hkbRigidBodyRagdollControlData(map.next_value()?))
                    }
                    "hkbRigidBodyRagdollControlsModifier" => {
                        Ok(
                            Classes::hkbRigidBodyRagdollControlsModifier(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbRoleAttribute" => {
                        Ok(Classes::hkbRoleAttribute(map.next_value()?))
                    }
                    "hkbRotateCharacterModifier" => {
                        Ok(Classes::hkbRotateCharacterModifier(map.next_value()?))
                    }
                    "hkbRotateCharacterModifierInternalState" => {
                        Ok(
                            Classes::hkbRotateCharacterModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbSenseHandleModifier" => {
                        Ok(Classes::hkbSenseHandleModifier(map.next_value()?))
                    }
                    "hkbSenseHandleModifierRange" => {
                        Ok(Classes::hkbSenseHandleModifierRange(map.next_value()?))
                    }
                    "hkbSequence" => Ok(Classes::hkbSequence(map.next_value()?)),
                    "hkbSequencedData" => {
                        Ok(Classes::hkbSequencedData(map.next_value()?))
                    }
                    "hkbSequenceInternalState" => {
                        Ok(Classes::hkbSequenceInternalState(map.next_value()?))
                    }
                    "hkbSequenceStringData" => {
                        Ok(Classes::hkbSequenceStringData(map.next_value()?))
                    }
                    "hkbSetBehaviorCommand" => {
                        Ok(Classes::hkbSetBehaviorCommand(map.next_value()?))
                    }
                    "hkbSetLocalTimeOfClipGeneratorCommand" => {
                        Ok(
                            Classes::hkbSetLocalTimeOfClipGeneratorCommand(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbSetNodePropertyCommand" => {
                        Ok(Classes::hkbSetNodePropertyCommand(map.next_value()?))
                    }
                    "hkbSetWordVariableCommand" => {
                        Ok(Classes::hkbSetWordVariableCommand(map.next_value()?))
                    }
                    "hkbSetWorldFromModelModifier" => {
                        Ok(Classes::hkbSetWorldFromModelModifier(map.next_value()?))
                    }
                    "hkbSimulationControlCommand" => {
                        Ok(Classes::hkbSimulationControlCommand(map.next_value()?))
                    }
                    "hkbSimulationStateInfo" => {
                        Ok(Classes::hkbSimulationStateInfo(map.next_value()?))
                    }
                    "hkbStateChooser" => Ok(Classes::hkbStateChooser(map.next_value()?)),
                    "hkbStateListener" => {
                        Ok(Classes::hkbStateListener(map.next_value()?))
                    }
                    "hkbStateMachine" => Ok(Classes::hkbStateMachine(map.next_value()?)),
                    "hkbStateMachineActiveTransitionInfo" => {
                        Ok(
                            Classes::hkbStateMachineActiveTransitionInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbStateMachineDelayedTransitionInfo" => {
                        Ok(
                            Classes::hkbStateMachineDelayedTransitionInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbStateMachineEventPropertyArray" => {
                        Ok(Classes::hkbStateMachineEventPropertyArray(map.next_value()?))
                    }
                    "hkbStateMachineInternalState" => {
                        Ok(Classes::hkbStateMachineInternalState(map.next_value()?))
                    }
                    "hkbStateMachineNestedStateMachineData" => {
                        Ok(
                            Classes::hkbStateMachineNestedStateMachineData(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbStateMachineProspectiveTransitionInfo" => {
                        Ok(
                            Classes::hkbStateMachineProspectiveTransitionInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbStateMachineStateInfo" => {
                        Ok(Classes::hkbStateMachineStateInfo(map.next_value()?))
                    }
                    "hkbStateMachineTimeInterval" => {
                        Ok(Classes::hkbStateMachineTimeInterval(map.next_value()?))
                    }
                    "hkbStateMachineTransitionInfo" => {
                        Ok(Classes::hkbStateMachineTransitionInfo(map.next_value()?))
                    }
                    "hkbStateMachineTransitionInfoArray" => {
                        Ok(
                            Classes::hkbStateMachineTransitionInfoArray(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbStateMachineTransitionInfoReference" => {
                        Ok(
                            Classes::hkbStateMachineTransitionInfoReference(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbStringCondition" => {
                        Ok(Classes::hkbStringCondition(map.next_value()?))
                    }
                    "hkbStringEventPayload" => {
                        Ok(Classes::hkbStringEventPayload(map.next_value()?))
                    }
                    "hkbTestStateChooser" => {
                        Ok(Classes::hkbTestStateChooser(map.next_value()?))
                    }
                    "hkbTimerModifier" => {
                        Ok(Classes::hkbTimerModifier(map.next_value()?))
                    }
                    "hkbTimerModifierInternalState" => {
                        Ok(Classes::hkbTimerModifierInternalState(map.next_value()?))
                    }
                    "hkbTransformVectorModifier" => {
                        Ok(Classes::hkbTransformVectorModifier(map.next_value()?))
                    }
                    "hkbTransformVectorModifierInternalState" => {
                        Ok(
                            Classes::hkbTransformVectorModifierInternalState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkbTransitionEffect" => {
                        Ok(Classes::hkbTransitionEffect(map.next_value()?))
                    }
                    "hkbTwistModifier" => {
                        Ok(Classes::hkbTwistModifier(map.next_value()?))
                    }
                    "hkbVariableBindingSet" => {
                        Ok(Classes::hkbVariableBindingSet(map.next_value()?))
                    }
                    "hkbVariableBindingSetBinding" => {
                        Ok(Classes::hkbVariableBindingSetBinding(map.next_value()?))
                    }
                    "hkbVariableInfo" => Ok(Classes::hkbVariableInfo(map.next_value()?)),
                    "hkbVariableValue" => {
                        Ok(Classes::hkbVariableValue(map.next_value()?))
                    }
                    "hkbVariableValueSet" => {
                        Ok(Classes::hkbVariableValueSet(map.next_value()?))
                    }
                    "hkbWorldEnums" => Ok(Classes::hkbWorldEnums(map.next_value()?)),
                    "hkbWorldFromModelModeData" => {
                        Ok(Classes::hkbWorldFromModelModeData(map.next_value()?))
                    }
                    "hkClass" => Ok(Classes::hkClass(map.next_value()?)),
                    "hkClassEnum" => Ok(Classes::hkClassEnum(map.next_value()?)),
                    "hkClassEnumItem" => Ok(Classes::hkClassEnumItem(map.next_value()?)),
                    "hkClassMember" => Ok(Classes::hkClassMember(map.next_value()?)),
                    "hkColor" => Ok(Classes::hkColor(map.next_value()?)),
                    "hkContactPoint" => Ok(Classes::hkContactPoint(map.next_value()?)),
                    "hkContactPointMaterial" => {
                        Ok(Classes::hkContactPointMaterial(map.next_value()?))
                    }
                    "hkCustomAttributes" => {
                        Ok(Classes::hkCustomAttributes(map.next_value()?))
                    }
                    "hkCustomAttributesAttribute" => {
                        Ok(Classes::hkCustomAttributesAttribute(map.next_value()?))
                    }
                    "hkDataObjectTypeAttribute" => {
                        Ok(Classes::hkDataObjectTypeAttribute(map.next_value()?))
                    }
                    "hkDescriptionAttribute" => {
                        Ok(Classes::hkDescriptionAttribute(map.next_value()?))
                    }
                    "hkDocumentationAttribute" => {
                        Ok(Classes::hkDocumentationAttribute(map.next_value()?))
                    }
                    "hkGeometry" => Ok(Classes::hkGeometry(map.next_value()?)),
                    "hkGeometryTriangle" => {
                        Ok(Classes::hkGeometryTriangle(map.next_value()?))
                    }
                    "hkGizmoAttribute" => {
                        Ok(Classes::hkGizmoAttribute(map.next_value()?))
                    }
                    "hkHalf8" => Ok(Classes::hkHalf8(map.next_value()?)),
                    "hkIndexedTransformSet" => {
                        Ok(Classes::hkIndexedTransformSet(map.next_value()?))
                    }
                    "hkLinkAttribute" => Ok(Classes::hkLinkAttribute(map.next_value()?)),
                    "hkLocalFrame" => Ok(Classes::hkLocalFrame(map.next_value()?)),
                    "hkLocalFrameGroup" => {
                        Ok(Classes::hkLocalFrameGroup(map.next_value()?))
                    }
                    "hkMemoryMeshBody" => {
                        Ok(Classes::hkMemoryMeshBody(map.next_value()?))
                    }
                    "hkMemoryMeshMaterial" => {
                        Ok(Classes::hkMemoryMeshMaterial(map.next_value()?))
                    }
                    "hkMemoryMeshShape" => {
                        Ok(Classes::hkMemoryMeshShape(map.next_value()?))
                    }
                    "hkMemoryMeshTexture" => {
                        Ok(Classes::hkMemoryMeshTexture(map.next_value()?))
                    }
                    "hkMemoryMeshVertexBuffer" => {
                        Ok(Classes::hkMemoryMeshVertexBuffer(map.next_value()?))
                    }
                    "hkMemoryResourceContainer" => {
                        Ok(Classes::hkMemoryResourceContainer(map.next_value()?))
                    }
                    "hkMemoryResourceHandle" => {
                        Ok(Classes::hkMemoryResourceHandle(map.next_value()?))
                    }
                    "hkMemoryResourceHandleExternalLink" => {
                        Ok(
                            Classes::hkMemoryResourceHandleExternalLink(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkMemoryTrackerAttribute" => {
                        Ok(Classes::hkMemoryTrackerAttribute(map.next_value()?))
                    }
                    "hkMeshBody" => Ok(Classes::hkMeshBody(map.next_value()?)),
                    "hkMeshBoneIndexMapping" => {
                        Ok(Classes::hkMeshBoneIndexMapping(map.next_value()?))
                    }
                    "hkMeshMaterial" => Ok(Classes::hkMeshMaterial(map.next_value()?)),
                    "hkMeshSection" => Ok(Classes::hkMeshSection(map.next_value()?)),
                    "hkMeshSectionCinfo" => {
                        Ok(Classes::hkMeshSectionCinfo(map.next_value()?))
                    }
                    "hkMeshShape" => Ok(Classes::hkMeshShape(map.next_value()?)),
                    "hkMeshTexture" => Ok(Classes::hkMeshTexture(map.next_value()?)),
                    "hkMeshVertexBuffer" => {
                        Ok(Classes::hkMeshVertexBuffer(map.next_value()?))
                    }
                    "hkModelerNodeTypeAttribute" => {
                        Ok(Classes::hkModelerNodeTypeAttribute(map.next_value()?))
                    }
                    "hkMonitorStreamColorTable" => {
                        Ok(Classes::hkMonitorStreamColorTable(map.next_value()?))
                    }
                    "hkMonitorStreamColorTableColorPair" => {
                        Ok(
                            Classes::hkMonitorStreamColorTableColorPair(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkMonitorStreamFrameInfo" => {
                        Ok(Classes::hkMonitorStreamFrameInfo(map.next_value()?))
                    }
                    "hkMonitorStreamStringMap" => {
                        Ok(Classes::hkMonitorStreamStringMap(map.next_value()?))
                    }
                    "hkMonitorStreamStringMapStringMap" => {
                        Ok(Classes::hkMonitorStreamStringMapStringMap(map.next_value()?))
                    }
                    "hkMoppBvTreeShapeBase" => {
                        Ok(Classes::hkMoppBvTreeShapeBase(map.next_value()?))
                    }
                    "hkMotionState" => Ok(Classes::hkMotionState(map.next_value()?)),
                    "hkMultipleVertexBuffer" => {
                        Ok(Classes::hkMultipleVertexBuffer(map.next_value()?))
                    }
                    "hkMultipleVertexBufferElementInfo" => {
                        Ok(Classes::hkMultipleVertexBufferElementInfo(map.next_value()?))
                    }
                    "hkMultipleVertexBufferLockedElement" => {
                        Ok(
                            Classes::hkMultipleVertexBufferLockedElement(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkMultipleVertexBufferVertexBufferInfo" => {
                        Ok(
                            Classes::hkMultipleVertexBufferVertexBufferInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkMultiThreadCheck" => {
                        Ok(Classes::hkMultiThreadCheck(map.next_value()?))
                    }
                    "hkp2dAngConstraintAtom" => {
                        Ok(Classes::hkp2dAngConstraintAtom(map.next_value()?))
                    }
                    "hkpAabbPhantom" => Ok(Classes::hkpAabbPhantom(map.next_value()?)),
                    "hkPackedVector3" => Ok(Classes::hkPackedVector3(map.next_value()?)),
                    "hkPackfileHeader" => {
                        Ok(Classes::hkPackfileHeader(map.next_value()?))
                    }
                    "hkPackfileSectionHeader" => {
                        Ok(Classes::hkPackfileSectionHeader(map.next_value()?))
                    }
                    "hkpAction" => Ok(Classes::hkpAction(map.next_value()?)),
                    "hkpAgent1nSector" => {
                        Ok(Classes::hkpAgent1nSector(map.next_value()?))
                    }
                    "hkpAngConstraintAtom" => {
                        Ok(Classes::hkpAngConstraintAtom(map.next_value()?))
                    }
                    "hkpAngFrictionConstraintAtom" => {
                        Ok(Classes::hkpAngFrictionConstraintAtom(map.next_value()?))
                    }
                    "hkpAngLimitConstraintAtom" => {
                        Ok(Classes::hkpAngLimitConstraintAtom(map.next_value()?))
                    }
                    "hkpAngMotorConstraintAtom" => {
                        Ok(Classes::hkpAngMotorConstraintAtom(map.next_value()?))
                    }
                    "hkpAngularDashpotAction" => {
                        Ok(Classes::hkpAngularDashpotAction(map.next_value()?))
                    }
                    "hkpArrayAction" => Ok(Classes::hkpArrayAction(map.next_value()?)),
                    "hkpBallAndSocketConstraintData" => {
                        Ok(Classes::hkpBallAndSocketConstraintData(map.next_value()?))
                    }
                    "hkpBallAndSocketConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpBallAndSocketConstraintDataAtoms(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpBallGun" => Ok(Classes::hkpBallGun(map.next_value()?)),
                    "hkpBallSocketChainData" => {
                        Ok(Classes::hkpBallSocketChainData(map.next_value()?))
                    }
                    "hkpBallSocketChainDataConstraintInfo" => {
                        Ok(
                            Classes::hkpBallSocketChainDataConstraintInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpBallSocketConstraintAtom" => {
                        Ok(Classes::hkpBallSocketConstraintAtom(map.next_value()?))
                    }
                    "hkpBinaryAction" => Ok(Classes::hkpBinaryAction(map.next_value()?)),
                    "hkpBoxMotion" => Ok(Classes::hkpBoxMotion(map.next_value()?)),
                    "hkpBoxShape" => Ok(Classes::hkpBoxShape(map.next_value()?)),
                    "hkpBreakableBody" => {
                        Ok(Classes::hkpBreakableBody(map.next_value()?))
                    }
                    "hkpBreakableConstraintData" => {
                        Ok(Classes::hkpBreakableConstraintData(map.next_value()?))
                    }
                    "hkpBridgeAtoms" => Ok(Classes::hkpBridgeAtoms(map.next_value()?)),
                    "hkpBridgeConstraintAtom" => {
                        Ok(Classes::hkpBridgeConstraintAtom(map.next_value()?))
                    }
                    "hkpBroadPhaseHandle" => {
                        Ok(Classes::hkpBroadPhaseHandle(map.next_value()?))
                    }
                    "hkpBvShape" => Ok(Classes::hkpBvShape(map.next_value()?)),
                    "hkpBvTreeShape" => Ok(Classes::hkpBvTreeShape(map.next_value()?)),
                    "hkpCachingShapePhantom" => {
                        Ok(Classes::hkpCachingShapePhantom(map.next_value()?))
                    }
                    "hkpCallbackConstraintMotor" => {
                        Ok(Classes::hkpCallbackConstraintMotor(map.next_value()?))
                    }
                    "hkpCapsuleShape" => Ok(Classes::hkpCapsuleShape(map.next_value()?)),
                    "hkpCdBody" => Ok(Classes::hkpCdBody(map.next_value()?)),
                    "hkpCenterOfMassChangerModifierConstraintAtom" => {
                        Ok(
                            Classes::hkpCenterOfMassChangerModifierConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpCharacterControllerCinfo" => {
                        Ok(Classes::hkpCharacterControllerCinfo(map.next_value()?))
                    }
                    "hkpCharacterMotion" => {
                        Ok(Classes::hkpCharacterMotion(map.next_value()?))
                    }
                    "hkpCharacterProxyCinfo" => {
                        Ok(Classes::hkpCharacterProxyCinfo(map.next_value()?))
                    }
                    "hkpCharacterRigidBodyCinfo" => {
                        Ok(Classes::hkpCharacterRigidBodyCinfo(map.next_value()?))
                    }
                    "hkpCogWheelConstraintAtom" => {
                        Ok(Classes::hkpCogWheelConstraintAtom(map.next_value()?))
                    }
                    "hkpCogWheelConstraintData" => {
                        Ok(Classes::hkpCogWheelConstraintData(map.next_value()?))
                    }
                    "hkpCogWheelConstraintDataAtoms" => {
                        Ok(Classes::hkpCogWheelConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpCollidable" => Ok(Classes::hkpCollidable(map.next_value()?)),
                    "hkpCollidableBoundingVolumeData" => {
                        Ok(Classes::hkpCollidableBoundingVolumeData(map.next_value()?))
                    }
                    "hkpCollidableCollidableFilter" => {
                        Ok(Classes::hkpCollidableCollidableFilter(map.next_value()?))
                    }
                    "hkpCollisionFilter" => {
                        Ok(Classes::hkpCollisionFilter(map.next_value()?))
                    }
                    "hkpCollisionFilterList" => {
                        Ok(Classes::hkpCollisionFilterList(map.next_value()?))
                    }
                    "hkpCompressedMeshShape" => {
                        Ok(Classes::hkpCompressedMeshShape(map.next_value()?))
                    }
                    "hkpCompressedMeshShapeBigTriangle" => {
                        Ok(Classes::hkpCompressedMeshShapeBigTriangle(map.next_value()?))
                    }
                    "hkpCompressedMeshShapeChunk" => {
                        Ok(Classes::hkpCompressedMeshShapeChunk(map.next_value()?))
                    }
                    "hkpCompressedMeshShapeConvexPiece" => {
                        Ok(Classes::hkpCompressedMeshShapeConvexPiece(map.next_value()?))
                    }
                    "hkpCompressedSampledHeightFieldShape" => {
                        Ok(
                            Classes::hkpCompressedSampledHeightFieldShape(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpConeLimitConstraintAtom" => {
                        Ok(Classes::hkpConeLimitConstraintAtom(map.next_value()?))
                    }
                    "hkpConstrainedSystemFilter" => {
                        Ok(Classes::hkpConstrainedSystemFilter(map.next_value()?))
                    }
                    "hkpConstraintAtom" => {
                        Ok(Classes::hkpConstraintAtom(map.next_value()?))
                    }
                    "hkpConstraintChainData" => {
                        Ok(Classes::hkpConstraintChainData(map.next_value()?))
                    }
                    "hkpConstraintChainInstance" => {
                        Ok(Classes::hkpConstraintChainInstance(map.next_value()?))
                    }
                    "hkpConstraintChainInstanceAction" => {
                        Ok(Classes::hkpConstraintChainInstanceAction(map.next_value()?))
                    }
                    "hkpConstraintCollisionFilter" => {
                        Ok(Classes::hkpConstraintCollisionFilter(map.next_value()?))
                    }
                    "hkpConstraintData" => {
                        Ok(Classes::hkpConstraintData(map.next_value()?))
                    }
                    "hkpConstraintInstance" => {
                        Ok(Classes::hkpConstraintInstance(map.next_value()?))
                    }
                    "hkpConstraintInstanceSmallArraySerializeOverrideType" => {
                        Ok(
                            Classes::hkpConstraintInstanceSmallArraySerializeOverrideType(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpConstraintMotor" => {
                        Ok(Classes::hkpConstraintMotor(map.next_value()?))
                    }
                    "hkpConvexListFilter" => {
                        Ok(Classes::hkpConvexListFilter(map.next_value()?))
                    }
                    "hkpConvexListShape" => {
                        Ok(Classes::hkpConvexListShape(map.next_value()?))
                    }
                    "hkpConvexPieceMeshShape" => {
                        Ok(Classes::hkpConvexPieceMeshShape(map.next_value()?))
                    }
                    "hkpConvexPieceStreamData" => {
                        Ok(Classes::hkpConvexPieceStreamData(map.next_value()?))
                    }
                    "hkpConvexShape" => Ok(Classes::hkpConvexShape(map.next_value()?)),
                    "hkpConvexTransformShape" => {
                        Ok(Classes::hkpConvexTransformShape(map.next_value()?))
                    }
                    "hkpConvexTransformShapeBase" => {
                        Ok(Classes::hkpConvexTransformShapeBase(map.next_value()?))
                    }
                    "hkpConvexTranslateShape" => {
                        Ok(Classes::hkpConvexTranslateShape(map.next_value()?))
                    }
                    "hkpConvexVerticesConnectivity" => {
                        Ok(Classes::hkpConvexVerticesConnectivity(map.next_value()?))
                    }
                    "hkpConvexVerticesShape" => {
                        Ok(Classes::hkpConvexVerticesShape(map.next_value()?))
                    }
                    "hkpConvexVerticesShapeFourVectors" => {
                        Ok(Classes::hkpConvexVerticesShapeFourVectors(map.next_value()?))
                    }
                    "hkpCylinderShape" => {
                        Ok(Classes::hkpCylinderShape(map.next_value()?))
                    }
                    "hkpDashpotAction" => {
                        Ok(Classes::hkpDashpotAction(map.next_value()?))
                    }
                    "hkpDefaultConvexListFilter" => {
                        Ok(Classes::hkpDefaultConvexListFilter(map.next_value()?))
                    }
                    "hkpDefaultWorldMemoryWatchDog" => {
                        Ok(Classes::hkpDefaultWorldMemoryWatchDog(map.next_value()?))
                    }
                    "hkpDisableEntityCollisionFilter" => {
                        Ok(Classes::hkpDisableEntityCollisionFilter(map.next_value()?))
                    }
                    "hkpDisplayBindingData" => {
                        Ok(Classes::hkpDisplayBindingData(map.next_value()?))
                    }
                    "hkpDisplayBindingDataPhysicsSystem" => {
                        Ok(
                            Classes::hkpDisplayBindingDataPhysicsSystem(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpDisplayBindingDataRigidBody" => {
                        Ok(Classes::hkpDisplayBindingDataRigidBody(map.next_value()?))
                    }
                    "hkpEntity" => Ok(Classes::hkpEntity(map.next_value()?)),
                    "hkpEntityExtendedListeners" => {
                        Ok(Classes::hkpEntityExtendedListeners(map.next_value()?))
                    }
                    "hkpEntitySmallArraySerializeOverrideType" => {
                        Ok(
                            Classes::hkpEntitySmallArraySerializeOverrideType(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpEntitySpuCollisionCallback" => {
                        Ok(Classes::hkpEntitySpuCollisionCallback(map.next_value()?))
                    }
                    "hkpExtendedMeshShape" => {
                        Ok(Classes::hkpExtendedMeshShape(map.next_value()?))
                    }
                    "hkpExtendedMeshShapeShapesSubpart" => {
                        Ok(Classes::hkpExtendedMeshShapeShapesSubpart(map.next_value()?))
                    }
                    "hkpExtendedMeshShapeSubpart" => {
                        Ok(Classes::hkpExtendedMeshShapeSubpart(map.next_value()?))
                    }
                    "hkpExtendedMeshShapeTrianglesSubpart" => {
                        Ok(
                            Classes::hkpExtendedMeshShapeTrianglesSubpart(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpFastMeshShape" => {
                        Ok(Classes::hkpFastMeshShape(map.next_value()?))
                    }
                    "hkpFirstPersonGun" => {
                        Ok(Classes::hkpFirstPersonGun(map.next_value()?))
                    }
                    "hkpFixedRigidMotion" => {
                        Ok(Classes::hkpFixedRigidMotion(map.next_value()?))
                    }
                    "hkpGenericConstraintData" => {
                        Ok(Classes::hkpGenericConstraintData(map.next_value()?))
                    }
                    "hkpGenericConstraintDataScheme" => {
                        Ok(Classes::hkpGenericConstraintDataScheme(map.next_value()?))
                    }
                    "hkpGenericConstraintDataSchemeConstraintInfo" => {
                        Ok(
                            Classes::hkpGenericConstraintDataSchemeConstraintInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpGravityGun" => Ok(Classes::hkpGravityGun(map.next_value()?)),
                    "hkpGroupCollisionFilter" => {
                        Ok(Classes::hkpGroupCollisionFilter(map.next_value()?))
                    }
                    "hkpGroupFilter" => Ok(Classes::hkpGroupFilter(map.next_value()?)),
                    "hkpHeightFieldShape" => {
                        Ok(Classes::hkpHeightFieldShape(map.next_value()?))
                    }
                    "hkpHingeConstraintData" => {
                        Ok(Classes::hkpHingeConstraintData(map.next_value()?))
                    }
                    "hkpHingeConstraintDataAtoms" => {
                        Ok(Classes::hkpHingeConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpHingeLimitsData" => {
                        Ok(Classes::hkpHingeLimitsData(map.next_value()?))
                    }
                    "hkpHingeLimitsDataAtoms" => {
                        Ok(Classes::hkpHingeLimitsDataAtoms(map.next_value()?))
                    }
                    "hkpIgnoreModifierConstraintAtom" => {
                        Ok(Classes::hkpIgnoreModifierConstraintAtom(map.next_value()?))
                    }
                    "hkpKeyframedRigidMotion" => {
                        Ok(Classes::hkpKeyframedRigidMotion(map.next_value()?))
                    }
                    "hkpLimitedForceConstraintMotor" => {
                        Ok(Classes::hkpLimitedForceConstraintMotor(map.next_value()?))
                    }
                    "hkpLimitedHingeConstraintData" => {
                        Ok(Classes::hkpLimitedHingeConstraintData(map.next_value()?))
                    }
                    "hkpLimitedHingeConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpLimitedHingeConstraintDataAtoms(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpLinConstraintAtom" => {
                        Ok(Classes::hkpLinConstraintAtom(map.next_value()?))
                    }
                    "hkpLinearParametricCurve" => {
                        Ok(Classes::hkpLinearParametricCurve(map.next_value()?))
                    }
                    "hkpLinFrictionConstraintAtom" => {
                        Ok(Classes::hkpLinFrictionConstraintAtom(map.next_value()?))
                    }
                    "hkpLinkedCollidable" => {
                        Ok(Classes::hkpLinkedCollidable(map.next_value()?))
                    }
                    "hkpLinLimitConstraintAtom" => {
                        Ok(Classes::hkpLinLimitConstraintAtom(map.next_value()?))
                    }
                    "hkpLinMotorConstraintAtom" => {
                        Ok(Classes::hkpLinMotorConstraintAtom(map.next_value()?))
                    }
                    "hkpLinSoftConstraintAtom" => {
                        Ok(Classes::hkpLinSoftConstraintAtom(map.next_value()?))
                    }
                    "hkpListShape" => Ok(Classes::hkpListShape(map.next_value()?)),
                    "hkpListShapeChildInfo" => {
                        Ok(Classes::hkpListShapeChildInfo(map.next_value()?))
                    }
                    "hkpMalleableConstraintData" => {
                        Ok(Classes::hkpMalleableConstraintData(map.next_value()?))
                    }
                    "hkpMassChangerModifierConstraintAtom" => {
                        Ok(
                            Classes::hkpMassChangerModifierConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpMassProperties" => {
                        Ok(Classes::hkpMassProperties(map.next_value()?))
                    }
                    "hkpMaterial" => Ok(Classes::hkpMaterial(map.next_value()?)),
                    "hkpMaxSizeMotion" => {
                        Ok(Classes::hkpMaxSizeMotion(map.next_value()?))
                    }
                    "hkpMeshMaterial" => Ok(Classes::hkpMeshMaterial(map.next_value()?)),
                    "hkpMeshShape" => Ok(Classes::hkpMeshShape(map.next_value()?)),
                    "hkpMeshShapeSubpart" => {
                        Ok(Classes::hkpMeshShapeSubpart(map.next_value()?))
                    }
                    "hkpModifierConstraintAtom" => {
                        Ok(Classes::hkpModifierConstraintAtom(map.next_value()?))
                    }
                    "hkpMoppBvTreeShape" => {
                        Ok(Classes::hkpMoppBvTreeShape(map.next_value()?))
                    }
                    "hkpMoppCode" => Ok(Classes::hkpMoppCode(map.next_value()?)),
                    "hkpMoppCodeCodeInfo" => {
                        Ok(Classes::hkpMoppCodeCodeInfo(map.next_value()?))
                    }
                    "hkpMoppCodeReindexedTerminal" => {
                        Ok(Classes::hkpMoppCodeReindexedTerminal(map.next_value()?))
                    }
                    "hkpMotion" => Ok(Classes::hkpMotion(map.next_value()?)),
                    "hkpMotorAction" => Ok(Classes::hkpMotorAction(map.next_value()?)),
                    "hkpMountedBallGun" => {
                        Ok(Classes::hkpMountedBallGun(map.next_value()?))
                    }
                    "hkpMouseSpringAction" => {
                        Ok(Classes::hkpMouseSpringAction(map.next_value()?))
                    }
                    "hkpMovingSurfaceModifierConstraintAtom" => {
                        Ok(
                            Classes::hkpMovingSurfaceModifierConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpMultiRayShape" => {
                        Ok(Classes::hkpMultiRayShape(map.next_value()?))
                    }
                    "hkpMultiRayShapeRay" => {
                        Ok(Classes::hkpMultiRayShapeRay(map.next_value()?))
                    }
                    "hkpMultiSphereShape" => {
                        Ok(Classes::hkpMultiSphereShape(map.next_value()?))
                    }
                    "hkpMultithreadedVehicleManager" => {
                        Ok(Classes::hkpMultithreadedVehicleManager(map.next_value()?))
                    }
                    "hkpNamedMeshMaterial" => {
                        Ok(Classes::hkpNamedMeshMaterial(map.next_value()?))
                    }
                    "hkpNullCollisionFilter" => {
                        Ok(Classes::hkpNullCollisionFilter(map.next_value()?))
                    }
                    "hkPostFinishAttribute" => {
                        Ok(Classes::hkPostFinishAttribute(map.next_value()?))
                    }
                    "hkpOverwritePivotConstraintAtom" => {
                        Ok(Classes::hkpOverwritePivotConstraintAtom(map.next_value()?))
                    }
                    "hkpPairCollisionFilter" => {
                        Ok(Classes::hkpPairCollisionFilter(map.next_value()?))
                    }
                    "hkpPairCollisionFilterMapPairFilterKeyOverrideType" => {
                        Ok(
                            Classes::hkpPairCollisionFilterMapPairFilterKeyOverrideType(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpParametricCurve" => {
                        Ok(Classes::hkpParametricCurve(map.next_value()?))
                    }
                    "hkpPhantom" => Ok(Classes::hkpPhantom(map.next_value()?)),
                    "hkpPhantomCallbackShape" => {
                        Ok(Classes::hkpPhantomCallbackShape(map.next_value()?))
                    }
                    "hkpPhysicsData" => Ok(Classes::hkpPhysicsData(map.next_value()?)),
                    "hkpPhysicsSystem" => {
                        Ok(Classes::hkpPhysicsSystem(map.next_value()?))
                    }
                    "hkpPhysicsSystemWithContacts" => {
                        Ok(Classes::hkpPhysicsSystemWithContacts(map.next_value()?))
                    }
                    "hkpPlaneShape" => Ok(Classes::hkpPlaneShape(map.next_value()?)),
                    "hkpPointToPathConstraintData" => {
                        Ok(Classes::hkpPointToPathConstraintData(map.next_value()?))
                    }
                    "hkpPointToPlaneConstraintData" => {
                        Ok(Classes::hkpPointToPlaneConstraintData(map.next_value()?))
                    }
                    "hkpPointToPlaneConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpPointToPlaneConstraintDataAtoms(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpPositionConstraintMotor" => {
                        Ok(Classes::hkpPositionConstraintMotor(map.next_value()?))
                    }
                    "hkpPoweredChainData" => {
                        Ok(Classes::hkpPoweredChainData(map.next_value()?))
                    }
                    "hkpPoweredChainDataConstraintInfo" => {
                        Ok(Classes::hkpPoweredChainDataConstraintInfo(map.next_value()?))
                    }
                    "hkpPoweredChainMapper" => {
                        Ok(Classes::hkpPoweredChainMapper(map.next_value()?))
                    }
                    "hkpPoweredChainMapperLinkInfo" => {
                        Ok(Classes::hkpPoweredChainMapperLinkInfo(map.next_value()?))
                    }
                    "hkpPoweredChainMapperTarget" => {
                        Ok(Classes::hkpPoweredChainMapperTarget(map.next_value()?))
                    }
                    "hkpPrismaticConstraintData" => {
                        Ok(Classes::hkpPrismaticConstraintData(map.next_value()?))
                    }
                    "hkpPrismaticConstraintDataAtoms" => {
                        Ok(Classes::hkpPrismaticConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpProjectileGun" => {
                        Ok(Classes::hkpProjectileGun(map.next_value()?))
                    }
                    "hkpProperty" => Ok(Classes::hkpProperty(map.next_value()?)),
                    "hkpPropertyValue" => {
                        Ok(Classes::hkpPropertyValue(map.next_value()?))
                    }
                    "hkpPulleyConstraintAtom" => {
                        Ok(Classes::hkpPulleyConstraintAtom(map.next_value()?))
                    }
                    "hkpPulleyConstraintData" => {
                        Ok(Classes::hkpPulleyConstraintData(map.next_value()?))
                    }
                    "hkpPulleyConstraintDataAtoms" => {
                        Ok(Classes::hkpPulleyConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpRackAndPinionConstraintAtom" => {
                        Ok(Classes::hkpRackAndPinionConstraintAtom(map.next_value()?))
                    }
                    "hkpRackAndPinionConstraintData" => {
                        Ok(Classes::hkpRackAndPinionConstraintData(map.next_value()?))
                    }
                    "hkpRackAndPinionConstraintDataAtoms" => {
                        Ok(
                            Classes::hkpRackAndPinionConstraintDataAtoms(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpRagdollConstraintData" => {
                        Ok(Classes::hkpRagdollConstraintData(map.next_value()?))
                    }
                    "hkpRagdollConstraintDataAtoms" => {
                        Ok(Classes::hkpRagdollConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpRagdollLimitsData" => {
                        Ok(Classes::hkpRagdollLimitsData(map.next_value()?))
                    }
                    "hkpRagdollLimitsDataAtoms" => {
                        Ok(Classes::hkpRagdollLimitsDataAtoms(map.next_value()?))
                    }
                    "hkpRagdollMotorConstraintAtom" => {
                        Ok(Classes::hkpRagdollMotorConstraintAtom(map.next_value()?))
                    }
                    "hkpRayCollidableFilter" => {
                        Ok(Classes::hkpRayCollidableFilter(map.next_value()?))
                    }
                    "hkpRayShapeCollectionFilter" => {
                        Ok(Classes::hkpRayShapeCollectionFilter(map.next_value()?))
                    }
                    "hkpRejectChassisListener" => {
                        Ok(Classes::hkpRejectChassisListener(map.next_value()?))
                    }
                    "hkpRemoveTerminalsMoppModifier" => {
                        Ok(Classes::hkpRemoveTerminalsMoppModifier(map.next_value()?))
                    }
                    "hkpReorientAction" => {
                        Ok(Classes::hkpReorientAction(map.next_value()?))
                    }
                    "hkpRigidBody" => Ok(Classes::hkpRigidBody(map.next_value()?)),
                    "hkpRotationalConstraintData" => {
                        Ok(Classes::hkpRotationalConstraintData(map.next_value()?))
                    }
                    "hkpRotationalConstraintDataAtoms" => {
                        Ok(Classes::hkpRotationalConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpSampledHeightFieldShape" => {
                        Ok(Classes::hkpSampledHeightFieldShape(map.next_value()?))
                    }
                    "hkpSerializedAgentNnEntry" => {
                        Ok(Classes::hkpSerializedAgentNnEntry(map.next_value()?))
                    }
                    "hkpSerializedDisplayMarker" => {
                        Ok(Classes::hkpSerializedDisplayMarker(map.next_value()?))
                    }
                    "hkpSerializedDisplayMarkerList" => {
                        Ok(Classes::hkpSerializedDisplayMarkerList(map.next_value()?))
                    }
                    "hkpSerializedDisplayRbTransforms" => {
                        Ok(Classes::hkpSerializedDisplayRbTransforms(map.next_value()?))
                    }
                    "hkpSerializedDisplayRbTransformsDisplayTransformPair" => {
                        Ok(
                            Classes::hkpSerializedDisplayRbTransformsDisplayTransformPair(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSerializedSubTrack1nInfo" => {
                        Ok(Classes::hkpSerializedSubTrack1nInfo(map.next_value()?))
                    }
                    "hkpSerializedTrack1nInfo" => {
                        Ok(Classes::hkpSerializedTrack1nInfo(map.next_value()?))
                    }
                    "hkpSetLocalRotationsConstraintAtom" => {
                        Ok(
                            Classes::hkpSetLocalRotationsConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSetLocalTransformsConstraintAtom" => {
                        Ok(
                            Classes::hkpSetLocalTransformsConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSetLocalTranslationsConstraintAtom" => {
                        Ok(
                            Classes::hkpSetLocalTranslationsConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSetupStabilizationAtom" => {
                        Ok(Classes::hkpSetupStabilizationAtom(map.next_value()?))
                    }
                    "hkpShape" => Ok(Classes::hkpShape(map.next_value()?)),
                    "hkpShapeCollection" => {
                        Ok(Classes::hkpShapeCollection(map.next_value()?))
                    }
                    "hkpShapeCollectionFilter" => {
                        Ok(Classes::hkpShapeCollectionFilter(map.next_value()?))
                    }
                    "hkpShapeContainer" => {
                        Ok(Classes::hkpShapeContainer(map.next_value()?))
                    }
                    "hkpShapeInfo" => Ok(Classes::hkpShapeInfo(map.next_value()?)),
                    "hkpShapeModifier" => {
                        Ok(Classes::hkpShapeModifier(map.next_value()?))
                    }
                    "hkpShapePhantom" => Ok(Classes::hkpShapePhantom(map.next_value()?)),
                    "hkpSimpleContactConstraintAtom" => {
                        Ok(Classes::hkpSimpleContactConstraintAtom(map.next_value()?))
                    }
                    "hkpSimpleContactConstraintDataInfo" => {
                        Ok(
                            Classes::hkpSimpleContactConstraintDataInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSimpleMeshShape" => {
                        Ok(Classes::hkpSimpleMeshShape(map.next_value()?))
                    }
                    "hkpSimpleMeshShapeTriangle" => {
                        Ok(Classes::hkpSimpleMeshShapeTriangle(map.next_value()?))
                    }
                    "hkpSimpleShapePhantom" => {
                        Ok(Classes::hkpSimpleShapePhantom(map.next_value()?))
                    }
                    "hkpSimpleShapePhantomCollisionDetail" => {
                        Ok(
                            Classes::hkpSimpleShapePhantomCollisionDetail(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSimulation" => Ok(Classes::hkpSimulation(map.next_value()?)),
                    "hkpSingleShapeContainer" => {
                        Ok(Classes::hkpSingleShapeContainer(map.next_value()?))
                    }
                    "hkpSoftContactModifierConstraintAtom" => {
                        Ok(
                            Classes::hkpSoftContactModifierConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpSphereMotion" => Ok(Classes::hkpSphereMotion(map.next_value()?)),
                    "hkpSphereRepShape" => {
                        Ok(Classes::hkpSphereRepShape(map.next_value()?))
                    }
                    "hkpSphereShape" => Ok(Classes::hkpSphereShape(map.next_value()?)),
                    "hkpSpringAction" => Ok(Classes::hkpSpringAction(map.next_value()?)),
                    "hkpSpringDamperConstraintMotor" => {
                        Ok(Classes::hkpSpringDamperConstraintMotor(map.next_value()?))
                    }
                    "hkpStiffSpringChainData" => {
                        Ok(Classes::hkpStiffSpringChainData(map.next_value()?))
                    }
                    "hkpStiffSpringChainDataConstraintInfo" => {
                        Ok(
                            Classes::hkpStiffSpringChainDataConstraintInfo(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpStiffSpringConstraintAtom" => {
                        Ok(Classes::hkpStiffSpringConstraintAtom(map.next_value()?))
                    }
                    "hkpStiffSpringConstraintData" => {
                        Ok(Classes::hkpStiffSpringConstraintData(map.next_value()?))
                    }
                    "hkpStiffSpringConstraintDataAtoms" => {
                        Ok(Classes::hkpStiffSpringConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpStorageExtendedMeshShape" => {
                        Ok(Classes::hkpStorageExtendedMeshShape(map.next_value()?))
                    }
                    "hkpStorageExtendedMeshShapeMaterial" => {
                        Ok(
                            Classes::hkpStorageExtendedMeshShapeMaterial(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpStorageExtendedMeshShapeMeshSubpartStorage" => {
                        Ok(
                            Classes::hkpStorageExtendedMeshShapeMeshSubpartStorage(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpStorageExtendedMeshShapeShapeSubpartStorage" => {
                        Ok(
                            Classes::hkpStorageExtendedMeshShapeShapeSubpartStorage(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpStorageMeshShape" => {
                        Ok(Classes::hkpStorageMeshShape(map.next_value()?))
                    }
                    "hkpStorageMeshShapeSubpartStorage" => {
                        Ok(Classes::hkpStorageMeshShapeSubpartStorage(map.next_value()?))
                    }
                    "hkpStorageSampledHeightFieldShape" => {
                        Ok(Classes::hkpStorageSampledHeightFieldShape(map.next_value()?))
                    }
                    "hkpThinBoxMotion" => {
                        Ok(Classes::hkpThinBoxMotion(map.next_value()?))
                    }
                    "hkpTransformShape" => {
                        Ok(Classes::hkpTransformShape(map.next_value()?))
                    }
                    "hkpTriangleShape" => {
                        Ok(Classes::hkpTriangleShape(map.next_value()?))
                    }
                    "hkpTriggerVolume" => {
                        Ok(Classes::hkpTriggerVolume(map.next_value()?))
                    }
                    "hkpTriggerVolumeEventInfo" => {
                        Ok(Classes::hkpTriggerVolumeEventInfo(map.next_value()?))
                    }
                    "hkpTriSampledHeightFieldBvTreeShape" => {
                        Ok(
                            Classes::hkpTriSampledHeightFieldBvTreeShape(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpTriSampledHeightFieldCollection" => {
                        Ok(
                            Classes::hkpTriSampledHeightFieldCollection(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpTwistLimitConstraintAtom" => {
                        Ok(Classes::hkpTwistLimitConstraintAtom(map.next_value()?))
                    }
                    "hkpTypedBroadPhaseHandle" => {
                        Ok(Classes::hkpTypedBroadPhaseHandle(map.next_value()?))
                    }
                    "hkpTyremarkPoint" => {
                        Ok(Classes::hkpTyremarkPoint(map.next_value()?))
                    }
                    "hkpTyremarksInfo" => {
                        Ok(Classes::hkpTyremarksInfo(map.next_value()?))
                    }
                    "hkpTyremarksWheel" => {
                        Ok(Classes::hkpTyremarksWheel(map.next_value()?))
                    }
                    "hkpUnaryAction" => Ok(Classes::hkpUnaryAction(map.next_value()?)),
                    "hkpVehicleAerodynamics" => {
                        Ok(Classes::hkpVehicleAerodynamics(map.next_value()?))
                    }
                    "hkpVehicleBrake" => Ok(Classes::hkpVehicleBrake(map.next_value()?)),
                    "hkpVehicleCastBatchingManager" => {
                        Ok(Classes::hkpVehicleCastBatchingManager(map.next_value()?))
                    }
                    "hkpVehicleData" => Ok(Classes::hkpVehicleData(map.next_value()?)),
                    "hkpVehicleDataWheelComponentParams" => {
                        Ok(
                            Classes::hkpVehicleDataWheelComponentParams(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleDefaultAerodynamics" => {
                        Ok(Classes::hkpVehicleDefaultAerodynamics(map.next_value()?))
                    }
                    "hkpVehicleDefaultAnalogDriverInput" => {
                        Ok(
                            Classes::hkpVehicleDefaultAnalogDriverInput(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleDefaultBrake" => {
                        Ok(Classes::hkpVehicleDefaultBrake(map.next_value()?))
                    }
                    "hkpVehicleDefaultBrakeWheelBrakingProperties" => {
                        Ok(
                            Classes::hkpVehicleDefaultBrakeWheelBrakingProperties(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleDefaultEngine" => {
                        Ok(Classes::hkpVehicleDefaultEngine(map.next_value()?))
                    }
                    "hkpVehicleDefaultSteering" => {
                        Ok(Classes::hkpVehicleDefaultSteering(map.next_value()?))
                    }
                    "hkpVehicleDefaultSuspension" => {
                        Ok(Classes::hkpVehicleDefaultSuspension(map.next_value()?))
                    }
                    "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters" => {
                        Ok(
                            Classes::hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleDefaultTransmission" => {
                        Ok(Classes::hkpVehicleDefaultTransmission(map.next_value()?))
                    }
                    "hkpVehicleDefaultVelocityDamper" => {
                        Ok(Classes::hkpVehicleDefaultVelocityDamper(map.next_value()?))
                    }
                    "hkpVehicleDriverInput" => {
                        Ok(Classes::hkpVehicleDriverInput(map.next_value()?))
                    }
                    "hkpVehicleDriverInputAnalogStatus" => {
                        Ok(Classes::hkpVehicleDriverInputAnalogStatus(map.next_value()?))
                    }
                    "hkpVehicleDriverInputStatus" => {
                        Ok(Classes::hkpVehicleDriverInputStatus(map.next_value()?))
                    }
                    "hkpVehicleEngine" => {
                        Ok(Classes::hkpVehicleEngine(map.next_value()?))
                    }
                    "hkpVehicleFrictionDescription" => {
                        Ok(Classes::hkpVehicleFrictionDescription(map.next_value()?))
                    }
                    "hkpVehicleFrictionDescriptionAxisDescription" => {
                        Ok(
                            Classes::hkpVehicleFrictionDescriptionAxisDescription(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleFrictionStatus" => {
                        Ok(Classes::hkpVehicleFrictionStatus(map.next_value()?))
                    }
                    "hkpVehicleFrictionStatusAxisStatus" => {
                        Ok(
                            Classes::hkpVehicleFrictionStatusAxisStatus(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleInstance" => {
                        Ok(Classes::hkpVehicleInstance(map.next_value()?))
                    }
                    "hkpVehicleInstanceWheelInfo" => {
                        Ok(Classes::hkpVehicleInstanceWheelInfo(map.next_value()?))
                    }
                    "hkpVehicleLinearCastBatchingManager" => {
                        Ok(
                            Classes::hkpVehicleLinearCastBatchingManager(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleLinearCastWheelCollide" => {
                        Ok(Classes::hkpVehicleLinearCastWheelCollide(map.next_value()?))
                    }
                    "hkpVehicleLinearCastWheelCollideWheelState" => {
                        Ok(
                            Classes::hkpVehicleLinearCastWheelCollideWheelState(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleManager" => {
                        Ok(Classes::hkpVehicleManager(map.next_value()?))
                    }
                    "hkpVehicleRayCastBatchingManager" => {
                        Ok(Classes::hkpVehicleRayCastBatchingManager(map.next_value()?))
                    }
                    "hkpVehicleRayCastWheelCollide" => {
                        Ok(Classes::hkpVehicleRayCastWheelCollide(map.next_value()?))
                    }
                    "hkpVehicleSteering" => {
                        Ok(Classes::hkpVehicleSteering(map.next_value()?))
                    }
                    "hkpVehicleSuspension" => {
                        Ok(Classes::hkpVehicleSuspension(map.next_value()?))
                    }
                    "hkpVehicleSuspensionSuspensionWheelParameters" => {
                        Ok(
                            Classes::hkpVehicleSuspensionSuspensionWheelParameters(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpVehicleTransmission" => {
                        Ok(Classes::hkpVehicleTransmission(map.next_value()?))
                    }
                    "hkpVehicleVelocityDamper" => {
                        Ok(Classes::hkpVehicleVelocityDamper(map.next_value()?))
                    }
                    "hkpVehicleWheelCollide" => {
                        Ok(Classes::hkpVehicleWheelCollide(map.next_value()?))
                    }
                    "hkpVelocityConstraintMotor" => {
                        Ok(Classes::hkpVelocityConstraintMotor(map.next_value()?))
                    }
                    "hkpViscousSurfaceModifierConstraintAtom" => {
                        Ok(
                            Classes::hkpViscousSurfaceModifierConstraintAtom(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkpWeldingUtility" => {
                        Ok(Classes::hkpWeldingUtility(map.next_value()?))
                    }
                    "hkpWheelConstraintData" => {
                        Ok(Classes::hkpWheelConstraintData(map.next_value()?))
                    }
                    "hkpWheelConstraintDataAtoms" => {
                        Ok(Classes::hkpWheelConstraintDataAtoms(map.next_value()?))
                    }
                    "hkpWorld" => Ok(Classes::hkpWorld(map.next_value()?)),
                    "hkpWorldCinfo" => Ok(Classes::hkpWorldCinfo(map.next_value()?)),
                    "hkpWorldObject" => Ok(Classes::hkpWorldObject(map.next_value()?)),
                    "hkQTransform" => Ok(Classes::hkQTransform(map.next_value()?)),
                    "hkRangeInt32Attribute" => {
                        Ok(Classes::hkRangeInt32Attribute(map.next_value()?))
                    }
                    "hkRangeRealAttribute" => {
                        Ok(Classes::hkRangeRealAttribute(map.next_value()?))
                    }
                    "hkReferencedObject" => {
                        Ok(Classes::hkReferencedObject(map.next_value()?))
                    }
                    "hkReflectedFileAttribute" => {
                        Ok(Classes::hkReflectedFileAttribute(map.next_value()?))
                    }
                    "hkResourceBase" => Ok(Classes::hkResourceBase(map.next_value()?)),
                    "hkResourceContainer" => {
                        Ok(Classes::hkResourceContainer(map.next_value()?))
                    }
                    "hkResourceHandle" => {
                        Ok(Classes::hkResourceHandle(map.next_value()?))
                    }
                    "hkRootLevelContainer" => {
                        Ok(Classes::hkRootLevelContainer(map.next_value()?))
                    }
                    "hkRootLevelContainerNamedVariant" => {
                        Ok(Classes::hkRootLevelContainerNamedVariant(map.next_value()?))
                    }
                    "hkSemanticsAttribute" => {
                        Ok(Classes::hkSemanticsAttribute(map.next_value()?))
                    }
                    "hkSimpleLocalFrame" => {
                        Ok(Classes::hkSimpleLocalFrame(map.next_value()?))
                    }
                    "hkSphere" => Ok(Classes::hkSphere(map.next_value()?)),
                    "hkSweptTransform" => {
                        Ok(Classes::hkSweptTransform(map.next_value()?))
                    }
                    "hkTraceStreamTitle" => {
                        Ok(Classes::hkTraceStreamTitle(map.next_value()?))
                    }
                    "hkTrackerSerializableScanSnapshot" => {
                        Ok(Classes::hkTrackerSerializableScanSnapshot(map.next_value()?))
                    }
                    "hkTrackerSerializableScanSnapshotAllocation" => {
                        Ok(
                            Classes::hkTrackerSerializableScanSnapshotAllocation(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkTrackerSerializableScanSnapshotBlock" => {
                        Ok(
                            Classes::hkTrackerSerializableScanSnapshotBlock(
                                map.next_value()?,
                            ),
                        )
                    }
                    "hkUiAttribute" => Ok(Classes::hkUiAttribute(map.next_value()?)),
                    "hkVertexFormat" => Ok(Classes::hkVertexFormat(map.next_value()?)),
                    "hkVertexFormatElement" => {
                        Ok(Classes::hkVertexFormatElement(map.next_value()?))
                    }
                    "hkWorldMemoryAvailableWatchDog" => {
                        Ok(Classes::hkWorldMemoryAvailableWatchDog(map.next_value()?))
                    }
                    "hkxAnimatedFloat" => {
                        Ok(Classes::hkxAnimatedFloat(map.next_value()?))
                    }
                    "hkxAnimatedMatrix" => {
                        Ok(Classes::hkxAnimatedMatrix(map.next_value()?))
                    }
                    "hkxAnimatedQuaternion" => {
                        Ok(Classes::hkxAnimatedQuaternion(map.next_value()?))
                    }
                    "hkxAnimatedVector" => {
                        Ok(Classes::hkxAnimatedVector(map.next_value()?))
                    }
                    "hkxAttribute" => Ok(Classes::hkxAttribute(map.next_value()?)),
                    "hkxAttributeGroup" => {
                        Ok(Classes::hkxAttributeGroup(map.next_value()?))
                    }
                    "hkxAttributeHolder" => {
                        Ok(Classes::hkxAttributeHolder(map.next_value()?))
                    }
                    "hkxCamera" => Ok(Classes::hkxCamera(map.next_value()?)),
                    "hkxEdgeSelectionChannel" => {
                        Ok(Classes::hkxEdgeSelectionChannel(map.next_value()?))
                    }
                    "hkxEnum" => Ok(Classes::hkxEnum(map.next_value()?)),
                    "hkxEnumItem" => Ok(Classes::hkxEnumItem(map.next_value()?)),
                    "hkxEnvironment" => Ok(Classes::hkxEnvironment(map.next_value()?)),
                    "hkxEnvironmentVariable" => {
                        Ok(Classes::hkxEnvironmentVariable(map.next_value()?))
                    }
                    "hkxIndexBuffer" => Ok(Classes::hkxIndexBuffer(map.next_value()?)),
                    "hkxLight" => Ok(Classes::hkxLight(map.next_value()?)),
                    "hkxMaterial" => Ok(Classes::hkxMaterial(map.next_value()?)),
                    "hkxMaterialEffect" => {
                        Ok(Classes::hkxMaterialEffect(map.next_value()?))
                    }
                    "hkxMaterialProperty" => {
                        Ok(Classes::hkxMaterialProperty(map.next_value()?))
                    }
                    "hkxMaterialShader" => {
                        Ok(Classes::hkxMaterialShader(map.next_value()?))
                    }
                    "hkxMaterialShaderSet" => {
                        Ok(Classes::hkxMaterialShaderSet(map.next_value()?))
                    }
                    "hkxMaterialTextureStage" => {
                        Ok(Classes::hkxMaterialTextureStage(map.next_value()?))
                    }
                    "hkxMesh" => Ok(Classes::hkxMesh(map.next_value()?)),
                    "hkxMeshSection" => Ok(Classes::hkxMeshSection(map.next_value()?)),
                    "hkxMeshUserChannelInfo" => {
                        Ok(Classes::hkxMeshUserChannelInfo(map.next_value()?))
                    }
                    "hkxNode" => Ok(Classes::hkxNode(map.next_value()?)),
                    "hkxNodeAnnotationData" => {
                        Ok(Classes::hkxNodeAnnotationData(map.next_value()?))
                    }
                    "hkxNodeSelectionSet" => {
                        Ok(Classes::hkxNodeSelectionSet(map.next_value()?))
                    }
                    "hkxScene" => Ok(Classes::hkxScene(map.next_value()?)),
                    "hkxSkinBinding" => Ok(Classes::hkxSkinBinding(map.next_value()?)),
                    "hkxSparselyAnimatedBool" => {
                        Ok(Classes::hkxSparselyAnimatedBool(map.next_value()?))
                    }
                    "hkxSparselyAnimatedEnum" => {
                        Ok(Classes::hkxSparselyAnimatedEnum(map.next_value()?))
                    }
                    "hkxSparselyAnimatedInt" => {
                        Ok(Classes::hkxSparselyAnimatedInt(map.next_value()?))
                    }
                    "hkxSparselyAnimatedString" => {
                        Ok(Classes::hkxSparselyAnimatedString(map.next_value()?))
                    }
                    "hkxTextureFile" => Ok(Classes::hkxTextureFile(map.next_value()?)),
                    "hkxTextureInplace" => {
                        Ok(Classes::hkxTextureInplace(map.next_value()?))
                    }
                    "hkxTriangleSelectionChannel" => {
                        Ok(Classes::hkxTriangleSelectionChannel(map.next_value()?))
                    }
                    "hkxVertexBuffer" => Ok(Classes::hkxVertexBuffer(map.next_value()?)),
                    "hkxVertexBufferVertexData" => {
                        Ok(Classes::hkxVertexBufferVertexData(map.next_value()?))
                    }
                    "hkxVertexDescription" => {
                        Ok(Classes::hkxVertexDescription(map.next_value()?))
                    }
                    "hkxVertexDescriptionElementDecl" => {
                        Ok(Classes::hkxVertexDescriptionElementDecl(map.next_value()?))
                    }
                    "hkxVertexFloatDataChannel" => {
                        Ok(Classes::hkxVertexFloatDataChannel(map.next_value()?))
                    }
                    "hkxVertexIntDataChannel" => {
                        Ok(Classes::hkxVertexIntDataChannel(map.next_value()?))
                    }
                    "hkxVertexSelectionChannel" => {
                        Ok(Classes::hkxVertexSelectionChannel(map.next_value()?))
                    }
                    "hkxVertexVectorDataChannel" => {
                        Ok(Classes::hkxVertexVectorDataChannel(map.next_value()?))
                    }
                    _ => {
                        Err(
                            _serde::de::Error::unknown_field(
                                class_name,
                                &[
                                    "BGSGamebryoSequenceGenerator",
                                    "BSBoneSwitchGenerator",
                                    "BSBoneSwitchGeneratorBoneData",
                                    "BSComputeAddBoneAnimModifier",
                                    "BSCyclicBlendTransitionGenerator",
                                    "BSDecomposeVectorModifier",
                                    "BSDirectAtModifier",
                                    "BSDistTriggerModifier",
                                    "BSEventEveryNEventsModifier",
                                    "BSEventOnDeactivateModifier",
                                    "BSEventOnFalseToTrueModifier",
                                    "BSGetTimeStepModifier",
                                    "BSInterpValueModifier",
                                    "BSIsActiveModifier",
                                    "BSIStateManagerModifier",
                                    "BSIStateManagerModifierBSiStateData",
                                    "BSIStateManagerModifierBSIStateManagerStateListener",
                                    "BSiStateTaggingGenerator",
                                    "BSLimbIKModifier",
                                    "BSLookAtModifier",
                                    "BSLookAtModifierBoneData",
                                    "BSModifyOnceModifier",
                                    "BSOffsetAnimationGenerator",
                                    "BSPassByTargetTriggerModifier",
                                    "BSRagdollContactListenerModifier",
                                    "BSSpeedSamplerModifier",
                                    "BSSynchronizedClipGenerator",
                                    "BSTimerModifier",
                                    "BSTweenerModifier",
                                    "hkAabb",
                                    "hkAabbHalf",
                                    "hkAabbUint32",
                                    "hkaAnimatedReferenceFrame",
                                    "hkaAnimation",
                                    "hkaAnimationBinding",
                                    "hkaAnimationContainer",
                                    "hkaAnimationPreviewColorContainer",
                                    "hkaAnnotationTrack",
                                    "hkaAnnotationTrackAnnotation",
                                    "hkaBone",
                                    "hkaBoneAttachment",
                                    "hkaDefaultAnimatedReferenceFrame",
                                    "hkaDeltaCompressedAnimation",
                                    "hkaDeltaCompressedAnimationQuantizationFormat",
                                    "hkaFootstepAnalysisInfo",
                                    "hkaFootstepAnalysisInfoContainer",
                                    "hkaInterleavedUncompressedAnimation",
                                    "hkaKeyFrameHierarchyUtility",
                                    "hkaKeyFrameHierarchyUtilityControlData",
                                    "hkAlignSceneToNodeOptions",
                                    "hkaMeshBinding",
                                    "hkaMeshBindingMapping",
                                    "hkaQuantizedAnimation",
                                    "hkaQuantizedAnimationTrackCompressionParams",
                                    "hkaRagdollInstance",
                                    "hkArrayTypeAttribute",
                                    "hkaSkeleton",
                                    "hkaSkeletonLocalFrameOnBone",
                                    "hkaSkeletonMapper",
                                    "hkaSkeletonMapperData",
                                    "hkaSkeletonMapperDataChainMapping",
                                    "hkaSkeletonMapperDataSimpleMapping",
                                    "hkaSplineCompressedAnimation",
                                    "hkaSplineCompressedAnimationAnimationCompressionParams",
                                    "hkaSplineCompressedAnimationTrackCompressionParams",
                                    "hkaWaveletCompressedAnimation",
                                    "hkaWaveletCompressedAnimationCompressionParams",
                                    "hkaWaveletCompressedAnimationQuantizationFormat",
                                    "hkBaseObject",
                                    "hkbAttachmentModifier",
                                    "hkbAttachmentSetup",
                                    "hkbAttributeModifier",
                                    "hkbAttributeModifierAssignment",
                                    "hkbAuxiliaryNodeInfo",
                                    "hkbBehaviorEventsInfo",
                                    "hkbBehaviorGraph",
                                    "hkbBehaviorGraphData",
                                    "hkbBehaviorGraphInternalState",
                                    "hkbBehaviorGraphInternalStateInfo",
                                    "hkbBehaviorGraphStringData",
                                    "hkbBehaviorInfo",
                                    "hkbBehaviorInfoIdToNamePair",
                                    "hkbBehaviorReferenceGenerator",
                                    "hkbBindable",
                                    "hkbBlendCurveUtils",
                                    "hkbBlenderGenerator",
                                    "hkbBlenderGeneratorChild",
                                    "hkbBlenderGeneratorChildInternalState",
                                    "hkbBlenderGeneratorInternalState",
                                    "hkbBlendingTransitionEffect",
                                    "hkbBlendingTransitionEffectInternalState",
                                    "hkbBoneIndexArray",
                                    "hkbBoneWeightArray",
                                    "hkbBoolVariableSequencedData",
                                    "hkbBoolVariableSequencedDataSample",
                                    "hkbCameraShakeEventPayload",
                                    "hkbCharacter",
                                    "hkbCharacterAddedInfo",
                                    "hkbCharacterControlCommand",
                                    "hkbCharacterControllerControlData",
                                    "hkbCharacterControllerModifier",
                                    "hkbCharacterControllerModifierInternalState",
                                    "hkbCharacterData",
                                    "hkbCharacterDataCharacterControllerInfo",
                                    "hkbCharacterInfo",
                                    "hkbCharacterSetup",
                                    "hkbCharacterSkinInfo",
                                    "hkbCharacterSteppedInfo",
                                    "hkbCharacterStringData",
                                    "hkbClientCharacterState",
                                    "hkbClipGenerator",
                                    "hkbClipGeneratorEcho",
                                    "hkbClipGeneratorInternalState",
                                    "hkbClipTrigger",
                                    "hkbClipTriggerArray",
                                    "hkbCombineTransformsModifier",
                                    "hkbCombineTransformsModifierInternalState",
                                    "hkbCompiledExpressionSet",
                                    "hkbCompiledExpressionSetToken",
                                    "hkbComputeDirectionModifier",
                                    "hkbComputeDirectionModifierInternalState",
                                    "hkbComputeRotationFromAxisAngleModifier",
                                    "hkbComputeRotationFromAxisAngleModifierInternalState",
                                    "hkbComputeRotationToTargetModifier",
                                    "hkbComputeRotationToTargetModifierInternalState",
                                    "hkbCondition",
                                    "hkbContext",
                                    "hkbDampingModifier",
                                    "hkbDampingModifierInternalState",
                                    "hkbDefaultMessageLog",
                                    "hkbDelayedModifier",
                                    "hkbDelayedModifierInternalState",
                                    "hkbDetectCloseToGroundModifier",
                                    "hkbDetectCloseToGroundModifierInternalState",
                                    "hkbEvaluateExpressionModifier",
                                    "hkbEvaluateExpressionModifierInternalExpressionData",
                                    "hkbEvaluateExpressionModifierInternalState",
                                    "hkbEvaluateHandleModifier",
                                    "hkbEvent",
                                    "hkbEventBase",
                                    "hkbEventDrivenModifier",
                                    "hkbEventDrivenModifierInternalState",
                                    "hkbEventInfo",
                                    "hkbEventPayload",
                                    "hkbEventPayloadList",
                                    "hkbEventProperty",
                                    "hkbEventRaisedInfo",
                                    "hkbEventRangeData",
                                    "hkbEventRangeDataArray",
                                    "hkbEventSequencedData",
                                    "hkbEventSequencedDataSequencedEvent",
                                    "hkbEventsFromRangeModifier",
                                    "hkbEventsFromRangeModifierInternalState",
                                    "hkbExpressionCondition",
                                    "hkbExpressionData",
                                    "hkbExpressionDataArray",
                                    "hkbExtractRagdollPoseModifier",
                                    "hkbFootIkControlData",
                                    "hkbFootIkControlsModifier",
                                    "hkbFootIkControlsModifierLeg",
                                    "hkbFootIkDriverInfo",
                                    "hkbFootIkDriverInfoLeg",
                                    "hkbFootIkGains",
                                    "hkbFootIkModifier",
                                    "hkbFootIkModifierInternalLegData",
                                    "hkbFootIkModifierLeg",
                                    "hkbGenerator",
                                    "hkbGeneratorOutputListener",
                                    "hkbGeneratorSyncInfo",
                                    "hkbGeneratorSyncInfoSyncPoint",
                                    "hkbGeneratorTransitionEffect",
                                    "hkbGeneratorTransitionEffectInternalState",
                                    "hkbGetHandleOnBoneModifier",
                                    "hkbGetUpModifier",
                                    "hkbGetUpModifierInternalState",
                                    "hkbGetWorldFromModelModifier",
                                    "hkbGetWorldFromModelModifierInternalState",
                                    "hkbHandIkControlData",
                                    "hkbHandIkControlsModifier",
                                    "hkbHandIkControlsModifierHand",
                                    "hkbHandIkDriverInfo",
                                    "hkbHandIkDriverInfoHand",
                                    "hkbHandIkModifier",
                                    "hkbHandIkModifierHand",
                                    "hkbHandle",
                                    "hkbIntEventPayload",
                                    "hkbIntVariableSequencedData",
                                    "hkbIntVariableSequencedDataSample",
                                    "hkBitField",
                                    "hkbKeyframeBonesModifier",
                                    "hkbKeyframeBonesModifierKeyframeInfo",
                                    "hkbLinkedSymbolInfo",
                                    "hkbLookAtModifier",
                                    "hkbLookAtModifierInternalState",
                                    "hkbManualSelectorGenerator",
                                    "hkbManualSelectorGeneratorInternalState",
                                    "hkbMessageLog",
                                    "hkbMirroredSkeletonInfo",
                                    "hkbMirrorModifier",
                                    "hkbModifier",
                                    "hkbModifierGenerator",
                                    "hkbModifierList",
                                    "hkbModifierWrapper",
                                    "hkbMoveCharacterModifier",
                                    "hkbMoveCharacterModifierInternalState",
                                    "hkbNamedEventPayload",
                                    "hkbNamedIntEventPayload",
                                    "hkbNamedRealEventPayload",
                                    "hkbNamedStringEventPayload",
                                    "hkbNode",
                                    "hkbNodeInternalStateInfo",
                                    "hkbParticleSystemEventPayload",
                                    "hkbPoseMatchingGenerator",
                                    "hkbPoseMatchingGeneratorInternalState",
                                    "hkbPoweredRagdollControlData",
                                    "hkbPoweredRagdollControlsModifier",
                                    "hkbProjectData",
                                    "hkbProjectStringData",
                                    "hkbProxyModifier",
                                    "hkbProxyModifierProxyInfo",
                                    "hkbRaiseEventCommand",
                                    "hkbRealEventPayload",
                                    "hkbRealVariableSequencedData",
                                    "hkbRealVariableSequencedDataSample",
                                    "hkbReferencePoseGenerator",
                                    "hkbRegisteredGenerator",
                                    "hkbRigidBodyRagdollControlData",
                                    "hkbRigidBodyRagdollControlsModifier",
                                    "hkbRoleAttribute",
                                    "hkbRotateCharacterModifier",
                                    "hkbRotateCharacterModifierInternalState",
                                    "hkbSenseHandleModifier",
                                    "hkbSenseHandleModifierRange",
                                    "hkbSequence",
                                    "hkbSequencedData",
                                    "hkbSequenceInternalState",
                                    "hkbSequenceStringData",
                                    "hkbSetBehaviorCommand",
                                    "hkbSetLocalTimeOfClipGeneratorCommand",
                                    "hkbSetNodePropertyCommand",
                                    "hkbSetWordVariableCommand",
                                    "hkbSetWorldFromModelModifier",
                                    "hkbSimulationControlCommand",
                                    "hkbSimulationStateInfo",
                                    "hkbStateChooser",
                                    "hkbStateListener",
                                    "hkbStateMachine",
                                    "hkbStateMachineActiveTransitionInfo",
                                    "hkbStateMachineDelayedTransitionInfo",
                                    "hkbStateMachineEventPropertyArray",
                                    "hkbStateMachineInternalState",
                                    "hkbStateMachineNestedStateMachineData",
                                    "hkbStateMachineProspectiveTransitionInfo",
                                    "hkbStateMachineStateInfo",
                                    "hkbStateMachineTimeInterval",
                                    "hkbStateMachineTransitionInfo",
                                    "hkbStateMachineTransitionInfoArray",
                                    "hkbStateMachineTransitionInfoReference",
                                    "hkbStringCondition",
                                    "hkbStringEventPayload",
                                    "hkbTestStateChooser",
                                    "hkbTimerModifier",
                                    "hkbTimerModifierInternalState",
                                    "hkbTransformVectorModifier",
                                    "hkbTransformVectorModifierInternalState",
                                    "hkbTransitionEffect",
                                    "hkbTwistModifier",
                                    "hkbVariableBindingSet",
                                    "hkbVariableBindingSetBinding",
                                    "hkbVariableInfo",
                                    "hkbVariableValue",
                                    "hkbVariableValueSet",
                                    "hkbWorldEnums",
                                    "hkbWorldFromModelModeData",
                                    "hkClass",
                                    "hkClassEnum",
                                    "hkClassEnumItem",
                                    "hkClassMember",
                                    "hkColor",
                                    "hkContactPoint",
                                    "hkContactPointMaterial",
                                    "hkCustomAttributes",
                                    "hkCustomAttributesAttribute",
                                    "hkDataObjectTypeAttribute",
                                    "hkDescriptionAttribute",
                                    "hkDocumentationAttribute",
                                    "hkGeometry",
                                    "hkGeometryTriangle",
                                    "hkGizmoAttribute",
                                    "hkHalf8",
                                    "hkIndexedTransformSet",
                                    "hkLinkAttribute",
                                    "hkLocalFrame",
                                    "hkLocalFrameGroup",
                                    "hkMemoryMeshBody",
                                    "hkMemoryMeshMaterial",
                                    "hkMemoryMeshShape",
                                    "hkMemoryMeshTexture",
                                    "hkMemoryMeshVertexBuffer",
                                    "hkMemoryResourceContainer",
                                    "hkMemoryResourceHandle",
                                    "hkMemoryResourceHandleExternalLink",
                                    "hkMemoryTrackerAttribute",
                                    "hkMeshBody",
                                    "hkMeshBoneIndexMapping",
                                    "hkMeshMaterial",
                                    "hkMeshSection",
                                    "hkMeshSectionCinfo",
                                    "hkMeshShape",
                                    "hkMeshTexture",
                                    "hkMeshVertexBuffer",
                                    "hkModelerNodeTypeAttribute",
                                    "hkMonitorStreamColorTable",
                                    "hkMonitorStreamColorTableColorPair",
                                    "hkMonitorStreamFrameInfo",
                                    "hkMonitorStreamStringMap",
                                    "hkMonitorStreamStringMapStringMap",
                                    "hkMoppBvTreeShapeBase",
                                    "hkMotionState",
                                    "hkMultipleVertexBuffer",
                                    "hkMultipleVertexBufferElementInfo",
                                    "hkMultipleVertexBufferLockedElement",
                                    "hkMultipleVertexBufferVertexBufferInfo",
                                    "hkMultiThreadCheck",
                                    "hkp2dAngConstraintAtom",
                                    "hkpAabbPhantom",
                                    "hkPackedVector3",
                                    "hkPackfileHeader",
                                    "hkPackfileSectionHeader",
                                    "hkpAction",
                                    "hkpAgent1nSector",
                                    "hkpAngConstraintAtom",
                                    "hkpAngFrictionConstraintAtom",
                                    "hkpAngLimitConstraintAtom",
                                    "hkpAngMotorConstraintAtom",
                                    "hkpAngularDashpotAction",
                                    "hkpArrayAction",
                                    "hkpBallAndSocketConstraintData",
                                    "hkpBallAndSocketConstraintDataAtoms",
                                    "hkpBallGun",
                                    "hkpBallSocketChainData",
                                    "hkpBallSocketChainDataConstraintInfo",
                                    "hkpBallSocketConstraintAtom",
                                    "hkpBinaryAction",
                                    "hkpBoxMotion",
                                    "hkpBoxShape",
                                    "hkpBreakableBody",
                                    "hkpBreakableConstraintData",
                                    "hkpBridgeAtoms",
                                    "hkpBridgeConstraintAtom",
                                    "hkpBroadPhaseHandle",
                                    "hkpBvShape",
                                    "hkpBvTreeShape",
                                    "hkpCachingShapePhantom",
                                    "hkpCallbackConstraintMotor",
                                    "hkpCapsuleShape",
                                    "hkpCdBody",
                                    "hkpCenterOfMassChangerModifierConstraintAtom",
                                    "hkpCharacterControllerCinfo",
                                    "hkpCharacterMotion",
                                    "hkpCharacterProxyCinfo",
                                    "hkpCharacterRigidBodyCinfo",
                                    "hkpCogWheelConstraintAtom",
                                    "hkpCogWheelConstraintData",
                                    "hkpCogWheelConstraintDataAtoms",
                                    "hkpCollidable",
                                    "hkpCollidableBoundingVolumeData",
                                    "hkpCollidableCollidableFilter",
                                    "hkpCollisionFilter",
                                    "hkpCollisionFilterList",
                                    "hkpCompressedMeshShape",
                                    "hkpCompressedMeshShapeBigTriangle",
                                    "hkpCompressedMeshShapeChunk",
                                    "hkpCompressedMeshShapeConvexPiece",
                                    "hkpCompressedSampledHeightFieldShape",
                                    "hkpConeLimitConstraintAtom",
                                    "hkpConstrainedSystemFilter",
                                    "hkpConstraintAtom",
                                    "hkpConstraintChainData",
                                    "hkpConstraintChainInstance",
                                    "hkpConstraintChainInstanceAction",
                                    "hkpConstraintCollisionFilter",
                                    "hkpConstraintData",
                                    "hkpConstraintInstance",
                                    "hkpConstraintInstanceSmallArraySerializeOverrideType",
                                    "hkpConstraintMotor",
                                    "hkpConvexListFilter",
                                    "hkpConvexListShape",
                                    "hkpConvexPieceMeshShape",
                                    "hkpConvexPieceStreamData",
                                    "hkpConvexShape",
                                    "hkpConvexTransformShape",
                                    "hkpConvexTransformShapeBase",
                                    "hkpConvexTranslateShape",
                                    "hkpConvexVerticesConnectivity",
                                    "hkpConvexVerticesShape",
                                    "hkpConvexVerticesShapeFourVectors",
                                    "hkpCylinderShape",
                                    "hkpDashpotAction",
                                    "hkpDefaultConvexListFilter",
                                    "hkpDefaultWorldMemoryWatchDog",
                                    "hkpDisableEntityCollisionFilter",
                                    "hkpDisplayBindingData",
                                    "hkpDisplayBindingDataPhysicsSystem",
                                    "hkpDisplayBindingDataRigidBody",
                                    "hkpEntity",
                                    "hkpEntityExtendedListeners",
                                    "hkpEntitySmallArraySerializeOverrideType",
                                    "hkpEntitySpuCollisionCallback",
                                    "hkpExtendedMeshShape",
                                    "hkpExtendedMeshShapeShapesSubpart",
                                    "hkpExtendedMeshShapeSubpart",
                                    "hkpExtendedMeshShapeTrianglesSubpart",
                                    "hkpFastMeshShape",
                                    "hkpFirstPersonGun",
                                    "hkpFixedRigidMotion",
                                    "hkpGenericConstraintData",
                                    "hkpGenericConstraintDataScheme",
                                    "hkpGenericConstraintDataSchemeConstraintInfo",
                                    "hkpGravityGun",
                                    "hkpGroupCollisionFilter",
                                    "hkpGroupFilter",
                                    "hkpHeightFieldShape",
                                    "hkpHingeConstraintData",
                                    "hkpHingeConstraintDataAtoms",
                                    "hkpHingeLimitsData",
                                    "hkpHingeLimitsDataAtoms",
                                    "hkpIgnoreModifierConstraintAtom",
                                    "hkpKeyframedRigidMotion",
                                    "hkpLimitedForceConstraintMotor",
                                    "hkpLimitedHingeConstraintData",
                                    "hkpLimitedHingeConstraintDataAtoms",
                                    "hkpLinConstraintAtom",
                                    "hkpLinearParametricCurve",
                                    "hkpLinFrictionConstraintAtom",
                                    "hkpLinkedCollidable",
                                    "hkpLinLimitConstraintAtom",
                                    "hkpLinMotorConstraintAtom",
                                    "hkpLinSoftConstraintAtom",
                                    "hkpListShape",
                                    "hkpListShapeChildInfo",
                                    "hkpMalleableConstraintData",
                                    "hkpMassChangerModifierConstraintAtom",
                                    "hkpMassProperties",
                                    "hkpMaterial",
                                    "hkpMaxSizeMotion",
                                    "hkpMeshMaterial",
                                    "hkpMeshShape",
                                    "hkpMeshShapeSubpart",
                                    "hkpModifierConstraintAtom",
                                    "hkpMoppBvTreeShape",
                                    "hkpMoppCode",
                                    "hkpMoppCodeCodeInfo",
                                    "hkpMoppCodeReindexedTerminal",
                                    "hkpMotion",
                                    "hkpMotorAction",
                                    "hkpMountedBallGun",
                                    "hkpMouseSpringAction",
                                    "hkpMovingSurfaceModifierConstraintAtom",
                                    "hkpMultiRayShape",
                                    "hkpMultiRayShapeRay",
                                    "hkpMultiSphereShape",
                                    "hkpMultithreadedVehicleManager",
                                    "hkpNamedMeshMaterial",
                                    "hkpNullCollisionFilter",
                                    "hkPostFinishAttribute",
                                    "hkpOverwritePivotConstraintAtom",
                                    "hkpPairCollisionFilter",
                                    "hkpPairCollisionFilterMapPairFilterKeyOverrideType",
                                    "hkpParametricCurve",
                                    "hkpPhantom",
                                    "hkpPhantomCallbackShape",
                                    "hkpPhysicsData",
                                    "hkpPhysicsSystem",
                                    "hkpPhysicsSystemWithContacts",
                                    "hkpPlaneShape",
                                    "hkpPointToPathConstraintData",
                                    "hkpPointToPlaneConstraintData",
                                    "hkpPointToPlaneConstraintDataAtoms",
                                    "hkpPositionConstraintMotor",
                                    "hkpPoweredChainData",
                                    "hkpPoweredChainDataConstraintInfo",
                                    "hkpPoweredChainMapper",
                                    "hkpPoweredChainMapperLinkInfo",
                                    "hkpPoweredChainMapperTarget",
                                    "hkpPrismaticConstraintData",
                                    "hkpPrismaticConstraintDataAtoms",
                                    "hkpProjectileGun",
                                    "hkpProperty",
                                    "hkpPropertyValue",
                                    "hkpPulleyConstraintAtom",
                                    "hkpPulleyConstraintData",
                                    "hkpPulleyConstraintDataAtoms",
                                    "hkpRackAndPinionConstraintAtom",
                                    "hkpRackAndPinionConstraintData",
                                    "hkpRackAndPinionConstraintDataAtoms",
                                    "hkpRagdollConstraintData",
                                    "hkpRagdollConstraintDataAtoms",
                                    "hkpRagdollLimitsData",
                                    "hkpRagdollLimitsDataAtoms",
                                    "hkpRagdollMotorConstraintAtom",
                                    "hkpRayCollidableFilter",
                                    "hkpRayShapeCollectionFilter",
                                    "hkpRejectChassisListener",
                                    "hkpRemoveTerminalsMoppModifier",
                                    "hkpReorientAction",
                                    "hkpRigidBody",
                                    "hkpRotationalConstraintData",
                                    "hkpRotationalConstraintDataAtoms",
                                    "hkpSampledHeightFieldShape",
                                    "hkpSerializedAgentNnEntry",
                                    "hkpSerializedDisplayMarker",
                                    "hkpSerializedDisplayMarkerList",
                                    "hkpSerializedDisplayRbTransforms",
                                    "hkpSerializedDisplayRbTransformsDisplayTransformPair",
                                    "hkpSerializedSubTrack1nInfo",
                                    "hkpSerializedTrack1nInfo",
                                    "hkpSetLocalRotationsConstraintAtom",
                                    "hkpSetLocalTransformsConstraintAtom",
                                    "hkpSetLocalTranslationsConstraintAtom",
                                    "hkpSetupStabilizationAtom",
                                    "hkpShape",
                                    "hkpShapeCollection",
                                    "hkpShapeCollectionFilter",
                                    "hkpShapeContainer",
                                    "hkpShapeInfo",
                                    "hkpShapeModifier",
                                    "hkpShapePhantom",
                                    "hkpSimpleContactConstraintAtom",
                                    "hkpSimpleContactConstraintDataInfo",
                                    "hkpSimpleMeshShape",
                                    "hkpSimpleMeshShapeTriangle",
                                    "hkpSimpleShapePhantom",
                                    "hkpSimpleShapePhantomCollisionDetail",
                                    "hkpSimulation",
                                    "hkpSingleShapeContainer",
                                    "hkpSoftContactModifierConstraintAtom",
                                    "hkpSphereMotion",
                                    "hkpSphereRepShape",
                                    "hkpSphereShape",
                                    "hkpSpringAction",
                                    "hkpSpringDamperConstraintMotor",
                                    "hkpStiffSpringChainData",
                                    "hkpStiffSpringChainDataConstraintInfo",
                                    "hkpStiffSpringConstraintAtom",
                                    "hkpStiffSpringConstraintData",
                                    "hkpStiffSpringConstraintDataAtoms",
                                    "hkpStorageExtendedMeshShape",
                                    "hkpStorageExtendedMeshShapeMaterial",
                                    "hkpStorageExtendedMeshShapeMeshSubpartStorage",
                                    "hkpStorageExtendedMeshShapeShapeSubpartStorage",
                                    "hkpStorageMeshShape",
                                    "hkpStorageMeshShapeSubpartStorage",
                                    "hkpStorageSampledHeightFieldShape",
                                    "hkpThinBoxMotion",
                                    "hkpTransformShape",
                                    "hkpTriangleShape",
                                    "hkpTriggerVolume",
                                    "hkpTriggerVolumeEventInfo",
                                    "hkpTriSampledHeightFieldBvTreeShape",
                                    "hkpTriSampledHeightFieldCollection",
                                    "hkpTwistLimitConstraintAtom",
                                    "hkpTypedBroadPhaseHandle",
                                    "hkpTyremarkPoint",
                                    "hkpTyremarksInfo",
                                    "hkpTyremarksWheel",
                                    "hkpUnaryAction",
                                    "hkpVehicleAerodynamics",
                                    "hkpVehicleBrake",
                                    "hkpVehicleCastBatchingManager",
                                    "hkpVehicleData",
                                    "hkpVehicleDataWheelComponentParams",
                                    "hkpVehicleDefaultAerodynamics",
                                    "hkpVehicleDefaultAnalogDriverInput",
                                    "hkpVehicleDefaultBrake",
                                    "hkpVehicleDefaultBrakeWheelBrakingProperties",
                                    "hkpVehicleDefaultEngine",
                                    "hkpVehicleDefaultSteering",
                                    "hkpVehicleDefaultSuspension",
                                    "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters",
                                    "hkpVehicleDefaultTransmission",
                                    "hkpVehicleDefaultVelocityDamper",
                                    "hkpVehicleDriverInput",
                                    "hkpVehicleDriverInputAnalogStatus",
                                    "hkpVehicleDriverInputStatus",
                                    "hkpVehicleEngine",
                                    "hkpVehicleFrictionDescription",
                                    "hkpVehicleFrictionDescriptionAxisDescription",
                                    "hkpVehicleFrictionStatus",
                                    "hkpVehicleFrictionStatusAxisStatus",
                                    "hkpVehicleInstance",
                                    "hkpVehicleInstanceWheelInfo",
                                    "hkpVehicleLinearCastBatchingManager",
                                    "hkpVehicleLinearCastWheelCollide",
                                    "hkpVehicleLinearCastWheelCollideWheelState",
                                    "hkpVehicleManager",
                                    "hkpVehicleRayCastBatchingManager",
                                    "hkpVehicleRayCastWheelCollide",
                                    "hkpVehicleSteering",
                                    "hkpVehicleSuspension",
                                    "hkpVehicleSuspensionSuspensionWheelParameters",
                                    "hkpVehicleTransmission",
                                    "hkpVehicleVelocityDamper",
                                    "hkpVehicleWheelCollide",
                                    "hkpVelocityConstraintMotor",
                                    "hkpViscousSurfaceModifierConstraintAtom",
                                    "hkpWeldingUtility",
                                    "hkpWheelConstraintData",
                                    "hkpWheelConstraintDataAtoms",
                                    "hkpWorld",
                                    "hkpWorldCinfo",
                                    "hkpWorldObject",
                                    "hkQTransform",
                                    "hkRangeInt32Attribute",
                                    "hkRangeRealAttribute",
                                    "hkReferencedObject",
                                    "hkReflectedFileAttribute",
                                    "hkResourceBase",
                                    "hkResourceContainer",
                                    "hkResourceHandle",
                                    "hkRootLevelContainer",
                                    "hkRootLevelContainerNamedVariant",
                                    "hkSemanticsAttribute",
                                    "hkSimpleLocalFrame",
                                    "hkSphere",
                                    "hkSweptTransform",
                                    "hkTraceStreamTitle",
                                    "hkTrackerSerializableScanSnapshot",
                                    "hkTrackerSerializableScanSnapshotAllocation",
                                    "hkTrackerSerializableScanSnapshotBlock",
                                    "hkUiAttribute",
                                    "hkVertexFormat",
                                    "hkVertexFormatElement",
                                    "hkWorldMemoryAvailableWatchDog",
                                    "hkxAnimatedFloat",
                                    "hkxAnimatedMatrix",
                                    "hkxAnimatedQuaternion",
                                    "hkxAnimatedVector",
                                    "hkxAttribute",
                                    "hkxAttributeGroup",
                                    "hkxAttributeHolder",
                                    "hkxCamera",
                                    "hkxEdgeSelectionChannel",
                                    "hkxEnum",
                                    "hkxEnumItem",
                                    "hkxEnvironment",
                                    "hkxEnvironmentVariable",
                                    "hkxIndexBuffer",
                                    "hkxLight",
                                    "hkxMaterial",
                                    "hkxMaterialEffect",
                                    "hkxMaterialProperty",
                                    "hkxMaterialShader",
                                    "hkxMaterialShaderSet",
                                    "hkxMaterialTextureStage",
                                    "hkxMesh",
                                    "hkxMeshSection",
                                    "hkxMeshUserChannelInfo",
                                    "hkxNode",
                                    "hkxNodeAnnotationData",
                                    "hkxNodeSelectionSet",
                                    "hkxScene",
                                    "hkxSkinBinding",
                                    "hkxSparselyAnimatedBool",
                                    "hkxSparselyAnimatedEnum",
                                    "hkxSparselyAnimatedInt",
                                    "hkxSparselyAnimatedString",
                                    "hkxTextureFile",
                                    "hkxTextureInplace",
                                    "hkxTriangleSelectionChannel",
                                    "hkxVertexBuffer",
                                    "hkxVertexBufferVertexData",
                                    "hkxVertexDescription",
                                    "hkxVertexDescriptionElementDecl",
                                    "hkxVertexFloatDataChannel",
                                    "hkxVertexIntDataChannel",
                                    "hkxVertexSelectionChannel",
                                    "hkxVertexVectorDataChannel",
                                ],
                            ),
                        )
                    }
                }
            }
        }
        deserializer
            .deserialize_class_index(ClassesVisitor {
                marker: core::marker::PhantomData,
            })
    }
}
