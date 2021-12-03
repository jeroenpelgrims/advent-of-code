import { readFileSync } from "fs";
import { EOL } from "os";
import { transpose, sum, countBy, identity } from "ramda";

const values = readFileSync("./input")
    .toString()
    .split(EOL)
    .map(x => x.split("").map(x => parseInt(x)));

const toDecimal = (x: number[]) => parseInt(x.join(""), 2);
const transposedValues = transpose(values);
const gamma = transposedValues.map(column => sum(column) > column.length / 2 ? 1 : 0);
const epsilon = gamma.map(x => Math.abs(x - 1));
console.log("part1", toDecimal(gamma) * toDecimal(epsilon));

