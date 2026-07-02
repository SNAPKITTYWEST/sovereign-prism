(* carrier.ml — Typed artifact carriers *)

(** Hash label type *)
type hash_label =
  | Sha256d of string
  | SnapSha256d of string

(** Admission result *)
type admission =
  | Admit
  | Reject of string

(** Typed carrier wrapping payload with canonical bytes *)
type 'a carrier = {
  payload : 'a;
  canonical_bytes : bytes;
}

(** Create a carrier from payload and canonical bytes *)
let create payload canonical_bytes =
  { payload; canonical_bytes }

(** Get the canonical bytes of a carrier *)
let canonical carrier = carrier.canonical_bytes

(** Get the payload of a carrier *)
let payload carrier = carrier.payload

(** Map over a carrier's payload *)
let map f carrier =
  { payload = f carrier.payload;
    canonical_bytes = carrier.canonical_bytes }
