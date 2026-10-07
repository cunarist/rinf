---
description: Update the docs with every change to versions, behavior, or usage, however slight, keeping their format and making them very easy to read
---

Whenever a change touches versions, toolchain requirements, behavior, commands,
or usage, update the documentation in the same change, even when the change
feels too small to mention. Docs live in `documentation/source`, plus the
changelogs and package READMEs.

- Keep the existing format. Extend tables, lists, and headings the way they
  are already written instead of introducing a new layout.
- Write for humans first: short sentences, plain words, and the one thing the
  reader needs to do or know. Leave out internal detail users never act on.
- Example: raising the Rust minimum means a new row in the toolchain table in
  `installing-toolchains.md`, not only a new `rust-version` in `Cargo.toml`.

See [[pin-exact-versions]] and [[research-before-every-change]].
