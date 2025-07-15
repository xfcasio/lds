#include <stdio.h>
#include <stdint.h>
#include "allocator.h"
#include "expressions.h"

int main() {
  expr_t e = Quotient(
    &Sum(&Var('p'), &Product(&Var('q'), &Sum(&Const(6), &Sum(&Sum(&Const(5), &Product(&Const(3), &Inverse(&Const(2)))), &Const(5))))),
    &Inverse(&Inverse(&Inverse(&Const(8))))
  );

  char *buffer GPA_DEALLOC = (char*)gpa_allocator.alloc(sizeof(char) * serialized_expr_size(&e));

  usize written = serialize_expr(buffer, &e);
  
  printf("expr: %s\n\n", buffer);

  simplify(&e);

  memset(buffer, 0, written);
  serialize_expr(buffer, &e);

  printf("simplification: %s\n", buffer);
}
