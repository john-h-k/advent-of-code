open Core

let discrim a b c = (b *. b) -. (4.0 *. a *. c)

let quad_solve a b c =
  let d = discrim a b c in
  if Float.compare d 0.0 < 0
  then None
  else (
    let root_d = Float.sqrt d in
    Some ((-.b +. root_d) /. (2.0 *. a), (-.b -. root_d) /. (2.0 *. a)))
;;

let float_is_int f =
  let parts = Float.modf f in
  Float.equal (Float.Parts.fractional parts) 0.0
;;

let possible_win_count time distance =
  match quad_solve (-1.0) (float_of_int time) (-.float_of_int distance) with
  | None -> 0
  | Some (l, r) ->
    let l, r = Float.min l r, Float.max l r in
    let ans = Float.round_down r -. Float.round_up l |> int_of_float in
    if float_is_int l && float_is_int r then ans - 1 else ans + 1
;;

let split_last_exn on value =
  let split = String.split value ~on in
  List.nth_exn split (List.length split - 1)
;;

let part1 lines =
  let lines =
    List.map lines ~f:(fun l ->
      l
      |> split_last_exn ':'
      |> String.split ~on:' '
      |> List.filter_map ~f:(fun s -> s |> int_of_string_opt))
  in
  match lines with
  | times :: [ distances ] ->
    List.zip_exn times distances
    |> List.map ~f:(fun (time, distance) -> possible_win_count time distance)
    |> List.fold_left ~init:1 ~f:( * )
    |> Printf.sprintf "%d\n"
  | _ -> failwith "invalid input"
;;

let part2 lines =
  let lines =
    List.map lines ~f:(fun l ->
      l
      |> split_last_exn ':'
      |> String.filter ~f:(fun c -> c |> Char.is_whitespace |> not)
      |> int_of_string)
  in
  match lines with
  | time :: [ distance ] -> possible_win_count time distance |> Printf.sprintf "%d\n"
  | _ -> failwith "invalid input"
;;
