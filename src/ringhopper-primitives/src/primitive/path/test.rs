use crate::{parse::TagData, primitive::TagGroup};
use crate::primitive::TagPath;

use super::TagReference;

#[test]
fn parse_tag_reference() {
    let bytes_zero_id: &[u8] = &mut [
        0x77, 0x65, 0x61, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0x00,
        0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x73, 0x5C, 0x70, 0x69, 0x73, 0x74, 0x6F, 0x6C, 0x5C, 0x70, 0x69, 0x73, 0x74, 0x6F, 0x6C, 0x00
    ];

    let bytes_set_id: &[u8] = &mut [
        0x77, 0x65, 0x61, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0xFF, 0xFF, 0xFF, 0xFF,
        0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x73, 0x5C, 0x70, 0x69, 0x73, 0x74, 0x6F, 0x6C, 0x5C, 0x70, 0x69, 0x73, 0x74, 0x6F, 0x6C, 0x00
    ];

    let mut cursor = 0x10;
    let tag = TagReference::read_from_tag_file(&bytes_set_id, 0, cursor, &mut cursor).expect("this is valid");
    assert_eq!(cursor, bytes_set_id.len());

    let mut cursor = 0x10;
    let tag_zero = TagReference::read_from_tag_file(&bytes_zero_id, 0, cursor, &mut cursor).expect("this is valid");
    assert_eq!(cursor, bytes_zero_id.len());
    assert_eq!(tag_zero, tag);

    assert_eq!(TagGroup::Weapon, tag.group());
    assert_eq!("weapons\\pistol\\pistol.weapon", tag.path().unwrap().to_internal_path());

    let mut new_bytes = vec![0u8; 16];
    tag.write_to_tag_file(&mut new_bytes, 0, 16).unwrap();
    assert_eq!(bytes_set_id, new_bytes.as_slice());
}

#[test]
fn banned_tag_paths() {
    assert!(TagPath::from_path("weapons\\pistol\\pistol.weapon").is_ok());
    assert!(TagPath::from_path("weapons\\PISTOL\\pistol.weapon").is_ok());
    assert!(TagPath::from_path("weapons\\<pistol>\\pistol.weapon").is_err());
    assert!(TagPath::from_path("weapons\\pistol\x00\\pistol.weapon").is_err());
    assert!(TagPath::from_path("weapons\\pistol\n\\pistol.weapon").is_err());
    assert!(TagPath::from_path("weapons\\pistol\t\\pistol.weapon").is_err());
    assert!(TagPath::from_path("weapons\\con\\pistol.weapon").is_err());
    assert!(TagPath::from_path("weapons\\pistol.\\pistol.weapon").is_err());
    assert!(TagPath::from_path("\\weapons\\pistol\\pistol.weapon").is_err());

    assert_eq!(TagPath::from_path("weapons\\PISTOL\\pistol.weapon").unwrap(), TagPath::from_path("weapons\\pistol\\pistol.weapon").unwrap());
    assert_eq!(TagPath::from_path("weapons\\pistol\\\\pistol.weapon").unwrap(), TagPath::from_path("weapons\\pistol\\pistol.weapon").unwrap());
}
