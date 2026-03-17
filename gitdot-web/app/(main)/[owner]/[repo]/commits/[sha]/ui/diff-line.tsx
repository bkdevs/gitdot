"use client";

import { useState } from "react";
import {
  useDiffFile,
  useReviewComments,
} from "@/(main)/[owner]/[repo]/reviews/[number]/review-comment-context";
import { cn, timeAgo } from "@/util";

export function DiffLine({
  children,
  "data-line-number": lineNumber,
  "data-line-type": lineType,
  "data-side": dataSide,
}: {
  children: React.ReactNode;
  "data-line-number": number;
  "data-line-type": "sentinel" | "normal" | "added" | "removed";
  "data-side"?: "old" | "new";
}) {
  const ctx = useReviewComments();
  const filePath = useDiffFile();

  const side =
    dataSide ??
    (lineType === "added" ? "new" : lineType === "removed" ? "old" : "new");

  if (!ctx || !filePath || lineType === "sentinel") {
    return (
      <span
        className={cn(
          "inline-flex w-full",
          lineType === "added" && "bg-diff-green",
          lineType === "removed" && "bg-diff-red",
        )}
      >
        <span className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none">
          {lineType === "sentinel" ? ".." : lineNumber}
        </span>
        {children}
      </span>
    );
  }

  const lineComments = ctx.comments.filter(
    (c) =>
      c.file_path === filePath &&
      c.line_number === lineNumber &&
      c.side === side,
  );

  const isActiveInput =
    ctx.activeInput?.filePath === filePath &&
    ctx.activeInput?.lineNumber === lineNumber &&
    ctx.activeInput?.side === side;

  return (
    <DiffLineWithComments
      lineNumber={lineNumber}
      lineType={lineType}
      side={side}
      filePath={filePath}
      comments={lineComments}
      isActiveInput={isActiveInput}
      canComment={ctx.canComment}
      onClickAdd={() => ctx.setActiveInput({ filePath, lineNumber, side })}
      onSubmit={(body) => {
        ctx.addComment(filePath, lineNumber, side, body);
        ctx.setActiveInput(null);
      }}
      onCancel={() => ctx.setActiveInput(null)}
    >
      {children}
    </DiffLineWithComments>
  );
}

function DiffLineWithComments({
  children,
  lineNumber,
  lineType,
  comments,
  isActiveInput,
  canComment,
  onClickAdd,
  onSubmit,
  onCancel,
}: {
  children: React.ReactNode;
  lineNumber: number;
  lineType: "normal" | "added" | "removed";
  side: "old" | "new";
  filePath: string;
  comments: { author_name: string; body: string; created_at: string }[];
  isActiveInput: boolean;
  canComment: boolean;
  onClickAdd: () => void;
  onSubmit: (body: string) => void;
  onCancel: () => void;
}) {
  const [inputValue, setInputValue] = useState("");
  const hasComments = comments.length > 0 || isActiveInput;

  return (
    <div className="group/line flex flex-col w-full">
      <span
        className={cn(
          "inline-flex w-full relative",
          lineType === "added" && "bg-diff-green",
          lineType === "removed" && "bg-diff-red",
        )}
      >
        <span className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none relative">
          {lineNumber}
          {canComment && !isActiveInput && (
            <button
              type="button"
              className="absolute left-0 top-0 size-full hidden group-hover/line:flex items-center justify-center text-blue-500 hover:text-blue-400 font-bold text-xs cursor-pointer"
              onClick={onClickAdd}
            >
              +
            </button>
          )}
        </span>
        {children}
      </span>

      {hasComments && (
        <div className="w-full bg-background border-y border-border px-3 py-2 flex flex-col gap-1.5 font-sans text-xs">
          {comments.map((c, i) => (
            <div key={`${c.created_at}-${i}`} className="flex flex-col gap-0.5">
              <div className="flex items-center gap-2">
                <span className="font-medium text-foreground">
                  {c.author_name}
                </span>
                <span className="text-muted-foreground">
                  {timeAgo(new Date(c.created_at))}
                </span>
              </div>
              <p className="text-muted-foreground">{c.body}</p>
            </div>
          ))}

          {isActiveInput && (
            <input
              className="w-full bg-transparent border border-border rounded px-2 py-1 text-sm outline-none focus:border-ring font-sans"
              type="text"
              placeholder="Write a comment..."
              autoFocus
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onBlur={() => {
                setInputValue("");
                onCancel();
              }}
              onKeyDown={(e) => {
                if (e.key === "Escape") {
                  setInputValue("");
                  onCancel();
                } else if (e.key === "Enter" && inputValue.trim()) {
                  onSubmit(inputValue.trim());
                  setInputValue("");
                }
              }}
            />
          )}
        </div>
      )}
    </div>
  );
}
