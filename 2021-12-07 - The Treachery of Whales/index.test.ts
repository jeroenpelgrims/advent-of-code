import test from "ava";
import { readFileSync } from "fs";
import { bestPosition, countFuel, parseInput } from ".";

const testinput = readFileSync(__dirname + "/testinput").toString();

test('solution', t => {
    const positions = parseInput(readFileSync(__dirname + "/input").toString());
    const bestPos = bestPosition(positions);

    console.log("2021-12-07 part 1", countFuel(bestPos, positions));
    t.is(true, true);
});

test('parseInput', t => {
    t.deepEqual(parseInput(testinput), [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
});

test('countFuel', t => {
    const positions = parseInput(testinput);
    t.deepEqual(countFuel(2, positions), 37);
    t.deepEqual(countFuel(1, positions), 41);
    t.deepEqual(countFuel(3, positions), 39);
    t.deepEqual(countFuel(10, positions), 71);
});

test('bestPosition', t => {
    const positions = parseInput(testinput);
    t.is(bestPosition(positions), 2);
});