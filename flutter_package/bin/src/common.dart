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
