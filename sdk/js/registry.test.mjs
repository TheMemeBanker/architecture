import { test } from "node:test";
import assert from "node:assert/strict";
import { loadRegistry, validate, handshakes, partners, byTag, getEntry, tagCounts } from "./registry.mjs";

const REG_PATH = new URL("../../data/registry.json", import.meta.url).pathname;

test("loads and validates the checked-in registry", async () => {
  const reg = await loadRegistry(REG_PATH);
  assert.equal(reg.title, "arc_registry");
  assert.equal(validate(reg).length, 0);
});

test("handshake count matches the declared total", async () => {
  const reg = await loadRegistry(REG_PATH);
  assert.equal(handshakes(reg).length, reg.total_handshakes);
});

test("partners and projects partition sensibly", async () => {
  const reg = await loadRegistry(REG_PATH);
  assert.equal(partners(reg).length, 8);
  assert.ok(getEntry(reg, "askjimmy").tags.includes("forge"));
});

test("byTag filters correctly", async () => {
  const reg = await loadRegistry(REG_PATH);
  const defi = byTag(reg, "defi").map((e) => e.id);
  assert.deepEqual(defi.sort(), ["askjimmy", "listen-rs"]);
});

test("tagCounts is a descending histogram", async () => {
  const reg = await loadRegistry(REG_PATH);
  const counts = tagCounts(reg);
  for (let i = 1; i < counts.length; i++) assert.ok(counts[i - 1][1] >= counts[i][1]);
});

test("validate flags broken registries", () => {
  const bad = { title: "x", total_handshakes: 2, tags: ["a"], entries: [
    { id: "One", type: "mystery", tags: ["zzz"], summary: "" },
    { id: "One", type: "handshake", tags: [], summary: "" },
  ]};
  const problems = validate(bad);
  assert.ok(problems.some((p) => p.includes("duplicate id")));
  assert.ok(problems.some((p) => p.includes("kebab-case")));
  assert.ok(problems.some((p) => p.includes("unknown tag")));
  assert.ok(problems.some((p) => p.includes("unknown type")));
  assert.ok(problems.some((p) => p.includes("total_handshakes")));
});
