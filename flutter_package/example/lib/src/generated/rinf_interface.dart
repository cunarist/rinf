part of 'generated.dart';

typedef SendDartSignalExtern = Void Function(
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);

extension SampleFractalDartSignalExt on SampleFractal {
  @Native<SendDartSignalExtern>(
    isLeaf: true,
    symbol: 'rinf_send_dart_signal_sample_fractal',
  )
  external static void sendDartSignalExtern(
    Pointer<Uint8> messageBytesAddress,
    int messageBytesLength,
    Pointer<Uint8> binaryAddress,
    int binaryLength,
  );

  void sendSignalToRust() {
    final messageBytes = this.bincodeSerialize();
    final binary = Uint8List(0);
    if (useLocalSpaceSymbol) {
      sendDartSignal(
        'rinf_send_dart_signal_sample_fractal',
        messageBytes,
        binary,
      );
    } else {
      sendDartSignalExtern(
        messageBytes.address,
        messageBytes.length,
        binary.address,
        binary.length,
      );
    }
  }
}

extension SampleFractalRustSignalExt on SampleFractal {
  static final rustStreamContoller =
      StreamController<RustSignal<SampleFractal>>();
  static final rustSignalStream =
      rustStreamContoller.stream.asBroadcastStream();
}

extension SampleNumberInputDartSignalExt on SampleNumberInput {
  @Native<SendDartSignalExtern>(
    isLeaf: true,
    symbol: 'rinf_send_dart_signal_sample_number_input',
  )
  external static void sendDartSignalExtern(
    Pointer<Uint8> messageBytesAddress,
    int messageBytesLength,
    Pointer<Uint8> binaryAddress,
    int binaryLength,
  );

  void sendSignalToRust() {
    final messageBytes = this.bincodeSerialize();
    final binary = Uint8List(0);
    if (useLocalSpaceSymbol) {
      sendDartSignal(
        'rinf_send_dart_signal_sample_number_input',
        messageBytes,
        binary,
      );
    } else {
      sendDartSignalExtern(
        messageBytes.address,
        messageBytes.length,
        binary.address,
        binary.length,
      );
    }
  }
}

extension SampleNumberInputRustSignalExt on SampleNumberInput {
  static final rustStreamContoller =
      StreamController<RustSignal<SampleNumberInput>>();
  static final rustSignalStream =
      rustStreamContoller.stream.asBroadcastStream();
}

extension SampleNumberOutputDartSignalExt on SampleNumberOutput {
  @Native<SendDartSignalExtern>(
    isLeaf: true,
    symbol: 'rinf_send_dart_signal_sample_number_output',
  )
  external static void sendDartSignalExtern(
    Pointer<Uint8> messageBytesAddress,
    int messageBytesLength,
    Pointer<Uint8> binaryAddress,
    int binaryLength,
  );

  void sendSignalToRust() {
    final messageBytes = this.bincodeSerialize();
    final binary = Uint8List(0);
    if (useLocalSpaceSymbol) {
      sendDartSignal(
        'rinf_send_dart_signal_sample_number_output',
        messageBytes,
        binary,
      );
    } else {
      sendDartSignalExtern(
        messageBytes.address,
        messageBytes.length,
        binary.address,
        binary.length,
      );
    }
  }
}

extension SampleNumberOutputRustSignalExt on SampleNumberOutput {
  static final rustStreamContoller =
      StreamController<RustSignal<SampleNumberOutput>>();
  static final rustSignalStream =
      rustStreamContoller.stream.asBroadcastStream();
}

extension SampleSchemaDartSignalExt on SampleSchema {
  @Native<SendDartSignalExtern>(
    isLeaf: true,
    symbol: 'rinf_send_dart_signal_sample_schema',
  )
  external static void sendDartSignalExtern(
    Pointer<Uint8> messageBytesAddress,
    int messageBytesLength,
    Pointer<Uint8> binaryAddress,
    int binaryLength,
  );

  void sendSignalToRust() {
    final messageBytes = this.bincodeSerialize();
    final binary = Uint8List(0);
    if (useLocalSpaceSymbol) {
      sendDartSignal(
        'rinf_send_dart_signal_sample_schema',
        messageBytes,
        binary,
      );
    } else {
      sendDartSignalExtern(
        messageBytes.address,
        messageBytes.length,
        binary.address,
        binary.length,
      );
    }
  }
}

extension SampleSchemaRustSignalExt on SampleSchema {
  static final rustStreamContoller =
      StreamController<RustSignal<SampleSchema>>();
  static final rustSignalStream =
      rustStreamContoller.stream.asBroadcastStream();
}
