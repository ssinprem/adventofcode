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
            assert_eq!($solver(input), $expect);
        }
    };
}

#[macro_export]
macro_rules! test_cases {
    (
        $fn_name:ident,
        $solver1:path,
        $expect1:expr,
        $solver2:path,
        $expect2:expr,
        $input:expr
    ) => {
        paste::paste! {
        #[test]
        fn [< $fn_name _part1 >]() {
            let input = $input.to_string();
            assert_eq!($solver1(input),$expect1);
        }

        #[test]
        fn [< $fn_name _part2 >]() {
            let input = $input.to_string();
            assert_eq!($solver2(input),$expect2);
        }
        }
    };
}

#[macro_export]
macro_rules! test_case_n {
    (
        $fn_name:ident,
        $input:expr,
        $( $part:ident: ($solver:path, $expect:expr $(, $args:expr)*) ),+
    ) => {
        paste::paste! {
            $(
                #[test]
                fn [< $fn_name _ $part >]() {
                    let input = $input.to_string();
                    assert_eq!($solver(input $(, $args)*), $expect);
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! test_case_n_ignore {
    (
        $fn_name:ident,
        $input:expr,
        $( $part:ident: ($solver:path, $expect:expr $(, $args:expr)*) ),+
    ) => {
        paste::paste! {
            $(
                #[test]
                #[ignore]
                fn [< $fn_name _ $part >]() {
                    let input = $input.to_string();
                    assert_eq!($solver(input $(, $args)*), $expect);
                }
            )+
        }
    };
}
