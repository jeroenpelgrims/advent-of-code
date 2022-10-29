module Day8.Test

open Xunit
open System
open System.IO

open Day8.Lib

let singleInput = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
let input = File.ReadAllText "testinput.txt"

[<Fact>]
let ``parseInputLine test`` () =
    let actual = parseInputLine singleInput
    let expected: PatternPair = (
        Seq.map Digit ["acedgfb"; "cdfbe"; "gcdfa"; "fbcad"; "dab"; "cefabd"; "cdfgeb"; "eafb"; "cagedb"; "ab"],
        Seq.map Digit ["cdfeb"; "fcadb"; "cdfeb"; "cdbaf"]
    )
    Assert.Equal<PatternPair>(expected, actual)

[<Fact>]
let ``countUniqueDigits test`` () =
    //Assert.True (part1 < 1707)
    //Assert.NotEqual(part1, 393)
    Assert.Equal(countUniqueDigits (parseInput input), 26)

[<Fact>]
let ``identifyDigits test`` () =
    let (signals, _) = Seq.head <| parseInput singleInput
    let actual = Seq.toArray(identifyDigits signals)
    //let expected = Seq.toArray(seq { "cagedb"; "ab"; "gcdfa"; "fbcad"; "eafb"; "cdfbe"; "cdfgeb"; "dab"; "acedgfb"; "cefabd" } |> Seq.map (fun s -> s.ToCharArray() :> Digit))
    //Assert.Equal<seq<Digit>>(actual, expected)
    let eq = "abcd" = "dcba"
    ignore