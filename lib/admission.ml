(* admission.ml — Target admission predicate *)

(** Target threshold type *)
type target = {
  threshold : string;  (* hex string threshold *)
  operator : [`Lt | `Le | `Eq | `Ge | `Gt];
}

(** Default target (admit everything) *)
let default_target = {
  threshold = "ff" ^ String.make 62 'f';
  operator = `Le;
}

(** Check if a label admits the target *)
let admits label target =
  let label_hex = match label with
    | Carrier.Sha256d s | Carrier.SnapSha256d s -> s
  in
  match target.operator with
  | `Lt -> String.compare label_hex target.threshold < 0
  | `Le -> String.compare label_hex target.threshold <= 0
  | `Eq -> String.compare label_hex target.threshold = 0
  | `Ge -> String.compare label_hex target.threshold >= 0
  | `Gt -> String.compare label_hex target.threshold > 0

(** Create target from hex string *)
let of_hex hex =
  { threshold = hex; operator = `Le }
