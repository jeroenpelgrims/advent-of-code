import { sum, median, converge, divide, length } from "ramda";

const average = (xs: number[]) => divide(sum(xs), length(xs));

export function parseInput(input: string) {
    return input.split(",").map(x => parseInt(x));
}

export function countFuel(targetPosition: number, positions: number[]) {
    const values = positions.map(position => Math.abs(targetPosition - position));
    return sum(values);
}

export function singleFuel(from: number, to: number) {
    const n = Math.abs(from - to);
    return (n * (n + 1)) / 2;
}

export function countFuel2(targetPosition: number, positions: number[]) {
    const values = positions.map(position => singleFuel(targetPosition, position));
    return sum(values);
}

export function bestPosition(positions: number[]) {
    return median(positions);
}

export function bestPosition2(positions: number[]) {
    return Math.round(average(positions));
}