#include <assert.h>
#include <string.h>
#include "expressions.h"

void test_expressions() {
    char *buffer = (char *)malloc(sizeof(char) * 1024);
    size_t written = 0;

    // Basic tests
    expr_t simple_const = Const(42);
    written = serialize_expr(buffer, &simple_const);
    assert(strcmp(buffer, "42") == 0);
    memset(buffer, 0, written);
    
    expr_t simple_var = Var('x');
    written = serialize_expr(buffer, &simple_var);
    assert(strcmp(buffer, "x") == 0);
    memset(buffer, 0, written);
    
    // Simple operations
    expr_t sum_test = Sum(&Const(3), &Const(5));
    written = serialize_expr(buffer, &sum_test);
    assert(strcmp(buffer, "3+5") == 0);
    memset(buffer, 0, written);
    
    expr_t product_test = Product(&Const(3), &Var('x'));
    written = serialize_expr(buffer, &product_test);
    assert(strcmp(buffer, "3x") == 0);
    memset(buffer, 0, written);
    
    expr_t product_var_const = Product(&Var('x'), &Const(3));
    written = serialize_expr(buffer, &product_var_const);
    assert(strcmp(buffer, "3x") == 0);
    memset(buffer, 0, written);
    
    // Nested parentheses hell
    expr_t nested_sum = Sum(&Sum(&Const(1), &Const(2)), &Sum(&Const(3), &Const(4)));
    written = serialize_expr(buffer, &nested_sum);
    assert(strcmp(buffer, "(1+2)+(3+4)") == 0);
    memset(buffer, 0, written);
    
    // Complex polynomial: 3x^2 + 2x - 5
    expr_t polynomial = Sum(&Sum(&Product(&Const(3), &Power(&Var('x'), &Const(2))), 
                                         &Product(&Const(2), &Var('x'))), 
                             &Negation(&Const(5)));
    written = serialize_expr(buffer, &polynomial);
    assert(strcmp(buffer, "(3*(x^2)+2x)+(-5)") == 0);
    memset(buffer, 0, written);
    
    // Fraction madness: (x+1)/(x-1)
    expr_t fraction = Quotient(&Sum(&Var('x'), &Const(1)), 
                              &Difference(&Var('x'), &Const(1)));
    written = serialize_expr(buffer, &fraction);
    assert(strcmp(buffer, "(x+1)/(x-1)") == 0);
    memset(buffer, 0, written);
    
    // Triple nested power: x^(y^z)
    expr_t triple_power = Power(&Var('x'), &Power(&Var('y'), &Var('z')));
    written = serialize_expr(buffer, &triple_power);
    assert(strcmp(buffer, "x^(y^z)") == 0);
    memset(buffer, 0, written);
    
    // Inverse chaos: (x+1)⁻¹
    expr_t inverse_sum = Inverse(&Sum(&Var('x'), &Const(1)));
    written = serialize_expr(buffer, &inverse_sum);
    assert(strcmp(buffer, "(x+1)⁻¹") == 0);
    memset(buffer, 0, written);
    
    // Absolute unit: ((2x+3)/(x-1))^((y+z)/2)
    expr_t absolute_unit = Power(&Quotient(&Sum(&Product(&Const(2), &Var('x')), &Const(3)),
                                          &Difference(&Var('x'), &Const(1))),
                                &Quotient(&Sum(&Var('y'), &Var('z')), &Const(2)));
    written = serialize_expr(buffer, &absolute_unit);
    assert(strcmp(buffer, "((2x+3)/(x-1))^((y+z)/2)") == 0);
    memset(buffer, 0, written);
    
    // Zero case
    expr_t zero = Const(0);
    written = serialize_expr(buffer, &zero);
    assert(strcmp(buffer, "0") == 0);
    memset(buffer, 0, written);
    
    // Single digit vs multi-digit
    expr_t single_digit = Const(7);
    expr_t multi_digit = Const(1337);
    written = serialize_expr(buffer, &single_digit);
    assert(strcmp(buffer, "7") == 0);
    memset(buffer, 0, written);
    written = serialize_expr(buffer, &multi_digit);
    assert(strcmp(buffer, "1337") == 0);
    memset(buffer, 0, written);
    
    // Deep nesting stress test: (((((x+1)+2)+3)+4)+5)
    expr_t deep_nest = Sum(&Sum(&Sum(&Sum(&Sum(&Var('x'), &Const(1)), &Const(2)), &Const(3)), &Const(4)), &Const(5));
    written = serialize_expr(buffer, &deep_nest);
    assert(strcmp(buffer, "((((x+1)+2)+3)+4)+5") == 0);
    memset(buffer, 0, written);
    
    // Product chain without parentheses: 2*3*x*y
    expr_t product_chain = Product(&Product(&Product(&Const(2), &Const(3)), &Var('x')), &Var('y'));
    written = serialize_expr(buffer, &product_chain);
    assert(strcmp(buffer, "2*3*x*y") == 0);
    memset(buffer, 0, written);
    
    // Mixed operations: -x^2 + 3x - 1
    expr_t mixed = Sum(&Sum(&Negation(&Power(&Var('x'), &Const(2))), 
                           &Product(&Const(3), &Var('x'))), 
                      &Negation(&Const(1)));
    written = serialize_expr(buffer, &mixed);
    assert(strcmp(buffer, "((-(x^2))+3x)+(-1)") == 0);
    memset(buffer, 0, written);
    
    // Inverse of negation: (-x)⁻¹
    expr_t inverse_neg = Inverse(&Negation(&Var('x')));
    written = serialize_expr(buffer, &inverse_neg);
    assert(strcmp(buffer, "(-x)⁻¹") == 0);
    memset(buffer, 0, written);
    
    // Maximum int edge case
    expr_t max_int = Const(2147483647);
    written = serialize_expr(buffer, &max_int);
    assert(strcmp(buffer, "2147483647") == 0);
    memset(buffer, 0, written);

    printf("All tests passed!\n");

    free(buffer);
}

int main() {
    test_expressions();
    return 0;
}
