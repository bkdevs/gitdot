"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
} from "react";

export type ShortcutKey = string;

export interface ShortcutCommand {
  name: string;
  execute: () => void;
}

export type ShortcutMap = Record<ShortcutKey, ShortcutCommand>;

interface ShortcutsContext {
  register: (map: ShortcutMap) => () => void;
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

function mergeMaps(registry: Map<number, ShortcutMap>): ShortcutMap {
  const merged: ShortcutMap = {};
  for (const id of [...registry.keys()].sort((a, b) => a - b)) {
    Object.assign(merged, registry.get(id));
  }
  return merged;
}

export function ShortcutsProvider({ children }: { children: React.ReactNode }) {
  const registryRef = useRef<Map<number, ShortcutMap>>(new Map());
  const counterRef = useRef(0);
  const merged = useRef<ShortcutMap>({});

  const register = useCallback((map: ShortcutMap): (() => void) => {
    const id = ++counterRef.current;
    registryRef.current.set(id, map);
    merged.current = mergeMaps(registryRef.current);

    return () => {
      registryRef.current.delete(id);
      merged.current = mergeMaps(registryRef.current);
    };
  }, []);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.ctrlKey || event.altKey || event.metaKey) return;
      if (isInputFocused()) return;
      const command = merged.current[event.key];
      if (command) {
        event.preventDefault();
        command.execute();
      }
    }
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  return <ShortcutsContext value={{ register }}>{children}</ShortcutsContext>;
}

export function useShortcuts(map: ShortcutMap): void {
  const ctx = useContext(ShortcutsContext);
  if (!ctx) {
    throw new Error("useShortcuts must be used within ShortcutsProvider");
  }

  const { register } = ctx;
  useEffect(() => {
    const unregister = register(map);
    return unregister;
  }, [register, map]);
}
