import test from "node:test";
import assert from "node:assert/strict";
import * as citationberg from "../pkg/citationberg_js.js";
// import { readFile } from "node:fs/promises";

// const ieee = await readFile(new URL(import.meta.resolve("./ieee.csl")), "utf8");

test("exports", () => {
  for (const name of [
    "DateVariable",
    "Kind",
    "Locator",
    "NameVariable",
    "NumberVariable",
    "OtherTerm",
    "StandardVariable",
    "Term",
    "TermConversionError",
    "Variable",
  ]) {
    // assert(name in citationberg.taxonomy);
    assert(name in citationberg);
  }
});

test("NumberVariable", () => {
  const a = citationberg.NumberVariable.ChapterNumber();
  console.log(a.tag, a.val);
  a.is_number_of_variable();
});

test("OtherTerm", () => {
  const a = citationberg.OtherTerm.OrdinalN(4);
  console.log(a.tag, a.val);
  a.is_n_ordinal();

  const b = citationberg.OtherTerm.PersonalCommunication();
  console.log(b.tag, b.val);
  b.is_ordinal();
});

test("Term", () => {
  const a = citationberg.Term.NumberVariable(
    citationberg.NumberVariable.ChapterNumber()
  );
  console.log(a.tag, a.val);
  a.is_gendered();
});

test("Variable", () => {
  const a = citationberg.Variable.Standard(
    citationberg.StandardVariable.CallNumber
  );
  console.log(a.tag, a.val);
  a.is_number_of_variable();
});
