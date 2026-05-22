"use client";

import { useTheme } from "next-themes";
import { useEffect, useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";

const OPTIONS: { value: "system" | "light" | "dark"; label: string }[] = [
  { value: "system", label: "System" },
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

export function SettingsAppearance() {
  const { theme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);
  useEffect(() => setMounted(true), []);

  const currentValue = mounted ? (theme ?? "system") : "system";
  const currentLabel =
    OPTIONS.find((o) => o.value === currentValue)?.label ?? "System";

  return (
    <div className="divide-y divide-border">
      <div className="p-3">
        <p className="text-sm font-medium dark:font-normal">Theme</p>
        <p className="text-sm text-muted-foreground">
          Choose how gitdot looks. Pick System to follow your operating system
          or override it with Light or Dark.
        </p>
        <div className="flex justify-start mt-3">
          <DropdownMenu>
            <DropdownMenuTrigger className="text-sm underline underline-offset-2 cursor-pointer transition-colors text-muted-foreground hover:text-foreground">
              {currentLabel} <span className="text-[0.5em]">▼</span>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {OPTIONS.map((option) => (
                <DropdownMenuItem
                  key={option.value}
                  onSelect={() => setTheme(option.value)}
                  className="text-xs"
                >
                  {option.label}
                </DropdownMenuItem>
              ))}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
    </div>
  );
}
