module public Program

open System.IO
open Day7.Lib

[<EntryPoint>]
let main args =
    let positions = File.ReadAllText "input.txt" |> parseInput
    
    printfn "Part 1: %i" part1
    printfn "Part 2: %i" part2
    0