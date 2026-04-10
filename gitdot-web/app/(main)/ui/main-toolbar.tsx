"use client";

import { Files, History, Settings } from "lucide-react";
import { usePathname } from "next/navigation";
import { QuestionMark } from "@/icons";
import { cn } from "@/util";

export function MainToolbar({ className }: { className?: string }) {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);
  const isRepoPage = segments.length >= 2;

  return (
    <div className={cn("flex items-center gap-1", className)}>
      <ToolbarButton
        icon={History}
        label="History"
        onClick={() => window.dispatchEvent(new Event("openHistoryDialog"))}
      />
      {isRepoPage && (
        <ToolbarButton
          icon={Files}
          label="Files"
          onClick={() => window.dispatchEvent(new Event("openFileSearch"))}
        />
      )}
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
      className="size-5 flex items-center justify-center hover:bg-sidebar-accent transition-colors shrink-0"
    >
      <Icon className="size-3.5" />
      <span className="sr-only">{label}</span>
    </button>
  );
}
