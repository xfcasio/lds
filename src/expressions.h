#ifndef _LDS_EXPRESSIONS_H
#define _LDS_EXPRESSIONS_H

#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "primitives.h"
#include "allocator.h"

struct __expr_t;

typedef struct { struct __expr_t* x; } unary_expr_t;
typedef struct { struct __expr_t* x, *y; } binary_expr_t;

typedef enum : u16 {
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
    f64 constant;
    char variable;
    unary_expr_t arg;
    binary_expr_t args;
  };

  expr_tag_t variant;
};

typedef struct __expr_t expr_t;

#define Const(c) (expr_t) { .constant = (f64)c, .variant = EXPR_CONSTANT }
#define Var(c) (expr_t) { .variable = (char)c, .variant = EXPR_VARIABLE }

#define Product(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_PRODUCT }
#define Quotient(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_QUOTIENT }

#define Sum(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_SUM }
#define Difference(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_DIFFERENCE }

#define Exponential(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_EXPONENTIAL }
#define Power(a, b) (expr_t) { .args = (binary_expr_t){ .x = (a), .y = (b) }, .variant = EXPR_POWER }

#define Negation(e) (expr_t) { .arg = (unary_expr_t){e}, .variant = EXPR_NEGATION }
#define Inverse(e) (expr_t) { .arg = (unary_expr_t){e}, .variant = EXPR_INVERSE }


static usize count_serialized_expr_size(expr_t *e, usize depth) {
  expr_tag_t variant = e->variant;
  bool parens_condition = (depth > 0) &&
    (variant != EXPR_CONSTANT) &&
    (variant != EXPR_VARIABLE) &&
    (variant != EXPR_PRODUCT);

  usize offset_written = 0;

  if (parens_condition) offset_written++;

  switch (variant) {
    case EXPR_CONSTANT: {
      i32 num_of_digits;

      num_of_digits = snprintf(NULL, 0, "%.3g", e->constant);

      offset_written += num_of_digits;
      break;
    }
    case EXPR_VARIABLE: {
      offset_written++;
      break;
    }
    case EXPR_PRODUCT: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;

      if (((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_VARIABLE)) || (x_variant == EXPR_SUM)) {
        offset_written += count_serialized_expr_size(e->args.x, depth + 1);
        offset_written += count_serialized_expr_size(e->args.y, depth + 1);
      } else if (((x_variant == EXPR_VARIABLE) && (y_variant == EXPR_CONSTANT)) || (y_variant == EXPR_SUM)) {
        offset_written += count_serialized_expr_size(e->args.y, depth + 1);
        offset_written += count_serialized_expr_size(e->args.x, depth + 1);
      } else {
        offset_written += count_serialized_expr_size(e->args.x, depth + 1);
        offset_written++;
        offset_written += count_serialized_expr_size(e->args.y, depth + 1);
      }
      break;
    }

    case EXPR_QUOTIENT: {
      offset_written += count_serialized_expr_size(e->args.x, depth + 1);
      offset_written++;
      offset_written += count_serialized_expr_size(e->args.y, depth + 1);
      break;
    }

    case EXPR_SUM: {
      offset_written += count_serialized_expr_size(e->args.x, depth + 1);
      offset_written++;
      offset_written += count_serialized_expr_size(e->args.y, depth + 1);
      break;
    }

    case EXPR_DIFFERENCE: {
      offset_written += count_serialized_expr_size(e->args.x, depth + 1);
      offset_written++;
      offset_written += count_serialized_expr_size(e->args.y, depth + 1);
      break;
    }

    case EXPR_EXPONENTIAL:
    case EXPR_POWER: {
      offset_written += count_serialized_expr_size(e->args.x, depth + 1);
      offset_written++;
      offset_written += count_serialized_expr_size(e->args.y, depth + 1);
      break;
    }

    case EXPR_NEGATION: {
      offset_written++;
      offset_written += count_serialized_expr_size(e->arg.x, depth + 1);
      break;
    }

    case EXPR_INVERSE: {
      offset_written += count_serialized_expr_size(e->arg.x, depth + 1);
      offset_written += strlen("⁻¹");
      break;
    }

    default:
      puts("count_serialized_expr_size: corrupted/unhandled expression variant");
      asm volatile ("int3");
  }

  if (parens_condition) offset_written++;

  return offset_written;
}

static usize counted_serialize_expr(char *serialization_buffer, expr_t *e, usize depth) {
  expr_tag_t variant = e->variant;
  bool parens_condition = (depth > 0) &&
    (variant != EXPR_CONSTANT) &&
    (variant != EXPR_VARIABLE) &&
    (variant != EXPR_PRODUCT);

  usize offset_written = 0;

  if (parens_condition) serialization_buffer[offset_written++] = '(';

  switch (variant) {
    case EXPR_CONSTANT: {
      i32 num_of_digits;

      num_of_digits = snprintf(NULL, 0, "%.3g", e->constant);
      sprintf(serialization_buffer + offset_written, "%.3g", e->constant);

      offset_written += num_of_digits;
      break;
    }
    case EXPR_VARIABLE: {
      sprintf (serialization_buffer + offset_written, "%c", e->variable);
      offset_written++;
      break;
    }
    case EXPR_PRODUCT: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;

      if (((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_VARIABLE)) || (x_variant == EXPR_SUM)) {
        offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
        offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      } else if (((x_variant == EXPR_VARIABLE) && (y_variant == EXPR_CONSTANT)) || (y_variant == EXPR_SUM)) {
        offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
        offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      } else {
        offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
        serialization_buffer[offset_written++] = '*';
        offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      }
      break;
    }

    case EXPR_QUOTIENT: {
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '/';
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_SUM: {
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '+';
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_DIFFERENCE: {
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '-';
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_EXPONENTIAL:
    case EXPR_POWER: {
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.x, depth + 1);
      serialization_buffer[offset_written++] = '^';
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->args.y, depth + 1);
      break;
    }

    case EXPR_NEGATION: {
      serialization_buffer[offset_written++] = '-';
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->arg.x, depth + 1);
      break;
    }

    case EXPR_INVERSE: {
      offset_written += counted_serialize_expr(serialization_buffer + offset_written, e->arg.x, depth + 1);
      strncpy(serialization_buffer + offset_written, "⁻¹", strlen("⁻¹"));
      offset_written += strlen("⁻¹");
      break;
    }

    default:
      puts("counted_serialize_expr: corrupted/unhandled expression variant");
      asm volatile ("int3");
  }

  if (parens_condition) serialization_buffer[offset_written++] = ')';

  return offset_written;
}

