let a = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: true,
};

let b = SvPrimaryLiteralIntegral {
    data_01: vec![9223372036854775808],
    data_xz: None,
    size: 64,
    signed: false,
};

let c = a.case_eq(b.clone());
let one = SvPrimaryLiteralIntegral {
    data_01: vec![1],
    data_xz: None,
    size: 1,
    signed: false,
};

assert_eq!(c, one);

let actual_string = format!("{}", c);