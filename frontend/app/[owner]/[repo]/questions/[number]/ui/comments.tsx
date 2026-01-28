"use client";

import { useState } from "react";
import { TriangleUp } from "@/lib/icons";
import { timeAgoFull } from "@/util";

export function Comments() {
  const [showInput, setShowInput] = useState(false);
  const [comment, setComment] = useState("");

  return (
    <div className="flex flex-col text-xs">
      <div className="my-0.5 border-b border-border" />
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

      <div className="flex flex-row w-full pt-1">
        {showInput ? (
          <input
          className="border-b border-bg ring-0 outline-none h-5 w-full"
          type="email"
          placeholder="Write comment..."
          autoFocus
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          onBlur={() => {
            if (comment.length === 0) {
              setShowInput(false);
            }
          }}
          onKeyDown={(e) => {
            if (e.key === "Escape") {
              setShowInput(false);
            } else if (e.key === "Enter" && comment.length > 0) {
              // handleSubmit();
            }
          }}
          />
        ) : (
          <button
            type="button"
            className="underline text-muted-foreground cursor-pointer h-5"
            onClick={() => setShowInput(true)}
          >
            Add comment..
          </button>
        )}
      </div>
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
    <div className="flex flex-row items-center border-border border-b py-1">
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
            <span className="text-blue-400 cursor-pointer">{author}</span>{" "}
            {timeAgoFull(created_at)}
          </span>
        </p>
      </div>
    </div>
  );
}
