part of generated_types;

@immutable
class SampleNumberInput {
  const SampleNumberInput({
    required this.letter,
    required this.dummyOne,
    required this.dummyTwo,
    required this.dummyThree,
  });

  static SampleNumberInput deserialize(BinaryDeserializer deserializer) {
    deserializer.increaseContainerDepth();
    final instance = SampleNumberInput(
      letter: :: prost :: alloc :: string :: String.deserialize(deserializer),
      dummyOne: deserializer.deserializeUint32(),
      dummyTwo: :: core :: option :: Option < SampleSchema >.deserialize(deserializer),
      dummyThree: :: prost :: alloc :: vec :: Vec < i32 >.deserialize(deserializer),
    );
    deserializer.decreaseContainerDepth();
    return instance;
  }

  static SampleNumberInput bincodeDeserialize(Uint8List input) {
    final deserializer = BincodeDeserializer(input);
    final value = SampleNumberInput.deserialize(deserializer);
    if (deserializer.offset < input.length) {
      throw Exception('Some input bytes were not read');
    }
    return value;
  }

  final :: prost :: alloc :: string :: String letter;
  final int dummyOne;
  final :: core :: option :: Option < SampleSchema > dummyTwo;
  final :: prost :: alloc :: vec :: Vec < i32 > dummyThree;

  SampleNumberInput copyWith({
    :: prost :: alloc :: string :: String? letter,
    int? dummyOne,
    :: core :: option :: Option < SampleSchema >? dummyTwo,
    :: prost :: alloc :: vec :: Vec < i32 >? dummyThree,
  }) {
    return SampleNumberInput(
      letter: letter ?? this.letter,
      dummyOne: dummyOne ?? this.dummyOne,
      dummyTwo: dummyTwo ?? this.dummyTwo,
      dummyThree: dummyThree ?? this.dummyThree,
    );
  }

  void serialize(BinarySerializer serializer) {
    serializer.increaseContainerDepth();
    letter.serialize(serializer);
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

    return other is SampleNumberInput
      && letter == other.letter
      && dummyOne == other.dummyOne
      && dummyTwo == other.dummyTwo
      && dummyThree == other.dummyThree;
  }

  @override
  int get hashCode => Object.hash(
        letter,
        dummyOne,
        dummyTwo,
        dummyThree,
      );

  @override
  String toString() {
    String? fullString;

    assert(() {
      fullString = '$runtimeType('
        'letter: $letter, '
        'dummyOne: $dummyOne, '
        'dummyTwo: $dummyTwo, '
        'dummyThree: $dummyThree'
        ')';
      return true;
    }());

    return fullString ?? 'SampleNumberInput';
  }
}
