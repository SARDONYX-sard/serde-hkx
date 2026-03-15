#include "./wrapper.h"

#include <memory>

#include "serde_hkx_features/src/kf/to_hkx/bridge.rs.h"

namespace nifbridge {
using namespace Niflib;

unique_ptr<vector<NiControllerSequenceRef>> read_nif_list(const string &name) {
  vector<NiObjectRef> blocks = ReadNifList(name, NULL); // NOTE: throwable error

  auto out = make_unique<vector<NiControllerSequenceRef>>();

  for (auto &ref : blocks) {
    out->push_back(DynamicCast<NiControllerSequence>(ref));
  }

  return out;
}

unique_ptr<vector<ControllerLink>>
get_controller_links(const NiControllerSequenceRef &seq) {
  auto pair = seq->GetControlledBlocks();
  auto vec = make_unique<std::vector<ControllerLink>>(pair);
  return vec;
}

unique_ptr<string> get_target_name(const NiControllerSequenceRef &r) {
  return std::make_unique<string>(r->GetTargetName());
}

float get_start_time(const NiControllerSequenceRef &r) {
  return r->GetStartTime();
}

float get_stop_time(const NiControllerSequenceRef &r) {
  return r->GetStopTime();
}

// ---------------------------------------------------------------------------------------------------------------------

bool is_transform_type(const ControllerLink &r) {
  return r.interpolator->IsSameType(NiTransformInterpolator::TYPE);
}
unique_ptr<NiTransformDataRef> get_transform_ref(const ControllerLink &r) {
  return std::make_unique<NiTransformDataRef>(
      StaticCast<NiTransformInterpolator>(r.interpolator)->GetData());
}
const string &get_node_name(const ControllerLink &r) { return r.nodeName; }

// ---------------------------------------------------------------------------------------------------------------------

unique_ptr<TransformKeys> get_transform_keys(const NiTransformDataRef &r) {

  auto translate_keys = make_unique<vector<Vector3Key>>();
  auto rotate_keys = make_unique<vector<QuaternionKey>>();
  auto scale_keys = make_unique<vector<FloatKey>>();

  // --- translation ---
  {
    const auto &src = r->GetTranslateKeys();
    translate_keys->reserve(src.size());
    for (const auto &k : src) {
      translate_keys->push_back(
          Vector3Key{k.time, {k.data.x, k.data.y, k.data.z}});
    }
  }

  // --- rotation ---
  {
    const auto &src = r->GetQuatRotateKeys();
    rotate_keys->reserve(src.size());
    for (const auto &k : src) {
      rotate_keys->push_back(
          QuaternionKey{k.time, {k.data.x, k.data.y, k.data.z, k.data.w}});
    }
  }

  // --- scale ---
  {
    const auto &src = r->GetScaleKeys();
    scale_keys->reserve(src.size());
    for (const auto &k : src) {
      scale_keys->push_back(FloatKey{k.time, k.data});
    }
  }

  return make_unique<TransformKeys>(TransformKeys{std::move(translate_keys),
                                                  std::move(rotate_keys),
                                                  std::move(scale_keys)});
}

KeyType get_translate_key_type(const NiTransformDataRef &r) {
  return r->GetTranslateType();
}
KeyType get_rotate_key_type(const NiTransformDataRef &r) {
  return r->GetRotateType();
}
KeyType get_scale_key_type(const NiTransformDataRef &r) {
  return r->GetScaleType();
}

// ---------------------------------------------------------------------------------------------------------------------

bool is_float_type(const ControllerLink &r) {
  return r.interpolator->IsSameType(NiFloatInterpolator::TYPE);
}
unique_ptr<NiFloatDataRef> get_float_ref(const ControllerLink &r) {
  return std::make_unique<NiFloatDataRef>(
      StaticCast<NiFloatInterpolator>(r.interpolator)->GetData());
}

unique_ptr<vector<FloatKey>> get_float_keys(const NiFloatDataRef &r) {
  auto scale_keys = make_unique<vector<FloatKey>>();
  const auto &src = r->GetKeys();
  scale_keys->reserve(src.size());
  for (const auto &k : src) {
    scale_keys->push_back(FloatKey{k.time, k.data});
  }

  return scale_keys;
}

KeyType get_float_key_type(const NiFloatDataRef &r) { return r->GetKeyType(); }

// ---------------------------------------------------------------------------------------------------------------------

} // namespace nifbridge
