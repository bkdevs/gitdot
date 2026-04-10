"use client";

import { Settings } from "lucide-react";
import { QuestionMark } from "@/icons";
import { cn } from "@/util";

export function MainToolbar({ className }: { className?: string }) {
  return (
    <div className={cn("flex items-center", className)}>
      <ToolbarButton
        icon={Settings}
        label="Settings"
        onClick={() => window.dispatchEvent(new CustomEvent("openSettings"))}
      />
      <ToolbarButton
        icon={QuestionMark}
        label="Shortcuts"
        onClick={() => window.dispatchEvent(new CustomEvent("openShortcuts"))}
      />
    </div>
  );
}

function ToolbarButton({
  icon: Icon,
  label,
  onClick,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="size-6 flex items-center justify-center hover:bg-sidebar-accent transition-colors shrink-0"
    >
      <Icon className={cn("size-4")} />
      <span className="sr-only">{label}</span>
    </button>
  );
}
