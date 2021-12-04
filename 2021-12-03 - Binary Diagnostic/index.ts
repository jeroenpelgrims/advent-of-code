import { time } from "console";
import { readFileSync } from "fs";
import { EOL } from "os";
import { range } from "ramda";

const values = readFileSync("./testinput")
    .toString()
    .split(EOL);
const numberLength = values[0].length;

function mostUsed(numbers: string[], position: number) {
    const ones = numbers.filter(number => number[position] === "1").length;
    const zeroes = numbers.length - ones;
    return ones > zeroes ? "1" : "0";
}

function inverse(binary: string) {
    return binary.split("").map(x => x === "1" ? "0" : "1").join("");
}

function toDecimal(binary: string) {
    return parseInt(binary, 2);
}

const gamma = range(0)(numberLength).map((_, position) => mostUsed(values, position)).join("");
const epsilon = inverse(gamma);
console.log("part 1", toDecimal(gamma) * toDecimal(epsilon));

function splitMostLeast(
    [mosts, leasts]: [string[], string[]], 
    position: number
): [string, string] {
    if (position === numberLength) {
        return [ mosts[0], leasts[0] ];
    }

    const most = gamma[position];
    const least = epsilon[position];
    return splitMostLeast([
        mosts.length === 1 ? mosts : mosts.filter(x => x[position] === most),
        leasts.length === 1 ? leasts : leasts.filter(x => x[position] === least)
    ], position + 1);
}

const [ oxygen, co2 ] = splitMostLeast([ values, values ], 0);
console.log("part 2", toDecimal(oxygen) * toDecimal(co2));