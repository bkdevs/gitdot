"use client";

import { useMemo } from "react";
import { type Shortcut, useShortcuts } from "@/(main)/context/shortcuts";

export function CommitsShortcuts({
  setStartDate,
  setEndDate,
}: {
  setStartDate: (v: string | null) => void;
  setEndDate: (v: string | null) => void;
}) {
  const shortcuts = useMemo<Shortcut[]>(
    () => [
      {
        name: "ClearDates",
        description: "Clear date selection",
        keys: ["Escape"],
        execute: () => {
          setStartDate(null);
          setEndDate(null);
        },
      },
    ],
    [setStartDate, setEndDate],
  );

  useShortcuts(shortcuts);
  return null;
}
