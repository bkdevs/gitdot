"use client";

import Link from "@/ui/link";
import { User } from "lucide-react";
import { usePathname } from "next/navigation";

export function MainHeader() {
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
    <div className="shrink-0 flex flex-row w-full h-9 items-center justify-between border-b bg-sidebar">
      <div className="flex-1 pl-2 text-sm font-mono flex items-center">
        {pathLinks}
      </div>
      <div className="w-9 h-9 flex items-center justify-center hover:bg-sidebar-accent">
        <User className="size-4"/>
      </div>
    </div>
  );
}
