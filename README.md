# snap-prism-ocaml

SnapKitty Prism Compiler — Non-recursive artifact inference.

## Usage

```bash
# Infer witness from artifact
snap-prism infer artifact.json --target ffffffffffff...

# Example
echo '{"prime": 42}' > test.json
snap-prism infer test.json
```

## Algorithm

```
Carrier → ψ-pipeline → hash label → admission seal → WORM witness
```

## Rules

- No recursive algorithm path
- No mining language
- One structural inference per carrier
- Rejected artifacts return typed rejection receipts
- Accepted artifacts return witness + seal

## License

Sovereign Source License — see SOVEREIGN.md
