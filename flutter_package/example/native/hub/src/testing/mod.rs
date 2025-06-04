use crate::signals::{
  CStyleEnum, ComplexSignalTestResult, List, NewTypeStruct, NotSerializable,
  OtherTypes, PrimitiveTypes, SerdeData, SimpleList, Struct, Tree, TupleStruct,
  UnitStruct, UnitTestEnd, UnitTestStart,
};
use rinf::{DartSignal, RustSignal};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter::repeat_n;
use std::time::Duration;
use tokio::time::sleep;
use tokio_with_wasm::alias as tokio;

/// Sends complex signals to Dart and wait for them to come back.
/// This function is used for unit testing.
pub async fn run_unit_tests() {
  // Wait until the start signal arrives.
  let start_receiver = UnitTestStart::get_dart_signal_receiver();
  start_receiver.recv().await;

  // Pass signals back and forth.
  let duration = Duration::from_millis(100);
  let signal_receiver = SerdeData::get_dart_signal_receiver();
  let complex_signals = get_complex_signals();
  for sent in complex_signals {
    sent.clone().send_signal_to_dart();
    let signal_pack = match signal_receiver.recv().await {
      Some(inner) => inner,
      None => continue,
    };
    let received = signal_pack.message;
    ComplexSignalTestResult(sent == received).send_signal_to_dart();
    sleep(duration).await;
  }

  // Tell Dart that the test is done.
  UnitTestEnd.send_signal_to_dart();
}

