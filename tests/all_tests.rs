mod infra;

// Your tests go here!
success_tests! {
    number: "37",
    add1: "73",
    sub1: "-2",
    add: "15",
    add_and_sub: "72",
    nested_arith: "25",
    binding: "5",
    multiple_correct_binds: "6",
    nested_binds: "12",
    bind_chain: "5",
    shadow_bind: "12",
    bind_in_funny_place: "7",
}

failure_tests! {
    unbound_id: "Unbound variable identifier x",
    duplicate_binding: "Duplicate binding",
    parse_let_fail: "Invalid expression provided"
}
