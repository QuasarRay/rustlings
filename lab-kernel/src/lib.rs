#![forbid(unsafe_code)]
//! Small shared decision kernel. OS, parsers, observers and specifications remain
//! outside its formal proof boundary. The production runner calls these functions.
// Keep comparison primitives within the shared Rust/Verus supported subset.
#[allow(clippy::manual_range_contains)]
mod kernel;
pub use kernel::*;

#[cfg(kani)]
mod proofs {
    use super::*;
    #[kani::proof]
    fn reservations_are_atomic_exact_and_never_overcommit() {
        let used: u64 = kani::any();
        let requested: u64 = kani::any();
        let capacity: u64 = kani::any();
        let next = reserve(used, requested, capacity);
        assert!(next >= used);
        if next != used {
            assert!(requested > 0 && next <= capacity);
            assert_eq!(next - used, requested);
        } else {
            assert!(requested == 0 || used > capacity || requested > capacity - used);
        }
        if used <= capacity {
            assert!(next <= capacity);
        }
    }
    #[kani::proof]
    fn no_credit_without_all_domains_and_coupling() {
        let o: u8 = kani::any();
        let n: u8 = kani::any();
        let i: u8 = kani::any();
        let c: bool = kani::any();
        let p: bool = kani::any();
        let f: bool = kani::any();
        let pass = triad(o, n, i, c, p, f);
        assert_eq!(pass, o == 2 && n == 2 && i == 2 && c && p && f);
        if o != 2 || n != 2 || i != 2 {
            assert!(!pass);
        }
    }
    #[kani::proof]
    fn wrong_execution_tier_cannot_pass() {
        let required: u8 = kani::any();
        let observed: u8 = kani::any();
        let accepted = tier_matches(required, observed);
        assert_eq!(
            accepted,
            required >= 1 && required <= 4 && observed == required
        );
    }
    #[kani::proof]
    fn replay_and_wrong_exercise_cannot_pass() {
        let attempt: u64 = kani::any();
        let receipt: u64 = kani::any();
        let exercise: u16 = kani::any();
        let observed: u16 = kani::any();
        if current_receipt(attempt, receipt, exercise, observed) {
            assert!(attempt != 0 && attempt == receipt && exercise == observed);
        }
    }
    #[kani::proof]
    fn no_overcommit_or_zero_admission() {
        let request: u64 = kani::any();
        let allocation: u64 = kani::any();
        let physical: u64 = kani::any();
        if admitted(request, allocation, physical) {
            assert!(request > 0 && request <= allocation && allocation <= physical);
        }
    }
    #[kani::proof]
    fn every_required_facet_is_present() {
        let required: u64 = kani::any();
        let observed: u64 = kani::any();
        let bit: u8 = kani::any();
        kani::assume(bit < 64);
        if facets_complete(required, observed) && (required & (1u64 << bit)) != 0 {
            assert!((observed & (1u64 << bit)) != 0);
        }
        assert!(!facets_complete(0, observed));
    }
    #[kani::proof]
    fn progress_is_monotonic_bounded_and_ordered() {
        let current: u16 = kani::any();
        let exercise: u16 = kani::any();
        let total: u16 = kani::any();
        let passed: bool = kani::any();
        kani::assume(current <= total);
        let next = advance(current, exercise, total, passed);
        assert!(current <= next && next <= total);
        if next != current {
            assert!(passed && exercise == current && next == current + 1);
        }
        if !passed || exercise != current {
            assert_eq!(next, current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_status_combinations_require_three_passes() {
        for o in 0..=3 {
            for n in 0..=3 {
                for i in 0..=3 {
                    assert_eq!(triad(o, n, i, true, true, true), o == 2 && n == 2 && i == 2);
                }
            }
        }
    }
    #[test]
    fn terminal_progress_does_not_overflow() {
        assert_eq!(advance(u16::MAX, u16::MAX, u16::MAX, true), u16::MAX);
        assert_eq!(advance(4, 8, 10, true), 4);
    }
}
