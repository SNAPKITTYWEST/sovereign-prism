# SOVEREIGN.md — snap-prism-ocaml Covenant

## SnapKitty Prism Compiler

**SNAP-PRISM-OCAML-001**

This OCaml project implements non-recursive artifact inference for the SnapKitty ecosystem.

## Core Thesis

```
Carrier enters.
ψ-pipeline folds.
κ-label emits.
Admission seals.
No recursion. No borrowed flag.
```

## Algorithm

1. Read artifact JSON
2. Create typed carrier with canonical bytes
3. Run ψ-pipeline: nerve → postnikov_tower → homotopy_groups → k_invariants
4. Compute SHA-256d label
5. Check admission predicate
6. Generate WORM witness
7. Return witness + seal

## Invariants

1. **No recursive algorithm path**
2. **No mining language**
3. **One structural inference per carrier**
4. **Rejected artifacts return typed rejection receipts**
5. **Accepted artifacts return witness + seal**

## Pipeline Stages

| Stage | Description |
|-------|-------------|
| `nerve` | Identity transform |
| `postnikov_tower` | Identity transform |
| `homotopy_groups` | Identity transform |
| `k_invariants` | Compute hash label |

## Seal

```
SNAP-PRISM-OCAML-001
Carrier enters.
ψ-pipeline folds.
κ-label emits.
Admission seals.
No recursion. No borrowed flag.
```
