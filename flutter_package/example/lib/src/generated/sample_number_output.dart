part of generated_types;

@immutable
class SampleNumberOutput {
  const SampleNumberOutput({
    required this.currentNumber,
    required this.dummyOne,
    required this.dummyTwo,
    required this.dummyThree,
  });

  static SampleNumberOutput deserialize(BinaryDeserializer deserializer) {
    deserializer.increaseContainerDepth();
    final instance = SampleNumberOutput(
      currentNumber: deserializer.deserializeInt32(),
      dummyOne: deserializer.deserializeUint32(),
      dummyTwo: :: core :: option :: Option < SampleSchema >.deserialize(deserializer),
      dummyThree: :: prost :: alloc :: vec :: Vec < i32 >.deserialize(deserializer),
    );
    deserializer.decreaseContainerDepth();
    return instance;
  }

  static SampleNumberOutput bincodeDeserialize(Uint8List input) {
    final deserializer = BincodeDeserializer(input);
    final value = SampleNumberOutput.deserialize(deserializer);
    if (deserializer.offset < input.length) {
      throw Exception('Some input bytes were not read');
    }
    return value;
  }

  final int currentNumber;
  final int dummyOne;
  final :: core :: option :: Option < SampleSchema > dummyTwo;
  final :: prost :: alloc :: vec :: Vec < i32 > dummyThree;

  SampleNumberOutput copyWith({
    int? currentNumber,
    int? dummyOne,
    :: core :: option :: Option < SampleSchema >? dummyTwo,
    :: prost :: alloc :: vec :: Vec < i32 >? dummyThree,
  }) {
    return SampleNumberOutput(
      currentNumber: currentNumber ?? this.currentNumber,
      dummyOne: dummyOne ?? this.dummyOne,
      dummyTwo: dummyTwo ?? this.dummyTwo,
      dummyThree: dummyThree ?? this.dummyThree,
    );
  }

  void serialize(BinarySerializer serializer) {
    serializer.increaseContainerDepth();
    serializer.serializeInt32(currentNumber);
    serializer.serializeUint32(dummyOne);
    dummyTwo.serialize(serializer);
    dummyThree.serialize(serializer);
    serializer.decreaseContainerDepth();
  }

  Uint8List bincodeSerialize() {
      final serializer = BincodeSerializer();
      serialize(serializer);
      return serializer.bytes;
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    if (other.runtimeType != runtimeType) return false;

    return other is SampleNumberOutput
      && currentNumber == other.currentNumber
      && dummyOne == other.dummyOne
      && dummyTwo == other.dummyTwo
      && dummyThree == other.dummyThree;
  }

  @override
  int get hashCode => Object.hash(
        currentNumber,
        dummyOne,
        dummyTwo,
        dummyThree,
      );

  @override
  String toString() {
    String? fullString;

    assert(() {
      fullString = '$runtimeType('
        'currentNumber: $currentNumber, '
        'dummyOne: $dummyOne, '
        'dummyTwo: $dummyTwo, '
        'dummyThree: $dummyThree'
        ')';
      return true;
    }());

    return fullString ?? 'SampleNumberOutput';
  }
}
