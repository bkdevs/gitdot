"use client";

import type { OrganizationResource, RepositoryResource } from "gitdot-api";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { useCommands } from "@/(main)/hooks/use-commands";
import { useTypewriter } from "@/hooks/use-typewriter";
import Link from "@/ui/link";

export function MainCommandBar() {
  const { user, repositories, organizations } = useUserContext();
  const username = user === undefined ? null : (user?.name ?? "ghost");

  const typed = useTypewriter(username ?? "", 35);
  const [done, setDone] = useState(false);

  // biome-ignore lint/correctness/useExhaustiveDependencies: trigger typewriter when username changes
  useEffect(() => {
    setDone(false);
  }, [username]);

  useEffect(() => {
    if (!username || typed !== username) return;
    const t = setTimeout(() => setDone(true), 60);
    return () => clearTimeout(t);
  }, [typed, username]);

  if (username === null) return null;

  if (!done) {
    return (
      <span className="flex flex-1 items-center px-2 text-sm text-muted-foreground">
        {typed}
      </span>
    );
  }

  return (
    <CommandBar
      username={username}
      user={user ?? null}
      repositories={repositories}
      organizations={organizations}
    />
  );
}

function CommandBar({
  username,
  user,
  repositories,
  organizations,
}: {
  username: string;
  user: { name: string } | null;
  repositories: RepositoryResource[] | null | undefined;
  organizations: OrganizationResource[] | null | undefined;
}) {
  const [open, setOpen] = useState(false);
  const [hovered, setHovered] = useState(false);
  const [input, setInput] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);

  const close = useCallback(() => {
    setOpen(false);
    setInput("");
    setSelectedIdx(0);
  }, []);

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Command",
          description: "Open command",
          keys: [";", ":", "Mod+k", "Mod+x"],
          execute: () => setOpen(true),
        },
      ],
      [],
    ),
  );

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openCommandBar", handle);
    return () => window.removeEventListener("openCommandBar", handle);
  }, []);

  const commands = useCommands({ user, repositories, organizations });

  const filteredCommands = useMemo(() => {
    const q = input.trim().toLowerCase();
    if (!q) return commands;
    return commands.filter((item) => item.label.toLowerCase().includes(q));
  }, [commands, input]);

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
        setSelectedIdx((i) => Math.min(i + 1, filteredCommands.length - 1));
      } else if (e.key === "ArrowUp" || (e.ctrlKey && e.key === "p")) {
        e.preventDefault();
        setSelectedIdx((i) => Math.max(i - 1, 0));
      } else if (e.key === "Tab") {
        e.preventDefault();
        const selected = filteredCommands[selectedIdx];
        if (selected) setInput(selected.label);
      } else if (e.key === "Enter") {
        e.preventDefault();
        filteredCommands[selectedIdx]?.execute();
        close();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, filteredCommands, selectedIdx, close]);

  return (
    <>
      {open && <div className="fixed inset-0 z-40" onClick={close} />}
      {open && (
        <div className="fixed bottom-6 left-0 z-50 flex flex-col border-t border-r bg-background font-mono text-sm">
          {filteredCommands.length === 0 ? (
            <span className="px-2 py-0.5 text-muted-foreground">
              no results
            </span>
          ) : (
            filteredCommands.map((item, idx) => (
              <button
                key={item.label}
                type="button"
                className={`flex w-full items-center px-2 py-0.5 cursor-pointer ${
                  idx === selectedIdx ? "bg-accent text-accent-foreground" : ""
                }`}
                onMouseEnter={() => setSelectedIdx(idx)}
                onClick={() => {
                  item.execute();
                  close();
                }}
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
        {user ? (
          <Link
            href={`/${user.name}`}
            className="shrink-0 cursor-pointer text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200"
          >
            {username}
          </Link>
        ) : (
          <button
            type="button"
            onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
            className="shrink-0 cursor-pointer text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200"
          >
            {username}
          </button>
        )}
        <span
          className={`flex flex-1 items-center transition-colors duration-200 ${open ? "text-foreground cursor-default" : "text-muted-foreground hover:text-foreground cursor-pointer"}`}
          onClick={() => setOpen(true)}
          onMouseEnter={() => setHovered(true)}
          onMouseLeave={() => setHovered(false)}
        >
          <span className="mx-1">$</span>
          {open && (
            <input
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
              style={{
                height: "12px",
                animation:
                  hovered && !open ? "blink 1s step-end infinite" : "none",
              }}
            />
          ) : (
            <span className="text-foreground">_</span>
          )}
        </span>
      </span>
    </>
  );
}
