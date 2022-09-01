let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: Some(vec![9223372036854775808]),
    size: 64,
    signed: true,
};

let b: SvPrimaryLiteralIntegral = a >> 2;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![2305843009213693952],
    data_xz: Some(vec![2305843009213693952]),
    size: 64,
    signed: true,
};

assert_eq!(b, exp);

let actual_string = format!("{}", b);