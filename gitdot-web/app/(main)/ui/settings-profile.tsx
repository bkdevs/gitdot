"use client";

import type { UserResource } from "gitdot-api";
import { User } from "lucide-react";
import Image from "next/image";
import { useRef, useState } from "react";
import { uploadUserImageAction } from "@/actions";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/ui/tooltip";
import { formatDate, timeAgo } from "@/util/date";

export function SettingsProfile({
  user,
  location,
  onLocationChange,
  links,
  onLinksChange,
  readme,
  onReadmeChange,
  company,
  onCompanyChange,
}: {
  user: UserResource | null;
  location: string;
  onLocationChange: (v: string) => void;
  links: string[];
  onLinksChange: (v: string[]) => void;
  readme: string;
  onReadmeChange: (v: string) => void;
  company: string;
  onCompanyChange: (v: string) => void;
}) {
  const linkInputRefs = useRef<(HTMLInputElement | null)[]>([]);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [image, setImage] = useState<string | null>(user?.image ?? null);

  const MAX_IMAGE_BYTES = 5 * 1024 * 1024;

  async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    setUploadError(null);
    if (file.size > MAX_IMAGE_BYTES) {
      setUploadError("Image must be under 5 MB — try a smaller file.");
      return;
    }
    setUploading(true);
    const formData = new FormData();
    formData.set("image", file);
    const result = await uploadUserImageAction(formData);
    setUploading(false);
    if ("error" in result) {
      setUploadError(result.error);
    } else {
      setImage(result.data.image ?? null);
    }
  }

  if (!user) return null;

  return (
    <>
      {uploadError && (
        <p className="fixed top-4 right-4 z-50 text-xs text-destructive">
          {uploadError}
        </p>
      )}
      <div className="max-w-lg space-y-6 mx-auto py-4">
        <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-0 items-end">
          <input
            ref={fileInputRef}
            type="file"
            accept="image/jpeg,image/png,image/webp"
            className="hidden"
            onChange={handleFileChange}
          />
          <Tooltip>
            <TooltipTrigger asChild>
              {image ? (
                <div
                  className="relative size-8 mb-1.5 cursor-pointer"
                  onClick={() => !uploading && fileInputRef.current?.click()}
                >
                  <Image
                    src={`data:image/webp;base64,${image}`}
                    alt="Profile"
                    width={32}
                    height={32}
                    unoptimized
                    className={`rounded-full transition-opacity duration-300${uploading ? " opacity-60" : ""}`}
                  />
                  <div className={`absolute -inset-0.5 rounded-full border border-transparent border-t-foreground/50 animate-spin transition-opacity duration-300${uploading ? "" : " opacity-0"}`} />
                </div>
              ) : (
                <button
                  type="button"
                  className="relative mb-1.5 cursor-pointer"
                  onClick={() => !uploading && fileInputRef.current?.click()}
                >
                  <User className={`size-8 transition-opacity duration-300${uploading ? " opacity-60" : ""}`} />
                  <div className={`absolute -inset-0.5 rounded-full border border-transparent border-t-foreground/50 animate-spin transition-opacity duration-300${uploading ? "" : " opacity-0"}`} />
                </button>
              )}
            </TooltipTrigger>
            <TooltipContent>Upload photo</TooltipContent>
          </Tooltip>
          <span className="text-sm font-semibold mb-1.5">{user.name}</span>
          <span className="text-sm text-muted-foreground">email</span>
          <span className="text-sm">{user.email}</span>
          <span className="text-sm text-muted-foreground">joined</span>
          <span className="text-sm text-muted-foreground">
            {formatDate(new Date(user.created_at))} (
            {timeAgo(new Date(user.created_at))})
          </span>
        </div>

        <div className="space-y-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>
            about
          </p>
          <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 items-end">
            <span className="text-sm text-muted-foreground">company</span>
            <input
              value={company}
              onChange={(e) => onCompanyChange(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") e.currentTarget.blur();
              }}
              className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
              placeholder="company name"
            />
            <span className="text-sm text-muted-foreground">location</span>
            <input
              value={location}
              onChange={(e) => onLocationChange(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") e.currentTarget.blur();
              }}
              className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
              placeholder="city, country"
            />
          </div>
        </div>

        <div className="space-y-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>
            links
          </p>
          <div className="space-y-1">
            {links.map((link, i) => (
              <input
                // biome-ignore lint/suspicious/noArrayIndexKey: no reordering
                key={i}
                ref={(el) => {
                  linkInputRefs.current[i] = el;
                }}
                value={link}
                onChange={(e) => {
                  const next = [...links];
                  next[i] = e.target.value;
                  onLinksChange(next);
                }}
                onKeyDown={(e) => {
                  if (e.key === "Enter") e.currentTarget.blur();
                }}
                onBlur={() => {
                  if (!links[i]?.trim()) {
                    onLinksChange(links.filter((_, j) => j !== i));
                  }
                }}
                className="text-sm bg-transparent border-b border-border outline-none w-full placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
                placeholder="https://..."
              />
            ))}
            <button
              type="button"
              onClick={() => {
                const next = [...links, ""];
                onLinksChange(next);
                setTimeout(() => {
                  linkInputRefs.current[next.length - 1]?.focus();
                }, 0);
              }}
              className="mt-0.5 text-xs text-muted-foreground/60 hover:text-muted-foreground transition-colors cursor-pointer block"
            >
              new link
            </button>
          </div>
        </div>

        <div className="space-y-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>
            README.md
          </p>
          <textarea
            value={readme}
            onChange={(e) => onReadmeChange(e.target.value)}
            className="text-sm bg-transparent border-r border-border outline-none w-full min-h-24 placeholder:text-muted-foreground/40 transition-colors focus:border-foreground resize-none field-sizing-content"
            placeholder="tell us about yourself..."
          />
        </div>
      </div>
    </>
  );
}
