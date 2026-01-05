#ifdef _MSC_VER
#pragma comment(lib, "ntdll.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "userenv.lib")
#endif

#include "./serde_hkx_ffi.h"

#include <ctype.h>
#include <stdio.h>
#include <string.h>

/* ASCII case-insensitive compare */
static int ascii_eq_ignore_case(const char *a, const char *b) {
  while (*a && *b) {
    if (tolower((unsigned char)*a) != tolower((unsigned char)*b))
      return 0;
    ++a;
    ++b;
  }
  return *a == *b;
}

static int parse_output_format(const char *s, OutputFormat *out) {
  if (ascii_eq_ignore_case(s, "xml")) {
    *out = SERDE_HKX_XML;
    return 1;
  }
  if (ascii_eq_ignore_case(s, "amd64")) {
    *out = SERDE_HKX_AMD64;
    return 1;
  }
  if (ascii_eq_ignore_case(s, "win32")) {
    *out = SERDE_HKX_WIN32;
    return 1;
  }
  return 0;
}

static const char *err_to_string(SerdeHkxError err) {
  switch (err) {
  case SERDE_HKX_OK:
    return "OK";
  case SERDE_HKX_INVALID_ARGUMENT:
    return "INVALID_ARGUMENT";
  case SERDE_HKX_UNSUPPORTED_FORMAT:
    return "UNSUPPORTED_FORMAT";
  case SERDE_HKX_IO_ERROR:
    return "IO_ERROR";
  case SERDE_HKX_WALK_ERROR:
    return "WALK_ERROR";
  case SERDE_HKX_SERIALIZE_ERROR:
    return "SERIALIZE_ERROR";
  case SERDE_HKX_DESERIALIZE_ERROR:
    return "DESERIALIZE_ERROR";
  case SERDE_HKX_CONVERT_ERROR:
    return "CONVERT_ERROR";
  case SERDE_HKX_REPRODUCE_ERROR:
    return "REPRODUCE_ERROR";
  case SERDE_HKX_THREAD_ERROR:
    return "THREAD_ERROR";
  case SERDE_HKX_EXTERNAL_ERROR:
    return "EXTERNAL_ERROR";
  case SERDE_HKX_INTERNAL_ERROR:
    return "INTERNAL_ERROR";
  default:
    return "UNKNOWN_ERROR";
  }
}

static void print_usage(const char *exe) {
  fprintf(stderr,
          "Usage:\n"
          "  %s -i <input> [-o <output>] -v <XML|AMD64|win32>\n",
          exe);
}

// cl.exe /std:c11 ./crates/serde_hkx_ffi/test/main.c
// ./target/release/serde_hkx_ffi.lib
// -I./crates/serde_hkx_ffi/test/serde_hkx_ffi.h

int main(int argc, char **argv) {
  const char *input = NULL;
  const char *output = NULL;
  OutputFormat format;
  int have_format = 0;

  for (int i = 1; i < argc; ++i) {
    if (strcmp(argv[i], "-i") == 0 && i + 1 < argc) {
      input = argv[++i];
    } else if (strcmp(argv[i], "-o") == 0 && i + 1 < argc) {
      output = argv[++i];
    } else if (strcmp(argv[i], "-v") == 0 && i + 1 < argc) {
      if (!parse_output_format(argv[++i], &format)) {
        fprintf(stderr, "Invalid format: %s\n", argv[i]);
        return 1;
      }
      have_format = 1;
    } else {
      print_usage(argv[0]);
      return 1;
    }
  }

  if (!input || !have_format) {
    print_usage(argv[0]);
    return 1;
  }

  SerdeHkxError err = serde_hkx_ffi_convert(input, output, format);

  if (err != SERDE_HKX_OK) {
    fprintf(stderr, "convert failed: %s (%d)\n", err_to_string(err), err);
    return 1;
  }

  printf("convert success\n");
  return 0;
}
