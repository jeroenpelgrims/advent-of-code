import { readFileSync } from "fs";
import { EOL } from "os";
import { flatten, head, last, sum, tail, transpose } from "ramda";

type Grid = string[][];
type WinningGrid = { grid: Grid, lastNumber: string }
type State = { grids: Grid[], winners: WinningGrid[] };

function input(): [string[], Grid[]] {
    const data = readFileSync("./input")
        .toString()
        .split(EOL + EOL)
        .map(x => x.trim());
    const [numbers, ...rest] = data;
    const grids = rest.map(grid => grid.split(EOL).map(row => row.trim().split(/\s+/)));
    
    return [numbers.split(","), grids];
}

function removeNumber(grid: Grid, number: string): Grid {
    return grid.map(row =>
        row.map(elem =>
            elem === number ? '' : elem
        )
    )
}

function isWinner(grid: Grid) {
    const rowWinner = grid.find(row =>
        row.join("") === ""
    );
    const colWinner = transpose(grid).find(col =>
        col.join("") === ""
    );

    return rowWinner || colWinner;
}

function sumGrid(grid: Grid) {
    const numbers = flatten(grid)
        .filter(x => x !== '')
        .map(x => parseInt(x));
    return sum(numbers);
}

const [numbers, grids] = input();
const { winners } = numbers.reduce((state: State, number: string): State => {
    if (state.grids.length === 0) {
        return state;
    }

    const grids = state.grids.map(grid => removeNumber(grid, number));
    const newWinners = grids.filter(isWinner);

    return {
        grids: grids.filter(grid => !newWinners.includes(grid)),
        winners: [...state.winners, ...newWinners.map(grid => ({ grid, lastNumber: number }))]
    };
}, { grids, winners: [] });

const firstWinner = head(winners)!;
console.log("part1", sumGrid(firstWinner.grid) * parseInt(firstWinner.lastNumber));

const lastWinner = last(winners)!;
console.log("part2", sumGrid(lastWinner.grid) * parseInt(lastWinner.lastNumber));