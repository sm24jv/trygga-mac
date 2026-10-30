// Prints what one measuring run answered, as Markdown, from the folder of its logs.
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const LATE_MS = 65_000;

function read(dir, file) {
  const path = join(dir, file);
  return existsSync(path) ? readFileSync(path, "utf8").trim() : "";
}

/** Each line of a log, as the moment the app received it and what the page put in it. */
function ticks(dir, file) {
  return read(dir, file)
    .split("\n")
    .filter(Boolean)
    .map((line) => {
      const [receivedAt, ...rest] = line.split(" ");
      const text = decodeURIComponent(rest.join(" "));
      const gap = text.match(/gap_ms=(\d+)/);
      const mode = text.match(/display_mode=(\S+)/);
      const visibility = text.match(/visibility=(\S+)/);
      return {
        receivedAt: Number(receivedAt),
        gapMs: gap ? Number(gap[1]) : null,
        displayMode: mode?.[1],
        visibility: visibility?.[1],
      };
    });
}

function gapLine(entries) {
  const gaps = entries.slice(1).map((entry) => entry.gapMs).filter((gap) => gap !== null);
  if (gaps.length === 0) return "no gaps";
  const late = gaps.filter((gap) => gap > LATE_MS).length;
  return `${gaps.length} gaps, min ${Math.min(...gaps)} ms, max ${Math.max(...gaps)} ms, ${late} over ${LATE_MS} ms`;
}

const dir = process.argv[2];
const page = ticks(dir, "page.log");
const title = ticks(dir, "title.log").filter((entry) => entry.gapMs !== null);
const last = page.at(-1) ?? title.at(-1);

console.log(`### ${read(dir, "macos.txt").split("\n").map((line) => line.split(":")[1]?.trim()).filter(Boolean).join(" ") || "unknown macOS"}`);
console.log(`- App at the end: ${read(dir, "app-state.txt") || "unknown"}`);
if (!last) {
  console.log("- Not measurable: the page wrote nothing");
} else {
  console.log(`- M1 page ticks: ${page.length}, ${gapLine(page)}`);
  console.log(`- M2 titles the app received: ${title.length}, ${gapLine(title)}`);
  console.log(`- M9 display mode: ${last.displayMode}, visibility: ${last.visibility}`);
}
