mod infra;

// Your tests go here!
success_tests! {
    {
        name: number,
        file: "number.snek",
        expected: "37",
    },
    {
        name: add1,
        file: "add1.snek",
        expected: "73",
    },
    {
        name: sub1,
        file: "sub1.snek",
        expected: "-2",
    },
    {
        name: add_and_sub,
        file: "add_and_sub.snek",
        expected: "72",
    },
    {
        name: nested_arith,
        file: "nested_arith.snek",
        expected: "25",
    },
    {
        name: multiple_correct_binds,
        file: "multiple_correct_binds.snek",
        expected: "6",
    },
    {
        name: nested_binds,
        file: "nested_binds.snek",
        expected: "12",
    },
    {
        name: bind_chain,
        file: "bind_chain.snek",
        expected: "5",
    },
    {
        name: shadow_bind,
        file: "shadow_bind.snek",
        expected: "12",
    },
    {
        name: bind_in_funny_place,
        file: "bind_in_funny_place.snek",
        expected: "7",
    },
    {
        name: input_compare_1,
        file: "input_compare.snek",
        input: "2",
        expected: "false",
    },
    {
        name: input_compare_2,
        file: "input_compare.snek",
        input: "10",
        expected: "true",
    },
    {
        name: let_and_set,
        file: "let_and_set.snek",
        expected: "6",
    },
    {
        name: big_loop,
        file: "big_loop.snek",
        expected: "-6",
    },
    {
        name: input_factorial_2,
        file: "input_factorial.snek",
        input: "2",
        expected: "2",
    },
    {
        name: input_factorial_7,
        file: "input_factorial.snek",
        input: "7",
        expected: "5040",
    },
    {
        name: negative_input_comparison,
        file: "negative_input_cmp.snek",
        input: "-5",
        expected: "true",
    },
    {
        name: big_mul,
        file: "big_mul.snek",
        expected: "-2305843009213693952"
    },
    {
        name: print_default,
        file: "print.snek",
        expected: "false\nfalse",
    },
    {
        name: print_number,
        file: "print.snek",
        input: "-5",
        expected: "-5\n-5",
    },
    {
        name: double,
        file: "double.snek",
        input: "10",
        expected: "20",
    },
    {
        name: even_odd_5,
        file: "even_odd.snek",
        input: "5",
        expected: "5\nfalse\nfalse",
    },
    {
        name: even_odd_6,
        file: "even_odd.snek",
        input: "6",
        expected: "6\ntrue\ntrue",
    },
    {
        name: fact_small,
        file: "fact.snek",
        input: "3",
        expected: "6"
    },
    {
        name: fact_big,
        file: "fact.snek",
        input: "15",
        expected: "1307674368000",
    },
    {
        name: fact_big_recursive,
        file: "recursive_fact.snek",
        input: "15",
        expected: "1307674368000",
    },
    {
        name: print_in_func,
        file: "print_in_func.snek",
        input: "5",
        expected: "10\n10",
    },
    {
        name: infinity_calls,
        file: "infinity_calls.snek",
        input: "10",
        expected: "10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n0"
    },
    {
        name: simple_examples,
        file: "simple_examples.snek",
        expected: "1\ntrue\n7"
    },
    {
        name: points,
        file: "points.snek",
        expected: "(tuple 1 2)\n(tuple 7 9)"
    },
    {
        name: tuple_update,
        file: "tuple_update.snek",
        expected: "(tuple 1 5 3)"
    },
    {
        name: bst,
        file: "bst.snek",
        expected: "(tuple 5 (tuple 2 nil nil) nil)\n(tuple 5 (tuple 2 nil nil) (tuple 6 nil nil))\ntrue\nfalse\n(tuple 5 (tuple 2 (tuple 1 nil nil) nil) (tuple 6 nil nil))\ntrue",
    },
    {
        name: tuple_equal,
        file: "tuple_equal.snek",
        expected: "true\ntrue\nfalse\ntrue\nfalse\ntrue"
    }
}

runtime_error_tests! {
    {
        name: invalid_argument,
        file: "invalid_argument.snek",
        expected: "invalid argument",
    },
    {
        name: double_bool,
        file: "double.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: input_compare_3,
        file: "input_compare.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: add1_boolean,
        file: "add1_boolean.snek",
        expected: "invalid argument",
    },
    {
        name: add1_overflow,
        file: "add1_overflow.snek",
        expected: "overflow",
    },
    {
        name: sub_overflow,
        file: "sub_overflow.snek",
        expected: "overflow",
    },
    {
        name: invalid_argument_comparison,
        file: "invalid_argument_cmp.snek",
        expected: "invalid argument",
    },
    {
        name: mul_overflow,
        file: "mul_overflow.snek",
        expected: "overflow"
    },
    {
        name: bad_equal,
        file: "bad_equal.snek",
        expected: "invalid argument",
    },
    {
        name: fact_invalid,
        file: "fact.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: eventual_overflow,
        file: "eventual_overflow.snek",
        expected: "overflow",
    },
    {
        name: error_tag,
        file: "error-tag.snek",
        expected: "invalid argument",
    },
    {
        name: error_bounds,
        file: "error-bounds.snek",
        expected: "out of bounds",
    },
    {
        name: tuple_equal_invalid_arg,
        file: "tuple_equal_on_num.snek",
        expected: "invalid argument",
    }
}

static_error_tests! {
    {
        name: unbound_id,
        file: "unbound_id.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: duplicate_binding,
        file: "duplicate_binding.snek",
        expected: "Duplicate binding",
    },
    {
        name: parse_let_fail,
        file: "parse_let_fail.snek",
        expected: "Invalid",
    },
    {
        name: number_bounds_fail,
        file: "number_bounds_fail.snek",
        expected: "Invalid",
    },
    {
        name: bad_block,
        file: "bad_block.snek",
        expected: "Invalid",
    },
    {
        name: bad_func_arity,
        file: "bad_func_arity.snek",
        expected: "Invalid",
    },
    {
        name: bad_func_name,
        file: "bad_func_name.snek",
        expected: "Invalid",
    },
    {
        name: duplicate_func,
        file: "duplicate_func.snek",
        expected: "Invalid",
    },
    {
        name: duplicate_params,
        file: "duplicate_params.snek",
        expected: "Invalid",
    },
    {
        name: no_func_def,
        file: "no_func_def.snek",
        expected: "Invalid",
    },
    {
        name: no_input_in_func,
        file: "no_input_in_func.snek",
        expected: "Unbound variable identifier input",
    },
    {
        name: error3,
        file: "error3.snek",
        expected: "Invalid",
    },
}
