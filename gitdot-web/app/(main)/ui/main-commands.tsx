"use client";

import { useParams, usePathname, useRouter } from "next/navigation";
import { useTheme } from "next-themes";
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { signout } from "@/actions";
import Link from "@/ui/link";

type Command = {
  label: string;
  type: "cmd";
  execute: () => void;
};

export function MainCommands() {
  const { user } = useUserContext();
  const { refreshUser } = useUserContext();
  const [open, setOpen] = useState(false);
  const [hovered, setHovered] = useState(false);
  const [input, setInput] = useState("");
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [dropdownLeft, setDropdownLeft] = useState(0);

  const pathname = usePathname();
  const params = useParams();
  const router = useRouter();
  const { resolvedTheme, setTheme } = useTheme();

  const promptRef = useRef<HTMLSpanElement>(null);
  // biome-ignore lint/correctness/useExhaustiveDependencies: re-measure when breadcrumbs change
  useLayoutEffect(() => {
    if (promptRef.current) {
      setDropdownLeft(promptRef.current.getBoundingClientRect().left);
    }
  }, [pathname]);

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
        {
          name: "Home",
          description: "Go home",
          keys: ["h"],
          execute: () => {
            if (user) router.push(`/${user.name}`);
          },
        },
      ],
      [user, router],
    ),
  );

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openCommandBar", handle);
    return () => window.removeEventListener("openCommandBar", handle);
  }, []);

  const commands = useMemo<Command[]>(() => {
    if (!user) {
      return [
        {
          type: "cmd",
          label: "login",
          execute: () => window.dispatchEvent(new Event("toggleAuthDialog")),
        },
        {
          type: "cmd",
          label: "shortcuts",
          execute: () => window.dispatchEvent(new Event("openShortcuts")),
        },
        {
          type: "cmd",
          label: "toggle theme",
          execute: () => setTheme(resolvedTheme === "dark" ? "light" : "dark"),
        },
      ];
    }

    return [
      {
        type: "cmd",
        label: "home",
        execute: () => router.push(`/${user.name}`),
      },
      {
        type: "cmd",
        label: "repos",
        execute: () => window.dispatchEvent(new CustomEvent("openRepos")),
      },
      {
        type: "cmd",
        label: "settings",
        execute: () => window.dispatchEvent(new CustomEvent("openSettings")),
      },
      {
        type: "cmd",
        label: "shortcuts",
        execute: () => window.dispatchEvent(new Event("openShortcuts")),
      },
      {
        type: "cmd",
        label: "new repo",
        execute: () => window.dispatchEvent(new CustomEvent("openNewRepo")),
      },
      {
        type: "cmd",
        label: "new org",
        execute: () => window.dispatchEvent(new CustomEvent("openNewOrg")),
      },
      {
        type: "cmd",
        label: "migrate repo",
        execute: () => window.dispatchEvent(new CustomEvent("openMigrateRepo")),
      },
      {
        type: "cmd",
        label: "toggle theme",
        execute: () => setTheme(resolvedTheme === "dark" ? "light" : "dark"),
      },
      {
        type: "cmd",
        label: "logout",
        execute: async () => {
          await signout();
          refreshUser();
        },
      },
    ];
  }, [user, router, refreshUser, resolvedTheme, setTheme]);

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

  const segments = pathname.split("/").filter(Boolean);
  const pathLinks: React.ReactNode[] = [];
  segments.forEach((segment, index) => {
    let path = `/${segments.slice(0, index + 1).join("/")}`;
    if ("path" in params && index === 1) {
      path = `${path}/files`;
    }
    if (index > 0) {
      pathLinks.push(<span key={`sep-${segment}-${index}`}>/</span>);
    }
    pathLinks.push(
      <Link
        className="hover:underline"
        href={path}
        key={`segment-${segment}-${index}`}
        prefetch={true}
      >
        {segment}
      </Link>,
    );
  });

  return (
    <>
      {open && <div className="fixed inset-0 z-40" onClick={close} />}
      {open && (
        // manual math to align first char of input with first char of dropdown labels
        <div
          className="fixed bottom-7 z-50 flex flex-col border-x border-t bg-background font-mono text-sm"
          style={{ left: `${dropdownLeft + 5}px` }}
        >
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
        <span className="flex items-center text-foreground">{pathLinks}</span>
        <span
          className={`flex flex-1 items-center transition-colors duration-200 ${open ? "text-foreground cursor-default" : "text-muted-foreground hover:text-foreground cursor-pointer"}`}
          onClick={() => setOpen(true)}
          onMouseEnter={() => setHovered(true)}
          onMouseLeave={() => setHovered(false)}
        >
          <span ref={promptRef} className="mx-1.5">
            »
          </span>
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
