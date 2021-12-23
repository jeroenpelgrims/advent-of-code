module Day7.Test

open Xunit
open System
open Day7.Lib

let input = "16,1,2,0,4,2,7,1,2,14"

[<Fact>]
let ``parseInput test`` () =
    Assert.Equal(
        [16; 1; 2; 0; 4; 2; 7; 1; 2; 14],
        parseInput input
    )

[<Fact>]
let ``fuel1 test`` () =
    let fuel = fuel1
    Assert.Equal(14, fuel 16 2)
    Assert.Equal(1, fuel 1 2)
    Assert.Equal(0, fuel 2 2)
    Assert.Equal(2, fuel 0 2)
    Assert.Equal(2, fuel 4 2)
    Assert.Equal(0, fuel 2 2)
    Assert.Equal(5, fuel 7 2)
    Assert.Equal(1, fuel 1 2)
    Assert.Equal(12, fuel 14 2)

[<Fact>]
let ``fuel2 test`` () =
    let fuel = fuel2
    Assert.Equal(66, fuel 16 5)
    Assert.Equal(10, fuel 1 5)
    Assert.Equal(6, fuel 2 5)
    Assert.Equal(15, fuel 0 5)
    Assert.Equal(1, fuel 4 5)
    Assert.Equal(6, fuel 2 5)
    Assert.Equal(3, fuel 7 5)
    Assert.Equal(10, fuel 1 5)
    Assert.Equal(45, fuel 14 5)

[<Fact>]
let ``cheapestPosition1 test`` () =
    let positions = parseInput input
    Assert.Equal(2, cheapestPosition1 positions)

[<Fact>]
let ``cheapestPosition2 test`` () =
    let positions = parseInput input
    Assert.Equal(5, cheapestPosition2 positions Math.Round)

[<Fact>]
let ``totalFuel test`` () =
    let positions = parseInput input
    Assert.Equal(37, totalFuel positions fuel1 2)
    Assert.Equal(41, totalFuel positions fuel1 1)
    Assert.Equal(39, totalFuel positions fuel1 3)
    Assert.Equal(71, totalFuel positions fuel1 10)
    Assert.Equal(168, totalFuel positions fuel2 5)
    Assert.Equal(206, totalFuel positions fuel2 2)



// 105461988