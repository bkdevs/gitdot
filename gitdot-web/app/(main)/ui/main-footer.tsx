"use client";

import {
  useParams,
  usePathname,
  useSelectedLayoutSegments,
} from "next/navigation";
import { useMetricsContext } from "@/context/metrics";
import { useAnimateNumber } from "@/hooks/use-animate-number";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { MainCommandBar } from "./main-command-bar";

export function MainFooter() {
  const segments = useSelectedLayoutSegments();
  if (segments.at(-2) === "reviews") return null;

  return (
    <div className="relative shrink-0 flex w-full h-6 items-center border-t bg-sidebar text-xs font-mono">
      <MainCommandBar />
      <div className="ml-auto flex items-baseline pr-2">
        <Breadcrumbs />
        <PageVitals />
      </div>
    </div>
  );
}

function Breadcrumbs() {
  const pathname = usePathname();
  const params = useParams();
  const pathLinks: React.ReactNode[] = [];
  const segments = pathname.split("/").filter(Boolean);
  segments.forEach((segment, index) => {
    let path = `/${segments.slice(0, index + 1).join("/")}`;
    if ("path" in params && index === 1) {
      path = `${path}/files`;
    }

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

  return pathLinks;
}

function PageVitals() {
  const { FCP, TTFB, CLS, INP } = useMetricsContext();
  const animatedFCP = useAnimateNumber(FCP);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="w-[5ch] text-center text-muted-foreground ml-1.5 hover:text-foreground transition-colors outline-none cursor-pointer select-none p-0 leading-none"
        >
          {animatedFCP != null ? `${animatedFCP}ms` : "0ms"}
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        side="top"
        align="end"
        sideOffset={12}
        alignOffset={-8}
      >
        <div className="px-2 py-1.5 text-xs font-mono space-y-1">
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">FCP</span>
            <span>{FCP != null ? `${Math.round(FCP)}ms` : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">TTFB</span>
            <span>{TTFB != null ? `${Math.round(TTFB)}ms` : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">CLS</span>
            <span>{CLS != null ? CLS.toFixed(3) : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">INP</span>
            <span>{INP != null ? `${Math.round(INP)}ms` : "-"}</span>
          </div>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
