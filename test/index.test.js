import test from "node:test";
import assert from "node:assert/strict";
import * as citationberg from "../pkg/citationberg_js.js";
import { readFile } from "node:fs/promises";

const ieee = await readFile(new URL(import.meta.resolve("./ieee.csl")), "utf8");

console.log(citationberg);
