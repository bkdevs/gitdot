"use client";

import type { RepositoryDiffFileResource } from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { useCallback, useRef, useState } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { useUserContext } from "@/(main)/context/user";
import type { DiffData } from "@/actions";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diff,
  data,
}: {
  diff: RepositoryDiffFileResource;
  data: DiffData;
}) {
  const { user } = useUserContext();
  const [open, setOpen] = useState(false);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const [bubbleTop, setBubbleTop] = useState<number | null>(null);

  const handleBubble = useCallback((viewportTop: number | null) => {
    if (viewportTop === null) {
      setBubbleTop(null);
      return;
    }
    const rect = wrapperRef.current?.getBoundingClientRect();
    setBubbleTop(rect ? viewportTop - rect.top : null);
  }, []);

  return (
    <div ref={wrapperRef} className="relative">
      <div
        data-diff-file
        className="rounded-sm border border-border overflow-hidden"
      >
        <ReviewDiffFileHeader diff={diff} onClick={() => setOpen(true)} />
        <ContextMenu>
          <ContextMenuTrigger asChild>
            <div>
              <ReviewDiffFileBody data={data} onBubble={handleBubble} />
            </div>
          </ContextMenuTrigger>
          <ContextMenuContent>
            <ContextMenuItem onSelect={() => setOpen(true)}>
              <Maximize2 />
              Expand
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </div>
      {bubbleTop !== null && (
        <div
          className="absolute z-50 flex flex-row items-center gap-1.5 left-full ml-2 px-2 py-0.5 bg-background"
          style={{ top: bubbleTop }}
        >
          <UserImage userId={user?.id} px={16} />
          <span className="text-xs font-sans text-muted-foreground">1</span>
        </div>
      )}
      <ReviewDiffFileDialog
        diff={diff}
        data={data}
        open={open}
        setOpen={setOpen}
      />
    </div>
  );
}
