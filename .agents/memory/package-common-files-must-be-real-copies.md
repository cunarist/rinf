---
description: Symlinking CHANGELOG, LICENSE and README into package folders was tried and removed the same day because publishing broke
---

On 2023-10-18 the common files were replaced with symlinks (6bda82d3), then restored as real files (74e6d7ee "Fix publishing issues", 1c51a4f0 "Remove files that Git still treats as symlinks"). 4.12.1 to 4.12.4 were publishing, badge and file-permission fixes (feb2169b). Keep real copies in each published package and check Git file modes after moving files. See [[publish-only-from-linux-ci]].

Evidence: commits 6bda82d3, 3d04824c, 74e6d7ee, 1c51a4f0, feb2169b; CHANGELOG 4.12.1 to 4.12.4
