import type { RepositoryTree, RepositoryTreeEntry } from "@/lib/dto";

export type LineSelection = {
  start: number;
  end: number;
};

export function parseLineSelection(
  param: string | string[] | undefined,
): LineSelection | null {
  if (!param || typeof param !== "string") return null;
  if (param.includes("-")) {
    const [start, end] = param.split("-").map(Number);
    if (
      !Number.isNaN(start) &&
      !Number.isNaN(end) &&
      start > 0 &&
      end >= start
    ) {
      return { start, end };
    }
  } else {
    const line = Number(param);
    if (!Number.isNaN(line) && line > 0) {
      return { start: line, end: line };
    }
  }
  return null;
}

export function formatLineSelection(selection: LineSelection): string {
  return selection.start === selection.end
    ? `${selection.start}`
    : `${selection.start}-${selection.end}`;
}
