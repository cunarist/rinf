---
description: Publish to pub.dev only from Linux CI; publishing from Windows broke line endings and executable bits, and shell scripts must be LF
---

Releases 4.10 and 4.11 were published from Windows with CRLF line endings, so Cargokit's `build_pod.sh` failed on macOS/iOS with `set: -e: invalid option`. Other releases lost executable bits (4.12.3, 4.12.4). The fix was a manual GitHub Actions publish workflow on Ubuntu (4.11.2 was the first good release), and LF enforcement on clone (cdf51beb). 6.12.1 was released solely to fix linefeeds in published files.

`set: invalid option` reports are a likely CRLF problem. Maintenance scripts live outside the published package (`automate` excluded via `.pubignore`); the example dir stays published because the template copies from it. For the Windows git-checkout variant see [[windows-path-hazards]]; also [[package-common-files-must-be-real-copies]].

Evidence: #64, #110, #192, #196, #166, #239, commits cdf51beb, 6afa9827, f474fdb2, CHANGELOG 4.11.2, 6.12.1
