#ifndef _EXPRESSIONS_H
#define _EXPRESSIONS_H

#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct __expr_t;

typedef struct { struct __expr_t* x; } unary_expr_t;
typedef struct { struct __expr_t* x, *y; } binary_expr_t;

typedef enum : uint16_t {
  EXPR_CONSTANT,
  EXPR_VARIABLE,

  EXPR_PRODUCT,
  EXPR_QUOTIENT,

  EXPR_SUM,
  EXPR_DIFFERENCE,

  EXPR_EXPONENTIAL,
  EXPR_POWER,

  EXPR_NEGATION,
  EXPR_INVERSE
} expr_tag_t;

struct [[gnu::packed]] __expr_t {
  union {
    uint32_t constant;
    char variable;
    unary_expr_t arg;
    binary_expr_t args;
  };

  expr_tag_t variant;
};

typedef struct __expr_t expr_t;

#define Const(c) (expr_t) { .constant = (uint32_t)c, .variant = EXPR_CONSTANT }
#define Var(c) (expr_t) { .variable = (char)c, .variant = EXPR_VARIABLE }

#define Product(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_PRODUCT }
#define Quotient(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_QUOTIENT }

#define Sum(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_SUM }
#define Difference(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_DIFFERENCE }

#define Exponential(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_EXPONENTIAL }
#define Power(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_POWER }

#define Negation(e) (expr_t) { .arg = (unary_expr_t){e}, .variant = EXPR_NEGATION }
#define Inverse(e) (expr_t) { .arg = (unary_expr_t){e}, .variant = EXPR_INVERSE }


static size_t __counted_serialize_expr(char *serialization_buffer, expr_t *e, size_t depth) {
  expr_tag_t variant = e->variant;
  bool parens_condition = (depth > 0) &&
    (variant != EXPR_CONSTANT) &&
    (variant != EXPR_VARIABLE) &&
    (variant != EXPR_PRODUCT) &&
    (variant != EXPR_INVERSE);

  size_t offset_written = 0;

  if (parens_condition) serialization_buffer[offset_written++] = '(';

  switch (variant) {
    case EXPR_CONSTANT: {
      int num_of_digits = snprintf(NULL, 0, "%u", e->constant);
      sprintf (serialization_buffer + offset_written, "%u", e->constant);
      offset_written += num_of_digits;
      break;
    }
    case EXPR_VARIABLE: {
      sprintf (serialization_buffer + offset_written, "%c", e->variable);
      offset_written++;
      break;
    }
    case EXPR_PRODUCT: {
      // print '3x' instead of 3*x or x*3
      if ((e->args.x->variant == EXPR_CONSTANT) && (e->args.y->variant == EXPR_VARIABLE)) {
        offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
        offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      } else if ((e->args.x->variant == EXPR_VARIABLE) && (e->args.y->variant == EXPR_CONSTANT)) {
        offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
        offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      } else {
        offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
        serialization_buffer[offset_written++] = '*';
        offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      }
      break;
    }

    case EXPR_QUOTIENT: {
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '/';
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_SUM: {
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '+';
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_DIFFERENCE: {
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '-';
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_EXPONENTIAL:
    case EXPR_POWER: {
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '^';
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_NEGATION: {
      serialization_buffer[offset_written++] = '-';
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->arg.x, depth + 1);
      break;
    }

    case EXPR_INVERSE: {
      offset_written += __counted_serialize_expr(serialization_buffer + offset_written, e->arg.x, depth + 1);
      strncpy(serialization_buffer + offset_written, "⁻¹", strlen("⁻¹"));
      offset_written += strlen("⁻¹");
      break;
    }

    default:
      puts("corrupted/unhandled expression variant");
      asm volatile ("int3");
  }

  if (parens_condition) serialization_buffer[offset_written++] = ')';

  return offset_written;
}

size_t serialize_expr(char *buffer, expr_t *e) {
  size_t written = __counted_serialize_expr(buffer, e, 0);
  return written;
}

#endif
