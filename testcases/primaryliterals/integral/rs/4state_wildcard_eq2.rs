let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec!{0, 0}),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808, 0],
    data_xz: Some(vec!{0, 0}),
    size: 66,
    signed: true,
};

let c = a.wildcard_eq(b.clone());

assert_eq!(c, logic1b_1());

let actual_string = format!("{}", c);