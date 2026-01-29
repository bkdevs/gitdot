"use client";

import { CreateCommentActionResult } from "@/actions";
import { useActionState, useRef, useState } from "react";

export function CommentInput({
  createComment,
  addOptimisticComment,
}: {
  createComment: (formData: FormData) => Promise<CreateCommentActionResult>;
  addOptimisticComment: (body: string) => void;
}) {
  const [showInput, setShowInput] = useState(false);
  const formRef = useRef<HTMLFormElement>(null);

  const [, formAction] = useActionState(
    async (
      _prevState: CreateCommentActionResult | null,
      formData: FormData,
    ) => {
      const body = formData.get("body") as string;
      formRef.current?.reset();
      (document.activeElement as HTMLElement)?.blur();
      setShowInput(false);
      addOptimisticComment(body);

      return await createComment(formData);
    },
    null,
  );

  return (
    <div className="flex flex-row w-full pt-1">
      {showInput ? (
        <form ref={formRef} action={formAction} className="w-full">
          <input
            className="border-b border-bg ring-0 outline-none h-5 w-full"
            type="text"
            name="body"
            placeholder="Write comment..."
            autoFocus
            onBlur={(e) => {
              if (e.target.value.length === 0) {
                setShowInput(false);
              }
            }}
            onKeyDown={(e) => {
              if (e.key === "Escape") {
                setShowInput(false);
              } else if (
                e.key === "Enter" &&
                e.currentTarget.value.length === 0
              ) {
                e.preventDefault();
              }
            }}
          />
        </form>
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
  );
}
