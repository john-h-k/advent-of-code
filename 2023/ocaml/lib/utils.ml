open Core

let option_to_int = function
  | None -> 0
  | Some s -> s
;;

let remove_nones l = List.map ~f:option_to_int l

let rec sum_list = function
  | [] -> 0
  | head :: tail -> head + sum_list tail
;;

let rec sum_list_opt = function
  | [] -> 0
  | head :: tail ->
    (match head with
     | Some x -> x
     | None -> 0)
    + sum_list_opt tail
;;
