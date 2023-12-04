open Core

let parse_numbers str =
  String.split str ~on:' '
  |> List.filter ~f:(fun s -> s |> String.is_empty |> not)
  |> List.map ~f:int_of_string
;;

let intersection l r = List.filter l ~f:(fun e -> List.mem r e ~equal:( = ))

let wins line =
  match String.split line ~on:':' with
  | _ :: [ rest ] ->
    (match String.split rest ~on:'|' with
     | winners :: [ numbers ] ->
       let winners = parse_numbers winners in
       let numbers = parse_numbers numbers in
       let win_count = intersection winners numbers |> List.length in
       if Int.equal win_count 0 then 0 else Int.shift_left 1 (win_count - 1)
     | _ -> failwith "invalid")
  | _ -> failwith "invalid"
;;

let part1 lines =
  List.map lines ~f:wins |> List.fold ~f:( + ) ~init:0 |> Printf.sprintf "%d\n"
;;

let part2 lines =
  List.map lines ~f:wins |> List.fold ~f:( + ) ~init:0 |> Printf.sprintf "TODO NOT DONE %d\n"
;;
