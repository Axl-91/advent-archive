import { readFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function main() {
  let filePath = path.join(__dirname, "input_test");
  let inputStr = readFileSync(filePath, "utf-8");
  const lines = inputStr.split("\n");

  for (const line of lines) {
    console.log(line);
  }
  return 0;
}

main();
