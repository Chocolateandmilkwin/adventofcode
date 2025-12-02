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

//      ______ _      ______ _____          _   _ _______
//     |  ____| |    |  ____/ ____|   /\   | \ | |__   __|
//     | |__  | |    | |__ | |  __   /  \  |  \| |  | |
//     |  __| | |    |  __|| | |_ | / /\ \ | . ` |  | |
//     | |____| |____| |___| |__| |/ ____ \| |\  |  | |
//     |______|______|______\_____/_/    \_\_| \_|  |_|
let final = file
  .split(",")
  .map((r) => r.split("-"))
  .map((b) => {
    let start_len = b[0].length;
    let end_len = b[1].length;
    if (start_len % 2 !== 0 && end_len % 2) return [];
    let start_a = parseInt(b[0].slice(0, Math.floor(start_len / 2)));
    if (start_a !== start_a) start_a = 0;
    let start_b = parseInt(b[0].slice(Math.floor(start_len / 2)));
    if (end_len % 2) {
      b[1] = "9".padEnd(start_len, "9");
    }
    let end_a = parseInt(b[1].slice(0, Math.floor(end_len / 2)));
    if (end_a !== end_a) end_a = 0;
    let end_b = parseInt(b[1].slice(Math.floor(end_len / 2)));

    if (start_a.toString().length < end_a.toString().length) {
      start_a = Number("1".padEnd(end_a.toString().length, "0"));
      start_b = 0;
    }
    let invs: number[] = [];
    if (start_b <= start_a)
      invs.push(Number(start_a.toString() + start_a.toString()));
    for (let i = start_a + 1; i < end_a; i++)
      invs.push(Number(i.toString() + i.toString()));
    if (end_a !== start_a && end_a <= end_b)
      invs.push(Number(end_a.toString() + end_a.toString()));

    return invs;
  }, 0);

warn(
  "Final value:",
  final.reduce((a, b) => a + b.reduce((a, b) => a + b, 0), 0)
);

//      ____  _____  _    _ _______ ______   ______ ____  _____   _____ ______
//     |  _ \|  __ \| |  | |__   __|  ____| |  ____/ __ \|  __ \ / ____|  ____|
//     | |_) | |__) | |  | |  | |  | |__    | |__ | |  | | |__) | |    | |__
//     |  _ <|  _  /| |  | |  | |  |  __|   |  __|| |  | |  _  /| |    |  __|
//     | |_) | | \ \| |__| |  | |  | |____  | |   | |__| | | \ \| |____| |____
//     |____/|_|  \_\\____/   |_|  |______| |_|    \____/|_|  \_\\_____|______|
let final2 = file
  .split(",")
  .map((r) => r.split("-"))
  .map((r) => [parseInt(r[0]), parseInt(r[1])])
  .map((b) => {
    let invs: number[] = [];
    for (let i = b[0]; i <= b[1]; i++) {
      let str = i.toString();
      let len = str.length;
      if (len % 2 !== 0) continue;
      let half_len = Math.floor(len / 2);
      let first_half = str.slice(0, half_len);
      let second_half = str.slice(half_len);
      if (first_half === second_half) {
        invs.push(i);
      }
    }
    // warn("Inverses in range", b[0], "-", b[1], ":", invs);

    return invs;
  }, 0);

warn(
  "Final value:",
  final2.reduce((a, b) => a + b.reduce((a, b) => a + b, 0), 0)
);

for (let i = 0; i < final.length; i++) {
  for (let j = 0; j < final[i].length; j++) {
    if (final[i][j] !== final2[i][j]) {
      warn(
        `Mismatch at range ${i} index ${j}: ${final[i][j]} != ${final2[i][j]}`
      );
    }
  }
}
