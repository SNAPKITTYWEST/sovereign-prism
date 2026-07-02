(* worm.ml — WORM witness seal *)

(** WORM seal *)
type seal = {
  label : Carrier.hash_label;
  seal_hash : string;
  timestamp : float;
  content_hash : string;
}

(** Seal a label *)
let seal label =
  let label_str = match label with
    | Carrier.Sha256d s | Carrier.SnapSha256d s -> s
  in
  let content = Printf.sprintf "WORM:%s:%f" label_str (Unix.gettimeofday ()) in
  let seal_hash = Sha256d.hash_string content in
  { label;
    seal_hash;
    timestamp = Unix.gettimeofday ();
    content_hash = Sha256d.hash_string label_str }

(** Verify a seal *)
let verify seal =
  let label_str = match seal.label with
    | Carrier.Sha256d s | Carrier.SnapSha256d s -> s
  in
  let expected = Printf.sprintf "WORM:%s:%f" label_str seal.timestamp in
  let expected_hash = Sha256d.hash_string expected in
  seal.seal_hash = expected_hash
