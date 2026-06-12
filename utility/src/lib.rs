#[macro_export]
macro_rules! test_case {
    (
        $fn_name:ident,
        $solver:path,
        $expect:expr,
        $input:expr
    ) => {
#[test]
fn $fn_name() {
    let input = $input.to_string();
    assert_eq!($solver(input),$expect);
}
    };
}