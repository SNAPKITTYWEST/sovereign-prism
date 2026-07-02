(* snap_prism_cli.ml — CLI for snap-prism *)

open Snap_prism

let infer artifact_path target_hex =
  (* Read artifact *)
  let ic = open_in artifact_path in
  let content = really_input_string ic (in_channel_length ic) in
  close_in ic;

  (* Parse JSON *)
  let json = Yojson.Safe.from_string content in

  (* Create carrier *)
  let canonical = Canonical.canonical_bytes json in
  let carrier = Carrier.create json canonical in

  (* Run ψ pipeline *)
  let processed = Psi_pipeline.psi_pipeline carrier in

  (* Check admission *)
  let target = Admission.of_hex target_hex in
  let label = match Bytes.to_string processed.Carrier.canonical_bytes with
    | s when String.length s > 10 ->
      if String.sub s 0 10 = "sha256d:" then
        Carrier.SnapSha256d (String.sub s 10 (String.length s - 10))
      else
        Carrier.SnapSha256d (Sha256d.hash_bytes processed.Carrier.canonical_bytes)
    | _ ->
      Carrier.SnapSha256d (Sha256d.hash_bytes processed.Carrier.canonical_bytes)
  in

  if Admission.admits label target then
    let seal = Worm.seal label in
    let witness = Witness.create label seal.Worm.seal_hash
      ["nerve"; "postnikov_tower"; "homotopy_groups"; "k_invariants"] in
    Printf.printf "%s\n" (Witness.to_json witness)
  else
    let witness = Witness.create_rejection label "target_admission_failed" in
    Printf.printf "%s\n" (Witness.to_json witness)

let () =
  let open Cmdliner in
  let infer_cmd =
    let artifact =
      let doc = "Path to artifact JSON file." in
      Arg.(required & pos 0 (some string) None & info [] ~docv:"ARTIFACT" ~doc)
    in
    let target =
      let doc = "Target hex threshold for admission." in
      Arg.(value & opt string "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" & info ["target"] ~docv:"TARGET" ~doc)
    in
    let info = Cmd.info "infer" ~doc:"Infer witness from artifact." in
    Cmd.v info Term.(const infer $ artifact $ target)
  in
  let info = Cmd.info "snap-prism" ~doc:"SnapKitty Prism Compiler." in
  let cmd = Cmd.group info [infer_cmd] in
  exit (Cmd.eval cmd)
