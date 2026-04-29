"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Pencil, Trash2 } from "lucide-react";
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
};

export function ReviewComment({ comment }: { comment: ReviewCommentResource }) {
  const { draftComments, activeComment } = useReviewContext();
  const { user } = useUserContext();

  const isDraft = draftComments.some((d) => d.id === comment.id);
  const isActive = activeComment?.id === comment.id;
  const isAuthor = comment.author_id === user?.id;
  const name = comment.author?.name ?? comment.author_id;

  const props: CommentProps = { comment, isActive, name };

  if (isDraft) return <DraftComment {...props} />;
  if (isAuthor) return <UserComment {...props} />;
  return <ReviewerComment {...props} />;
}

function DraftComment({ comment, isActive, name }: CommentProps) {
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

  const action = isEditing ? (
    <div className="flex items-center gap-1.5">
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
    </div>
  ) : (
    <div className="relative flex items-center">
      <span className="absolute inset-0 flex items-center justify-end text-xs text-muted-foreground italic pointer-events-none transition-opacity duration-200 opacity-100 group-hover:opacity-0">
        draft
      </span>
      <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
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
      </div>
    </div>
  );

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div className="group flex flex-col py-0.5">
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

function UserComment({ comment, name }: CommentProps) {
  const { updateComment } = useReviewContext();
  const [isEditing, setIsEditing] = useState(false);
  const [commentBody, setCommentBody] = useState(comment.body);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (isEditing) textareaRef.current?.focus();
  }, [isEditing]);

  useEffect(() => {
    if (!isEditing) setCommentBody(comment.body);
  }, [comment.body, isEditing]);

  async function save() {
    await updateComment(comment.id, commentBody);
    setIsEditing(false);
  }

  const action = isEditing ? (
    <div className="flex items-center gap-1.5">
      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          setCommentBody(comment.body);
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
    </div>
  ) : (
    <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
      <button
        type="button"
        onClick={(e) => {
          e.stopPropagation();
          setCommentBody(comment.body);
          setIsEditing(true);
        }}
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        edit
      </button>
    </div>
  );

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div className="group flex flex-col py-0.5">
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <CommentHeader name={name} action={action} />
              <textarea
                ref={textareaRef}
                value={commentBody}
                onChange={(e) => setCommentBody(e.target.value)}
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

function ReviewerComment({ comment, name }: CommentProps) {
  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div className="group flex flex-col py-0.5">
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <CommentHeader name={name} />
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
  );
}

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
