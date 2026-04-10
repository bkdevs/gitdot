"use client";

import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { useHistoryContext } from "@/(main)/context/history";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import Link from "@/ui/link";

const KEY_MAP: Record<string, number> = {
  "1": 0,
  "2": 1,
  "3": 2,
  "4": 3,
  "5": 4,
  "6": 5,
  "7": 6,
  "8": 7,
  "9": 8,
  "0": 9,
};

export function HistoryDialog() {
  const { history } = useHistoryContext();
  const router = useRouter();
  const [open, setOpen] = useState(false);

  useShortcuts(
    useMemo(
      () => [
        {
          name: "History",
          description: "Open history dialog",
          keys: ["e"],
          execute: () => setOpen(true),
        },
      ],
      [],
    ),
  );

  useEffect(() => {
    const handleOpen = () => setOpen(true);
    window.addEventListener("openHistoryDialog", handleOpen);
    return () => window.removeEventListener("openHistoryDialog", handleOpen);
  }, []);

  useEffect(() => {
    if (!open) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        setOpen(false);
        return;
      }

      const idx = KEY_MAP[e.key];
      if (idx !== undefined && idx < history.length) {
        e.preventDefault();
        router.push(history[idx]);
        setOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, history, router.push]);

  const labels = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-80! w-full p-3 pt-4 top-[45vh]!"
        aria-describedby={undefined}
        showOverlay={false}
      >
        <DialogTitle className="absolute -top-2 left-2 bg-background px-1 font-mono text-xs">
          history
        </DialogTitle>
        <div className="flex flex-col gap-y-1 font-mono text-sm">
          {history.length === 0 ? (
            <span className="text-muted-foreground">no history yet</span>
          ) : (
            history.map((path, i) => (
              <Link
                key={path}
                href={path}
                onClick={() => setOpen(false)}
                className="flex items-baseline gap-2"
              >
                <span className="text-muted-foreground shrink-0">
                  {labels[i]}.
                </span>
                <span className="truncate">{path}</span>
              </Link>
            ))
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
