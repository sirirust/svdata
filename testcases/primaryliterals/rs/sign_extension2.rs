let mut a = SvPrimaryLiteral {
    data01: vec![1, 9223372036854775808],
    num_bits: 65,
    signed: true,
};

a._sign_extension();

let exp = SvPrimaryLiteral {
    data01: vec![18446744073709551615, 9223372036854775808],
    num_bits: 128,
    signed: true,
};

assert_eq!(a, exp);

let actual_string = format!("{}", a);