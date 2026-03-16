#include "./wrapper_write.h"

#include "include/obj/NiTextKeyExtraData.h"

#include "serde_hkx_features/src/kf/to_kf/bridge.rs.h"

using namespace Niflib;

namespace nifbridge_write {

// ── constants matching the original C++ tool ──────────────────────────────
static const unsigned int kIntegerInf = 0x7f7fffff;
static const float kFloatINF = *reinterpret_cast<const float *>(&kIntegerInf);
static const float kEpsilon = 1e-5f;

static inline bool float_eq(float a, float b) {
  return std::fabs(a - b) < kEpsilon;
}
static inline bool vec3_eq(const Niflib::Vector3 &a, const Niflib::Vector3 &b) {
  return float_eq(a.x, b.x) && float_eq(a.y, b.y) && float_eq(a.z, b.z);
}
static inline bool quat_eq(const Niflib::Quaternion &a,
                           const Niflib::Quaternion &b) {
  return float_eq(a.w, b.w) && float_eq(a.x, b.x) && float_eq(a.y, b.y) &&
         float_eq(a.z, b.z);
}

void write_kf(rust::Str out_path, rust::Str seq_name, rust::Str target_name,
              float duration, rust::Vec<TransformTrackFfi> transform_tracks,
              rust::Vec<FloatTrackFfi> float_tracks, uint32_t nif_version,
              uint32_t user_version, uint32_t user_version2)

{
  // ── NiControllerSequence setup (mirrors doExport) ─────────────────────
  NiControllerSequenceRef seq = new NiControllerSequence();
  seq->SetName(std::string(seq_name));
  seq->SetStartTime(0.0f);
  seq->SetStopTime(duration);
  seq->SetFrequency(1.0f);
  seq->SetCycleType(CYCLE_CLAMP);
  seq->SetTargetName(std::string(target_name));

  // ── Text keys: "start" at 0, "end" at duration ────────────────────────
  {
    NiTextKeyExtraDataRef textKeys = new NiTextKeyExtraData();
    std::vector<Niflib::Key<std::string>> keys(2);
    keys[0].time = 0.0f;
    keys[0].data = "start";
    keys[1].time = duration;
    keys[1].data = "end";
    textKeys->SetKeys(keys);
    seq->SetTextKey(textKeys);
  }

  std::vector<Niflib::ControllerLink> blocks;

  // ── Transform tracks ──────────────────────────────────────────────────
  for (const auto &tt : transform_tracks) {
    NiTransformInterpolatorRef iplr = new NiTransformInterpolator();
    iplr->SetData(new NiTransformData());
    iplr->GetData()->SetTranslateType(LINEAR_KEY);
    iplr->GetData()->SetRotateType(QUADRATIC_KEY);
    iplr->GetData()->SetScaleType(LINEAR_KEY);

    // Translate keys
    {
      std::vector<Niflib::Key<Niflib::Vector3>> tkeys;
      tkeys.reserve(tt.translate_keys.size());
      for (const auto &k : tt.translate_keys) {
        Niflib::Key<Niflib::Vector3> nk;
        nk.time = k.time;
        nk.data = Niflib::Vector3(k.x, k.y, k.z);
        tkeys.push_back(nk);
      }
      iplr->GetData()->SetTranslateKeys(tkeys);

      // Set interpolator translation to first key value as reference.
      if (!tkeys.empty())
        iplr->SetTranslation(tkeys[0].data);
    }

    // Rotate keys
    {
      std::vector<Niflib::Key<Niflib::Quaternion>> rkeys;
      rkeys.reserve(tt.rotate_keys.size());
      for (const auto &k : tt.rotate_keys) {
        Niflib::Key<Niflib::Quaternion> nk;
        nk.time = k.time;
        // Niflib Quaternion: (w, x, y, z)
        nk.data = Niflib::Quaternion(k.w, k.x, k.y, k.z);
        rkeys.push_back(nk);
      }
      iplr->GetData()->SetQuatRotateKeys(rkeys);
      if (!rkeys.empty())
        iplr->SetRotation(rkeys[0].data);
    }

    // Scale keys
    {
      std::vector<Niflib::Key<float>> skeys;
      skeys.reserve(tt.scale_keys.size());
      for (const auto &k : tt.scale_keys) {
        Niflib::Key<float> nk;
        nk.time = k.time;
        nk.data = k.data;
        skeys.push_back(nk);
      }
      iplr->GetData()->SetScaleKeys(skeys);
      if (!skeys.empty())
        iplr->SetScale(skeys[0].data);
    }

    Niflib::ControllerLink link;
    link.nodeName = std::string(tt.bone_name);
    link.interpolator = Niflib::StaticCast<NiInterpolator>(iplr);
    link.variable1 = "Havok Transform Track";
    blocks.push_back(link);
  }

  // ── Float tracks ──────────────────────────────────────────────────────
  for (const auto &ft : float_tracks) {
    NiFloatInterpolatorRef iplr = new NiFloatInterpolator();
    iplr->SetData(new NiFloatData());
    iplr->GetData()->SetKeyType(LINEAR_KEY);

    std::vector<Niflib::Key<float>> fkeys;
    fkeys.reserve(ft.keys.size());
    for (const auto &k : ft.keys) {
      Niflib::Key<float> nk;
      nk.time = k.time;
      nk.data = k.data;
      fkeys.push_back(nk);
    }
    iplr->GetData()->SetKeys(fkeys);

    Niflib::ControllerLink link;
    link.nodeName = std::string(ft.slot_name);
    link.interpolator = Niflib::StaticCast<NiInterpolator>(iplr);
    link.variable1 = "Havok Float Track";
    blocks.push_back(link);
  }

  seq->SetControlledBlocks(blocks);

  // ── Write NIF ─────────────────────────────────────────────────────────
  Niflib::NifInfo info;
  info.version = nif_version;
  info.userVersion = user_version;
  info.userVersion2 = user_version2;

  Niflib::WriteNifTree(std::string(out_path), seq, info);
}

} // namespace nifbridge_write
