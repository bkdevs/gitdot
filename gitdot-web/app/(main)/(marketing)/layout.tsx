"use client";

import { usePathname } from "next/navigation";
import Link from "@/ui/link";
import { cn } from "@/util";

const NAV_LINKS: { label: string; href: string }[] = [
  { label: "/home", href: "/" },
  { label: "/faq", href: "/faq" },
  { label: "/weeks", href: "/weeks" },
  { label: "/decisions", href: "/decisions" },
  { label: "/releases", href: "/releases" },
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
    <div className="grid grid-cols-1 md:grid-cols-[1fr_min(100%,48rem)_1fr] h-full overflow-hidden">
      <div className="hidden md:flex pl-4 pt-4 flex-col gap-1 items-start">
        {NAV_LINKS.map((link) => {
          const active = isActive(pathname, link.href);
          return (
            <Link
              key={link.href}
              href={link.href}
              className={cn(
                "text-sm font-mono transition-colors",
                active
                  ? "text-foreground underline"
                  : "text-muted-foreground hover:text-foreground",
              )}
            >
              {link.label}
            </Link>
          );
        })}
      </div>

      {children}
    </div>
  );
}
