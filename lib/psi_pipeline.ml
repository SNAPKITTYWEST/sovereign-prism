(* psi_pipeline.ml — Non-recursive ψ pipeline *)

(** Pipeline stage type *)
type 'a stage = 'a -> 'a

(** Nerve stage — identity transform *)
let nerve carrier = carrier

(** Postnikov tower stage — identity transform *)
let postnikov_tower carrier = carrier

(** Homotopy groups stage — identity transform *)
let homotopy_groups carrier = carrier

(** K-invariants stage — compute hash label *)
let k_invariants carrier =
  let open Carrier in
  let digest = Sha256d.hash_bytes carrier.canonical_bytes in
  { carrier with canonical_bytes = Bytes.of_string ("sha256d:" ^ digest) }

(** Full ψ pipeline *)
let psi_pipeline carrier =
  carrier
  |> nerve
  |> postnikov_tower
  |> homotopy_groups
  |> k_invariants
