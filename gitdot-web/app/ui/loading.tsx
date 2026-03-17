import type { ComponentProps } from "react";
import { cn } from "@/util";

export function Loading(props: ComponentProps<"div">) {
  return (
    <div
      className={cn(
        "flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground",
        props.className,
      )}
      {...props}
    >
      loading...
    </div>
  );
}
