"use client";

import type { RepositoryDiffFileResource } from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { useCallback, useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import type { DiffSpans } from "@/actions";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileBubbles } from "./review-diff-file-bubbles";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diffId,
  revisionId,
  diffFile,
  diffSpans,
}: {
  diffId: string;
  revisionId: string;
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
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
        <ReviewDiffFileHeader
          diffFile={diffFile}
          onClick={() => setOpen(true)}
        />
        <ContextMenu>
          <ContextMenuTrigger asChild>
            <div>
              <ReviewDiffFileBody
                diffId={diffId}
                revisionId={revisionId}
                diffFile={diffFile}
                diffSpans={diffSpans}
                onBubble={handleBubble}
              />
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
      <ReviewDiffFileBubbles bubbleTop={bubbleTop} userId={user?.id} />
      <ReviewDiffFileDialog
        diff={diffFile}
        spans={diffSpans}
        open={open}
        setOpen={setOpen}
      />
    </div>
  );
}
