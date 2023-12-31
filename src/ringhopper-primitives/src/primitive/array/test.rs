use crate::primitive::*;
use super::*;

#[test]
fn parse_bounds() {
    let array_of_bounds: &[u8] = &[0xBF, 0x80, 0x00, 0x00, 0x3F, 0x80, 0x00, 0x00];
    let b = Bounds::<f32>::read_from_tag_file(&array_of_bounds, 0, 8, &mut 8).expect("should work");
    assert_eq!(Bounds { lower: -1.0f32, upper: 1.0f32 }, b);
}

#[test]
fn parse_array() {
    let array_of_references: &[u8] = &[
        0x77, 0x65, 0x61, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1D, 0xFF, 0xFF, 0xFF, 0xFF,
        0x77, 0x65, 0x61, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0x6D, 0x6F, 0x64, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0xFF, 0xFF, 0xFF, 0xFF,
        0x70, 0x72, 0x6F, 0x6A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0xFF, 0xFF, 0xFF, 0xFF,

        // weapon index 0 is `weapons\someweapon\someweapon.weapon`
        0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x73, 0x5C, 0x73, 0x6F, 0x6D, 0x65, 0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x5C, 0x73, 0x6F, 0x6D, 0x65, 0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x00,

        // weapon index 1 is null
        // --

        // weapon index 2 is `weapons\anotherweapon\anotherweapon.model`
        0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x73, 0x5C, 0x61, 0x6E, 0x6F, 0x74, 0x68, 0x65, 0x72, 0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x5C, 0x61, 0x6E, 0x6F, 0x74, 0x68, 0x65, 0x72, 0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x00,

        // weapon index 3 is `weapons\thefinalweapon\thefinalweapon.projectile`
        0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x73, 0x5C, 0x74, 0x68, 0x65, 0x66, 0x69, 0x6E, 0x61, 0x6C, 0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x5C, 0x74, 0x68, 0x65, 0x66, 0x69, 0x6E, 0x61, 0x6C, 0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x00
    ];

    let references = <[TagReference; 4]>::read_from_tag_file(&array_of_references, 0, 0x40, &mut 0x40).expect("should work");
    assert_eq!(TagReference::Set(TagPath::new("weapons\\someweapon\\someweapon", TagGroup::Weapon).unwrap()), references[0]);
    assert_eq!(TagReference::Null(TagGroup::Weapon), references[1]);
    assert_eq!(TagReference::Set(TagPath::new("weapons\\anotherweapon\\anotherweapon", TagGroup::Model).unwrap()), references[2]);
    assert_eq!(TagReference::Set(TagPath::new("weapons\\thefinalweapon\\thefinalweapon", TagGroup::Projectile).unwrap()), references[3]);
}
