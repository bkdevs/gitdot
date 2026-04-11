"use client";

import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import Link from "@/ui/link";

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

export function MainCommandBar() {
  const [open, setOpen] = useState(false);
  const [hovered, setHovered] = useState(false);
  const { user } = useUserContext();
  const router = useRouter();
  const [input, setInput] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);

  const name = user === undefined ? null : (user?.name ?? "ghost");

  const close = () => {
    setOpen(false);
    setInput("");
    setSelectedIdx(0);
  };

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Command",
          description: "Open command",
          keys: [" "],
          execute: () => setOpen(true),
        },
      ],
      [setOpen],
    ),
  );

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openCommandBar", handle);
    return () => window.removeEventListener("openCommandBar", handle);
  }, [setOpen]);

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
      } else if (e.ctrlKey && e.key === "u") {
        e.preventDefault();
        setInput("");
      } else if (e.key === "ArrowDown" || (e.ctrlKey && e.key === "n")) {
        e.preventDefault();
        setSelectedIdx((i) => Math.min(i + 1, filteredItems.length - 1));
      } else if (e.key === "ArrowUp" || (e.ctrlKey && e.key === "p")) {
        e.preventDefault();
        setSelectedIdx((i) => Math.max(i - 1, 0));
      } else if (e.key === "Tab") {
        e.preventDefault();
        const selected = filteredItems[selectedIdx];
        if (selected) setInput(selected.label);
      } else if (e.key === "Enter") {
        e.preventDefault();
        filteredItems[selectedIdx]?.execute();
        close();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, filteredItems, selectedIdx]);

  if (name === null) return null;

  return (
    <>
      {open && (
        <div className="fixed bottom-6 left-0 z-50 flex flex-col border-t border-r bg-background font-mono text-sm">
          {filteredItems.length === 0 ? (
            <span className="px-2 py-0.5 text-muted-foreground">no results</span>
          ) : (
            filteredItems.map((item, idx) => (
              <button
                key={item.label}
                type="button"
                className={`flex w-full items-center px-2 py-0.5 cursor-pointer ${
                  idx === selectedIdx ? "bg-accent text-accent-foreground" : ""
                }`}
                onMouseEnter={() => setSelectedIdx(idx)}
                onClick={() => { item.execute(); close(); }}
              >
                <span className="truncate">{item.label}</span>
                <span className="ml-auto shrink-0 pl-4 text-muted-foreground">
                  {item.type}
                </span>
              </button>
            ))
          )}
        </div>
      )}
      <span className="flex flex-1 items-center px-2 text-sm">
        <Link
          href={`/${user?.name ?? "ghost"}`}
          className="shrink-0 text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200"
        >
          {name}
        </Link>
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: space key handled via useShortcuts */}
        <span
          className={`flex flex-1 items-center cursor-pointer transition-colors duration-200 ${open ? "text-foreground" : "text-muted-foreground hover:text-foreground"}`}
          onClick={() => setOpen(true)}
          onMouseEnter={() => setHovered(true)}
          onMouseLeave={() => setHovered(false)}
        >
          <span className="mx-1">$</span>
          {open && (
            <input
              // biome-ignore lint/a11y/noAutofocus: intentional — opens on keypress
              autoFocus
              className="bg-transparent outline-none"
              style={{ width: `${input.length}ch`, caretColor: "transparent" }}
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Escape") {
                  e.stopPropagation();
                  close();
                }
              }}
            />
          )}
          {open || hovered ? (
            <span
              className="inline-block w-[7px] bg-foreground align-text-bottom"
              style={{ height: "12px", animation: hovered && !open ? "blink 1s step-end infinite" : "none" }}
            />
          ) : (
            <span className="text-foreground">_</span>
          )}
        </span>
      </span>
    </>
  );
}
