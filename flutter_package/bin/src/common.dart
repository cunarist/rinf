import 'dart:io';

extension UriJoin on Uri {
  Uri join(String path) {
    if (path.isEmpty || path == '/') {
      // By default, `resolve` method returns root directory
      // when provided string is empty or a slash.
      // We need to override this.
      return this;
    } else {
      // If the path is not empty and not a slash,
      // `resolve` method should handle things properly.
      return this.resolve(path);
    }
  }
}

void clearCliLine() {
  stdout.write('\x1B[2K'); // Clear the line
}

/// Removes an existing line from the CLI.
void removeCliLines(int lines) {
  for (var i = 0; i < lines; i++) {
    stdout.write('\x1B[1A'); // Move the cursor up one line
    clearCliLine();
  }
}
