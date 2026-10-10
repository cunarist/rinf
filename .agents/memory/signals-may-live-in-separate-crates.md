---
description: Signals may live in other crates only if the crate is under `native/` and listed in `gen_input_crates`; recursive `Vec<Self>` derive overflow was fixed in 8.5.0
---

Answer to the recurring question (#629). The hub crate name itself is still required (#684, unanswered). Recursive `Vec<Self>` overflowed the derive (#613; workaround was a manual `impl SignalPiece`), fixed by #615 in 8.5.0; #597 was the same problem. See [[recursive-signals-option-box]].

Evidence: #597, #613, #615, #629, #684
