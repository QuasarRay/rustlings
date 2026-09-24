"""Insert contracts into exact executable bodies, rejecting unmodeled source.

This parser is intentionally limited to the existing pure scalar kernel. Any new
Rust item or construct needs an explicit compiler/specification review. Validation
uses exceptions, so Python optimization cannot disable the acceptance boundary.
The transformation, specifications and verification tools remain trusted.
"""
import hashlib
from pathlib import Path
import re
import sys

CONTRACTS={
 'triad':'result == (ops == 2 && net == 2 && infra == 2 && coupled && provenance && facets)',
 'tier_matches':'result == (1 <= required && required <= 4 && observed == required)',
 'current_receipt':'result == (attempt != 0 && attempt == receipt && exercise == observed)',
 'admitted':'result == (0 < requested && requested <= allocated && allocated <= physical)',
 'facets_complete':'result == (required != 0 && (required & observed) == required)',
 'advance':'result as int == (if passed && exercise == current && current < total { current as int + 1 } else { current as int }), (current <= total ==> current <= result && result <= total)',
 'reserve':'result as int == (if requested > 0 && used <= capacity && requested as int <= capacity as int - used as int { used as int + requested as int } else { used as int }), result >= used, (used <= capacity ==> result <= capacity), (result > used ==> result as int - used as int == requested as int)',
}

def generate(source):
    clean = re.sub(r"//[^\n]*", "", source)
    if "/*" in clean or "*/" in clean or re.search(r"\b(?:unsafe|assume|admit|extern|external_body|macro_rules|cfg|loop|while|for|requires|ensures)\b", clean):
        raise ValueError("unsupported or proof-bypassing kernel construct")
    signature = re.compile(r"\s*pub fn (\w+)\(([^)]*)\) -> (bool|u16|u64) \{")
    position = 0
    names = []
    while clean[position:].strip():
        match = signature.match(clean, position)
        if not match:
            raise ValueError("every source item must be a modeled public scalar function")
        name, parameters, _ = match.groups()
        if name in names or name not in CONTRACTS:
            raise ValueError("duplicate or unmodeled function: " + name)
        params = parameters.split(",")
        if not params or any(not re.fullmatch(r"\s*[a-z][a-z0-9_]*: (?:bool|u8|u16|u64)\s*", p) for p in params):
            raise ValueError("unsupported scalar signature: " + name)
        names.append(name)
        position = match.end()
        start, depth = position, 1
        while depth and position < len(clean):
            depth += (clean[position] == "{") - (clean[position] == "}")
            position += 1
        if depth:
            raise ValueError("unbalanced executable body")
        body = clean[start:position-1]
        if re.search(r"[;!#\[\]\\\"]|\b[a-zA-Z_]\w*\s*\(", body.replace("!=", "<ne>")):
            raise ValueError("unmodeled statement, call or attribute in " + name)
    if set(names) != set(CONTRACTS):
        raise ValueError("every contract must bind exactly one exported function")
    transformed = source
    for name, contract in CONTRACTS.items():
        pattern = rf"(?m)^(pub fn {name}\([^{{]+?\)) -> (bool|u16|u64) \{{"
        transformed, count = re.subn(pattern, lambda m: m[1] + f" -> (result: {m[2]})\n    ensures {contract}\n{{", transformed)
        if count != 1:
            raise ValueError("cannot uniquely insert contract for " + name)
    return ("// Input SHA256: " + hashlib.sha256(source.encode()).hexdigest()
            + "\nuse vstd::prelude::*;\nverus! {\n" + transformed + "\n}\nfn main() {}\n")


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: generate_verus.py OUTPUT")
    root = Path(__file__).resolve().parents[1]
    result = generate((root / "src/kernel.rs").read_text())
    Path(sys.argv[1]).write_text(result)


if __name__ == "__main__": main()
