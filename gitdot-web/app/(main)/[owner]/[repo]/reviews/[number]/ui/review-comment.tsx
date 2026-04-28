"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Pencil, Trash2 } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { cn } from "@/util";
import { UserImage } from "../../../../ui/user-image";
import { useReviewContext } from "../context";

type CommentProps = {
  comment: ReviewCommentResource;
  isActive: boolean;
  name: string;
  handleClick: () => void;
};

function CommentHeader({
  name,
  action,
}: {
  name: string;
  action?: React.ReactNode;
}) {
  return (
    <div className="flex items-center justify-between w-full">
      <span className="text-xs text-foreground">{name}</span>
      {action}
    </div>
  );
}

function CommentReply({
  commentId,
  isReplying,
  onClose,
}: {
  commentId: string;
  isReplying: boolean;
  onClose: () => void;
}) {
  const { addComment } = useReviewContext();
  const [reply, setReply] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (isReplying) textareaRef.current?.focus();
  }, [isReplying]);

  return (
    <div
      className={cn(
        "pl-6 overflow-hidden",
        isReplying ? "max-h-40" : "max-h-0",
      )}
    >
      <textarea
        ref={textareaRef}
        value={reply}
        onChange={(e) => setReply(e.target.value)}
        placeholder="reply..."
        className="text-sm text-foreground w-full resize-none bg-transparent outline-none field-sizing-content border-b border-black dark:border-white placeholder:text-muted-foreground"
        onKeyDown={(e) => {
          if (e.key === "Escape") {
            onClose();
          } else if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            const body = reply.trim();
            if (!body) return;
            addComment({ body, parent_id: commentId });
            setReply("");
            onClose();
          }
        }}
      />
    </div>
  );
}

function DraftComment({
  comment,
  isActive,
  name,
  handleClick,
}: CommentProps) {
  const { deleteDraftComment, updateDraftComment } = useReviewContext();

  function save() {
    updateDraftComment(comment.id, draftCommentBody);
    setIsEditing(false);
  }
  const [isEditing, setIsEditing] = useState(false);
  const [draftCommentBody, setDraftCommentBody] = useState(comment.body);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (!isActive) setIsEditing(false);
  }, [isActive]);

  useEffect(() => {
    if (isEditing) textareaRef.current?.focus();
  }, [isEditing]);

  const action = isActive ? (
    <div className="flex items-center gap-1.5">
      {isEditing ? (
        <>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              setDraftCommentBody(comment.body);
              setIsEditing(false);
            }}
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            cancel
          </button>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              save();
            }}
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            save
          </button>
        </>
      ) : (
        <>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              setDraftCommentBody(comment.body);
              setIsEditing(true);
            }}
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            edit
          </button>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              deleteDraftComment(comment.id);
            }}
            className="text-xs text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
          >
            delete
          </button>
        </>
      )}
    </div>
  ) : (
    <span className="text-xs text-muted-foreground italic">draft</span>
  );

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          className="group flex flex-col cursor-pointer py-1 select-none"
          onClick={handleClick}
        >
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <CommentHeader name={name} action={action} />
              <textarea
                ref={textareaRef}
                value={draftCommentBody}
                onChange={(e) => setDraftCommentBody(e.target.value)}
                readOnly={!isEditing}
                onClick={(e) => {
                  if (isEditing) e.stopPropagation();
                }}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault();
                    save();
                  }
                }}
                className={cn(
                  "text-sm text-foreground w-full resize-none bg-transparent outline-none field-sizing-content border-b transition-colors duration-150",
                  isEditing
                    ? "select-text border-black dark:border-white"
                    : "pointer-events-none border-transparent",
                )}
              />
            </div>
          </div>
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem>
          <Pencil />
          Edit
        </ContextMenuItem>
        <ContextMenuItem variant="destructive">
          <Trash2 />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

function UserComment({
  comment,
  isActive,
  name,
  handleClick,
}: CommentProps) {
  const [isReplying, setIsReplying] = useState(false);

  useEffect(() => {
    if (!isActive) setIsReplying(false);
  }, [isActive]);

  const action = isActive ? (
    <div className="flex items-center gap-1.5">
      <button
        type="button"
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        edit
      </button>
      <button
        type="button"
        className="text-xs text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
      >
        delete
      </button>
      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          setIsReplying(true);
        }}
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        reply
      </button>
    </div>
  ) : null;

  return (
    <div>
      <ContextMenu>
        <ContextMenuTrigger asChild>
          <div
            className="group flex flex-col cursor-pointer py-1 select-none"
            onClick={handleClick}
          >
            <div className="flex gap-1.5">
              <div className="pt-0.5">
                <UserImage userId={comment.author_id} px={18} />
              </div>
              <div className="flex flex-col flex-1 min-w-0">
                <CommentHeader name={name} action={action} />
                <span className="text-sm text-foreground">{comment.body}</span>
              </div>
            </div>
          </div>
        </ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuItem>
            <Pencil />
            Edit
          </ContextMenuItem>
          <ContextMenuItem variant="destructive">
            <Trash2 />
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
      <CommentReply
        commentId={comment.id}
        isReplying={isReplying}
        onClose={() => setIsReplying(false)}
      />
    </div>
  );
}

function ReviewerComment({
  comment,
  isActive,
  name,
  handleClick,
}: CommentProps) {
  const [isReplying, setIsReplying] = useState(false);

  useEffect(() => {
    if (!isActive) setIsReplying(false);
  }, [isActive]);

  const action = isActive ? (
    <div className="flex items-center gap-1.5">
      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          setIsReplying(true);
        }}
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        reply
      </button>
    </div>
  ) : null;

  return (
    <div>
      <ContextMenu>
        <ContextMenuTrigger asChild>
          <div
            className="group flex flex-col cursor-pointer py-1 select-none"
            onClick={handleClick}
          >
            <div className="flex gap-1.5">
              <div className="pt-0.5">
                <UserImage userId={comment.author_id} px={18} />
              </div>
              <div className="flex flex-col flex-1 min-w-0">
                <CommentHeader name={name} action={action} />
                <span className="text-sm text-foreground">{comment.body}</span>
              </div>
            </div>
          </div>
        </ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuItem>
            <Pencil />
            Edit
          </ContextMenuItem>
          <ContextMenuItem variant="destructive">
            <Trash2 />
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
      <CommentReply
        commentId={comment.id}
        isReplying={isReplying}
        onClose={() => setIsReplying(false)}
      />
    </div>
  );
}

export function ReviewComment({ comment }: { comment: ReviewCommentResource }) {
  const { draftComments, activeComment, setActiveComment, diffs } =
    useReviewContext();
  const { user } = useUserContext();
  const router = useRouter();
  const pathname = usePathname();

  const isDraft = draftComments.some((d) => d.id === comment.id);
  const isActive = activeComment?.id === comment.id;
  const isAuthor = comment.author_id === user?.id;

  const name = comment.author?.name ?? comment.author_id;

  function handleClick() {
    if (isActive) {
      setActiveComment(null);
      return;
    }
    setActiveComment(comment);
    const diff = diffs.find((d) => d.id === comment.diff_id);
    if (diff) router.push(`${pathname}?diff=${diff.position}`);
  }

  const props: CommentProps = {
    comment,
    isActive,
    name,
    handleClick,
  };

  if (isDraft) return <DraftComment {...props} />;
  if (isAuthor) return <UserComment {...props} />;
  return <ReviewerComment {...props} />;
}
