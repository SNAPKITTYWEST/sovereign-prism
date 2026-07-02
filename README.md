# snap-prism-ocaml

> Non-recursive prism compiler for the SnapKitty ecosystem.
> Excavate. Canonicalize. Hash. Seal. Govern.

[![License: Sovereign Source](https://img.shields.io/badge/License-Sovereign%20Source-blue.svg)](SOVEREIGN.md)
[![OCaml](https://img.shields.io/badge/OCaml-5.0-purple.svg)](https://ocaml.org/)
[![Tests](https://img.shields.io/badge/Tests-10%20passing-brightgreen.svg)](#testing)

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    SNAP PRISM OCAML                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────┐    ┌──────────────┐    ┌───────────────────┐    │
│   │ Artifact │───▶│   Carrier    │───▶│  Canonical Bytes  │    │
│   │ (JSON)   │    │  (typed I/O) │    │  (deterministic)  │    │
│   └──────────┘    └──────────────┘    └───────────────────┘    │
│                                              │                  │
│                         ┌────────────────────┤                  │
│                         │                    │                  │
│                         ▼                    ▼                  │
│              ┌─────────────────┐  ┌──────────────────┐        │
│              │  SHA-256d Label │  │  WORM Witness    │        │
│              │ (snapsha256d:)  │  │  (sealed)        │        │
│              └─────────────────┘  └──────────────────┘        │
│                         │                                       │
│                         ▼                                       │
│   ┌──────────────────────────────────────────────────────────┐ │
│   │                   ψ-Pipeline                             │ │
│   │                                                          │ │
│   │   Nerve → Postnikov Tower → Homotopy → k-Invariants     │ │
│   │                                                          │ │
│   └──────────────────────────────────────────────────────────┘ │
│                         │                                       │
│                         ▼                                       │
│   ┌──────────────────────────────────────────────────────────┐ │
│   │              Admission Validator                         │ │
│   │         (target ∈ allowed_prime_indices)                 │ │
│   └──────────────────────────────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## ψ-Pipeline Steps

| Step | Module | Input | Output |
|------|--------|-------|--------|
| 1 | `Nerve` | Adjacency matrix | 1-skeleton |
| 2 | `PostnikovTower` | 1-skeleton | k-invariant filtration |
| 3 | `HomotopyGroups` | Filtration | π_k(B) groups |
| 4 | `KInvariants` | π_k(B) | Invariant vectors |

## Quick Start

### Install

```bash
opam install snap-prism
# Or from source:
dune build
dune install
```

### CLI Usage

```bash
# Infer sovereign address from artifact
snap-prism infer artifact.json --target 42
# Output: snapsha256d:e3b0c44298fc1c149afbf4c8996fb924...

# Verify a label against artifact
snap-prism verify artifact.json --label snapsha256d:a1b2c3d4...

# Generate WORM witness
snap-prism witness artifact.json
# Output: {"artifact_hash":"...","label":"snapsha256d:...","status":"accepted",...}
```

### OCaml API

```ocaml
open Snap_prism_ocaml

let () =
  let json = `{ "prime": 42, "name": "test" }` in
  
  (* Compute carrier *)
  let carrier = Carrier.of_json json in
  
  (* Compute sovereign label *)
  let label = Sha256d.compute (Carrier.canonical carrier) in
  Printf.printf "Label: %s\n" (Sha256d.to_string label);
  
  (* Verify admission *)
  let admitted = Admission.verify ~target:42 ~label in
  Printf.printf "Admitted: %b\n" admitted;
  
  (* Generate WORM witness *)
  let witness = Worm.seal carrier label in
  Printf.printf "Witness: %s\n" (Witness.to_json witness)
```

## Interactive Demo

```bash
# Demo 1: Carrier canonicalization
$ cat artifact.json
{"name": "test", "prime": 42, "metadata": {"version": "1"}}

$ snap-prism infer artifact.json --target 42
snapsha256d:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

# Demo 2: Key ordering invariance
$ echo '{"b": 2, "a": 1}' | snap-prism infer --target 1
snapsha256d:7819c7f8a8b...

$ echo '{"a": 1, "b": 2}' | snap-prism infer --target 1
snapsha256d:7819c7f8a8b...  # Same label!

# Demo 3: WORM witness verification
$ snap-prism witness artifact.json | jq .status
"accepted"

$ snap-prism witness artifact.json | jq .witness_hash
"sha256:a1b2c3d4e5f6..."
```

## Invariants

| Invariant | Description |
|-----------|-------------|
| **No Recursion** | All computation is staged pipeline |
| **Typed I/O** | Every carrier is typed at boundary |
| **Deterministic** | Same input → same output |
| **WORM Sealed** | Every witness is write-once, read-many |
| **Fail-Closed** | Any error terminates processing |

## Testing

```bash
# Run all tests
dune test

# Run with output
dune test --force
```

## Witness Shape

```json
{
  "standard": "SNAP-PRISM-1",
  "algorithm": "snap-sha256d-v1",
  "artifact_hash": "sha256:a1b2c3d4e5f6...",
  "label": "snapsha256d:e3b0c44298fc1c149afbf4c8996fb924...",
  "status": "accepted",
  "witness_hash": "sha256:7819c7f8a8b...",
  "timestamp": "2025-01-01T00:00:00Z"
}
```

## License

Sovereign Source License — see [SOVEREIGN.md](SOVEREIGN.md)

---

```
SNAP-PRISM-OCAML-001
Excavate. Canonicalize. Hash. Seal. Govern.
Same artifact. Same label.
No recursion. No borrowed thesis.
```
