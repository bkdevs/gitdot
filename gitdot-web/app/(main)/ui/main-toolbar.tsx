"use client";

import { Settings } from "lucide-react";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { useUserContext } from "@/(main)/context/user";
import { QuestionMark } from "@/icons";
import Link from "@/ui/link";
import { cn } from "@/util";

export function MainToolbar({ className }: { className?: string }) {
  const { user } = useUserContext();

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
      <div className="size-8 border-l border-border flex items-center justify-center shrink-0">
        {user && (
          <Link href={`/${user.name}`} className="flex items-center justify-center">
            <UserImage user={user} px={20} />
          </Link>
        )}
      </div>
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
      className="size-8 border-l border-border flex items-center justify-center hover:bg-sidebar-accent transition-colors shrink-0"
    >
      <Icon className={cn("size-4")} />
      <span className="sr-only">{label}</span>
    </button>
  );
}
