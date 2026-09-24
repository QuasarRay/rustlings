"""Proof-generation boundaries must survive python -O."""
import hashlib
from pathlib import Path
import unittest
from generate_verus import generate, CONTRACTS

SOURCE = (Path(__file__).resolve().parents[1] / "src/kernel.rs").read_text()


class GenerationBoundary(unittest.TestCase):
    def test_source_identity_and_contract_inventory(self):
        generated = generate(SOURCE)
        self.assertIn(hashlib.sha256(SOURCE.encode()).hexdigest(), generated)
        self.assertEqual(generated.count("    ensures "), len(CONTRACTS))

    def test_unmodeled_or_hidden_items_cannot_escape_coverage(self):
        for extra in ["pub fn unknown(x: bool) -> bool { x }", "fn hidden() {}", "const X: bool = true;",
                      "#[cfg(any())]\npub fn unknown(x: bool) -> bool { x }", "impl Foo {}"]:
            with self.subTest(extra=extra), self.assertRaises(ValueError): generate(SOURCE + "\n" + extra)

    def test_missing_duplicate_and_proof_bypass_constructs_are_rejected(self):
        for source in [SOURCE.replace("pub fn triad", "pub fn unknown"), SOURCE + SOURCE,
                       SOURCE + "\n/* admit(false); */", SOURCE.replace("ops == 2", "assume(ops == 2)"),
                       SOURCE.replace("pub fn triad", "#[verifier::external_body]\npub fn triad")]:
            with self.assertRaises(ValueError): generate(source)


if __name__ == "__main__": unittest.main()
