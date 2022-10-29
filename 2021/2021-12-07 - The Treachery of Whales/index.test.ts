import test from "ava";
import { readFileSync } from "fs";
import { bestPosition, bestPosition2, fuel, fuel2, parseInput, part1, part2, sumOfFuels } from ".";

const testinput = readFileSync(__dirname + "/testinput").toString();

test('solution', t => {
    const input = readFileSync(__dirname + "/input").toString();
    console.log("2021-12-07 part 1", part1(input));
    console.log("2021-12-07 part 2", part2(input));
    t.is(part1(input), 356958);
});

test('parseInput', t => {
    t.deepEqual(parseInput(testinput), [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
});

test('fuel', t => {
    const target = 2;

    t.is(fuel(16, target), 14);
    t.is(fuel(1, target), 1);
    t.is(fuel(2, target), 0);
    t.is(fuel(0, target), 2);
    t.is(fuel(4, target), 2);
    t.is(fuel(7, target), 5);
    t.is(fuel(14, target), 12);
});

test('fuel2', t => {
    const target = 5;

    t.is(fuel2(16, target), 66);
    t.is(fuel2(1, target), 10);
    t.is(fuel2(2, target), 6);
    t.is(fuel2(0, target), 15);
    t.is(fuel2(4, target), 1);
    t.is(fuel2(7, target), 3);
    t.is(fuel2(14, target), 45);
});

test('sumOfFuels', t => {
    const positions = parseInput(testinput);
    t.is(sumOfFuels(fuel, 2, positions), 37);
    t.is(sumOfFuels(fuel, 1, positions), 41);
    t.is(sumOfFuels(fuel, 3, positions), 39);
    t.is(sumOfFuels(fuel, 10, positions), 71);

    t.is(sumOfFuels(fuel2, 5, positions), 168);
    t.is(sumOfFuels(fuel2, 2, positions), 206);
});

test('bestPosition', t => {
    const positions = parseInput(testinput);
    t.is(bestPosition(positions), 2);
});

test('bestPosition2', t => {
    const positions = parseInput(testinput);
    t.is(bestPosition2(positions), 5);
});