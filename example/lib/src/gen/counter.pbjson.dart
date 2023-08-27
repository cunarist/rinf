//
//  Generated code. Do not modify.
//  source: counter.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use counterRequestDescriptor instead')
const CounterRequest$json = {
  '1': 'CounterRequest',
  '2': [
    {'1': 'letter', '3': 1, '4': 1, '5': 9, '10': 'letter'},
    {'1': 'before_number', '3': 2, '4': 1, '5': 13, '10': 'beforeNumber'},
    {'1': 'dummy_one', '3': 3, '4': 1, '5': 13, '10': 'dummyOne'},
    {'1': 'dummy_two', '3': 4, '4': 1, '5': 13, '10': 'dummyTwo'},
    {'1': 'dummy_three', '3': 5, '4': 3, '5': 13, '10': 'dummyThree'},
  ],
};

/// Descriptor for `CounterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List counterRequestDescriptor = $convert.base64Decode(
    'Cg5Db3VudGVyUmVxdWVzdBIWCgZsZXR0ZXIYASABKAlSBmxldHRlchIjCg1iZWZvcmVfbnVtYm'
    'VyGAIgASgNUgxiZWZvcmVOdW1iZXISGwoJZHVtbXlfb25lGAMgASgNUghkdW1teU9uZRIbCglk'
    'dW1teV90d28YBCABKA1SCGR1bW15VHdvEh8KC2R1bW15X3RocmVlGAUgAygNUgpkdW1teVRocm'
    'Vl');

@$core.Deprecated('Use counterResponseDescriptor instead')
const CounterResponse$json = {
  '1': 'CounterResponse',
  '2': [
    {'1': 'after_number', '3': 1, '4': 1, '5': 5, '10': 'afterNumber'},
    {'1': 'dummy_one', '3': 2, '4': 1, '5': 5, '10': 'dummyOne'},
    {'1': 'dummy_two', '3': 3, '4': 1, '5': 5, '10': 'dummyTwo'},
    {'1': 'dummy_three', '3': 4, '4': 3, '5': 5, '10': 'dummyThree'},
  ],
};

/// Descriptor for `CounterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List counterResponseDescriptor = $convert.base64Decode(
    'Cg9Db3VudGVyUmVzcG9uc2USIQoMYWZ0ZXJfbnVtYmVyGAEgASgFUgthZnRlck51bWJlchIbCg'
    'lkdW1teV9vbmUYAiABKAVSCGR1bW15T25lEhsKCWR1bW15X3R3bxgDIAEoBVIIZHVtbXlUd28S'
    'HwoLZHVtbXlfdGhyZWUYBCADKAVSCmR1bW15VGhyZWU=');

