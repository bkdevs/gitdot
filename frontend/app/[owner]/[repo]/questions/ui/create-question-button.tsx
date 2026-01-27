"use client";

import { Plus } from "lucide-react";
import { useRouter } from "next/navigation";
import { useActionState, useState } from "react";
import { createQuestionAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle, DialogTrigger } from "@/ui/dialog";

export function CreateQuestionButton({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");

  const [state, formAction, isPending] = useActionState(
    async (_prevState: { error?: string } | null, formData: FormData) => {
      const result = await createQuestionAction(formData);
      if (result.success) {
        setOpen(false);
        setTitle("");
        setBody("");
        router.refresh();
        return null;
      }
      return { error: result.error };
    },
    null,
  );

  const isValid = title.trim() !== "" && body.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <button
          type="button"
          className="flex flex-row h-full items-center px-2 border-border border-l bg-green-600 text-xs text-primary-foreground cursor-pointer outline-0! ring-0!"
        >
          <Plus className="size-3 mr-1.5" />
          Ask question
        </button>
      </DialogTrigger>
      <DialogContent
        className="max-w-2xl min-w-2xl border-border rounded-md shadow-2xl top-[35%]"
        animations={true}
        showOverlay={true}
      >
        <DialogTitle>
          <div className="flex flex-row w-full items-center justify-between">
            <span>Ask a question</span>
            <span className="text-xs font-normal text-muted-foreground">
              {owner}/{repo}
            </span>
          </div>
        </DialogTitle>
        <form action={formAction} className="flex flex-col gap-1">
          <input type="hidden" name="owner" value={owner} />
          <input type="hidden" name="repo" value={repo} />
          <input
            type="text"
            name="title"
            placeholder="Title..."
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-0 bg-background outline-none border-none"
            disabled={isPending}
            autoFocus
          />
          <textarea
            name="body"
            placeholder="Description..."
            value={body}
            onChange={(e) => setBody(e.target.value)}
            className="w-full px-0 text-sm bg-background outline-none border-none resize-none min-h-32"
            disabled={isPending}
          />
          {state?.error && (
            <p className="text-xs text-red-500">{state.error}</p>
          )}
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={!isValid || isPending}
              className="px-3 py-1.5 rounded-xs text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? "Creating..." : "Ask"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
