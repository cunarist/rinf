---
description: SQLite or file access from Rust on iOS fails outside the app sandbox; pass an app-writable directory from Dart and use create mode; files do not work on web
---

A user's sqlx sqlite setup worked in memory but hung on iOS: the Rust thread panicked writing `test.db` next to the binary and the URL lacked `?mode=rwc`. It was not a Rinf bug; any Rust crate works, but file operations do not on web. Pass the documents directory path from Dart.

Evidence: #133
