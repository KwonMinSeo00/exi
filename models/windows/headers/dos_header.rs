use super::super::Rule;

/*
    DOS signature: byte 0 is 0x4D ('M'), and byte 1 is 0x5A ('Z').
    Direct byte comparison needs no little-endian integer conversion.
*/
pub const DOS_SIGNATURE: Rule = Rule {
    id: "PE.DOS.MZ",
    offset: 0,
    expected: b"MZ",
};
