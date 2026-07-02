(* sha256d.ml — SHA-256d label generation *)

(** Hash bytes using SHA-256 *)
let hash_bytes b =
  let open Digestif.SHA256 in
  let ctx = init () in
  let ctx = feed_bytes ctx b in
  let digest = get ctx in
  Digestif.SHA256.to_hex digest

(** Hash a string using SHA-256 *)
let hash_string s = hash_bytes (Bytes.of_string s)

(** Compute double SHA-256 *)
let hash_double b =
  let first = hash_bytes b in
  hash_string first

(** Format as snapsha256d label *)
let snapsha256d b =
  "snapsha256d:" ^ hash_double b
