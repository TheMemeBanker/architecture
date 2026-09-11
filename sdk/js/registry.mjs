/**
 * arc_registry JS SDK — zero-dependency ESM module.
 *
 * Works in Node and the browser:
 *   import { loadRegistry, byTag, handshakes, tagCounts } from "./registry.mjs";
 *   const reg = await loadRegistry();            // Node: reads data/registry.json
 *   const reg = await loadRegistry("/data/registry.json"); // browser: fetches it
 */

const DEFAULT_PATH = "data/registry.json";

/** Load the registry from a file path (Node) or URL (browser). */
export async function loadRegistry(path = DEFAULT_PATH) {
  let raw;
  if (typeof window === "undefined") {
    const { readFile } = await import("node:fs/promises");
    raw = await readFile(path, "utf8");
  } else {
    const res = await fetch(path);
    if (!res.ok) throw new Error(`fetch ${path}: ${res.status}`);
    raw = await res.text();
  }
  const reg = JSON.parse(raw);
  const problems = validate(reg);
  if (problems.length) throw new Error(`invalid registry:\n- ${problems.join("\n- ")}`);
  return reg;
}

/** Structural validation; returns a list of problems (empty = valid). */
export function validate(reg) {
  const problems = [];
  const ids = new Set();
  for (const e of reg.entries ?? []) {
    if (ids.has(e.id)) problems.push(`duplicate id: ${e.id}`);
    ids.add(e.id);
    if (!/^[a-z0-9-]+$/.test(e.id)) problems.push(`id not kebab-case: ${e.id}`);
    if (!e.tags?.length) problems.push(`entry ${e.id} has no tags`);
    for (const t of e.tags ?? []) {
      if (!reg.tags?.includes(t)) problems.push(`entry ${e.id} uses unknown tag ${t}`);
    }
    if (!["project", "handshake", "ecosystem_partner"].includes(e.type)) {
      problems.push(`entry ${e.id} has unknown type ${e.type}`);
    }
  }
  const hs = (reg.entries ?? []).filter((e) => e.type === "handshake").length;
  if (hs !== reg.total_handshakes) {
    problems.push(`total_handshakes says ${reg.total_handshakes} but found ${hs}`);
  }
  return problems;
}

export const handshakes = (reg) => reg.entries.filter((e) => e.type === "handshake");
export const partners = (reg) => reg.entries.filter((e) => e.type === "ecosystem_partner");
export const byTag = (reg, tag) => reg.entries.filter((e) => e.tags.includes(tag));
export const getEntry = (reg, id) => reg.entries.find((e) => e.id === id);

/** Tag histogram as [tag, count] pairs, descending by count then name. */
export function tagCounts(reg) {
  const counts = new Map();
  for (const e of reg.entries) for (const t of e.tags) counts.set(t, (counts.get(t) ?? 0) + 1);
  return [...counts.entries()].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]));
}
