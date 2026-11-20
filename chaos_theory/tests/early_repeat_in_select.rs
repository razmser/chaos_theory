//! A `repeat_n` that stops early inside a `select` variant leaves the tape
//! validator in the wrong state for the next draw.

use chaos_theory::{Effect, check};

#[test]
fn early_ended_repeat_in_select_variant() {
    check(|src| {
        src.select("s", &["a", "b"], |src, _, _| {
            src.repeat_n("r", ..=3, |_| Effect::Noop);
        });
        let _: u8 = src.any("after");
    });
}
