let a = SvPrimaryLiteralIntegral {
    data_01: vec![0, 9223372036854775808],
    data_xz: None,
    size: 65,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 66,
    signed: false,
};

let c: bool = a.case_eq(b.clone());

assert_eq!(c, true);

let actual_string = format!("{}", c);