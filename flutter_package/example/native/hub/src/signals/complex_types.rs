//! These signals represent complex types used for unit tests.

// Copied from https://github.com/zefchain/serde-reflection

use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(
  Serialize, Deserialize, PartialEq, DartSignal, RustSignal, SignalPiece, Clone,
)]
pub enum SerdeData {
  PrimitiveTypes(PrimitiveTypes),
  OtherTypes(Box<OtherTypes>),
  UnitVariant,
  NewTypeVariant(String),
  TupleVariant(u32, #[serde(skip)] NotSerializable, u64),
  StructVariant {
    #[serde(skip)]
    ignored: NotSerializable,
    f0: UnitStruct,
    f1: NewTypeStruct,
    f2: TupleStruct,
    f3: Struct,
  },
  ListWithMutualRecursion(List),
  TreeWithMutualRecursion(Tree),
  TupleArray([u32; 3]),
  UnitVector(Vec<()>),
  SimpleList(SimpleList),
  CStyleEnum(CStyleEnum),
  ComplexMap(BTreeMap<([u32; 2], [u8; 4]), ()>),
  EmptyTupleVariant(),
  EmptyStructVariant {},
}

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct PrimitiveTypes {
  pub f_bool: bool,
  pub f_u8: u8,
  pub f_u16: u16,
  pub f_u32: u32,
  pub f_u64: u64,
  pub f_u128: u128,
  pub f_i8: i8,
  pub f_i16: i16,
  pub f_i32: i32,
  pub f_i64: i64,
  pub f_i128: i128,
  // The following types are not supported by our bincode runtime,
  // therefore  we don't populate them for testing.
  pub f_f32: Option<f32>,
  pub f_f64: Option<f64>,
  pub f_char: Option<char>,
}

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct OtherTypes {
  pub f_string: String,
  pub f_bytes: Vec<u8>,
  pub f_option: Option<Struct>,
  pub f_unit: (),
  pub f_seq: Vec<Struct>,
  pub f_opt_seq: Option<Vec<i32>>,
  pub f_tuple: (u8, u16),
  pub f_string_hashmap: HashMap<String, u32>,
  pub f_string_btreemap: BTreeMap<String, u32>,
  pub f_int_hashset: HashSet<u64>,
  pub f_int_btreeset: BTreeSet<u64>,
  pub f_nested_seq: Vec<Vec<Struct>>,
  pub f_boxed_struct: Box<Struct>,
}

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct UnitStruct;

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct NewTypeStruct(pub u64);

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct TupleStruct(pub u32, pub u64);

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct Struct {
  pub x: u32,
  pub y: u64,
}

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub enum List {
  Empty,
  Node(Box<SerdeData>, Box<List>),
}

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct Tree {
  pub value: Box<SerdeData>,
  pub children: Vec<Tree>,
}

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub struct SimpleList(pub Option<Box<SimpleList>>);

#[derive(Serialize, Deserialize, PartialEq, SignalPiece, Clone)]
pub enum CStyleEnum {
  A,
  B,
  C,
  D,
  E = 10,
}

#[derive(Default, PartialEq, Clone)]
pub struct NotSerializable;
