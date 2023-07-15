#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_RustRequest {
  struct wire_uint_8_list *address;
  int32_t operation;
  struct wire_uint_8_list *bytes;
} wire_RustRequest;

typedef struct wire_RustRequestUnique {
  int32_t id;
  struct wire_RustRequest request;
} wire_RustRequestUnique;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_prepare_rust_signal_stream(int64_t port_);

void wire_prepare_rust_response_stream(int64_t port_);

WireSyncReturn wire_prepare_channels(void);

void wire_check_rust_streams(int64_t port_);

void wire_start_rust_logic(int64_t port_);

WireSyncReturn wire_request_to_rust(struct wire_RustRequestUnique *request_unique);

struct wire_RustRequestUnique *new_box_autoadd_rust_request_unique_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_prepare_rust_signal_stream);
    dummy_var ^= ((int64_t) (void*) wire_prepare_rust_response_stream);
    dummy_var ^= ((int64_t) (void*) wire_prepare_channels);
    dummy_var ^= ((int64_t) (void*) wire_check_rust_streams);
    dummy_var ^= ((int64_t) (void*) wire_start_rust_logic);
    dummy_var ^= ((int64_t) (void*) wire_request_to_rust);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_rust_request_unique_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
