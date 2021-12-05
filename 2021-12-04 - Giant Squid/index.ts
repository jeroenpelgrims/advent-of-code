import { readFileSync } from "fs";
import { EOL } from "os";
import { flatten, sum, transpose } from "ramda";

type Grid = string[][];
type State = { grids: Grid[], winner?: Grid, lastNumber?: string };

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

const [numbers, grids] = input();
const { winner, lastNumber } = numbers.reduce((state: State, number: string) => {
    if (state.winner !== undefined) {
        return state;
    }

    const grids = state.grids.map(grid => removeNumber(grid, number));
    const winner = grids.find(isWinner);
    return { ...state, grids, winner, lastNumber: number };
}, { grids } as State);

const restNumbers = flatten(winner!)
    .filter(x => x !== '')
    .map(x => parseInt(x));

console.log("part1", sum(restNumbers) * parseInt(lastNumber!));