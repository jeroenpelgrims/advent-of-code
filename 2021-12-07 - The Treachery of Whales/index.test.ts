import test from "ava";
import { readFileSync } from "fs";
import { bestPosition, bestPosition2, countFuel, countFuel2, parseInput, singleFuel } from ".";

const testinput = readFileSync(__dirname + "/testinput").toString();

test('solution', t => {
    const positions = parseInput(readFileSync(__dirname + "/input").toString());

    console.log("2021-12-07 part 1", countFuel(bestPosition(positions), positions));
    console.log("2021-12-07 part 2", countFuel2(bestPosition2(positions), positions));
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

test('countFuel2', t => {
    const positions = parseInput(testinput);
    t.deepEqual(countFuel2(5, positions), 168);
    t.deepEqual(countFuel2(2, positions), 206);
});

test('bestPosition2', t => {
    const positions = parseInput(testinput);
    t.is(bestPosition2(positions), 5);
});

test('singleFuel', t => {
    t.is(singleFuel(16, 5), 66);
    t.is(singleFuel(1, 5), 10);
    t.is(singleFuel(2, 5), 6);
    t.is(singleFuel(0, 5), 15);
    t.is(singleFuel(4, 5), 1);
    t.is(singleFuel(7, 5), 3);
    t.is(singleFuel(14, 5), 45);
});