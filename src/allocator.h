#ifndef _LDS_ALLOCATOR_H
#define _LDS_ALLOCATOR_H
#include <stdlib.h>
#include <stdio.h>
#include "primitives.h"

#define LDS_ENABLE_GPA_ALLOCATOR


#ifdef LDS_ENABLE_GPA_ALLOCATOR
  #define GPA_DEALLOC [[gnu::cleanup(gpa_gnu_cleanup)]]
  
  static usize __active_gpa_allocations = 0;

  typedef struct {
    u8 *(*alloc)(usize);
    void (*dealloc)(u8*);
  } allocator_t;

  static u8 *gpa_alloc(usize size) {
    __active_gpa_allocations++;
    return (u8*)malloc(size);
  }

  static void gpa_dealloc(u8 *allocation) {
    __active_gpa_allocations--;
    free(allocation);
  }

  [[gnu::destructor]]
  void gpa_leak_detector() {
    if (__active_gpa_allocations > 0) {
      fprintf(stderr, "\n\033[1;31mgpa allocator: %zu allocations potentially leaked\033[0m\n", __active_gpa_allocations);
    } else {
      fprintf(stderr, "\n\033[1;32mgpa_allocator: no memory leaked I think\033[0m\n");
    }
  }

  allocator_t gpa_allocator = { .alloc = gpa_alloc, .dealloc = gpa_dealloc };

  void gpa_gnu_cleanup(void *_p) {
    void **p = (void**)_p;
    if (p && *p) {
      gpa_allocator.dealloc((u8*)*p);
    } else {
      puts("\n\033[1;31m[!] gpa cleanup attempted to free NULL memory (potential double-free)\033[0m");
    }
  }
#endif
#endif

