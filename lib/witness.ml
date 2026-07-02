(* witness.ml — Witness types and generation *)

(** Witness trace entry *)
type trace_entry = {
  stage : string;
  input_hash : string;
  output_hash : string;
}

(** Complete witness *)
type witness = {
  label : Carrier.hash_label;
  seal : string;
  trace : string list;
  trace_entries : trace_entry list;
  accepted : bool;
  rejection : string option;
}

(** Create a successful witness *)
let create label seal trace =
  { label;
    seal;
    trace;
    trace_entries = [];
    accepted = true;
    rejection = None }

(** Create a rejection witness *)
let create_rejection label reason =
  { label;
    seal = "";
    trace = [];
    trace_entries = [];
    accepted = false;
    rejection = Some reason }

(** Serialize witness to JSON *)
let to_json w =
  let label_str = match w.label with
    | Carrier.Sha256d s | Carrier.SnapSha256d s -> s
  in
  Printf.sprintf {|{"label":"%s","seal":"%s","accepted":%s,"rejection":%s,"trace":[%s]}|}
    label_str
    w.seal
    (if w.accepted then "true" else "false")
    (match w.rejection with Some r -> Printf.sprintf "\"%s\"" r | None -> "null")
    (String.concat "," (List.map (fun s -> Printf.sprintf "\"%s\"" s) w.trace))
