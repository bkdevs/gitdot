"use client";

import type { ImgHTMLAttributes } from "react";
import { useEffect, useState } from "react";
import { createPortal } from "react-dom";

function ImageDialog({
  src,
  alt,
  isOpen,
  onClose,
}: {
  src: string;
  alt: string;
  isOpen: boolean;
  onClose: () => void;
}) {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!isOpen || !mounted) return null;

  return createPortal(
    <button
      type="button"
      className="fixed inset-0 w-full z-50 flex items-center justify-center bg-black/80 p-4"
      onClick={(e) => {
        if (e.target === e.currentTarget) onClose();
      }}
      onKeyDown={(e) => {
        if (e.key === "Escape") onClose();
      }}
      aria-label="Close image dialog"
    >
      <div className="relative max-w-6xl max-h-[90vh]">
        {/* biome-ignore lint/performance/noImgElement: dynamic blog images have unknown dimensions */}
        <img
          src={src}
          alt={alt}
          className="max-w-full max-h-[90vh] object-contain rounded-lg"
        />
      </div>
    </button>,
    document.body,
  );
}

function ScaledImage({
  src,
  alt,
  height,
}: {
  src: string;
  alt: string;
  height: number;
}) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  // CSS-only approach:
  // - Mobile: overflow-x-auto for horizontal scroll
  // - Desktop: use left/transform to center image beyond container
  return (
    <span className="block relative group mb-4 overflow-x-auto md:overflow-visible">
      <button
        type="button"
        className="p-0 border-0 bg-transparent md:relative md:left-1/2 md:-translate-x-1/2"
        onClick={() => setIsDialogOpen(true)}
      >
        {/* biome-ignore lint/performance/noImgElement: dynamic blog images have unknown dimensions */}
        <img
          src={src}
          alt={alt || ""}
          style={{ height: `${height}px` }}
          className="w-auto max-w-none rounded-lg cursor-pointer group-hover:opacity-95 transition-opacity"
        />
      </button>
      <ImageDialog
        src={src}
        alt={alt || ""}
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </span>
  );
}

type ImageContentProps = Omit<ImgHTMLAttributes<HTMLImageElement>, "height"> & {
  height?: string;
  node?: unknown;
};

export function ImageContent({
  src,
  alt,
  height,
  node: _node,
  ...props
}: ImageContentProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  // If height is specified, use ScaledImage component
  const hasCustomHeight = height && parseInt(height, 10) > 0;

  if (hasCustomHeight) {
    return (
      <ScaledImage
        src={src ?? ""}
        alt={alt || ""}
        height={parseInt(height, 10)}
      />
    );
  }

  return (
    <span className="block relative group mb-4 max-w-full overflow-hidden">
      <button
        type="button"
        className="p-0 border-0 bg-transparent w-full"
        onClick={() => setIsDialogOpen(true)}
      >
        {/* biome-ignore lint/performance/noImgElement: dynamic blog images have unknown dimensions */}
        <img
          src={src}
          alt={alt || ""}
          className="w-full max-w-full rounded-lg cursor-pointer group-hover:opacity-95 transition-opacity"
          {...props}
        />
      </button>
      <span className="absolute bottom-2 right-2 bg-black/60 text-white p-1.5 rounded opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <title>Expand image</title>
          <polyline points="15 3 21 3 21 9" />
          <polyline points="9 21 3 21 3 15" />
          <line x1="21" y1="3" x2="14" y2="10" />
          <line x1="3" y1="21" x2="10" y2="14" />
        </svg>
      </span>
      <ImageDialog
        src={src ?? ""}
        alt={alt || ""}
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </span>
  );
}
