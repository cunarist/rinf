---
description: As of Aug 2026 the maintainer publicly asked for a successor with full authority over API design and the build system; expect slow reviews and prefer small safe changes
---

README commit 025ba642 (2026-08-12) added a "Looking for a New Maintainer" banner: the maintainer cannot dedicate as much time as the project deserves and wants someone to take over with full authority over API design and build system. Earlier, in #637, the maintainer apologized for slow replies and promised an answer within a week. Reporters note maintainer time is the bottleneck.

Recent work is mostly toolchain-floor maintenance, contributor-led platform support (ohos #665, Windows ARM64 #650, Gradle 9 #694) and fewer automated PRs ([[dependabot-limited-to-github-actions]]). Contributors carry their own validation (ohos could not run in CI) and should keep changes minimal and documented. Triage style: thank the reporter, ask for `flutter build -v` output or a minimal repro, confirm the bug, say PRs are welcome, and explain roadmap reasons for rejections ([[maintainer-stances-on-prs]]).

Evidence: #637, #650, #665, #694, #695, commit 025ba642
