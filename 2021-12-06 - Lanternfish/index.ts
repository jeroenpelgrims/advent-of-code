import { max, range, sum } from "ramda";

export function countDays(input: string) {
    const fish = input.split(",").map(x => parseInt(x));
    const highestDays = fish.reduce(max);
    return fish.reduce((result, fish) => {
        result[fish] += 1;
        return result;
    }, new Array(highestDays + 1).fill(0));
}

export function simulateStep(counts: number[]) {
    const [dead, ...rest] = counts;
    const result = new Array(dead > 0 ? 9 : rest.length)
        .fill(0)
        .map((a, i) => (rest[i] || 0) + a);

    if (dead > 0) {
        result[6] += dead;
        result[8] += dead;
    }

    return result;
}

export function simulate(
    input: string,
    days: number
): number {
    const counts = countDays(input);
    const finalCounts = range(0, days)
        .reduce(result => simulateStep(result), counts);
    return sum(finalCounts);
}