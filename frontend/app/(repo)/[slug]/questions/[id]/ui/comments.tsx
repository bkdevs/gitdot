"use client";

import { timeAgo } from "@/util";
import { ChevronUp } from "lucide-react"
import { useState } from "react";

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
      score={1}
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
  )
}

function Comment({ body, author, score, created_at }: { body: string, author: string, score: number, created_at: Date }) {
  return (
    <div className="flex items-start gap-1">
      <span className="w-4 text-right text-muted-foreground">{score}</span>
      <button className="text-muted-foreground hover:text-foreground">
        <ChevronUp className="w-4 h-4" />
      </button>
      <p className="flex-1">
        {body}
      </p>
      <span className="text-muted-foreground shrink-0">
        <span className="underline">{author}</span> {timeAgo(created_at)}
      </span>
    </div>
  )
}
