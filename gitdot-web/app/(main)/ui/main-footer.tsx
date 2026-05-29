"use client";

import type { UserResource } from "gitdot-api";
import { useSelectedLayoutSegments } from "next/navigation";
import { useEffect, useState } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { useUserContext } from "@/(main)/context/user";
import { useTypewriter } from "@/hooks/use-typewriter";
import Link from "@/ui/link";
import { cn } from "@/util";
import { MainCommands } from "./main-commands";

export function MainFooter() {
  const segments = useSelectedLayoutSegments();
  if (segments[2] === "reviews" && segments[3] !== undefined) return null;
  if (segments[0] === "(marketing)") return null;

  return (
    <div className="relative shrink-0 flex w-full h-7 items-center border-t bg-sidebar text-sm font-mono">
      <MainCommands />
      <div className="ml-auto flex items-center h-full">
        <AuthStatus />
      </div>
    </div>
  );
}

function AuthStatus() {
  const { user } = useUserContext();

  if (user === undefined) return null;
  if (user) return <AuthStatusLoggedIn user={user} />;
  return <AuthStatusGhost />;
}

function useTypewriterDone(text: string, speed = 25) {
  const typed = useTypewriter(text, speed);
  const [done, setDone] = useState(false);

  useEffect(() => {
    if (typed !== text) {
      setDone(false);
      return;
    }
    const t = setTimeout(() => setDone(true), 60);
    return () => clearTimeout(t);
  }, [typed, text]);

  return { typed, done };
}

function AuthStatusLoggedIn({ user }: { user: UserResource }) {
  const text = `logged in as ${user.name}`;
  const { typed, done } = useTypewriterDone(text);

  return (
    <Link
      href={`/${user.name}`}
      className="flex items-center gap-2 text-muted-foreground hover:text-foreground hover:underline transition-colors duration-200 mr-1.5"
    >
      <span
        className="inline-block whitespace-pre text-left"
        style={{ width: `${text.length}ch` }}
      >
        {typed}
      </span>
      <span
        className={cn(
          "transition-opacity duration-300",
          done ? "opacity-100" : "opacity-0",
        )}
      >
        <UserImage userId={user.id} updatedAt={user.image_updated_at} px={16} />
      </span>
    </Link>
  );
}

function AuthStatusGhost() {
  const text = "browsing as guest";
  const { typed, done } = useTypewriterDone(text);

  return (
    <div className="flex items-center gap-2 text-muted-foreground mr-1.5">
      <span
        className="inline-block whitespace-pre text-left"
        style={{ width: `${text.length}ch` }}
      >
        {typed}
      </span>
      <span
        className={cn(
          "text-foreground font-medium hover:underline transition-opacity duration-300 cursor-pointer",
          done ? "opacity-100" : "opacity-0",
        )}
        onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
      >
        (login)
      </span>
    </div>
  );
}
