library generated_types;

import 'dart:typed_data';
import 'package:meta/meta.dart';
import 'package:tuple/tuple.dart';
import '../serde/serde.dart';
import '../bincode/bincode.dart';

import 'dart:async';
import 'dart:ffi';
import 'package:rinf/rinf.dart';

export '../serde/serde.dart';

part 'trait_helpers.dart';
part 'sample_fractal.dart';
part 'sample_number_input.dart';
part 'sample_number_output.dart';
part 'sample_schema.dart';

typedef SendDartSignalExtern = Void Function(
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);
