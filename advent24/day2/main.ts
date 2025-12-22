import { readFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function isDiffValid(max: number, min: number) {
  return max - min >= 1 && max - min <= 3;
}

function main() {
  let result = 0;
  let filePath = path.join(__dirname, "input");
  let inputStr = readFileSync(filePath, "utf-8");
  const lines = inputStr.trim().split("\n");

  for (const line of lines) {
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
  console.log(result);

  return 0;
}

main();
