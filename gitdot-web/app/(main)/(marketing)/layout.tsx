"use client";

import Image from "next/image";
import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { cn } from "@/util";

const NAV_LINKS: { label: string; href: string }[] = [
  { label: "/home", href: "/" },
  { label: "/faq", href: "/faq" },
  { label: "/weeks", href: "/weeks" },
  { label: "/decisions", href: "/decisions" },
  { label: "/releases", href: "/releases" },
  { label: "/company", href: "/company" },
  { label: "/terms", href: "/terms" },
  { label: "/privacy", href: "/privacy" },
];

function isActive(pathname: string, href: string) {
  if (href === "/") return pathname === "/";
  if (href === "/weeks")
    return pathname === "/weeks" || pathname.startsWith("/weeks/");
  return pathname === href;
}

export default function MarketingLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const pathname = usePathname();

  return (
    <div className="grid grid-cols-[1fr_min(100%,48rem)_1fr] h-full overflow-hidden">
      <div />

      <div className="px-3 py-2 flex flex-col gap-4 h-full overflow-y-auto scrollbar-none">
        {children}
      </div>

      <div className="justify-self-end pr-4 pt-1 flex flex-col items-end">
        <Image
          className="dark:invert"
          src="/gitdot-long-black.svg"
          alt="gitdot logo"
          width={100}
          height={48}
        />
        <div className="flex flex-col gap-1 items-end">
          {NAV_LINKS.map((link) => {
            const active = isActive(pathname, link.href);
            return (
              <Link
                key={link.href}
                href={link.href}
                className={cn(
                  "text-xs font-mono transition-colors",
                  active
                    ? "text-foreground underline"
                    : "text-muted-foreground hover:text-foreground",
                )}
              >
                {link.label}
              </Link>
            );
          })}
          <button
            type="button"
            onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
            className="text-xs font-mono text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            /signup
          </button>
        </div>
      </div>
    </div>
  );
}
