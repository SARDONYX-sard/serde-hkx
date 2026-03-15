#pragma once
#include <memory>
#include <vector>

#include "include/gen/ControllerLink.h"
#include "include/nif_math.h"
#include "include/obj/NiObjectNET.h"

#include "include/niflib.h"
#include "include/obj/NiControllerSequence.h"
#include "include/obj/NiFloatData.h"
#include "include/obj/NiFloatInterpolator.h"
#include "include/obj/NiInterpolator.h"
#include "include/obj/NiObject.h"
#include "include/obj/NiTransformData.h"
#include "include/obj/NiTransformInterpolator.h"

using std::unique_ptr;
using std::vector;

namespace nifbridge {
// ------------------------------------------------------------------------------------------------
// Forward declarations / type aliases
// ------------------------------------------------------------------------------------------------

using ControllerLink = Niflib::ControllerLink;
using KeyType = Niflib::KeyType;
using NiControllerSequenceRef = Niflib::NiControllerSequenceRef;
using NiFloatDataRef = Niflib::NiFloatDataRef;
using NiTransformDataRef = Niflib::NiTransformDataRef;

// Since Niflib's type is not trivial, it cannot be used as Rust's return type,
// so it must be defined separately.
struct alignas(16) Quaternion {
  float x;
  float y;
  float z;
  float w;
};

// Since Niflib's type is not trivial, it cannot be used as Rust's return type,
// so it must be defined separately.
struct alignas(16) Vector3 {
  float x;
  float y;
  float z;
};

struct Vector3Key;
struct QuaternionKey;
struct FloatKey;

// struct Vector3Key {
//   float time;
//   Vector3 data;
// };
// struct QuaternionKey {
//   float time;
//   Quaternion data;
// };

// struct FloatKey {
//   float time;
//   float data;
// };

// ------------------------------------------------------------------------------------------------
// NIF sequence / controller
// ------------------------------------------------------------------------------------------------

unique_ptr<vector<NiControllerSequenceRef>> read_nif_list(const string &name);

unique_ptr<vector<ControllerLink>>
get_controller_links(const NiControllerSequenceRef &seq);

unique_ptr<string> get_target_name(const NiControllerSequenceRef &r);
float get_start_time(const NiControllerSequenceRef &r);
float get_stop_time(const NiControllerSequenceRef &r);

// ------------------------------------------------------------------------------------------------
// Transform controller
// ------------------------------------------------------------------------------------------------

bool is_transform_type(const ControllerLink &r);
unique_ptr<NiTransformDataRef> get_transform_ref(const ControllerLink &r);
const string &get_node_name(const ControllerLink &r);

// Holds all transform-related animation keys.
class TransformKeys {
  unique_ptr<vector<Vector3Key>> translate_keys;
  unique_ptr<vector<QuaternionKey>> rotate_keys;
  unique_ptr<vector<FloatKey>> scale_keys;

public:
  TransformKeys(unique_ptr<vector<Vector3Key>> transform_keys,
                unique_ptr<vector<QuaternionKey>> rorate_keys,
                unique_ptr<vector<FloatKey>> scale_keys)
      : translate_keys(std::move(transform_keys)),
        rotate_keys(std::move(rorate_keys)), scale_keys(std::move(scale_keys)) {
  }

  // Accessors for Rust
  const vector<Vector3Key> &get_translate_keys() const {
    return *translate_keys;
  }
  const vector<QuaternionKey> &get_rotate_keys() const { return *rotate_keys; }
  const vector<FloatKey> &get_scale_keys() const { return *scale_keys; }
};

unique_ptr<TransformKeys> get_transform_keys(const NiTransformDataRef &r);

KeyType get_translate_key_type(const NiTransformDataRef &r);
KeyType get_rotate_key_type(const NiTransformDataRef &r);
KeyType get_scale_key_type(const NiTransformDataRef &r);

// ------------------------------------------------------------------------------------------------
// Float controller
// ------------------------------------------------------------------------------------------------

bool is_float_type(const ControllerLink &r);
unique_ptr<NiFloatDataRef> get_float_ref(const ControllerLink &r);

unique_ptr<vector<FloatKey>> get_float_keys(const NiFloatDataRef &r);

KeyType get_float_key_type(const NiFloatDataRef &r);

} // namespace nifbridge
