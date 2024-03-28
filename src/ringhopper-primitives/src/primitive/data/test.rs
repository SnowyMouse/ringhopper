use crate::primitive::*;

use super::*;

#[test]
fn test_byte_array_rw() {
    let expected_data: [u8; 0x134] = [
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,

        0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A,
        0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A,
        0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A,
        0x1A, 0x1A, 0x1A, 0x1A
    ];

    let tag_data_file: &[u8] = &[
        0x00, 0x00, 0x01, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,

        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,
        0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86, 0x86,

        0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A,
        0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A,
        0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A, 0x1A,
        0x1A, 0x1A, 0x1A, 0x1A
    ];

    let read_data = Data::read_from_tag_file(tag_data_file, 0, 0x14, &mut 0x14).unwrap();
    assert_eq!(&expected_data[..], &read_data.bytes[..]);

    let mut write_data = [0u8; 0x14].to_vec();
    read_data.write_to_tag_file(&mut write_data, 0, 0x14).unwrap();

    assert_eq!(&write_data[..], tag_data_file);
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Vector3DHolder {
    pub vector: Vector3D
}

impl TagData for Vector3DHolder {
    fn size() -> usize {
        <Vector3D as TagData>::size()
    }
    fn read_from_tag_file(data: &[u8], at: usize, struct_end: usize, extra_data_cursor: &mut usize) -> RinghopperResult<Self> {
        Ok(Self {
            vector: Vector3D::read_from_tag_file(data, at, struct_end, extra_data_cursor)?
        })
    }
    fn write_to_tag_file(&self, data: &mut Vec<u8>, at: usize, struct_end: usize) -> RinghopperResult<()> {
        self.vector.write_to_tag_file(data, at, struct_end)
    }
    fn read_from_map<M: Map>(_map: &M, _address: usize, _domain_type: &DomainType) -> RinghopperResult<Self> where Self: Sized {
        unimplemented!()
    }
}
impl TagDataDefaults for Vector3DHolder {}

#[test]
fn reflexive_rw() {
    // Test reading a reflexive from tag data
    let array_of_vectors_bytes: &[u8] = &[
        0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

        0x3F, 0x80, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00,
        0x40, 0x80, 0x00, 0x00, 0x40, 0xA0, 0x00, 0x00, 0x40, 0xC0, 0x00, 0x00,
        0x40, 0xE0, 0x00, 0x00, 0x41, 0x00, 0x00, 0x00, 0x41, 0x10, 0x00, 0x00
    ];

    let vectors: Reflexive<Vector3DHolder> = Reflexive::<Vector3DHolder>::read_from_tag_file(array_of_vectors_bytes, 0, 0xC, &mut 0xC).unwrap();
    let expected = &[
        Vector3DHolder { vector: Vector3D { x: 1.0, y: 2.0, z: 3.0 } },
        Vector3DHolder { vector: Vector3D { x: 4.0, y: 5.0, z: 6.0 } },
        Vector3DHolder { vector: Vector3D { x: 7.0, y: 8.0, z: 9.0 } },
    ];
    assert_eq!(&vectors.items[..], expected);

    let mut new_array_bytes = Vec::new();
    new_array_bytes.resize(0xC, 0);
    vectors.write_to_tag_file(&mut new_array_bytes, 0, 0xC).unwrap();
    assert_eq!(array_of_vectors_bytes, &new_array_bytes[..]);
}

#[test]
fn reflexive_range_works() {
    // individual elements
    assert_eq!(parse_range("0", 1).unwrap(), [(0,0)]);
    assert_eq!(parse_range("2", 5).unwrap(), [(2,2)]);
    assert_eq!(parse_range("e", 5).unwrap(), [(4,4)]);

    // range selection
    assert_eq!(parse_range("0-e", 1).unwrap(), [(0,0)]);
    assert_eq!(parse_range("0-1", 2).unwrap(), [(0,1)]);
    assert_eq!(parse_range("e-e", 3).unwrap(), [(2,2)]); // you probably shouldn't do this, but you can

    // "*" specifically returns an empty vector if the set is empty; otherwise it returns everything in the set
    assert_eq!(parse_range("*", 0).unwrap(), []);
    assert_eq!(parse_range("*", 1).unwrap(), [(0,0)]);
    assert_eq!(parse_range("*", 2).unwrap(), [(0,1)]);

    // multiple ranges
    assert_eq!(parse_range("0-1,6-8,15,10,e", 20).unwrap(), [(0,1), (6,8), (10,10), (15,15), (19,19)]);
    assert_eq!(parse_range("2-3,*", 20).unwrap(), [(0,19)]); // overlapping is allowed, be careful!

    assert!(parse_range("notvalid", 0).is_err()); // error; invalid characters
    assert!(parse_range("0-1,,2-3", 0).is_err()); // error; empty range in middle
    assert!(parse_range("0-1-2", 0).is_err());    // error; multiple things here
    assert!(parse_range("0", 0).is_err());        // error; out-of-bounds
    assert!(parse_range("0-0", 0).is_err());      // error; out-of-bounds
    assert!(parse_range("e-e", 0).is_err());      // error; out-of-bounds
    assert!(parse_range("0-e", 0).is_err());      // error; same as 0-0, you should use '*' if you want to select everything even on empty sets
    assert!(parse_range("0-1", 1).is_err());      // error; 1 is out of bounds
    assert!(parse_range("2-1", 3).is_err());      // error; start can't be after end
}
