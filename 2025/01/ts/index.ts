import { warn } from "console";
import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

//Get file
let filePath = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  "input.txt"
);
let file = await fs.readFile(filePath, "utf-8");

//Dial
let lines = file.split("\n");
let dial = 50;
let count = 0;
for (let line of lines) {
  let number = Number(line.slice(1));
  let offset = line[0] === "L" ? -number : number;
  dial += offset % 100;
  if (dial < 0) dial += 100;
  else if (dial >= 100) dial -= 100;
  if (dial == 0) count++;
}
warn("Final dial value:", dial, count);
