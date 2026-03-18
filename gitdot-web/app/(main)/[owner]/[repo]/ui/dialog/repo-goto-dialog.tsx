"use client";

import { useParams, useRouter } from "next/navigation";
import { useEffect, useMemo, useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import Link from "@/ui/link";

type NavItem = {
  key: string;
  rest: string;
  href: string;
};

export function RepoGotoDialog() {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const { user } = useUserContext();
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const linkRefs = useRef<Map<string, HTMLAnchorElement>>(new Map());

  const items = useMemo<NavItem[]>(
    () => [
      { key: "h", rest: "ome", href: `/${owner}/${repo}` },
      { key: "r", rest: "eviews", href: `/${owner}/${repo}/reviews` },
      { key: "f", rest: "iles", href: `/${owner}/${repo}/files` },
      { key: "b", rest: "uilds", href: `/${owner}/${repo}/builds` },
      { key: "c", rest: "ommits", href: `/${owner}/${repo}/commits` },
      { key: "s", rest: "ettings", href: `/${owner}/${repo}/settings` },
      { key: "q", rest: "uestions", href: `/${owner}/${repo}/questions` },
      { key: "p", rest: "rofile", href: user ? `/${user.name}` : "/" },
    ],
    [owner, repo, user],
  );

  useEffect(() => {
    const handleOpen = () => setOpen(true);
    window.addEventListener("openGotoDialog", handleOpen);
    return () => window.removeEventListener("openGotoDialog", handleOpen);
  }, []);

  useEffect(() => {
    if (!open) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        setOpen(false);
        return;
      }

      const item = items.find((i) => i.key === e.key);
      if (item) {
        e.preventDefault();
        router.push(item.href);
        setOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, items, router]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-60! w-full p-3 pt-4 top-[45vh]!"
        aria-describedby={undefined}
        showOverlay={false}
      >
        <DialogTitle className="absolute -top-2 left-2 bg-background px-1 font-mono text-xs">
          goto
        </DialogTitle>
        <div className="grid grid-cols-2 gap-x-6 gap-y-1 font-mono text-sm">
          {items.map((item) => (
            <Link
              key={item.key}
              href={item.href}
              ref={(el) => {
                if (el) linkRefs.current.set(item.key, el);
              }}
              onClick={() => setOpen(false)}
            >
              /<span className="font-bold">{item.key}</span>
              {item.rest}
            </Link>
          ))}
        </div>
      </DialogContent>
    </Dialog>
  );
}
