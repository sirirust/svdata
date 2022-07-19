let a = SvPrimaryLiteral {
    data01: vec![3],
    num_bits: 3,
    signed: true,
};

let b = SvPrimaryLiteral {
    data01: vec![4],
    num_bits: 4,
    signed: true,
};

let c: SvPrimaryLiteral = a.mul(b.clone());

let exp = SvPrimaryLiteral {
    data01: vec![12],
    num_bits: 5,
    signed: true,
};

assert_eq!(c, exp);

let actual_string = format!("{}", c);