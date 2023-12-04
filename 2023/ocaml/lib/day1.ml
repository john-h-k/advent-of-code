open Core

let reverse str =
  let rec loop idx =
    match idx with
    | 0 -> String.make 1 str.[0]
    | _ -> String.make 1 str.[idx] ^ loop (idx - 1)
  in
  loop (String.length str - 1)
;;

let digit_word str idx =
  let words =
    [ "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine" ]
  in
  let str = String.sub str ~pos:idx ~len:(String.length str - idx) in
  let opt_word i word =
    if String.is_substring_at str ~pos:0 ~substring:word then Some (i + 1) else None
  in
  let matches = List.filter_map ~f:Fun.id (List.mapi ~f:opt_word words) in
  match matches with
  | [] -> None
  | item :: _rest -> Some item
;;

let first_digit_or_word str =
  let rec loop idx =
    if idx < String.length str
    then (
      let c = str.[idx] in
      match c with
      | '0' .. '9' -> Some (int_of_string (String.make 1 c))
      | _ ->
        (match digit_word str idx with
         | Some s -> Some s
         | None -> loop (idx + 1)))
    else None
  in
  loop 0
;;

let last_digit_or_word str =
  let rec loop idx =
    if idx >= 0
    then (
      let c = str.[idx] in
      match c with
      | '0' .. '9' -> Some (int_of_string (String.make 1 c))
      | _ ->
        (match digit_word str idx with
         | Some s -> Some s
         | None -> loop (idx - 1)))
    else None
  in
  loop (String.length str - 1)
;;

let first_digit str =
  let rec loop idx =
    if idx < String.length str
    then (
      let c = str.[idx] in
      match c with
      | '0' .. '9' -> Some (int_of_string (String.make 1 c))
      | _ -> loop (idx + 1))
    else None
  in
  loop 0
;;

let first_and_last_digit str =
  let first = first_digit str in
  let second = first_digit (reverse str) in
  match first, second with
  | Some first, Some second -> Some ((first * 10) + second)
  | _ -> None
;;

let first_and_last_digit_or_word str =
  let first = first_digit_or_word str in
  let second = last_digit_or_word str in
  match first, second with
  | Some first, Some second -> Some ((first * 10) + second)
  | _ -> None
;;

open Utils

let part1 lines =
  let arr = List.map ~f:first_and_last_digit lines in
  let sum = sum_list (remove_nones arr) in
  Printf.sprintf "%d\n" sum
;;

let part2 lines =
  let arr = List.map ~f:first_and_last_digit_or_word lines in
  let sum = sum_list (remove_nones arr) in
  Printf.sprintf "%d\n" sum
;;
