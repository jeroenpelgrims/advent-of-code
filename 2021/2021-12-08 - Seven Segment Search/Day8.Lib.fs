module Day8.Lib

open System
open System.IO
open FSharpPlus

type Segment = char
type Digit = seq<Segment>
type PatternPair = seq<Digit> * seq<Digit>

let toDigit (c: Segment): Digit = seq { c }

let (-) (d1: Digit) (d2: Digit): Digit =
    Seq.except d2 d1

let (+) (d1: Digit) (d2: Digit): Digit =
    Seq.append d2 d1 |> Seq.distinct

//let (=) (d1: Digit) (d2: Digit): bool =
//    let lengthsEqual = Seq.length d1 = Seq.length d2
//    let c = Seq.countBy(id)
//    lengthsEqual
//    //containedLength = Seq.length d2
let digitsEqual (d1: Digit) (d2: Digit) =
    let lengthsEqual = Seq.length d1 = Seq.length d2
    let d1Contents = Seq.countBy id d1
    let d2Contents = Seq.countBy id d2
    Seq.map (fun (char, count) -> ) d1Contents

let parseInputLine (s: string): PatternPair =
    match s.Split " | " with
    | [| heads; tails |] ->
        let heads' = heads.Split(" ") |> Seq.map (fun s -> s.ToCharArray() :> Digit)
        let tails' = tails.Split(" ") |> Seq.map (fun s -> s.ToCharArray() :> Digit)
        (heads', tails')
    | _ -> raise <| new Exception "Incorrect format"

let containsAll (toContain: seq<Segment>) (container: Digit) =
    Seq.forall (fun segment -> Seq.contains segment container) toContain

let identifyDigits (digits: seq<Digit>): seq<Digit> =
    let one: Digit = Seq.find (fun d -> Seq.length d = 2) digits
    let four: Digit = Seq.find (fun d -> Seq.length d = 4) digits
    let seven: Digit = Seq.find (fun d -> Seq.length d = 3) digits
    let eight: Digit = Seq.find (fun d -> Seq.length d = 7) digits
    let rest = Seq.except [one; four; seven; eight] digits
    let three = Seq.find (fun d -> (containsAll seven d) && Seq.length d = 5) rest
    let rest = Seq.except [three] rest
    let nine = Seq.find (fun d -> (containsAll three d) && Seq.length d = 6) rest
    let rest = Seq.except [nine] rest
    //let a = seven - one |> Seq.head
    let e = eight - nine |> Seq.head
    //let b = eight - three - seq { e } |> Seq.head
    let five = Seq.find (fun d -> (containsAll d nine) && Seq.length d = 5) rest
    let six = five + seq { e }
    let rest = Seq.except [five; six] rest
    //let c = eight - six |> Seq.head
    let zero = Seq.find (fun d -> containsAll d eight && Seq.length d = 6) rest
    let two = Seq.except [zero] rest |> Seq.head

    [zero; one; two; three; four; five; six; seven; eight; nine]

let parseInput (s: string) =
    (s.Split Environment.NewLine)
    |> Seq.map parseInputLine

let countUniqueDigits (pairs: seq<PatternPair>) =
    let tails = Seq.map snd pairs
    let tails': seq<Digit> = Linq.Enumerable.SelectMany(tails, id)
    Seq.map Seq.length tails'
    |> Seq.filter (fun (x: int) -> Seq.contains x [2; 4; 3; 7])
    |> Seq.length

let part1 =
    let input = File.ReadAllText "input.txt" |> parseInput
    countUniqueDigits input

let part2 =
    let input = File.ReadAllText "input.txt" |> parseInput
    countUniqueDigits input