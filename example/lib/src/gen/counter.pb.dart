//
//  Generated code. Do not modify.
//  source: counter.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class CounterRequest extends $pb.GeneratedMessage {
  factory CounterRequest({
    $core.String? letter,
    $core.int? beforeNumber,
    $core.int? dummyOne,
    $core.int? dummyTwo,
    $core.Iterable<$core.int>? dummyThree,
  }) {
    final $result = create();
    if (letter != null) {
      $result.letter = letter;
    }
    if (beforeNumber != null) {
      $result.beforeNumber = beforeNumber;
    }
    if (dummyOne != null) {
      $result.dummyOne = dummyOne;
    }
    if (dummyTwo != null) {
      $result.dummyTwo = dummyTwo;
    }
    if (dummyThree != null) {
      $result.dummyThree.addAll(dummyThree);
    }
    return $result;
  }
  CounterRequest._() : super();
  factory CounterRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory CounterRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CounterRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'counter'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'letter')
    ..a<$core.int>(
        2, _omitFieldNames ? '' : 'beforeNumber', $pb.PbFieldType.OU3)
    ..a<$core.int>(3, _omitFieldNames ? '' : 'dummyOne', $pb.PbFieldType.OU3)
    ..a<$core.int>(4, _omitFieldNames ? '' : 'dummyTwo', $pb.PbFieldType.OU3)
    ..p<$core.int>(5, _omitFieldNames ? '' : 'dummyThree', $pb.PbFieldType.KU3)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  CounterRequest clone() => CounterRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  CounterRequest copyWith(void Function(CounterRequest) updates) =>
      super.copyWith((message) => updates(message as CounterRequest))
          as CounterRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CounterRequest create() => CounterRequest._();
  CounterRequest createEmptyInstance() => create();
  static $pb.PbList<CounterRequest> createRepeated() =>
      $pb.PbList<CounterRequest>();
  @$core.pragma('dart2js:noInline')
  static CounterRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CounterRequest>(create);
  static CounterRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get letter => $_getSZ(0);
  @$pb.TagNumber(1)
  set letter($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasLetter() => $_has(0);
  @$pb.TagNumber(1)
  void clearLetter() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get beforeNumber => $_getIZ(1);
  @$pb.TagNumber(2)
  set beforeNumber($core.int v) {
    $_setUnsignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasBeforeNumber() => $_has(1);
  @$pb.TagNumber(2)
  void clearBeforeNumber() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get dummyOne => $_getIZ(2);
  @$pb.TagNumber(3)
  set dummyOne($core.int v) {
    $_setUnsignedInt32(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasDummyOne() => $_has(2);
  @$pb.TagNumber(3)
  void clearDummyOne() => clearField(3);

  @$pb.TagNumber(4)
  $core.int get dummyTwo => $_getIZ(3);
  @$pb.TagNumber(4)
  set dummyTwo($core.int v) {
    $_setUnsignedInt32(3, v);
  }

  @$pb.TagNumber(4)
  $core.bool hasDummyTwo() => $_has(3);
  @$pb.TagNumber(4)
  void clearDummyTwo() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.int> get dummyThree => $_getList(4);
}

class CounterResponse extends $pb.GeneratedMessage {
  factory CounterResponse({
    $core.int? afterNumber,
    $core.int? dummyOne,
    $core.int? dummyTwo,
    $core.Iterable<$core.int>? dummyThree,
  }) {
    final $result = create();
    if (afterNumber != null) {
      $result.afterNumber = afterNumber;
    }
    if (dummyOne != null) {
      $result.dummyOne = dummyOne;
    }
    if (dummyTwo != null) {
      $result.dummyTwo = dummyTwo;
    }
    if (dummyThree != null) {
      $result.dummyThree.addAll(dummyThree);
    }
    return $result;
  }
  CounterResponse._() : super();
  factory CounterResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory CounterResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CounterResponse',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'counter'),
      createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'afterNumber', $pb.PbFieldType.O3)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'dummyOne', $pb.PbFieldType.O3)
    ..a<$core.int>(3, _omitFieldNames ? '' : 'dummyTwo', $pb.PbFieldType.O3)
    ..p<$core.int>(4, _omitFieldNames ? '' : 'dummyThree', $pb.PbFieldType.K3)
    ..hasRequiredFields = false;

  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  CounterResponse clone() => CounterResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  CounterResponse copyWith(void Function(CounterResponse) updates) =>
      super.copyWith((message) => updates(message as CounterResponse))
          as CounterResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CounterResponse create() => CounterResponse._();
  CounterResponse createEmptyInstance() => create();
  static $pb.PbList<CounterResponse> createRepeated() =>
      $pb.PbList<CounterResponse>();
  @$core.pragma('dart2js:noInline')
  static CounterResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CounterResponse>(create);
  static CounterResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get afterNumber => $_getIZ(0);
  @$pb.TagNumber(1)
  set afterNumber($core.int v) {
    $_setSignedInt32(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasAfterNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearAfterNumber() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get dummyOne => $_getIZ(1);
  @$pb.TagNumber(2)
  set dummyOne($core.int v) {
    $_setSignedInt32(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasDummyOne() => $_has(1);
  @$pb.TagNumber(2)
  void clearDummyOne() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get dummyTwo => $_getIZ(2);
  @$pb.TagNumber(3)
  set dummyTwo($core.int v) {
    $_setSignedInt32(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasDummyTwo() => $_has(2);
  @$pb.TagNumber(3)
  void clearDummyTwo() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get dummyThree => $_getList(3);
}

const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
