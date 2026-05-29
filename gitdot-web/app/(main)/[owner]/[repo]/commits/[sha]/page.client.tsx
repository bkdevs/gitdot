"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import type { DiffEntry } from "gitdot-dal/client";
import { useCallback, useEffect, useRef, useState } from "react";
import { OverlayScroll } from "@/ui/scroll";
import { CommitBody } from "./ui/commit-body";
import { CommitHeader } from "./ui/commit-header";
import { CommitShortcuts } from "./ui/commit-shortcuts";
import { CommitSidebar } from "./ui/commit-sidebar";

type ScrollDirection = "up" | "down";

export function PageClient({
  owner,
  repo,
  commit,
  diffEntries,
}: {
  owner: string;
  repo: string;
  commit: RepositoryCommitResource;
  diffEntries: DiffEntry[];
}) {
  const [selectedIndex, setSelectedIndex] = useState(0);

  // we keep track of what direction the user is "scrolling in" to determine how to compute which sidebar element is active
  // - if user scrolls down, then the last element on the page is considered active
  // - if user scrolls up, then the first element on the page is considered active
  //
  // we do this so that as the user scrolls up / down, the active rows hits the top & bottom of the list.
  // i.e., if we use a heuristic to pick top, middle, or bottom, then we will always have some cut off
  const directionRef = useRef<ScrollDirection>("up");
  const bufferRef = useRef(0);
  const lastProxyRef = useRef<number | null>(null);
  const suppressUntilRef = useRef(0);

  useEffect(() => {
    if (diffEntries.length === 0) return;

    const update = () => {
      const files = Array.from(
        document.querySelectorAll<HTMLElement>("[data-diff-file]"),
      );
      if (files.length === 0) return;

      const viewportHeight = window.innerHeight;
      const proxy = files[0].getBoundingClientRect().top;
      const delta =
        lastProxyRef.current === null ? 0 : lastProxyRef.current - proxy;
      lastProxyRef.current = proxy;

      if (Date.now() < suppressUntilRef.current) return;

      const threshold = viewportHeight / 2;
      if (directionRef.current === "up" && delta > 0) {
        bufferRef.current += delta;
        if (bufferRef.current >= threshold) {
          directionRef.current = "down";
          bufferRef.current = 0;
        }
      } else if (directionRef.current === "down" && delta < 0) {
        bufferRef.current -= delta;
        if (bufferRef.current >= threshold) {
          directionRef.current = "up";
          bufferRef.current = 0;
        }
      } else if (delta !== 0) {
        bufferRef.current = 0;
      }

      const next = computeActiveIndex(
        files,
        directionRef.current,
        viewportHeight,
      );
      setSelectedIndex((prev) => (prev === next ? prev : next));
    };

    update();
    window.addEventListener("scroll", update, { capture: true, passive: true });
    window.addEventListener("wheel", update, { capture: true, passive: true });
    return () => {
      window.removeEventListener("scroll", update, { capture: true });
      window.removeEventListener("wheel", update, { capture: true });
    };
  }, [diffEntries.length]);

  // user jumps to a file (j, k) pin the highlight and suppress scroll updates for a bit to avoid jitter.
  const handleSelect = useCallback((index: number) => {
    setSelectedIndex(index);
    directionRef.current = "up";
    bufferRef.current = 0;
    suppressUntilRef.current = Date.now() + 400;
  }, []);

  return (
    <OverlayScroll>
      <div
        data-diff-top
        className="grid grid-cols-[minmax(18rem,1fr)_minmax(0,80rem)_minmax(0,1fr)] w-full"
      >
        <aside className="hidden xl:flex xl:flex-col sticky top-0 self-start max-h-screen overflow-y-auto py-5 pl-4 w-72 gap-6 justify-self-end">
          <CommitHeader
            owner={owner}
            repo={repo}
            sha={commit.sha}
            message={commit.message}
            date={commit.date}
            author={commit.author}
          />
          <CommitSidebar
            entries={diffEntries}
            selectedIndex={selectedIndex}
            onSelect={handleSelect}
          />
        </aside>
        <div className="w-full px-4 py-4 flex flex-col gap-6 min-w-0">
          <CommitBody entries={diffEntries} />
        </div>
        <div />
        <CommitShortcuts
          selectedIndex={selectedIndex}
          setSelectedIndex={handleSelect}
        />
      </div>
    </OverlayScroll>
  );
}

function computeActiveIndex(
  files: HTMLElement[],
  direction: ScrollDirection,
  viewportHeight: number,
): number {
  if (direction === "down") {
    for (let i = files.length - 1; i >= 0; i--) {
      const rect = files[i].getBoundingClientRect();
      if (rect.bottom > 0 && rect.top < viewportHeight) return i;
    }
  } else {
    for (let i = 0; i < files.length; i++) {
      const rect = files[i].getBoundingClientRect();
      if (rect.bottom > 0 && rect.top < viewportHeight) return i;
    }
  }

  return files[0].getBoundingClientRect().top >= viewportHeight
    ? 0
    : files.length - 1;
}
