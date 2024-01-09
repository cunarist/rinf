import 'dart:typed_data';

/// Available operations that a `RustRequest` object can hold.
/// There are 4 options, `Create`,`Read`,`Update`, and `Delete`.
enum RustOperation {
  Create,
  Read,
  Update,
  Delete,
}

/// Request object that is sent from Dart to Rust.
class RustRequest {
  final int resource;
  final RustOperation operation;
  final Uint8List? message;
  final Uint8List? blob;

  const RustRequest({
    required this.resource,
    required this.operation,
    this.message,
    this.blob,
  });
}

/// Wrapper for `RustRequest` with a unique ID.
class RustRequestUnique {
  final int id;
  final RustRequest request;

  const RustRequestUnique({
    required this.id,
    required this.request,
  });
}

/// Response object that is sent from Rust to Dart.
class RustResponse {
  final Uint8List? message;
  final Uint8List? blob;

  const RustResponse({
    this.message,
    this.blob,
  });
}

/// Wrapper for `RustResponse` with a unique ID.
class RustResponseUnique {
  final int id;
  final RustResponse? response;

  const RustResponseUnique({
    required this.id,
    this.response,
  });
}

/// Holds the data that Rust streams to Dart.
class RustSignal {
  final int resource;
  final Uint8List? message;
  final Uint8List? blob;

  const RustSignal({
    required this.resource,
    this.message,
    this.blob,
  });
}
