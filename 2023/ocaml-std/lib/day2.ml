let map_file fn path =
  let f = open_in path in
    let rec loop () =
      try
        let next = input_line f in
        fn(next) :: loop ()
      with End_of_file ->
        close_in f;
        []
    in loop ()
;;

let game_is_legal r g b = r > 12 || g > 13 || b > 14;;

let rec parse_round comps = 
  match List.map String.trim comps with
    [] -> true
    | head :: tail ->
      match String.split_on_char ' ' head with
        | number :: [color] ->
          let max_allowed = match String.trim color with 
            "red" -> 12
            | "green" -> 13
            | "blue" -> 14
            | _ -> failwith "invalid"
          in 
               String.trim number |> int_of_string <= max_allowed && parse_round tail
        | _ -> failwith "invalid"
;;

let parse_rounds comps = String.split_on_char ';' comps
  |> List.fold_left (fun acc round -> acc && (round |> String.split_on_char ',' |> parse_round))
        true
;;

let parse_game_id id = let parts = String.split_on_char ' ' id in
  (List.length parts - 1) |> List.nth parts |> int_of_string
;;

let parse_game line = 
  let parts = String.split_on_char ':' line in
    match parts with
      id :: [rest] -> if parse_rounds rest then Some(parse_game_id id) else None
      | _ -> failwith "invalid"
;;

let rec sum_list_opt = function
  [] -> 0
  | None :: tail -> sum_list_opt tail
  | Some(head) :: tail -> head + (sum_list_opt tail)
;;

let arr = map_file parse_game "test.txt" in
  let sum = sum_list_opt arr in
    print_int sum
;;



let map_file fn path =
  let f = open_in path in
    let rec loop () =
      try
        let next = input_line f in
        fn(next) :: loop ()
      with End_of_file ->
        close_in f;
        []
    in loop ()
;;

let rec parse_round maxs comps =
  match List.map String.trim comps with
    [] -> ();
    | head :: tail ->
      let () = parse_round maxs tail in
        match String.split_on_char ' ' head with
          | number :: [color] ->
            String.trim number |> int_of_string |> max (Hashtbl.find maxs color) |> Hashtbl.replace maxs color;

          | _ -> failwith "invalid";
;;

let parse_rounds comps = let maxs = Hashtbl.create 3 in
    List.iter (fun v -> Hashtbl.add maxs v 0) ["red"; "green"; "blue"];
      let rounds = String.split_on_char ';' comps in
        rounds |> List.iter (fun r -> parse_round maxs (String.split_on_char ',' r));
        maxs
;;

let power maxs = Hashtbl.to_seq_values maxs |> Seq.fold_left (fun acc v -> acc * v) 1

let parse_game_id id = let parts = String.split_on_char ' ' id in
  (List.length parts - 1) |> List.nth parts |> int_of_string
;;

let parse_game line = 
  let parts = String.split_on_char ':' line in
    match parts with
      id :: [rest] ->
        let maxs = parse_rounds rest in
          power maxs
      | _ -> failwith "invalid!"
;;

let rec sum_list = function
  [] -> 0
  | head :: tail -> head + (sum_list tail)
;;

let arr = map_file parse_game "input.txt" in
  let sum = sum_list arr in
    print_int sum
;;
