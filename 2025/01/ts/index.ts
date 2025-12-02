import { warn } from "console";
import fs from "fs/promises";

let file = await fs.readFile("input.txt", "utf-8");
warn("File content length:", file.length);
