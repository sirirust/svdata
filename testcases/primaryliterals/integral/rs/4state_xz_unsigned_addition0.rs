let a = SvPrimaryLiteralIntegral {
    data_01: vec![0],
    data_xz: Some(vec![9223372036854775808]),
    size: 64,
    signed: false,
};

let b: SvPrimaryLiteralIntegral = a + 9223372036854775808;

let exp = SvPrimaryLiteralIntegral {
    data_01: vec![0, 0],
    data_xz: Some(vec![18446744073709551615, 1]),
    size: 65,
    signed: false,
}

assert_eq!(b, exp);

let actual_string = format!("{}", b);