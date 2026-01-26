"use client";

import { Circle } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export function AppHeader() {
  const pathname = usePathname();
  const segments = pathname.split("/").filter(Boolean);

  const pathLinks: React.ReactNode[] = [];

  segments.forEach((segment, index) => {
    const path = `/${segments.slice(0, index + 1).join("/")}`;
    if (index > 0) {
      pathLinks.push(<span key={`sep-${segment}`}>/</span>);
    }
    pathLinks.push(
      <Link
        className="hover:underline"
        href={path}
        key={`segment-${segment}`}
        prefetch={true}
      >
        {segment}
      </Link>,
    );
  });

  return (
    <div className="shrink-0 flex flex-row w-full h-9 items-center border-b bg-sidebar">
      <div className="flex w-9 h-9 border-r items-center justify-center bg-foreground">
        <Circle className="size-2 fill-current text-background" />
      </div>
      <div className="flex-1 pl-2 text-sm font-mono flex items-center">
        {pathLinks}
      </div>
    </div>
  );
}
