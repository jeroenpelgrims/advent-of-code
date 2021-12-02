import { readFileSync } from "fs";

type Direction = "forward" | "up" | "down";
type Command = [Direction, number];
type Position = [number, number]; // x, y
type AimedPosition = [number, number, number]; // x, y, aim

const commands: Command[] = readFileSync("./input")
    .toString()
    .split("\n")
    .map(line => line.split(" "))
    .map(([command, value]) => [command as Direction, parseInt(value)]);

function processCommands(commands: Command[], start: Position = [0, 0]): Position {
    return commands.reduce(([x, y], [command, value]) => {
        switch (command) {
            case "forward":
                return [x + value, y];
            case "up":
                return [x, y - value];
            case "down":
                return [x, y + value];
        }
    }, start);
}

function processCommands2(commands: Command[], start: AimedPosition = [0, 0, 0]): AimedPosition {
    return commands.reduce(([x, y, aim], [command, value]) => {
        switch (command) {
            case "forward":
                return [x + value, y + aim * value, aim];
            case "up":
                return [x, y, aim - value];
            case "down":
                return [x, y, aim + value];
        }
    }, start);
}

const [x, y] = processCommands(commands);
console.log("part 1", x * y);
const [x2, y2] = processCommands2(commands);
console.log("part 2", x2 * y2);