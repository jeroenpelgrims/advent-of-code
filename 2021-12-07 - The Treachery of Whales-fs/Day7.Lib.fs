module Day7.Lib

open System
open System.IO
open FSharp.Stats

let parseInput (s: string) =
    s.Split ','
    |> (Seq.map Int32.Parse)

let fuel1 (from: int) (to': int) =
    Math.Abs (from - to')

let fuel2 (from: int) (to': int) =
    let diff = Math.Abs(from - to')
    diff * (diff + 1) / 2

let cheapestPosition1 positions =
    Seq.median positions

let cheapestPosition2 positions (round: float -> float) =
    Seq.meanBy float positions
    |> round
    |> Convert.ToInt32

let totalFuel (positions: seq<int>) fuel (targetPosition: int) : int =
    Seq.map (fuel targetPosition) positions
    |> Seq.sum

let part1 =
    let positions = File.ReadAllText "input.txt" |> parseInput
    totalFuel positions fuel1 (cheapestPosition1 positions)

let part2 =
    let positions = File.ReadAllText "input.txt" |> parseInput
    let a = (cheapestPosition2 positions Math.Ceiling)
    let b = (cheapestPosition2 positions Math.Floor)
    let best = Math.Min(a, b)
    totalFuel positions fuel2 best