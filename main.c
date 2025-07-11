#include <stdio.h>
#include <stdint.h>
#include "expressions.h"

int main() {
  [[gnu::unused]] expr_t e = Quotient(
    &Sum(&Var('p'), &Product(&Var('q'), &Const(12))),
    &Inverse(&Power(&Const(120), &Const(444)))
  );

  char *buffer = (char *)malloc(sizeof(char) * 1024);
  
  size_t written = serialize_expr(buffer, &e);
  printf("%zu\n", written);

  printf("%s\n", buffer);
  free(buffer);
}
