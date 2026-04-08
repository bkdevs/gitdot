import type { UserResource } from "gitdot-api";
import Image from "next/image";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/ui/tooltip";
import { formatDate, timeAgo } from "@/util/date";

export function SettingsProfile({
  user,
  location,
  onLocationChange,
}: {
  user: UserResource | null;
  location: string;
  onLocationChange: (v: string) => void;
}) {
  if (!user) return null;

  return (
    <div className="max-w-lg space-y-6">
      <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-0 items-end">
        <Tooltip>
          <TooltipTrigger asChild>
            <Image
              src="/paul-penguin.jpeg"
              alt="Profile picture"
              width={32}
              height={32}
              className="rounded-full mb-1.5 cursor-pointer"
            />
          </TooltipTrigger>
          <TooltipContent>Upload photo</TooltipContent>
        </Tooltip>
        <span className="text-sm font-semibold mb-1.5">{user.name}</span>
        <span className="text-sm text-muted-foreground">email</span>
        <span className="text-sm">{user.email}</span>
        <span className="text-sm text-muted-foreground">joined</span>
        <span className="text-sm text-muted-foreground">
          {formatDate(new Date(user.created_at))} (
          {timeAgo(new Date(user.created_at))})
        </span>
      </div>

      <div className="space-y-2">
        <p className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          links
        </p>
        <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 items-end">
          <span className="text-sm text-muted-foreground">website</span>
          <input
            className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
            placeholder="https://..."
          />
          <span className="text-sm text-muted-foreground">location</span>
          <input
            value={location}
            onChange={(e) => onLocationChange(e.target.value)}
            className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
            placeholder="city, country"
          />
        </div>
      </div>

      <div className="space-y-2">
        <p className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          README.md
        </p>
        <p className="text-sm">
          software engineer building tools for open-source maintainers. i care a
          lot about developer experience, fast feedback loops, and shipping
          things that actually work.
        </p>
        <p className="text-sm">
          currently working on gitdot — a github alternative built for
          maintainers who want more control. the stack is rust on the backend
          (axum + sqlx) and next.js on the frontend.
        </p>
      </div>
    </div>
  );
}
