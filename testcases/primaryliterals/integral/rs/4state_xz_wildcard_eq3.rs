let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec!{0, 9223372036854775809}),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: Some(vec!{1, 9223372036854775809}),
    size: 65,
    signed: true,
};

let c = a.wildcard_eq(b.clone());

assert_eq!(c, logic1b_x());

let actual_string = format!("{}", c);