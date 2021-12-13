import { sum, median, mean } from "ramda";

export function parseInput(input: string) {
    return input.split(",").map(x => parseInt(x));
}

export function fuel(from: number, to: number) {
    return Math.abs(from - to);
}

export function fuel2(from: number, to: number) {
    const n = Math.abs(from - to);
    return (n * (n + 1)) / 2;
}

export function bestPosition(positions: number[]) {
    return median(positions);
}

export function bestPosition2(positions: number[]) {
    return Math.round(mean(positions));
}

export function sumOfFuels(f: (from: number, to: number) => number, bestPosition: number, positions: number[]){
    return sum(positions.map(position => f(position, bestPosition)));
}

export function part1(input: string) {
    const positions = parseInput(input);
    const best = bestPosition(positions);
    return sumOfFuels(fuel, best, positions);
}

export function part2(input: string) {
    const positions = parseInput(input);
    const best = bestPosition2(positions);
    return sumOfFuels(fuel2, best, positions);
}