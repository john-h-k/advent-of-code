
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

let parse_nums line = line |> String.split_on_char ' ' |> List.filter (fun s -> String.length s > 0) |>  List.map int_of_string;;

let intersection l r = l |> List.filter (fun s -> r |> List.mem s);;

let parse_game line = 
  let parts = line |> String.split_on_char ':' |> List.tl |> List.hd |> String.split_on_char '|' in
      match parts with
        winners :: [numbers] -> 
        let winners = parse_nums winners in let numbers = parse_nums numbers in
          let num_winners = intersection winners numbers |> List.length in 
            if num_winners == 0 then 0 else Int.shift_left 1 ((num_winners) - 1)
        | _ -> failwith "invalid"
;;

let rec sum_list = function
  [] -> 0
  | head :: tail -> head + (sum_list tail)
;;

let arr = map_file parse_game "input.txt" in
  let () = List.iter (fun s -> Printf.printf "%d\n" s) arr in
  let sum = sum_list arr in
    print_int sum
;;


