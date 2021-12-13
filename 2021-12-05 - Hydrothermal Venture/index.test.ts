import test from "ava";
import { readFileSync } from "fs";
import { part1, expandToPointsStraight, Vector, Point, expandToPointsDiagonal, part2 } from '.';

test('testinput', t => {
    const input = readFileSync(__dirname + "/testinput").toString();
    const result1 = part1(input);
    const result2 = part2(input);
        
    t.is(result1, 5);
    t.is(result2, 12);
});

test('results', t => {
    const input = readFileSync(__dirname + "/input").toString();
    const result1 = part1(input);
    const result2 = part2(input);
        
    t.true(true);
    console.log("2021-12-05 part 1", result1);
    console.log("2021-12-05 part 2", result2);
});

test("expandToPointsStraight", t => {
    const expectations: [Vector, Point[]][] = [
        [
            [[1, 1], [1, 3]],
            [[1, 1], [1, 2], [1, 3]]
        ],
        [
            [[9, 7], [7, 7]],
            [[9, 7], [8, 7], [7, 7]]
        ]
    ];

    for (let [vector, expected] of expectations) {
        const actual = expandToPointsStraight(vector);

        for (let point of expected) {
            t.truthy(actual.find(([x, y]) =>
                x === point[0] && y === point[1]
            ), JSON.stringify([point, actual]));
        }
    }
});

test("expandToPointsDiagonal", t => {
    const expectations: [Vector, Point[]][] = [
        [
            [[1, 1], [3, 3]],
            [[1, 1], [2, 2], [3, 3]]
        ],
        [
            [[9, 7], [7, 9]],
            [[9, 7], [8, 8], [7, 9]]
        ],
    ];

    for (let [vector, expected] of expectations) {
        const actual = expandToPointsDiagonal(vector);

        for (let point of expected) {
            t.truthy(actual.find(([x, y]) =>
                x === point[0] && y === point[1]
            ), JSON.stringify([point, actual]));
        }
    }
});