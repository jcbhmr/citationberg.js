import test from "node:test"
import assert from "node:assert/strict"
import * as citationberg from "../pkg/citationberg.js"
import { readFile } from "node:fs/promises"

const csl = await readFile(new URL(import.meta.resolve("./ieee.csl")), "utf8")

test("", () => {
  console.log(citationberg)
})
