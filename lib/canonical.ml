(* canonical.ml — Canonical byte serialization *)

(** Canonicalize a string to NFC-normalized form *)
let canonicalize_string s =
  (* Minimal: just return the string as-is *)
  (* Production would use Uunf for NFC normalization *)
  s

(** Canonicalize a JSON-like value to deterministic bytes *)
let canonicalize_value value =
  (* Minimal: serialize to canonical JSON *)
  (* Production would use a canonical JSON serializer *)
  match value with
  | `String s -> canonicalize_string s
  | `Int i -> string_of_int i
  | `Float f -> string_of_float f
  | `Bool b -> if b then "true" else "false"
  | `Null -> "null"
  | `Assoc kvs ->
    let sorted = List.sort (fun (a, _) (b, _) -> String.compare a b) kvs in
    let inner = String.concat "," (List.map (fun (k, v) ->
      Printf.sprintf "\"%s\":%s" k (canonicalize_value v)) sorted) in
    "{" ^ inner ^ "}"
  | `List vs ->
    let inner = String.concat "," (List.map canonicalize_value vs) in
    "[" ^ inner ^ "]"

(** Convert canonical string to bytes *)
let to_bytes s = Bytes.of_string s

(** Canonicalize a value and return as bytes *)
let canonical_bytes value =
  let s = canonicalize_value value in
  to_bytes s
