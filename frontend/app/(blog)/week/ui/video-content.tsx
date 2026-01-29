"use client";

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
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4"
      onClick={onClose}
    >
      <div
        className="relative w-full max-w-6xl"
        onClick={(e) => e.stopPropagation()}
      >
        <video src={src} controls autoPlay className="w-full rounded-lg" />
      </div>
    </div>
  );
}

export function VideoContent({ src, ...props }: any) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const videoSrc =
    src || props.children?.find((child: any) => child?.props?.src)?.props?.src;

  return (
    <>
      <div
        className="relative cursor-pointer group mb-4"
        onClick={() => setIsDialogOpen(true)}
      >
        <video src={videoSrc} muted className="w-full rounded-lg" {...props} />
        <div className="absolute inset-0 flex items-center justify-center bg-black/20 group-hover:bg-black/30 transition-colors rounded-lg">
          <div className="bg-white/90 rounded-full p-4 group-hover:scale-110 transition-transform">
            <svg
              className="w-12 h-12 text-black"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M8 5v14l11-7z" />
            </svg>
          </div>
        </div>
      </div>
      <VideoDialog
        src={videoSrc}
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </>
  );
}
