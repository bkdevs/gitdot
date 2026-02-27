"use client";

import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function BuildConfigDialog({
  open,
  setOpen,
  configHtml,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  configHtml: string | null;
}) {
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        animations={true}
        className="h-[80vh] w-[70vw] max-w-[70vw]! flex flex-col overflow-hidden p-2 bg-white"
      >
        <DialogTitle className="sr-only">.gitdot-ci.toml</DialogTitle>
        {configHtml === null ? (
          <div className="flex flex-1 items-center justify-center text-sm text-muted-foreground">
            No config file found
          </div>
        ) : (
          <div
            className="flex-1 overflow-auto text-sm [&_pre]:min-h-full"
            // biome-ignore lint/security/noDangerouslySetInnerHtml: trusted Shiki-rendered HTML
            dangerouslySetInnerHTML={{ __html: configHtml }}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
