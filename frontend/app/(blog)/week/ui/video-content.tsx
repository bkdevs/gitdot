"use client";

import type { VideoHTMLAttributes } from "react";
import { useState } from "react";

function VideoDialog({
  src,
  isOpen,
  onClose,
}: {
  src: string;
  isOpen: boolean;
  onClose: () => void;
}) {
  if (!isOpen) return null;

  return (
    <button
      type="button"
      className="fixed inset-0 w-full z-50 flex items-center justify-center bg-black/80 p-4"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
      onKeyDown={(e) => {
        if (e.key === "Escape") onClose();
      }}
      aria-label="Close video dialog"
    >
      <div className="relative w-full max-w-6xl">
        <video src={src} controls autoPlay className="w-full rounded-lg">
          <track kind="captions" />
        </video>
      </div>
    </button>
  );
}

type VideoContentProps = VideoHTMLAttributes<HTMLVideoElement> & {
  node?: unknown;
};

export function VideoContent({ src, children, ...props }: VideoContentProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const strSrc = typeof src === "string" ? src : undefined;
  const videoSrc =
    strSrc ||
    (Array.isArray(children)
      ? (children as Array<{ props?: { src?: string } }>).find(
          (child) => child?.props?.src,
        )?.props?.src
      : undefined);

  return (
    <>
      <button
        type="button"
        className="relative cursor-pointer group mb-4 w-full p-0 border-0 bg-transparent"
        onClick={() => setIsDialogOpen(true)}
        aria-label="Play video"
      >
        <video src={videoSrc} muted className="w-full rounded-lg">
          <track kind="captions" />
        </video>
        <div className="absolute inset-0 flex items-center justify-center bg-black/20 group-hover:bg-black/30 transition-colors rounded-lg">
          <div className="bg-white/90 rounded-full p-4 group-hover:scale-110 transition-transform">
            <svg
              className="w-12 h-12 text-black"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <title>Play video</title>
              <path d="M8 5v14l11-7z" />
            </svg>
          </div>
        </div>
      </button>
      <VideoDialog
        src={videoSrc ?? ""}
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </>
  );
}
