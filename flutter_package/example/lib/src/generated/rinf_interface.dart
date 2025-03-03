part of 'generated.dart';

typedef SendDartSignalExtern = Void Function(
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);

extension SampleFractalRustSignalExt on SampleFractal {
  static final rustStreamContoller =
      StreamController<RustSignal<SampleFractal>>();
  static final rustSignalStream =
      rustStreamContoller.stream.asBroadcastStream();
}

extension SampleNumberOutputRustSignalExt on SampleNumberOutput {
  static final rustStreamContoller =
      StreamController<RustSignal<SampleNumberOutput>>();
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
    if (useLocalSpaceSymbols) {
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
