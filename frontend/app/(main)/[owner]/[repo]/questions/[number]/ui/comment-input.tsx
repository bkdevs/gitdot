"use client";

import { Check } from "lucide-react";
import { useActionState, useRef, useState } from "react";
import type { CreateCommentActionResult } from "@/actions";
import { cn } from "@/util";

export function CommentInput({
  createComment,
  addOptimisticComment,
}: {
  createComment: (formData: FormData) => Promise<CreateCommentActionResult>;
  addOptimisticComment: (body: string) => void;
}) {
  const [showInput, setShowInput] = useState(false);
  const [body, setBody] = useState("");
  const formRef = useRef<HTMLFormElement>(null);

  const [, formAction] = useActionState(
    async (
      _prevState: CreateCommentActionResult | null,
      formData: FormData,
    ) => {
      const body = formData.get("body") as string;
      (document.activeElement as HTMLElement)?.blur();
      addOptimisticComment(body);

      return await createComment(formData);
    },
    null,
  );

  return (
    <div className="flex flex-row w-full pt-1">
      {showInput ? (
        <form
          ref={formRef}
          action={formAction}
          className="flex flex-row border-primary border-b h-5 w-full"
        >
          <input
            className="ring-0 outline-none flex-1 h-5"
            type="text"
            name="body"
            placeholder="Write comment..."
            autoFocus
            value={body}
            onChange={(e) => setBody(e.target.value)}
            onBlur={(e) => {
              setBody("");
              setShowInput(false);
            }}
            onKeyDown={(e) => {
              if (e.key === "Escape") {
                setBody("");
                setShowInput(false);
              } else if (e.key === "Enter") {
                if (body.length > 0) {
                  setBody("");
                  setShowInput(false);
                  formRef.current?.requestSubmit();
                } else {
                  e.preventDefault();
                }
              }
            }}
          />

          <Check
            className={cn(
              "size-3 mt-0.5 hover:text-foreground hover:stroke-3 transition-opacity",
              body ? "opacity-100" : "opacity-0 pointer-events-none",
            )}
            onMouseDown={(e) => {
              e.preventDefault();
              setBody("");
              setShowInput(false);
              formRef.current?.requestSubmit();
            }}
          />
        </form>
      ) : (
        <button
          type="button"
          className="underline text-muted-foreground cursor-pointer h-5 border-b border-transparent"
          onClick={() => setShowInput(true)}
        >
          Add comment...
        </button>
      )}
    </div>
  );
}
