/// Status codes: pending=0, fail=1, pass=2, blocked=3. Unknown codes fail closed.
pub fn triad(ops: u8, net: u8, infra: u8, coupled: bool, provenance: bool, facets: bool) -> bool {
    ops == 2 && net == 2 && infra == 2 && coupled && provenance && facets
}

/// Execution classes are not a hierarchy: hardware cannot impersonate an API test.
pub fn tier_matches(required: u8, observed: u8) -> bool {
    required >= 1 && required <= 4 && required == observed
}

/// A receipt must belong to this nonzero attempt and this exercise.
pub fn current_receipt(attempt: u64, receipt: u64, exercise: u16, observed: u16) -> bool {
    attempt != 0 && attempt == receipt && exercise == observed
}

/// Admission is valid only when both scheduler and physical bounds hold.
pub fn admitted(requested: u64, allocated: u64, physical: u64) -> bool {
    requested > 0 && requested <= allocated && allocated <= physical
}

/// Each required bit represents a separately observed facet, maximum 64 per unit.
pub fn facets_complete(required: u64, observed: u64) -> bool {
    required != 0 && (required & observed) == required
}

/// Award only the next sequential exercise; reruns and out-of-order successes do not advance.
pub fn advance(current: u16, exercise: u16, total: u16, passed: bool) -> u16 {
    if passed && exercise == current && current < total {
        current + 1
    } else {
        current
    }
}