usize serialized_expr_size(expr_t *e) {
  return count_serialized_expr_size(e, 0);
}

usize serialize_expr(char *buffer, expr_t *e) {
  usize written = counted_serialize_expr(buffer, e, 0);
  return written;
}

bool is_binary_expression(expr_t *e) {
  switch (e->variant) {
    case EXPR_CONSTANT:
    case EXPR_VARIABLE:
    case EXPR_NEGATION:
    case EXPR_INVERSE: { return false; }

    case EXPR_PRODUCT:
    case EXPR_QUOTIENT:
    case EXPR_SUM:
    case EXPR_DIFFERENCE:
    case EXPR_EXPONENTIAL:
    case EXPR_POWER: { return true; }

    default:
      puts("is_binary_expression: corrupted/unhandled expression variant");
      asm volatile ("int3");
  }
}

bool is_simplifiable(expr_t *e) {
  switch (e->variant) {
    case EXPR_CONSTANT:
    case EXPR_VARIABLE: { return false; }

    case EXPR_PRODUCT:
    case EXPR_QUOTIENT:
    case EXPR_SUM:
    case EXPR_DIFFERENCE:
    case EXPR_POWER:
    case EXPR_EXPONENTIAL: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) return true;
      else return is_simplifiable(e->args.x) || is_simplifiable(e->args.y);
    }

    case EXPR_NEGATION:
    case EXPR_INVERSE: {
      if (e->arg.x->variant == EXPR_CONSTANT) return true;
      else return is_simplifiable(e->arg.x);
    }

    default:
      puts("is_simplifiable: corrupted/unhandled expression variant");
      asm volatile ("int3");
  }
}

void simplify(expr_t *e) {
  switch (e->variant) {
    case EXPR_CONSTANT:
    case EXPR_VARIABLE: break;

    case EXPR_PRODUCT: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;
      expr_t *x = e->args.x;
      expr_t *y = e->args.y;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) {
        *e = Const(x->constant * y->constant);
        break;
      }

      simplify(x);
      simplify(y);

      expr_t product = Product(x, y);
      
      if (is_simplifiable(&product)) simplify(&product);

      *e = product;
      break;
    }

    case EXPR_QUOTIENT: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;
      expr_t *x = e->args.x;
      expr_t *y = e->args.y;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) {
        *e = Const(x->constant / y->constant);
        break;
      }

      simplify(x);
      simplify(y);

      expr_t quotient = Quotient(x, y);

      if (is_simplifiable(&quotient)) simplify(&quotient);

      *e = quotient;
      break;
    }

    case EXPR_SUM: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;
      expr_t *x = e->args.x;
      expr_t *y = e->args.y;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) {
        *e = Const(x->constant + y->constant);
        break;
      }

      simplify(x);
      simplify(y);

      expr_t sum = Sum(x, y);
      if (is_simplifiable(&sum)) simplify(&sum);

      *e = sum;
      break;
    }

    case EXPR_DIFFERENCE: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;
      expr_t *x = e->args.x;
      expr_t *y = e->args.y;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) {
        *e = Const(x->constant - y->constant);
        break;
      }

      simplify(x);
      simplify(y);

      expr_t difference = Difference(x, y);
      if (is_simplifiable(&difference)) simplify(&difference);

      *e = difference;
      break;
    }

    case EXPR_EXPONENTIAL: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;
      expr_t *x = e->args.x;
      expr_t *y = e->args.y;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) {
        *e = Const(powf(x->constant, y->constant));
        break;
      }

      simplify(x);
      simplify(y);

      expr_t exponential = Exponential(x, y);
      if (is_simplifiable(&exponential)) simplify(&exponential);

      *e = exponential;
      break;
    }

    case EXPR_POWER: {
      expr_tag_t x_variant = e->args.x->variant;
      expr_tag_t y_variant = e->args.y->variant;
      expr_t *x = e->args.x;
      expr_t *y = e->args.y;

      if ((x_variant == EXPR_CONSTANT) && (y_variant == EXPR_CONSTANT)) {
        *e = Const(powf(x->constant, y->constant));
        break;
      }

      simplify(x);
      simplify(y);

      expr_t power = Power(x, y);
      if (is_simplifiable(&power)) simplify(&power);

      *e = power;
      break;
    }

    case EXPR_NEGATION: {
      expr_t *x = e->args.x;
      simplify(x);

      expr_t negation = Negation(x);
      if (is_simplifiable(&negation)) simplify(&negation);

      *e = negation;
      break;
    }

    case EXPR_INVERSE: {
      expr_tag_t x_variant = e->arg.x->variant;
      expr_t *x = e->arg.x;

      if (x_variant == EXPR_CONSTANT) {
        *e = Const((f64)1.0 / x->constant);
        break;
      }

      simplify(x);

      expr_t inverse = Inverse(x);
      if (is_simplifiable(&inverse)) simplify(&inverse);

      *e = inverse;
      break;
    }
  }
}

#endif
