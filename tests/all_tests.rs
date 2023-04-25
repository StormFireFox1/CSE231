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
}

runtime_error_tests! {
    {
        name: invalid_argument,
        file: "invalid_argument.snek",
        expected: "invalid argument",
    },
    {
        name: input_compare_3,
        file: "input_compare.snek",
        input: "true",
        expected: "invalid argument",
    },
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
}
