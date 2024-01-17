// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
#include "exponential.h"


__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void) old_size;
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}

// Component Adapters

__attribute__((__export_name__("docs:calculator/exp@0.1.0#exp")))
int32_t __wasm_export_exports_docs_calculator_exp_exp(int32_t arg, int32_t arg0) {
  uint32_t ret = exports_docs_calculator_exp_exp((uint32_t) (arg), (uint32_t) (arg0));
  return (int32_t) (ret);
}

extern void __component_type_object_force_link_exponential(void);
void __component_type_object_force_link_exponential_public_use_in_this_compilation_unit(void) {
  __component_type_object_force_link_exponential();
}