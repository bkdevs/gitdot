"use client";

import "overlayscrollbars/overlayscrollbars.css";
import { OverlayScrollbarsComponent } from "overlayscrollbars-react";

export function RepoScroll({ children }: { children: React.ReactNode }) {
  return (
    <OverlayScrollbarsComponent
      defer
      element="div"
      className="flex-1 min-w-0"
      options={{ scrollbars: { theme: "os-theme-gitdot", autoHide: "scroll", autoHideDelay: 800 } }}
    >
      {children}
    </OverlayScrollbarsComponent>
  );
}
