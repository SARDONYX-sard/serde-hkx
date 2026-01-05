// SPDX-FileCopyrightText: (C) 2025 SARDONYX
// SPDX-License-Identifier: Apache-2.0 OR MIT

/**
 * Error codes returned by serde_hkx FFI APIs.
 *
 * This enum represents a lossy projection of internal Rust errors.
 * No contextual information such as file paths is preserved.
 *
 * The numeric values are ABI-stable.
 */
typedef enum SerdeHkxError {
  /** Operation completed successfully. */
  SERDE_HKX_OK = 0,

  /**
   * Invalid argument was provided.
   *
   * Examples:
   * - Unsupported or missing file extension
   * - Invalid UTF-8 path
   * - Invalid stdout usage
   */
  SERDE_HKX_INVALID_ARGUMENT,

  /** Unsupported input or output format. */
  SERDE_HKX_UNSUPPORTED_FORMAT,

  /** I/O error occurred while reading or writing files. */
  SERDE_HKX_IO_ERROR,

  /** Directory walking or path manipulation failed. */
  SERDE_HKX_WALK_ERROR,

  /** Serialization failed. */
  SERDE_HKX_SERIALIZE_ERROR,

  /** Deserialization failed. */
  SERDE_HKX_DESERIALIZE_ERROR,

  /** Conversion failed for one or more files. */
  SERDE_HKX_CONVERT_ERROR,

  /** Reproduction or diff verification failed. */
  SERDE_HKX_REPRODUCE_ERROR,

  /** Threading or async task failed. */
  SERDE_HKX_THREAD_ERROR,

  /** Error from optional or external components. */
  SERDE_HKX_EXTERNAL_ERROR,

  /** Internal error or panic. */
  SERDE_HKX_INTERNAL_ERROR,
} SerdeHkxError;

/**
 * Output format used by serde_hkx FFI APIs.
 *
 * This enum specifies the target format of the conversion.
 * The numeric values are ABI-stable and must not be reordered.
 *
 * This enum is intended for C/C++ consumers.
 */
typedef enum OutputFormat {
  /**
   * Havok HKX format (64-bit / AMD64).
   *
   * This generates a binary HKX file targeting 64-bit platforms.
   */
  SERDE_HKX_AMD64 = 0,

  /**
   * Havok HKX format (32-bit / WIN32).
   *
   * This generates a binary HKX file targeting 32-bit platforms.
   */
  SERDE_HKX_WIN32 = 1,

  /**
   * Havok XML format.
   *
   * This generates a human-readable XML representation of HKX data.
   */
  SERDE_HKX_XML = 2,
} OutputFormat;

/**
 * Convert an input file or directory.
 *
 * @param input_path
 *   UTF-8 encoded path to an input file or directory.
 * @param output_path
 *   UTF-8 encoded path to an output file or directory.
 *   If null, the output is placed next to the input.
 *   Note: When converting from win32 to amd64, files with the same name will be
 * overwritten.
 * @param format
 *   Output format.
 *
 * @return
 *   SERDE_HKX_OK on success.
 *   Any other value on failure.
 *
 * @example
 * @code{.c}
 * #include "serde_hkx_ffi.h"
 *
 * int main(void)
 * {
 *     SerdeHkxError err = serde_hkx_ffi_convert(
 *         "input.hkx",
 *         "output.xml",
 *         SERDE_HKX_WIN32,
 *     );
 *
 *     if (err != SERDE_HKX_OK) {
 *         // handle error
 *         return 1;
 *     }
 *
 *     return 0;
 * }
 * @endcode
 */
SerdeHkxError serde_hkx_ffi_convert(const char *input_path,
                                    const char *output_path,
                                    OutputFormat format);
