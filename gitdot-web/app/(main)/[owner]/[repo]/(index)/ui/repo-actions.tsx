"use client";

import { Bell, Download, Star } from "lucide-react";
import { useState } from "react";
import { cn } from "@/util";

export function RepoActions() {
  const [starred, setStarred] = useState(false);
  const [subscribed, setSubscribed] = useState(false);
  const [cloned, setCloned] = useState(false);

  return (
    <div className="flex border-b">
      <RepoActionButton
        icon={
          <Star className="size-3.5" fill={starred ? "currentColor" : "none"} />
        }
        label={starred ? "Starred" : "Star"}
        active={starred}
        onClick={() => setStarred((v) => !v)}
      />
      <RepoActionButton
        icon={
          <Bell
            className="size-3.5"
            fill={subscribed ? "currentColor" : "none"}
          />
        }
        label={subscribed ? "Subscribed" : "Subscribe"}
        active={subscribed}
        onClick={() => setSubscribed((v) => !v)}
      />
      <RepoActionButton
        icon={<Download className="size-3.5" />}
        label="Clone"
        active={cloned}
        primary
        onClick={() => setCloned((v) => !v)}
      />
    </div>
  );
}

function RepoActionButton({
  icon,
  label,
  active,
  primary,
  onClick,
}: {
  icon: React.ReactNode;
  label: string;
  active: boolean;
  primary?: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={cn(
        "flex-1 h-7 flex items-center justify-center gap-1.5 border-r last:border-r-0 text-xs font-mono cursor-default transition-colors focus:outline-none",
        primary
          ? "bg-primary text-primary-foreground hover:bg-primary/90"
          : active
            ? "bg-accent text-foreground"
            : "hover:bg-accent/50 text-muted-foreground",
      )}
    >
      {icon}
      <span>{label}</span>
    </button>
  );
}
