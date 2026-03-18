"use client";

import { useState } from "react";
import {
  type DraftComment,
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
      activeReplyToId={
        isActiveInput ? (ctx.activeInput?.replyToId ?? null) : null
      }
      canComment={ctx.canComment}
      onClickAdd={() => ctx.setActiveInput({ filePath, lineNumber, side })}
      onSubmit={(body, parentId) => {
        ctx.addComment(filePath, lineNumber, side, body, parentId);
        ctx.setActiveInput(null);
      }}
      onClickReply={(commentId) =>
        ctx.setActiveInput({ filePath, lineNumber, side, replyToId: commentId })
      }
      onCancel={() => ctx.setActiveInput(null)}
    >
      {children}
    </DiffLineWithComments>
  );
}

interface CommentThread {
  root: DraftComment;
  replies: DraftComment[];
}

function groupIntoThreads(comments: DraftComment[]): CommentThread[] {
  const topLevel: DraftComment[] = [];
  const repliesByParent = new Map<string, DraftComment[]>();

  for (const c of comments) {
    if (c.parent_id) {
      const list = repliesByParent.get(c.parent_id) ?? [];
      list.push(c);
      repliesByParent.set(c.parent_id, list);
    } else {
      topLevel.push(c);
    }
  }

  return topLevel.map((root) => ({
    root,
    replies: root.id ? (repliesByParent.get(root.id) ?? []) : [],
  }));
}

function DiffLineWithComments({
  children,
  lineNumber,
  lineType,
  comments,
  isActiveInput,
  activeReplyToId,
  canComment,
  onClickAdd,
  onSubmit,
  onClickReply,
  onCancel,
}: {
  children: React.ReactNode;
  lineNumber: number;
  lineType: "normal" | "added" | "removed";
  side: "old" | "new";
  filePath: string;
  comments: DraftComment[];
  isActiveInput: boolean;
  activeReplyToId: string | null;
  canComment: boolean;
  onClickAdd: () => void;
  onSubmit: (body: string, parentId?: string) => void;
  onClickReply: (commentId: string) => void;
  onCancel: () => void;
}) {
  const [inputValue, setInputValue] = useState("");
  const threads = groupIntoThreads(comments);
  const isNewCommentInput = isActiveInput && !activeReplyToId;
  const hasComments = threads.length > 0 || isNewCommentInput;

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
        <div className="w-full bg-background border-y border-border px-3 py-2 flex flex-col gap-2 font-sans text-xs">
          {threads.map((thread) => (
            <CommentThreadView
              key={thread.root.id ?? thread.root.created_at}
              thread={thread}
              canComment={canComment}
              isReplyActive={activeReplyToId === thread.root.id}
              onClickReply={() => {
                if (thread.root.id) onClickReply(thread.root.id);
              }}
              onSubmitReply={(body) => {
                if (thread.root.id) onSubmit(body, thread.root.id);
              }}
              onCancel={onCancel}
            />
          ))}

          {isNewCommentInput && (
            <CommentInputField
              value={inputValue}
              onChange={setInputValue}
              onSubmit={() => {
                if (inputValue.trim()) {
                  onSubmit(inputValue.trim());
                  setInputValue("");
                }
              }}
              onCancel={() => {
                setInputValue("");
                onCancel();
              }}
              placeholder="Write a comment..."
              autoFocus
            />
          )}
        </div>
      )}
    </div>
  );
}

function CommentThreadView({
  thread,
  canComment,
  isReplyActive,
  onClickReply,
  onSubmitReply,
  onCancel,
}: {
  thread: CommentThread;
  canComment: boolean;
  isReplyActive: boolean;
  onClickReply: () => void;
  onSubmitReply: (body: string) => void;
  onCancel: () => void;
}) {
  const [replyValue, setReplyValue] = useState("");

  return (
    <div className="flex flex-col gap-1">
      <CommentBubble
        comment={thread.root}
        canReply={canComment && thread.root.id !== null}
        onClickReply={onClickReply}
      />

      {thread.replies.length > 0 && (
        <div className="ml-4 border-l border-border pl-3 flex flex-col gap-1">
          {thread.replies.map((reply, i) => (
            <CommentBubble
              key={reply.id ?? `${reply.created_at}-${i}`}
              comment={reply}
              canReply={false}
              onClickReply={() => {}}
            />
          ))}
        </div>
      )}

      {isReplyActive && (
        <div className="ml-4 pl-3">
          <CommentInputField
            value={replyValue}
            onChange={setReplyValue}
            onSubmit={() => {
              if (replyValue.trim()) {
                onSubmitReply(replyValue.trim());
                setReplyValue("");
              }
            }}
            onCancel={() => {
              setReplyValue("");
              onCancel();
            }}
            placeholder="Write a reply..."
            autoFocus
          />
        </div>
      )}
    </div>
  );
}

function CommentBubble({
  comment,
  canReply,
  onClickReply,
}: {
  comment: DraftComment;
  canReply: boolean;
  onClickReply: () => void;
}) {
  return (
    <div className="flex flex-col gap-0.5 group/comment">
      <div className="flex items-center gap-2">
        <span className="font-medium text-foreground">
          {comment.author_name}
        </span>
        <span className="text-muted-foreground">
          {timeAgo(new Date(comment.created_at))}
        </span>
        {canReply && (
          <button
            type="button"
            className="text-muted-foreground hover:text-foreground opacity-0 group-hover/comment:opacity-100 transition-opacity cursor-pointer"
            onClick={onClickReply}
          >
            reply
          </button>
        )}
      </div>
      <p className="text-muted-foreground">{comment.body}</p>
    </div>
  );
}

function CommentInputField({
  value,
  onChange,
  onSubmit,
  onCancel,
  placeholder,
  autoFocus,
}: {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  onCancel: () => void;
  placeholder: string;
  autoFocus?: boolean;
}) {
  return (
    <input
      className="w-full bg-transparent border border-border rounded px-2 py-1 text-sm outline-none focus:border-ring font-sans"
      type="text"
      placeholder={placeholder}
      autoFocus={autoFocus}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      onBlur={() => {
        onChange("");
        onCancel();
      }}
      onKeyDown={(e) => {
        if (e.key === "Escape") {
          onChange("");
          onCancel();
        } else if (e.key === "Enter" && value.trim()) {
          onSubmit();
        }
      }}
    />
  );
}
