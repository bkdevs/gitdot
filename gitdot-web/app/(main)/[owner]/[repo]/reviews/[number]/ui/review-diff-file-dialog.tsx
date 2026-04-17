"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { RepositoryDiffFileResource } from "gitdot-api";
import type { DiffData } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { DiffBody } from "../../../commits/[sha]/ui/diff-body";

export function ReviewDiffFileDialog({
  diff,
  data,
  open,
  setOpen,
}: {
  diff: RepositoryDiffFileResource;
  data: DiffData;
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const { path, lines_added, lines_removed, left_content, right_content } =
    diff;
  const isCreated = !left_content;
  const isDeleted = !right_content;

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="w-[80vw]! h-[80vh]! max-w-[80vw]! max-h-[80vh]! border-border! rounded-sm! p-0 gap-0 flex flex-col overflow-hidden"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <VisuallyHidden>
          <DialogTitle>{path}</DialogTitle>
        </VisuallyHidden>
        <div className="relative z-10 flex flex-row w-full h-7 shrink-0 items-center px-2 text-xs font-mono bg-sidebar border-b border-border select-none">
          <div className="flex flex-row items-center justify-between w-full gap-2">
            <span className="text-muted-foreground">{path}</span>
            {isCreated && <span className="text-green-600">created</span>}
            {isDeleted && <span className="text-red-600">deleted</span>}
            {!isCreated && !isDeleted && (
              <span className="flex flex-row font-mono select-none gap-1">
                <span className="text-green-600">+{lines_added}</span>
                <span className="text-red-600">-{lines_removed}</span>
              </span>
            )}
          </div>
        </div>
        <div className="flex-1 overflow-auto scrollbar-thin pr-px">
          <DiffBody data={data} layout="split" />
        </div>
      </DialogContent>
    </Dialog>
  );
}
