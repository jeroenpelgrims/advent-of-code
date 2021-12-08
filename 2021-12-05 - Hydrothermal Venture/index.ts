import { EOL } from "os";
import { range, last } from "ramda";

export type Point = [number, number];
export type Vector = [Point, Point];

function parseInput(input: string) {
    return input.split(EOL).map(line =>
        line.split(" -> ").map(part => 
            part.split(",").map(x => parseInt(x)) as Point
        ) as Vector
    );
}

function isStraight([[startx, starty], [endx, endy]]: Vector) {
    return startx === endx || starty === endy;
}

export function expandToPointsStraight(vector: Vector) {
    let [from, to] = vector;

    if (from[0] > to[0] || from[1] > to[1]) {
        [from, to] = [to, from];
    }

    return range(from[0])(to[0] + 1).map(x => 
        range(from[1])(to[1] + 1).map(y =>
            [x, y] as Point
        )
    ).flat(1);
}

export function expandToPointsDiagonal([[x1, y1], [x2, y2]]: Vector): Point[] {
    const length = Math.abs(x1 - x2);
    let modifier: Point;
    if (x1 < x2 && y1 > y2) {
        modifier = [1, -1];
    } else if (x1 < x2 && y1 < y2) {
        modifier = [1, 1];
    } else if (x1 > x2 && y1 < y2) {
        modifier = [-1, 1];
    } else if (x1 > x2 && y1 > y2) {
        modifier = [-1, -1];
    }
    const modifiers = range(0, length).map(_ => modifier);
    const points: Point[] = modifiers.reduce((result, modifier) => {
        const l = last(result)!;
        return [...result, [l[0] + modifier[0], l[1] + modifier[1]]];
    }, [[x1, y1]] as Point[]);

    return points;
}
 
function expandToPoints(vector: Vector, options?: { onlyStraight: boolean}) {
    if (isStraight(vector)) {
        return expandToPointsStraight(vector);
    } else if (!options?.onlyStraight) {
        return expandToPointsDiagonal(vector);
    }
    return [];
}

function countCrossings(points: Point[]) {
    const counts = points.reduce((count, [x, y]) => {
        const key = `${x},${y}`;
        if (!count.has(key)) {
            count.set(key, 0);
        }
        count.set(key, count.get(key)! + 1)
        return count;
    }, new Map<string, number>());
    const crossings = Array.from(counts.values()).filter(x => x > 1);
    return crossings.length;
}

export function part1(input: string): number {
    const data = parseInput(input);
    const points = data.map(vector => expandToPoints(vector, { onlyStraight: true })).flat(1);
    return countCrossings(points);
}

export function part2(input: string): number {
    const data = parseInput(input);
    const points = data.map(vector => expandToPoints(vector)).flat(1);    
    return countCrossings(points);
}