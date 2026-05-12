"use client";

import type { UserResource } from "gitdot-api";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import { updateUserAction, uploadUserImageAction } from "@/actions";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/ui/tooltip";
import { formatDate, timeAgo } from "@/util/date";

export function SettingsProfile({ user }: { user: UserResource }) {
  const { refreshUser } = useUserContext();
  const router = useRouter();
  const pathname = usePathname();
  const [location, setLocation] = useState(user.location ?? "");
  const [links, setLinks] = useState<string[]>(user.links ?? []);
  const [readme, setReadme] = useState(user.readme ?? "");
  const [displayName, setDisplayName] = useState(user.display_name ?? "");
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setLocation(user.location ?? "");
    setLinks(user.links ?? []);
    setReadme(user.readme ?? "");
    setDisplayName(user.display_name ?? "");
  }, [user]);

  const dirty =
    location !== (user.location ?? "") ||
    displayName !== (user.display_name ?? "") ||
    readme !== (user.readme ?? "") ||
    links.length !== (user.links?.length ?? 0) ||
    !links.every((l, i) => l === user.links?.[i]);

  async function handleSave() {
    if (!dirty || saving) return;
    setSaving(true);
    const formData = new FormData();
    formData.set("location", location);
    formData.set("links", JSON.stringify(links));
    formData.set("readme", readme);
    formData.set("display_name", displayName);
    await updateUserAction(null, formData);
    await refreshUser();
    if (pathname === `/${user.name}`) router.refresh();
    setSaving(false);
    toast.success("Profile saved.");
  }

  return (
    <div className="max-w-lg mx-auto p-4">
      <div className="space-y-6">
        <ProfilePrimary user={user} />
        <ProfileAbout
          displayName={displayName}
          location={location}
          onDisplayNameChange={setDisplayName}
          onLocationChange={setLocation}
        />
        <ProfileLinks links={links} onLinksChange={setLinks} />
        <ProfileReadme readme={readme} onReadmeChange={setReadme} />
      </div>
      <div className="flex justify-end mt-2">
        <button
          type="button"
          onClick={handleSave}
          disabled={!dirty || saving}
          className={`text-sm underline-offset-4 transition-colors cursor-pointer disabled:cursor-not-allowed ${
            saving ? "" : "underline"
          } ${dirty ? "text-foreground" : "text-muted-foreground"}`}
        >
          {saving ? "Saving..." : "Save profile"}
        </button>
      </div>
    </div>
  );
}

function ProfilePrimary({ user }: { user: UserResource }) {
  const { refreshUser } = useUserContext();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);

  async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    setUploadError(null);
    setUploading(true);
    const result = await uploadUserImageAction(file);
    setUploading(false);
    if ("error" in result) {
      setUploadError(result.error);
    } else {
      refreshUser();
    }
  }

  return (
    <>
      {uploadError && (
        <p className="fixed top-4 right-4 z-50 text-xs text-destructive">
          {uploadError}
        </p>
      )}
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
            <button
              type="button"
              className="relative size-8 mb-1.5 cursor-pointer appearance-none bg-transparent border-none p-0"
              onClick={() => !uploading && fileInputRef.current?.click()}
            >
              <span
                className={`transition-opacity duration-300${uploading ? " opacity-60" : ""}`}
              >
                <UserImage userId={user.id} />
              </span>
              <div
                className={`absolute -inset-0.5 rounded-full border border-transparent border-t-foreground/50 animate-spin transition-opacity duration-300${uploading ? "" : " opacity-0"}`}
              />
            </button>
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
    </>
  );
}

function ProfileAbout({
  displayName,
  location,
  onDisplayNameChange,
  onLocationChange,
}: {
  displayName: string;
  location: string;
  onDisplayNameChange: (v: string) => void;
  onLocationChange: (v: string) => void;
}) {
  return (
    <div className="space-y-2">
      <p className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>
        about
      </p>
      <div className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 items-end">
        <span className="text-sm text-muted-foreground">name</span>
        <input
          value={displayName}
          onChange={(e) => onDisplayNameChange(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") e.currentTarget.blur();
          }}
          className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
          placeholder="ada lovelace"
        />
        <span className="text-sm text-muted-foreground">location</span>
        <input
          value={location}
          onChange={(e) => onLocationChange(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") e.currentTarget.blur();
          }}
          className="text-sm bg-transparent border-b border-border outline-none w-full -mb-px placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
          placeholder="brooklyn, ny"
        />
      </div>
    </div>
  );
}

function ProfileLinks({
  links,
  onLinksChange,
}: {
  links: string[];
  onLinksChange: (v: string[]) => void;
}) {
  const linkInputRefs = useRef<(HTMLInputElement | null)[]>([]);
  const draftInputRef = useRef<HTMLInputElement | null>(null);
  const [draft, setDraft] = useState<string | null>(null);

  function commitDraft() {
    if (draft?.trim()) {
      onLinksChange([...links, draft.trim()]);
    }
    setDraft(null);
  }

  return (
    <div className="space-y-2">
      <p className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>
        links
      </p>
      <div className="space-y-1">
        {links.map((link, i) => (
          <input
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
              if (e.key === "Enter" || e.key === "Escape") {
                e.stopPropagation();
                e.currentTarget.blur();
              }
            }}
            onBlur={() => {
              if (!links[i]?.trim()) {
                onLinksChange(links.filter((_, j) => j !== i));
              }
            }}
            className="text-sm bg-transparent border-b border-border outline-none w-full placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
            placeholder="mastodon.social/@you"
          />
        ))}
        {draft !== null ? (
          <input
            ref={draftInputRef}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === "Escape") {
                e.stopPropagation();
                commitDraft();
              }
            }}
            onBlur={commitDraft}
            autoFocus
            className="h-5 text-sm bg-transparent border-b border-border outline-none w-full placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
            placeholder="mastodon.social/@you"
          />
        ) : (
          <button
            type="button"
            onClick={() => setDraft("")}
            className="h-5 text-xs text-muted-foreground/60 hover:text-muted-foreground transition-colors cursor-pointer block border-b border-transparent w-full text-left"
          >
            new link
          </button>
        )}
      </div>
    </div>
  );
}

function ProfileReadme({
  readme,
  onReadmeChange,
}: {
  readme: string;
  onReadmeChange: (v: string) => void;
}) {
  return (
    <div className="space-y-2">
      <p className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>
        README.md
      </p>
      <textarea
        value={readme}
        onChange={(e) => onReadmeChange(e.target.value)}
        className="text-sm bg-transparent border-l border-border pl-2 outline-none w-full min-h-24 placeholder:text-muted-foreground/40 transition-colors focus:border-foreground resize-none field-sizing-content"
        placeholder="what you love to do..."
      />
    </div>
  );
}
