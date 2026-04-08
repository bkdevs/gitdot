"use client";

import { cn } from "@/util";

export type SettingsTab = "profile" | "theme" | "shortcuts";

const GROUPS: { label: string; tabs: { id: SettingsTab; label: string }[] }[] = [
  {
    label: "General",
    tabs: [
      { id: "profile", label: "/profile" },
      { id: "theme", label: "/theme" },
      { id: "shortcuts", label: "/shortcuts" },
    ],
  },
];

export function SettingsSidebar({
  tab,
  onTabChange,
}: {
  tab: SettingsTab;
  onTabChange: (tab: SettingsTab) => void;
}) {
  return (
    <nav className="w-48 shrink-0 border-r border-border overflow-y-auto">
      {GROUPS.map((group) => (
        <div key={group.label}>
          <p className="px-4 pt-3 pb-1 text-xs text-muted-foreground">{group.label}</p>
          {group.tabs.map(({ id, label }) => (
            <button
              key={id}
              type="button"
              onClick={() => onTabChange(id)}
              className={cn(
                "w-full text-left px-4 h-7 flex items-center outline-none ring-0 underline underline-offset-2 transition-colors duration-200 cursor-pointer",
                tab === id ? "decoration-current" : "decoration-transparent hover:decoration-current",
              )}
            >
              {label}
            </button>
          ))}
        </div>
      ))}
    </nav>
  );
}
