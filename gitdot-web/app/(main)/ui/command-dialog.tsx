"use client";

import { useRouter } from "next/navigation";
import { useEffect, useMemo, useRef, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

const MOCK_REPOS = [
  { owner: "pybae", name: "gitdot" },
  { owner: "pybae", name: "dotfiles" },
  { owner: "pybae", name: "blog" },
];

const MOCK_ORGS = [{ name: "acme" }, { name: "vercel" }];

type Item = {
  label: string;
  type: "repo" | "org" | "action";
  execute: () => void;
};

export function CommandDialog() {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);
  const { user } = useUserContext();
  const router = useRouter();
  const inputRef = useRef<HTMLInputElement>(null);

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Command",
          description: "Open command dialog",
          keys: [" "],
          execute: () => setOpen(true),
        },
      ],
      [],
    ),
  );

  useEffect(() => {
    const handleOpen = () => setOpen(true);
    window.addEventListener("openCommandDialog", handleOpen);
    return () => window.removeEventListener("openCommandDialog", handleOpen);
  }, []);

  const close = () => {
    setOpen(false);
    setInput("");
    setSelectedIdx(0);
  };

  const allItems = useMemo<Item[]>(() => {
    const repos: Item[] = MOCK_REPOS.map((r) => ({
      type: "repo",
      label: `${r.owner}/${r.name}`,
      execute: () => router.push(`/${r.owner}/${r.name}`),
    }));

    const orgs: Item[] = MOCK_ORGS.map((o) => ({
      type: "org",
      label: o.name,
      execute: () => router.push(`/${o.name}`),
    }));

    const actions: Item[] = [
      {
        type: "action",
        label: "profile",
        execute: () => user && router.push(`/${user.name}`),
      },
      {
        type: "action",
        label: "settings",
        execute: () => window.dispatchEvent(new CustomEvent("openSettings")),
      },
      {
        type: "action",
        label: "history",
        execute: () => window.dispatchEvent(new Event("openHistoryDialog")),
      },
    ];

    return [...repos, ...orgs, ...actions];
  }, [user, router]);

  const filteredItems = useMemo(() => {
    const q = input.trim().toLowerCase();
    if (!q) return allItems;
    return allItems.filter((item) => item.label.toLowerCase().includes(q));
  }, [allItems, input]);


  useEffect(() => {
    setSelectedIdx(0);
  }, [input]);

  useEffect(() => {
    if (!open) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        close();
        return;
      }
      if (e.key === "ArrowDown" || (e.ctrlKey && e.key === "n")) {
        e.preventDefault();
        setSelectedIdx((i) => Math.min(i + 1, filteredItems.length - 1));
        return;
      }
      if (e.key === "ArrowUp" || (e.ctrlKey && e.key === "p")) {
        e.preventDefault();
        setSelectedIdx((i) => Math.max(i - 1, 0));
        return;
      }
      if (e.key === "Enter") {
        e.preventDefault();
        filteredItems[selectedIdx]?.execute();
        close();
        return;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, filteredItems, selectedIdx]);

  return (
    <Dialog open={open} onOpenChange={(v) => { if (!v) close(); }}>
      <DialogContent
        className="max-w-lg! w-full top-[45vh]! p-0! gap-0!"
        aria-describedby={undefined}
        showOverlay={false}
      >
        <DialogTitle className="sr-only">command</DialogTitle>
        <div className="flex px-2 h-9 items-center border-b font-mono text-sm">
          <span className="shrink-0 text-muted-foreground">{user?.name ?? "ghost"}</span>
          <span className="mx-1 text-muted-foreground">$</span>
          <input
            ref={inputRef}
            autoFocus
            className="flex-1 bg-transparent outline-none"
            value={input}
            onChange={(e) => setInput(e.target.value)}
          />
        </div>
        <div className="flex flex-col">
          {filteredItems.length === 0 ? (
            <span className="px-2 py-1 font-mono text-sm text-muted-foreground">
              no results
            </span>
          ) : (
            filteredItems.map((item, idx) => (
              <button
                key={item.label}
                type="button"
                className={`flex w-full px-2 py-0.5 font-mono text-sm cursor-pointer ${
                  idx === selectedIdx ? "bg-accent text-accent-foreground" : ""
                }`}
                onMouseEnter={() => setSelectedIdx(idx)}
                onClick={() => { item.execute(); close(); }}
              >
                <span className="truncate">{item.label}</span>
                <span className="ml-auto shrink-0 pl-4 text-xs text-muted-foreground">{item.type}</span>
              </button>
            ))
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
