"""Insert contracts into the exact executable bodies; never maintain a proof copy.

No body is rewritten. The generated file includes its input SHA-256; CI regenerates
it for each verification. This transformation and the toolchain are trusted.
"""
import hashlib
from pathlib import Path
import re
import sys

root=Path(__file__).resolve().parents[1]
source=(root/'src/kernel.rs').read_text()
contracts={
 'triad':'result == (ops == 2 && net == 2 && infra == 2 && coupled && provenance && facets)',
 'tier_matches':'result == (1 <= required && required <= 4 && observed == required)',
 'current_receipt':'result == (attempt != 0 && attempt == receipt && exercise == observed)',
 'admitted':'result == (0 < requested && requested <= allocated && allocated <= physical)',
 'facets_complete':'result == (required != 0 && (required & observed) == required)',
 'advance':'result as int == (if passed && exercise == current && current < total { current as int + 1 } else { current as int }), (current <= total ==> current <= result && result <= total)',
}
assert set(re.findall(r'pub fn (\w+)\(',source))==set(contracts), 'every exported function requires a contract'
for name,contract in contracts.items():
    pattern=rf'(pub fn {name}\([^{{]+?\)) -> (bool|u16) \{{'
    source,n=re.subn(pattern,lambda m:m[1]+f' -> (result: {m[2]})\n    ensures {contract}\n{{',source,count=1)
    assert n==1, f'cannot insert contract for {name}'
result='// Input SHA256: '+hashlib.sha256((root/'src/kernel.rs').read_bytes()).hexdigest()+'\nuse vstd::prelude::*;\nverus! {\n'+source+'\n}\nfn main() {}\n'
Path(sys.argv[1]).write_text(result)
