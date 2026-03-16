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

#include <rust/cxx.h>

namespace nifbridge_write {

struct Vector3KeyFfi;
struct QuaternionKeyFfi;
struct FloatKeyFfi;
struct TransformTrackFfi;
struct FloatTrackFfi;

void write_kf(rust::Str out_path, rust::Str seq_name, rust::Str target_name,
              float duration, rust::Vec<TransformTrackFfi> transform_tracks,
              rust::Vec<FloatTrackFfi> float_tracks, uint32_t nif_version,
              uint32_t user_version, uint32_t user_version2);
} // namespace nifbridge_write
