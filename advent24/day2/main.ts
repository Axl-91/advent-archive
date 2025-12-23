import { readFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function isDiffValid(max: number, min: number) {
  return max - min >= 1 && max - min <= 3;
}

function part1(inputLines: string[]) {
  let result = 0;

  for (const line of inputLines) {
    let numbers = line.split(" ").map(Number);

    let isAsc = numbers.every(
      (num, i, arr) =>
        i === 0 || (num > arr[i - 1]! && isDiffValid(num, arr[i - 1]!)),
    );
    let isDesc = numbers.every(
      (num, i, arr) =>
        i === 0 || (num < arr[i - 1]! && isDiffValid(arr[i - 1]!, num)),
    );

    if (isAsc || isDesc) result++;
  }

  console.log(`Part 1 Result: ${result}`);
}

function part2(inputLines: string[]) {
  let result = 0;

  for (const line of inputLines) {
    let numbers = line.split(" ").map(Number);

    for (let i = 0; i < numbers.length; i++) {
      let new_numbers = numbers.filter((_, idx) => i != idx);

      let isAsc = new_numbers.every(
        (num, i, arr) =>
          i === 0 || (num > arr[i - 1]! && isDiffValid(num, arr[i - 1]!)),
      );
      let isDesc = new_numbers.every(
        (num, i, arr) =>
          i === 0 || (num < arr[i - 1]! && isDiffValid(arr[i - 1]!, num)),
      );

      if (isAsc || isDesc) {
        result++;
        break;
      }
    }
  }
  console.log(`Part 2 Result: ${result}`);
}

function main() {
  let filePath = path.join(__dirname, "input");
  let inputStr = readFileSync(filePath, "utf-8");
  const inputLines = inputStr.trim().split("\n");

  part1(inputLines);
  part2(inputLines);

  return 0;
}

main();
