"use client";

import { useCallback, useRef, useState } from "react";
import { DiffCreated } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-created";
import { DiffSplit } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-split";
import { DiffUnified } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-unified";
import { DiffUnilateral } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-unilateral";
import { preferSplit } from "@/(main)/[owner]/[repo]/util";
import type { DiffData } from "@/actions";
import { cn } from "@/util";

const getTokenSpan = (target: EventTarget | null): HTMLElement | null => {
  if (!(target instanceof HTMLElement)) return null;
  const direct = target.closest(".diff-token");
  if (direct) return direct as HTMLElement;
  const line = target.closest(".diff-line");
  if (!line) return null;
  const tokens = line.querySelectorAll<HTMLElement>(".diff-token");
  return tokens.length ? tokens[tokens.length - 1] : null;
};

export function ReviewDiffFileBody({
  data,
  layout = "heuristic",
  className,
}: {
  data: DiffData;
  layout?: "split" | "unified" | "heuristic";
  className?: string;
}) {
  const containerRef = useRef<HTMLDivElement>(null);
  const startSpanRef = useRef<HTMLElement | null>(null);
  const allSpansRef = useRef<HTMLElement[]>([]);

  const [commentOpen, setCommentOpen] = useState(false);
  const [comment, setComment] = useState("");
  const [commentPos, setCommentPos] = useState({ x: 0, y: 0 });

  const clearSelection = useCallback(() => {
    const container = containerRef.current;
    if (!container) return;

    container
      .querySelectorAll(".token-selected")
      .forEach((el) => el.classList.remove("token-selected"));
    container.classList.remove("has-selection");
    setCommentOpen(false);
    setComment("");
  }, []);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      clearSelection();
      containerRef.current?.classList.add("is-dragging");
      startSpanRef.current = null;
      allSpansRef.current = Array.from(
        containerRef.current?.querySelectorAll(".diff-token") ?? [],
      ) as HTMLElement[];

      const token = getTokenSpan(e.target);
      if (token) {
        e.preventDefault();
        startSpanRef.current = token;
        token.classList.add("token-selected");
        containerRef.current?.classList.add("has-selection");
      }
    },
    [clearSelection],
  );

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!(e.buttons & 1)) return;
    if (!startSpanRef.current) return;

    const token = getTokenSpan(e.target);
    if (!token) return;

    const spans = allSpansRef.current;
    const startIdx = spans.indexOf(startSpanRef.current);
    const endIdx = spans.indexOf(token);
    if (startIdx === -1 || endIdx === -1) return;

    const [from, to] =
      startIdx <= endIdx ? [startIdx, endIdx] : [endIdx, startIdx];
    for (let i = 0; i < spans.length; i++) {
      spans[i].classList.toggle("token-selected", i >= from && i <= to);
    }
  }, []);

  const handleMouseUp = useCallback((e: React.MouseEvent) => {
    containerRef.current?.classList.remove("is-dragging");
    if (startSpanRef.current !== null) {
      setCommentPos({ x: e.clientX, y: e.clientY });
      setCommentOpen(true);
    }
    startSpanRef.current = null;
  }, []);

  const useSplit =
    data.kind === "split" &&
    (layout === "split" ||
      (layout === "heuristic" &&
        preferSplit(data.leftSpans, data.rightSpans, data.hunks)));

  return (
    <div
      ref={containerRef}
      className={cn(
        "w-full cursor-default select-none relative",
        "[&.is-dragging_.diff-token]:cursor-default",
        "[&.has-selection_.diff-token:not(.token-selected)]:opacity-40",
        "[&.has-selection_.diff-token:not(.token-selected)]:transition-opacity",
        "[&.has-selection_.diff-token:not(.token-selected)]:duration-200",
        "[&.has-selection_.diff-token.token-selected]:opacity-100",
        "[&.has-selection_.diff-token.token-selected]:transition-opacity",
        "[&.has-selection_.diff-token.token-selected]:duration-200",
        className,
      )}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
    >
      {data.kind === "split" &&
        (useSplit ? (
          <DiffSplit
            leftSpans={data.leftSpans}
            rightSpans={data.rightSpans}
            hunks={data.hunks}
          />
        ) : (
          <DiffUnified
            leftSpans={data.leftSpans}
            rightSpans={data.rightSpans}
            hunks={data.hunks}
          />
        ))}
      {data.kind === "unilateral" && (
        <DiffUnilateral
          spans={data.spans}
          hunks={data.hunks}
          side={data.side}
        />
      )}
      {data.kind === "created" && <DiffCreated spans={data.spans} />}
      {data.kind === "deleted" && (
        <div className="text-sm font-mono px-2 text-primary/50">
          File deleted.
        </div>
      )}
      {(!data || data.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
      {commentOpen && (
        <>
          <div className="fixed inset-0 z-40" onMouseDown={clearSelection} />
          <div
            className="fixed z-50 w-64 bg-background border border-border shadow-md overflow-hidden"
            style={{ top: commentPos.y + 12, left: commentPos.x + 12 }}
            onMouseDown={(e) => e.stopPropagation()}
          >
            <textarea
              autoFocus
              className="w-full p-2 text-xs bg-background outline-none resize-none min-h-16"
              placeholder="Leave a comment…"
              value={comment}
              onChange={(e) => setComment(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter" && !e.shiftKey && comment.trim()) {
                  e.preventDefault();
                  clearSelection();
                }
                if (e.key === "Escape") clearSelection();
              }}
            />
          </div>
        </>
      )}
    </div>
  );
}
