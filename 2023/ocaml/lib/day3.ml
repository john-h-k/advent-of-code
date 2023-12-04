open Core

let moves = let d = [-1; 0; 1] in List.cartesian_product d d |> List.filter ~f:(fun (l, r) -> ((Int.equal l 0) && (Int.equal r 0)) |> not) ;;

let is_symbol c = (Char.equal c '.' || Char.is_digit c) |> not;;

let extract_part_value x line =
  let text = String.slice line x 0 |> String.take_while ~f:Char.is_digit in
    text |> int_of_string_opt |> Option.map ~f:(fun value -> (value, String.length text))
;;

let extract_parts y line lines = 
  line |> String.to_list |> List.filter_mapi ~f:(fun x _c -> 
    if x > 0 && line.[x - 1] |> Char.is_digit then
      None
    else
    match extract_part_value x line with
      Some (value, len) -> let range = List.range x (x + len) in
        let symbols = List.map range ~f:(fun x -> List.filter moves ~f:(fun (dx, dy) -> 
            (List.nth lines (y + dy))
            |> Option.map ~f:String.to_list
            |> Option.bind ~f:((Fun.flip List.nth) (x + dx))
            |> Option.bind ~f:(fun c -> if is_symbol c then Some c else None)
            |> Option.is_some
        )) |> List.concat in
        if (List.is_empty symbols) then None else Some value
      | None -> None)
;;

open Utils

let part1 lines = List.mapi lines ~f:(fun y line -> extract_parts y line lines |> sum_list) |> sum_list |> Printf.sprintf "%d\n";;

let part2 _lines = failwith "todo";;
