import { max, range, sum, median } from "ramda";

export function parseInput(input: string) {
    return input.split(",").map(x => parseInt(x));
}

export function countFuel(targetPosition: number, positions: number[]) {
    const values = positions.map(position => Math.abs(targetPosition - position));
    return sum(values);
}

export function bestPosition(positions: number[]) {
    return median(positions);
}