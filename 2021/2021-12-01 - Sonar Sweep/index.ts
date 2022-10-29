import { readFileSync } from "fs";
import { zip, drop, flatten, sum } from "ramda";

const numbers = readFileSync("./src/input")
    .toString()
    .split("\n")
    .map(x => parseInt(x));

function countIncreases(numbers: number[]) {
    const pairs = zip(numbers, drop(1, numbers));
    const counts = pairs.filter(([a, b]) => b > a);
    return counts.length;
}

const pairs = zip(zip(numbers, drop(1, numbers)), drop(2, numbers)).map(flatten);
const sums = pairs.map(sum);

console.log("part 1", countIncreases(numbers));
console.log("part 2", countIncreases(sums));