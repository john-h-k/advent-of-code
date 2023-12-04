open Core

let game_is_legal r g b = r > 12 || g > 13 || b > 14;;

let rec parse_round_legal comps = 
  match List.map ~f:String.strip comps with
    [] -> true
    | head :: tail ->
      match String.split_on_chars head ~on:[' '] with
        | number :: [color] ->
          let max_allowed = match String.strip color with 
            "red" -> 12
            | "green" -> 13
            | "blue" -> 14
            | _ -> failwith "invalid"
          in 
               String.strip number |> int_of_string <= max_allowed && parse_round_legal tail
        | _ -> failwith "invalid"
;;

let parse_rounds_legal comps = String.split_on_chars comps ~on:[';']
  |> List.fold_left ~f:(fun acc round -> acc && (round |> String.split_on_chars ~on:[','] |> parse_round_legal)) ~init:true
;;

let parse_game_id id = let parts = String.split_on_chars id ~on:[' '] in
  (List.length parts - 1) |> List.nth_exn parts |> int_of_string
;;

let parse_game_legal line = 
  let parts = String.split_on_chars line ~on:[':'] in
    match parts with
      id :: [rest] -> if parse_rounds_legal rest then Some(parse_game_id id) else None
      | _ -> failwith "invalid"
;;

let rec parse_round_max maxs comps =
  match List.map ~f:String.strip comps with
    [] -> ();
    | head :: tail ->
      let () = parse_round_max maxs tail in
        match String.split_on_chars head ~on:[' '] with
          | number :: [color] ->
            let new_max = String.strip number |> int_of_string |> max (match Hashtbl.find maxs color with Some x -> x | None -> 0) in
              Hashtbl.update maxs color ~f:(fun _o -> new_max);

          | _ -> failwith "invalid";
;;

let parse_rounds_max comps = let maxs = Hashtbl.create (module String) in
     ["red"; "green"; "blue"] |> List.iter ~f:(fun v -> let _ = Hashtbl.add maxs ~key:v ~data:0 in ());
      let rounds = String.split_on_chars comps ~on:[';'] in
        rounds |> List.iter ~f:(fun r -> parse_round_max maxs (String.split_on_chars r ~on:[',']));
        maxs
;;

let power maxs = Hashtbl.fold maxs ~f:(fun ~key:_key ~data acc -> acc * data) ~init:1

let parse_game_max line = 
  let parts = String.split_on_chars line ~on:[':'] in
    match parts with
      _id :: [rest] ->
        let maxs = parse_rounds_max rest in
          power maxs
      | _ -> failwith "invalid!"
;;

open Utils

let part1 lines =
  let arr = List.map ~f:parse_game_legal lines in
    let sum = sum_list_opt arr in
      Printf.sprintf "%d\n" sum

let part2 lines =
  let arr = List.map ~f:parse_game_max lines in
    let sum = sum_list arr in
      Printf.sprintf "%d\n" sum
