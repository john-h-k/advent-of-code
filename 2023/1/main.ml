
let reverse str = 
  let rec loop idx = match idx with
    0 -> String.make 1 str.[0]
    | _ -> String.make 1 str.[idx] ^ loop (idx - 1)
  in loop (String.length str - 1)
;;

let digit_word str idx =
  let words = ["one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine"] in
  let str = String.sub str idx (String.length str - idx) in
    let _ = Printf.printf "%s\n" str in
    let opt_word i word = 
      if String.starts_with ~prefix:word str then
        Some (i + 1)
      else
        None
    in
  
    let matches = List.filter_map Fun.id (List.mapi opt_word words) in
      match matches with
        [] -> None
        | item :: rest -> Some item
;;

let first_digit_or_word str =
  let rec loop idx =
    if idx < String.length str then
      let c = str.[idx] in
        match c with
          '0' .. '9' -> Some (int_of_string (String.make 1 c))
          | _ -> match digit_word str idx with
            Some s -> Some s
            | None -> loop (idx + 1)
    else
      None

  in loop 0
;;

let last_digit_or_word str =
  let rec loop idx =
    if idx >= 0 then
      let c = str.[idx] in
        match c with
          '0' .. '9' -> Some (int_of_string (String.make 1 c))
          | _ -> match digit_word str idx with
            Some s -> Some s
            | None -> loop (idx - 1)
    else
      None

  in loop ((String.length str) - 1)
;;
  
  
let first_digit str =
  let rec loop idx =
    if idx < String.length str then
      let c = str.[idx] in
        match c with
          '0' .. '9' -> Some (c, idx)
          | _ -> loop (idx + 1)
    else
      None

  in loop 0
;;


let first_and_last_digit str =
  let first = first_digit_or_word str in
    let second = last_digit_or_word str in
      match first, second with
        Some first, Some second -> Some (first * 10 + second)
        | _ -> None
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

let option_to_int = function
  | None -> 0
  | Some s -> s
;;

let remove_nones l = List.map option_to_int l;;

let rec sum_list = function
  [] -> 0
  | head :: tail -> head + (sum_list tail)
;;

let arr = map_file first_and_last_digit "input.txt" in
  let sum = sum_list (remove_nones arr) in
    print_int sum
;;
