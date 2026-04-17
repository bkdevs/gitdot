import { cn } from "@/util";

export function Loading({ className }: { className?: string }) {
  return (
    <div
      className={cn(
        "flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground",
        className,
      )}
    >
      loading...
    </div>
  );
}
