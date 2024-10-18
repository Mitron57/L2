use unpack::{unpack, UnpackError};

#[test]
fn empty_string() {
    let string = "";
    let unpack = unpack(string).unwrap();
    assert_eq!(unpack, string);
}

#[test]
fn nothing_to_unpack() {
    let string = "abcdefg";
    let unpacked = unpack(string).unwrap();
    assert_eq!(string, unpacked);
}

#[test]
fn valid_unpack() {
    let string = "a3c4e";
    let unpacked = unpack(string).unwrap();
    assert_eq!("aaacccce", unpacked);
}

#[test]
fn escape_unpack() {
    let string = r"a3c4\5\6\\2";
    let unpacked = unpack(string).unwrap();
    assert_eq!(r"aaacccc56\\", unpacked);
}

#[test]
fn invalid_repeat_unpack() {
    let string = "a\\9999999999999999999999999999999";
    let error = unpack(string).err().unwrap();
    if let UnpackError::InvalidRepeatsNumber(_) = error {
        return;
    }
    panic!("Incorrect unpack error: {:?}", error);
}

#[test]
fn starts_with_number() {
    let string = "52";
    let error = unpack(string).err().unwrap();
    if let UnpackError::StartsWithNumber(_) = error {
        return;
    }
    panic!("Incorrect unpack error: {:?}", error);
}

#[test]
fn invalid_escaping() {
    let string = r"a\x";
    let error = unpack(string).err().unwrap();
    if let UnpackError::InvalidEscaping(_) = error {
        return;
    }
    panic!("Incorrect unpack error: {:?}", error);
}
