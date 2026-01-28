"use client";

import { ChevronUp } from "lucide-react";
import { useState } from "react";
import { TriangleUp } from "@/lib/icons";
import { timeAgo } from "@/util";

export function Comments() {
  const [showCommentInput, setShowCommentInput] = useState(false);

  return (
    <div className="flex flex-col gap-2 text-xs">
      <Comment
        body="@devuser123 yes, that was my first thought too. The changelog doesn't mention any breaking changes though."
        author="johndoe"
        score={3}
        created_at={new Date("2023-01-01T12:00:00Z")}
      />

      <Comment
        body="Can you share your environment variables (redacted)? Might be a config issue."
        author="helpfuldev"
        score={13}
        created_at={new Date("2023-01-01T12:45:00Z")}
      />

      {showCommentInput ? (
        <input
          type="text"
          placeholder="Write a comment..."
          className="w-full mt-2 px-2 py-1 text-xs border border-border rounded bg-transparent"
          autoFocus
          onBlur={() => setShowCommentInput(false)}
        />
      ) : (
        <button
          onClick={() => setShowCommentInput(true)}
          className="text-xs text-muted-foreground hover:text-foreground mt-2 block text-left"
        >
          Add a comment...
        </button>
      )}
    </div>
  );
}

function Comment({
  body,
  author,
  score,
  created_at,
}: {
  body: string;
  author: string;
  score: number;
  created_at: Date;
}) {
  return (
    <div className="flex flex-row items-center border-border border-b py-0.5">
      <div className="flex flex-row items-start">
        <div className="flex flex-row items-center justify-between w-8">
          <span className="text-left text-muted-foreground">{score}</span>
          <button
            type="button"
            className="text-muted-foreground hover:text-foreground cursor-pointer mb-0.5"
          >
            <TriangleUp className="size-3" />
          </button>
        </div>
        <p className="pl-4 flex-1">
          {body}
          <span className="text-muted-foreground shrink-0">
            {" — "}
            <span className="underline">{author}</span> {timeAgo(created_at)}
          </span>
        </p>
      </div>
    </div>
  );
}
