let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec!{1, 9223372036854775808}),
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec!{9223372036854775808}),
    size: 64,
    signed: true,
};

let c = a.case_eq(b.clone());

assert_eq!(c, bit1b_1());

let actual_string = format!("{}", c);