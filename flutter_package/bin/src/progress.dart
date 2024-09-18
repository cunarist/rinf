// CLI progress bar copied and modified from
// https://github.com/RohitEdathil/ConsoleBars

import 'dart:async';
import 'common.dart';

class ProgressBar {
  /// Total number of steps
  int _total;

  int _current = 0;
  int _progress = 0;
  late int max;

  /// Tracks time
  final _clock = Stopwatch();

  /// Whether a timer should be present
  bool time;

  /// Percentage should be displayed or not
  bool percentage;

  /// The description of the bar
  String _desc;

  /// The chararcter to used as space
  String space;

  /// The character to used as fill
  String fill;

  /// Width of the bar
  int width;

  /// Whether the instance should print nothing
  bool silent;

  int get total => _total;
  String get desc => _desc;

  set desc(String desc) {
    _desc = desc;
    _render();
  }

  set total(int total) {
    _total = total;
    _render();
  }

  /// Arguments:
  /// - `total` : Total number of steps
  /// - `desc` : Simple text shown after the bar (optional)
  /// - `space` : Character denoting empty space (default : '.')
  /// - `fill` : Character denoting filled space (default : '█')
  /// - `time` : Toggle timing mode (default : false)
  /// - `percentage` : Toggle percentage display (default : false)
  /// - `width` : Width of the bar drawn in the CLI (default: 40)
  ProgressBar({
    required int total,
    String desc = '',
    this.space = '.',
    this.fill = '█',
    this.time = false,
    this.percentage = false,
    this.width = 40,
    this.silent = false,
  })  : _desc = desc,
        _total = total {
    max = width;
    if (time) {
      _clock.start();
      scheduleMicrotask(autoRender);
    }
    if (!silent) {
      print('');
    }
    _render();
  }

  /// Updates the _current value to n
  void update(int n) {
    _current = n;
    _render();
  }

  /// Increments the _current value
  void increment({String? desc}) {
    if (desc != null) {
      this._desc = desc;
    }
    _current++;
    _render();
  }

  /// Automatically updates the frame asynchronously
  void autoRender() async {
    while (_clock.isRunning) {
      await Future.delayed(Duration(seconds: 1));
      _render();
    }
  }

  /// Renders a frame of the bar
  void _render() {
    if (silent) {
      return;
    }
    _progress = ((_current / _total) * max).toInt();
    if (_progress >= max) {
      _progress = max;
      if (_clock.isRunning) {
        _clock.stop();
      }
    }
    String? timeStr = null;
    if (time) {
      final rate = _clock.elapsedMicroseconds / (_current == 0 ? 1 : _current);
      final eta = Duration(microseconds: ((_total - _current) * rate).toInt());
      final elpsedStr = _clock.elapsed.toString().substring(0, 10);
      final etaStr = eta.toString().substring(0, 10);
      timeStr = '[ $elpsedStr / $etaStr ]';
    }
    String? perc = null;
    if (percentage) {
      perc = '${(_current * 100 / _total).toStringAsFixed(1)}%';
    }
    final bar = '${fill * _progress}${space * (max - _progress)}';
    final frameParts = [bar, '$_current/$_total', perc, timeStr, ':', _desc];
    final frame = frameParts.where((v) => v != null).toList().join(' ');
    removeCliLines(1);
    print(frame);
  }
}
