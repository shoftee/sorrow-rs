#[test]
fn compilation_checks() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/reactive/fail/01-enum.rs");
    t.compile_fail("tests/reactive/fail/02-union.rs");
    t.compile_fail("tests/reactive/fail/03-empty-struct.rs");
    t.compile_fail("tests/reactive/fail/04-generic-struct.rs");
    t.compile_fail("tests/reactive/fail/05-complex-field-type.rs");
    t.compile_fail("tests/reactive/fail/06-attr-without-nested.rs");
    t.compile_fail("tests/reactive/fail/07-attr-with-unknown-param.rs");
    t.compile_fail("tests/reactive/fail/08-non-reactive-nested.rs");
    t.compile_fail("tests/reactive/fail/09-use-reserved-ident.rs");

    t.pass("tests/reactive/pass/01-has-reactivity.rs");
    t.pass("tests/reactive/pass/02-inner-reactivity.rs");
    t.pass("tests/reactive/pass/03-struct-from-other-module.rs");
}
