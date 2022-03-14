#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Value_Tag {
  _Int,
  _Float,
} Value_Tag;

typedef struct Value {
  Value_Tag tag;
  union {
    struct {
      int32_t int_;
    };
    struct {
      double float_;
    };
  };
} Value;

typedef struct RStruct {
  const char *name;
  struct Value value;
} RStruct;

typedef struct FFIHttpRequest {
  const char *url;
  const char *method;
  const char *headers;
} FFIHttpRequest;

typedef void (*OnStateChanged)(uint32_t);

void ffi_say_hello(void);

const char *ffi_hello(const char *name);

void ffi_free_str(char *s);

int32_t ffi_double_input(int32_t input);

bool ffi_is_answer(int32_t input);

struct RStruct *ffi_data_new(void);

void ffi_data_free(struct RStruct *ptr);

uint64_t ffi_http_request(struct FFIHttpRequest request, uint64_t timeout);

uint32_t ffi_call_with_callback(uint32_t input, OnStateChanged cb);
