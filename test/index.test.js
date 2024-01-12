import test from "node:test";
import assert from "node:assert/strict";
import * as citationberg from "../dist/index.js";
import { readFile } from "node:fs/promises";

const ieee = await readFile(new URL(import.meta.resolve("./ieee.csl")), "utf8");

console.log(citationberg);
