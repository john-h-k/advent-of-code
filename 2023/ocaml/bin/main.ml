open Core
module Time = Time_float_unix

let year_and_day () =
  let today = Date.today ~zone:(Time.Zone.local |> Lazy.force) in
  Date.year today, Date.day today
;;

let day_str day =
  let suffix =
    match day % 10 with
    | 1 -> "st"
    | 2 -> "nd"
    | 3 -> "rd"
    | _ -> "th"
  in
  Printf.sprintf "%d%s" day suffix
;;

open Lwt
open Cohttp
open Cohttp_lwt_unix

let get_day_input year day =
  let session =
    match Sys.getenv "AOC_SESSION" with
    | Some s -> s
    | None ->
      failwith "must set `AOC_SESSION` env var to your advent of code session cookie"
  in
  let uri =
    Printf.sprintf "https://adventofcode.com/%d/day/%d/input" year day |> Uri.of_string
  in
  let headers = Header.init_with "Cookie" ("session=" ^ session) in
  Client.get ~headers uri >>= fun (_resp, body) -> body |> Cohttp_lwt.Body.to_string
;;

module type Day = sig
  val part1 : string list -> string
  val part2 : string list -> string
end

let run_day day lines =
  let module Day_To_Run = (val day : Day) in
  let () = Printf.printf "Running part1...\n" in
  let () = lines |> Day_To_Run.part1 |> Printf.printf "Part 1 result: \n%s\n" in
  let () = Printf.printf "\n-------------------------------\n" in
  let () = Printf.printf "Running part2...\n" in
  let () = lines |> Day_To_Run.part2 |> Printf.printf "Part 2 result: \n%s\n" in
  ()
;;

let () =
  let args = Sys.get_argv () in
  let year, day = year_and_day () in
  Printf.printf "Today is the %s of December %d\n\n" (day_str day) year;
  let day =
    if Array.length args > 1
    then (
      let day_arg = Array.last args in
      match int_of_string_opt day_arg with
      | Some d -> d
      | None -> failwith (Printf.sprintf "invalid day '%s'\n" day_arg))
    else day
  in
  Printf.printf "Running day %d...\n" day;
  if day < 0 || day > 25 then failwith "invalid date";
  let lines = Lwt_main.run (get_day_input year day) |> String.split_lines in
  let day_module =
    match day with
    | 1 -> (module Ocaml.Day1 : Day)
    | 2 -> (module Ocaml.Day2 : Day)
    | 3 -> (module Ocaml.Day3 : Day)
    | 4 -> (module Ocaml.Day4 : Day)
    | _ -> failwith (Printf.sprintf "day %d not implemented" day)
  in
  run_day day_module lines
;;
