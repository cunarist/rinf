---
description: Since Dart 3.8 the default formatting changed (trailing commas are no longer preserved); contributors on newer SDKs should set `formatter: trailing_commas: preserve` in `analysis_options.yaml` to keep the repo's style
---

During the ohos PR (#665) the maintainer explained that Dart's default formatting changed in recent versions (a new formatter arrived with Dart 3.8) and that Rinf's old-style trailing-comma formatting is kept by adding to `analysis_options.yaml`:

```yaml
formatter:
  trailing_commas: preserve
```

So large `dart format` diffs from a new SDK are a configuration mismatch, not a reason to reformat the repository. See also the formatter note in [[maintainer-stances-on-prs]].

Evidence: #665
