"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export interface Shortcut {
  name: string;
  description: string;
  keys: string[];
  execute: () => void;
}

interface ShortcutsContext {
  register: (shortcuts: Shortcut[]) => () => void;
}
const ShortcutsContext = createContext<ShortcutsContext | null>(null);

function isInputFocused(): boolean {
  const el = document.activeElement;
  if (!el) return false;
  const tag = el.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if ((el as HTMLElement).isContentEditable) return true;
  return false;
}

function isRadixModalOpen(): boolean {
  return !!document.querySelector(
    ['[aria-modal="true"]', '[role="dialog"][data-state="open"]'].join(","),
  );
}

function mergeShortcuts(
  registry: Map<number, Shortcut[]>,
): Map<string, Shortcut> {
  const merged = new Map<string, Shortcut>();
  for (const id of [...registry.keys()].sort((a, b) => a - b)) {
    for (const shortcut of registry.get(id) ?? []) {
      for (const key of shortcut.keys) {
        merged.set(key, shortcut);
      }
    }
  }
  return merged;
}

const helpShortcut: Shortcut = {
  name: "Help",
  description: "Shortcuts",
  keys: ["?"],
  execute: () => {},
};

export function ShortcutsProvider({ children }: { children: React.ReactNode }) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const registryRef = useRef<Map<number, Shortcut[]>>(new Map());
  const counterRef = useRef(0);
  const merged = useRef<Map<string, Shortcut>>(new Map());

  const register = useCallback((shortcuts: Shortcut[]): (() => void) => {
    const id = ++counterRef.current;
    registryRef.current.set(id, shortcuts);
    merged.current = mergeShortcuts(registryRef.current);

    return () => {
      registryRef.current.delete(id);
      merged.current = mergeShortcuts(registryRef.current);
    };
  }, []);

  useEffect(() => {
    function handleOpenShortcuts() {
      setDialogOpen(true);
    }
    window.addEventListener("openShortcuts", handleOpenShortcuts);
    return () =>
      window.removeEventListener("openShortcuts", handleOpenShortcuts);
  }, []);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (
        event.ctrlKey ||
        event.altKey ||
        event.metaKey ||
        event.defaultPrevented ||
        isInputFocused() ||
        isRadixModalOpen()
      ) {
        return;
      }

      if (event.key === "?") {
        event.preventDefault();
        setDialogOpen(true);
        return;
      }

      const shortcut = merged.current.get(event.key);
      if (!shortcut) return;

      event.preventDefault();
      shortcut.execute();
    }
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const allShortcuts = [...registryRef.current.values()]
    .flat()
    .concat(helpShortcut);

  return (
    <ShortcutsContext value={{ register }}>
      {children}
      <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent
          className="max-w-lg! w-full p-0! top-[40vh]!"
          aria-describedby={undefined}
          showOverlay={false}
        >
          <DialogTitle className="absolute -top-2 left-2 bg-background px-1 font-mono text-xs">
            shortcuts
          </DialogTitle>
          <div className="grid grid-cols-2 gap-x-8 gap-y-4 font-mono p-4">
            {allShortcuts.map((s) => (
              <div
                key={s.name}
                className="flex items-baseline justify-between gap-2"
              >
                <div>
                  <span className="text-sm">{s.name}</span>
                  <p className="text-xs">{s.description}</p>
                </div>
                <div className="flex items-baseline gap-1 shrink-0">
                  {s.keys.map((k, i) => (
                    <kbd key={k} className="text-sm bg-muted px-1 rounded-xs">
                      {k}
                      {i < s.keys.length - 1 ? "," : ""}
                    </kbd>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </DialogContent>
      </Dialog>
    </ShortcutsContext>
  );
}

export function useShortcuts(shortcuts: Shortcut[]): void {
  const ctx = useContext(ShortcutsContext);
  if (!ctx) {
    throw new Error("useShortcuts must be used within ShortcutsProvider");
  }

  const { register } = ctx;
  useEffect(() => {
    const unregister = register(shortcuts);
    return unregister;
  }, [register, shortcuts]);
}
