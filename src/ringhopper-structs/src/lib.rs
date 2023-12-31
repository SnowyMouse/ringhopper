extern crate ringhopper_structs_codegen;
extern crate ringhopper_primitives;

use std::any::Any;

use ringhopper_primitives::primitive::*;
use ringhopper_primitives::parse::*;
use ringhopper_primitives::error::*;
use ringhopper_primitives::tag::PrimaryTagStruct;
use ringhopper_primitives::dynamic::*;
use ringhopper_primitives::tag::{TagFile, ParseStrictness, PrimaryTagStructDyn};

ringhopper_structs_codegen::generate_ringhopper_structs!();
