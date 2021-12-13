import test from "ava";
import { readFileSync } from "fs";
import { countDays, simulate, simulateStep } from ".";

test('parseInput', t => {
    t.deepEqual(countDays("3,4,3,1,2"), [0, 1, 1, 2, 1]);
    t.deepEqual(countDays("0,1,2,3,4,5,6,7,8"), [1, 1, 1, 1, 1, 1, 1, 1, 1]);
    t.deepEqual(countDays("9"), [0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    t.deepEqual(countDays("2,3,2,0,1"), [1, 1, 2, 1]);
    t.deepEqual(countDays("1,2,1,6,0,8"), [1, 2, 1, 0, 0, 0, 1, 0, 1]);
});

test('testinput', t => {
    const input = readFileSync(__dirname + "/testinput").toString();
    t.is(simulate(input, 18), 26);
    t.is(simulate(input, 80), 5934);
    t.is(simulate(input, 256), 26984457539);
});

test('result', t => {
    const input = readFileSync(__dirname + "/input").toString();
    console.log("2021-12-06 part 1", simulate(input, 80));
    console.log("2021-12-06 part 2", simulate(input, 256));
    t.true(true);
});

test('simulateStep', t => {
    t.deepEqual(
        simulateStep(countDays("3,4,3,1,2")),
        countDays("2,3,2,0,1")
    );
    t.deepEqual(
        simulateStep(countDays("2,3,2,0,1")),
        countDays("1,2,1,6,0,8")
    );
    t.deepEqual(
        simulateStep(countDays("1,2,1,6,0,8")),
        countDays("0,1,0,5,6,7,8")
    );
    t.deepEqual(
        simulateStep(countDays("0,1,0,5,6,7,8")),
        countDays("6,0,6,4,5,6,7,8,8")
    );
    t.deepEqual(
        simulateStep(countDays("6,0,6,4,5,6,7,8,8")),
        countDays("5,6,5,3,4,5,6,7,7,8")
    );
});
