"use client";

import { useMemo } from "react";
import { type Shortcut, useShortcuts } from "@/(main)/context/shortcuts";
import type { TreeRowData } from "./folder-tree";

export function FolderShortcuts({
  rows,
  hoveredPath,
  onHover,
  onPin,
  onToggle,
}: {
  rows: TreeRowData[];
  hoveredPath: string | null;
  onHover: (path: string) => void;
  onPin: (path: string) => void;
  onToggle: (path: string) => void;
}) {
  const shortcuts = useMemo<Shortcut[]>(
    () => [
      {
        name: "TreeDown",
        description: "Next tree item",
        keys: ["j"],
        execute: () => {
          if (!rows.length) return;
          const idx = hoveredPath
            ? rows.findIndex((r) => r.path === hoveredPath)
            : -1;
          const next = idx === -1 ? 0 : Math.min(idx + 1, rows.length - 1);
          onHover(rows[next].path);
        },
      },
      {
        name: "TreeUp",
        description: "Previous tree item",
        keys: ["k"],
        execute: () => {
          if (!rows.length) return;
          const idx = hoveredPath
            ? rows.findIndex((r) => r.path === hoveredPath)
            : -1;
          const prev = idx === -1 ? rows.length - 1 : Math.max(idx - 1, 0);
          onHover(rows[prev].path);
        },
      },
      {
        name: "TreePin",
        description: "Pin focused tree item",
        keys: ["Enter", " "],
        execute: () => {
          if (!hoveredPath) return;
          onPin(hoveredPath);
          const row = rows.find((r) => r.path === hoveredPath);
          if (row?.isTree) onToggle(hoveredPath);
        },
      },
    ],
    [rows, hoveredPath, onHover, onPin, onToggle],
  );

  useShortcuts(shortcuts);
  return null;
}
