"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useActionState, useState } from "react";
import {
  type CreateQuestionActionResult,
  createQuestionAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function CreateQuestionDialog({
  open,
  setOpen,
  owner,
  repo,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  owner: string;
  repo: string;
}) {
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");

  const createQuestion = createQuestionAction.bind(null, owner, repo);
  const [state, formAction, isPending] = useActionState(
    async (_prev: CreateQuestionActionResult | null, formData: FormData) => {
      const result = await createQuestion(formData);
      if ("question" in result) {
        setOpen(false);
        setTitle("");
        setBody("");
      }
      return result;
    },
    null,
  );

  const isValid = title.trim() !== "" && body.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl min-w-2xl border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle title={"Create question"} />
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <input
            type="text"
            name="title"
            placeholder="Question title..."
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full p-2 bg-background outline-none border-b border-border"
            disabled={isPending}
            autoFocus
          />
          <textarea
            name="body"
            placeholder="Write a description..."
            value={body}
            onChange={(e) => setBody(e.target.value)}
            className="w-full p-2 text-sm bg-background outline-none resize-none min-h-32"
            disabled={isPending}
          />
          {state && "error" in state && (
            <p className="text-xs text-red-500 px-3 pb-2">{state.error}</p>
          )}
          <div className="flex items-center justify-between pl-2 py-2 border-t border-border h-9">
            <span className="text-xs text-muted-foreground">
              Ask a question in{" "}
              <span className="text-foreground">
                {owner}/{repo}
              </span>
            </span>
            <div>
              <button
                type="reset"
                className="px-3 py-1.5 h-9 text-xs border-b border-l border-r hover:bg-accent/50"
                onClick={() => setOpen(false)}
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || isPending}
                className="px-3 py-1.5 h-9 text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isPending ? "Creating..." : "Ask"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
