"use client";
import { useEffect, useState } from "react";

function useSidebarToggle(event: string) {
  const [open, setOpen] = useState(true);
  useEffect(() => {
    const handler = () => setOpen((v) => !v);
    window.addEventListener(event, handler);
    return () => window.removeEventListener(event, handler);
  }, [event]);
  return open;
}

export function useLeftSidebar() {
  return useSidebarToggle("toggleLeftSidebar");
}
export function useRightSidebar() {
  return useSidebarToggle("toggleRightSidebar");
}
