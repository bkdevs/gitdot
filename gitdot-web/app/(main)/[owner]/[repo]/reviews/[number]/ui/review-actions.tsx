"use client";

import { Send, Trash2 } from "lucide-react";

export function ReviewActions() {
  return (
    <div className="shrink-0 flex flex-col border-t border-border">
      <button
        type="button"
        className="flex w-full h-8 items-center gap-1.5 px-2 border-b border-border text-xs text-primary bg-accent outline-none underline decoration-transparent hover:decoration-current transition-colors duration-200"
      >
        <Send className="size-3.5" />
        Publish
      </button>
      <button
        type="button"
        className="flex w-full h-8 items-center gap-1.5 px-2 text-xs text-destructive hover:bg-accent outline-none underline decoration-transparent hover:decoration-current transition-colors duration-200"
      >
        <Trash2 className="size-3.5" />
        Discard
      </button>
    </div>
  );
}
