"use client";

import { useMemo } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";

const SCROLL_AMOUNT = 20;

export function FileViewerShortcuts() {
  const shortcuts = useMemo(
    () => [
      {
        name: "ScrollDown",
        description: "Scroll file down",
        keys: ["j"],
        execute: () => {
          document
            .querySelector<HTMLElement>("[data-page-scroll]")
            ?.scrollBy({ top: SCROLL_AMOUNT });
        },
      },
      {
        name: "ScrollUp",
        description: "Scroll file up",
        keys: ["k"],
        execute: () => {
          document
            .querySelector<HTMLElement>("[data-page-scroll]")
            ?.scrollBy({ top: -SCROLL_AMOUNT });
        },
      },
    ],
    [],
  );

  useShortcuts(shortcuts);
  return null;
}
