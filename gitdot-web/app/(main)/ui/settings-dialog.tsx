"use client";

import { useEffect, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function SettingsDialog() {
  const [open, setOpen] = useState(false);

  useShortcuts([
    {
      name: "Settings",
      description: "Open settings",
      keys: [","],
      execute: () => setOpen(true),
    },
  ]);

  useEffect(() => {
    const handle = () => setOpen(true);
    window.addEventListener("openSettings", handle);
    return () => window.removeEventListener("openSettings", handle);
  }, []);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-[80vw]! max-h-[85vh]! p-0! overflow-hidden flex flex-col"
        animations={true}
        showOverlay={true}
        aria-describedby={undefined}
      >
        <DialogTitle className="px-4 py-3 text-sm font-mono border-b border-border shrink-0">
          settings
        </DialogTitle>
        <div className="flex flex-1 min-h-0 overflow-hidden font-mono text-sm">
          <nav className="w-48 shrink-0 border-r border-border py-2 overflow-y-auto">
            {["General", "Appearance", "Editor", "Keybindings", "Notifications", "Privacy", "Advanced"].map((item) => (
              <button
                key={item}
                type="button"
                className="w-full text-left px-4 py-1.5 hover:bg-sidebar-accent transition-colors first:bg-sidebar-accent"
              >
                {item}
              </button>
            ))}
          </nav>
          <div className="flex-1 overflow-y-auto p-6 space-y-6">
            <section className="space-y-3">
              <h2 className="text-xs uppercase tracking-widest text-muted-foreground">General</h2>
              <p className="text-sm leading-relaxed text-muted-foreground">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.
              </p>
              <div className="space-y-2">
                {["Default branch name", "Merge strategy", "Pull request template", "Issue template"].map((label) => (
                  <div key={label} className="flex items-center justify-between py-2 border-b border-border">
                    <span className="text-sm">{label}</span>
                    <span className="text-xs text-muted-foreground">configure</span>
                  </div>
                ))}
              </div>
            </section>
            <section className="space-y-3">
              <h2 className="text-xs uppercase tracking-widest text-muted-foreground">Appearance</h2>
              <p className="text-sm leading-relaxed text-muted-foreground">
                Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident.
              </p>
              <div className="space-y-2">
                {["Theme", "Font size", "Line height", "Tab width", "Syntax highlighting"].map((label) => (
                  <div key={label} className="flex items-center justify-between py-2 border-b border-border">
                    <span className="text-sm">{label}</span>
                    <span className="text-xs text-muted-foreground">configure</span>
                  </div>
                ))}
              </div>
            </section>
            <section className="space-y-3">
              <h2 className="text-xs uppercase tracking-widest text-muted-foreground">Advanced</h2>
              <p className="text-sm leading-relaxed text-muted-foreground">
                Sunt in culpa qui officia deserunt mollit anim id est laborum. Curabitur pretium tincidunt lacus. Nulla gravida orci a odio.
              </p>
              <div className="space-y-2">
                {["API access", "Webhooks", "SSH keys", "Deploy keys", "Audit log"].map((label) => (
                  <div key={label} className="flex items-center justify-between py-2 border-b border-border">
                    <span className="text-sm">{label}</span>
                    <span className="text-xs text-muted-foreground">configure</span>
                  </div>
                ))}
              </div>
            </section>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