/// Manually generate sample values.
fn get_complex_signals() -> Vec<SerdeData> {
  let v0 = SerdeData::PrimitiveTypes(PrimitiveTypes {
    f_bool: false,
    f_u8: 6,
    f_u16: 5,
    f_u32: 4,
    f_u64: 3,
    f_u128: 2,
    f_i8: 1,
    f_i16: 0,
    f_i32: -1,
    f_i64: -2,
    f_i128: -3,
    f_f32: Some(0.4),
    f_f64: Some(35.21),
    f_char: None,
  });

  let v1 = SerdeData::PrimitiveTypes(PrimitiveTypes {
    f_bool: true,
    f_u8: u8::MAX,
    f_u16: u16::MAX,
    f_u32: u32::MAX,
    f_u64: u64::MAX,
    f_u128: u128::MAX,
    f_i8: i8::MIN,
    f_i16: i16::MIN,
    f_i32: i32::MIN,
    f_i64: i64::MIN,
    f_i128: i128::MIN,
    f_f32: Some(-4111.0),
    f_f64: Some(-0.0021),
    f_char: None,
  });

  let v2 = SerdeData::OtherTypes(Box::new(OtherTypes {
    f_string: "test".to_string(),
    f_bytes: b"bytes".to_vec(),
    f_option: Some(Struct { x: 2, y: 3 }),
    f_unit: (),
    f_seq: vec![Struct { x: 1, y: 3 }],
    f_opt_seq: Some(vec![1]),
    f_tuple: (4, 5),
    f_string_hashmap: {
      let mut map = HashMap::new();
      map.insert("foo".to_string(), 1);
      map.insert("bar".to_string(), 2);
      map
    },
    f_string_btreemap: {
      let mut map = BTreeMap::new();
      map.insert("foo".to_string(), 1);
      map.insert("bar".to_string(), 2);
      map
    },
    f_int_hashset: HashSet::new(),
    f_int_btreeset: BTreeSet::new(),
    f_nested_seq: vec![
      vec![Struct { x: 4, y: 5 }, Struct { x: 6, y: 7 }],
      vec![Struct { x: 8, y: 9 }],
    ],
    f_boxed_struct: Box::new(Struct { x: 10, y: 11 }),
  }));

  let v2bis = SerdeData::OtherTypes(Box::new(OtherTypes {
    f_string: "".to_string(),
    f_bytes: b"".to_vec(),
    f_option: None,
    f_unit: (),
    f_seq: Vec::new(),
    f_opt_seq: None,
    f_tuple: (4, 5),
    f_string_hashmap: HashMap::new(),
    f_string_btreemap: BTreeMap::new(),
    f_int_hashset: {
      let mut map = HashSet::new();
      map.insert(1);
      map.insert(5);
      map.insert(16);
      map.insert(64);
      map.insert(257);
      map.insert(1024);
      map
    },
    f_int_btreeset: {
      let mut map = BTreeSet::new();
      map.insert(1);
      map.insert(5);
      map.insert(16);
      map.insert(64);
      map.insert(257);
      map.insert(1024);
      map
    },
    f_nested_seq: Vec::new(),
    f_boxed_struct: Box::new(Struct { x: 0, y: 0 }),
  }));

  let v2ter = SerdeData::OtherTypes(Box::new(OtherTypes {
    f_string: "".to_string(),
    f_bytes: vec![1u8; 129],
    f_option: None,
    f_unit: (),
    f_seq: Vec::new(),
    f_opt_seq: None,
    f_tuple: (4, 5),
    f_string_hashmap: HashMap::new(),
    f_string_btreemap: BTreeMap::new(),
    f_int_hashset: repeat_n((), 10)
      .enumerate()
      .map(|(i, ())| i as u64)
      .collect(),
    f_int_btreeset: repeat_n((), 10)
      .enumerate()
      .map(|(i, ())| i as u64)
      .collect(),
    f_nested_seq: Vec::new(),
    f_boxed_struct: Box::new(Struct { x: 0, y: 0 }),
  }));

  let v3 = SerdeData::UnitVariant;

  let v4 = SerdeData::NewTypeVariant(
    "test.\u{10348}.\u{00a2}\u{0939}\u{20ac}\u{d55c}..".to_string(),
  );

  let v5 = SerdeData::TupleVariant(3, NotSerializable, 6);

  let v6 = SerdeData::StructVariant {
    ignored: NotSerializable,
    f0: UnitStruct,
    f1: NewTypeStruct(1),
    f2: TupleStruct(2, 3),
    f3: Struct { x: 4, y: 5 },
  };

  let v7 = SerdeData::ListWithMutualRecursion(List::Empty);

  let v8 = SerdeData::TreeWithMutualRecursion(Tree {
    value: Box::new(SerdeData::PrimitiveTypes(PrimitiveTypes {
      f_bool: false,
      f_u8: 0,
      f_u16: 1,
      f_u32: 2,
      f_u64: 3,
      f_u128: 4,
      f_i8: 5,
      f_i16: 6,
      f_i32: 7,
      f_i64: 8,
      f_i128: 9,
      f_f32: None,
      f_f64: None,
      f_char: None,
    })),
    children: vec![Tree {
      value: Box::new(SerdeData::PrimitiveTypes(PrimitiveTypes {
        f_bool: false,
        f_u8: 0,
        f_u16: 0,
        f_u32: 0,
        f_u64: 0,
        f_u128: 0,
        f_i8: 0,
        f_i16: 0,
        f_i32: 0,
        f_i64: 0,
        f_i128: 0,
        f_f32: None,
        f_f64: None,
        f_char: None,
      })),
      children: vec![],
    }],
  });

  let v9 = SerdeData::TupleArray([0, 2, 3]);

  let v10 = SerdeData::UnitVector(vec![(); 1000]);

  let v11 = SerdeData::SimpleList(SimpleList(Some(Box::new(SimpleList(None)))));

  let v12 = SerdeData::CStyleEnum(CStyleEnum::C);

  let v13 = SerdeData::ComplexMap({
    let mut map = BTreeMap::new();
    map.insert(([1, 2], [3, 4, 5, 6]), ());
    map
  });

  let v14 = SerdeData::EmptyTupleVariant();
  let v15 = SerdeData::EmptyStructVariant {};

  vec![
    v0, v1, v2, v2bis, v2ter, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13,
    v14, v15,
  ]
}
