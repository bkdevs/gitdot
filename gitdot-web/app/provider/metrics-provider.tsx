"use client";

import { usePathname } from "next/navigation";
import { createContext, useContext, useEffect, useRef, useState } from "react";
import { onCLS, onFCP, onINP, onTTFB } from "web-vitals";
import { createSpanAction } from "@/actions/otel";

// note: we have multiple navigations here as it's possible for a navigation to be cancelled
// in which case we append both entries, but only report the latency to the one resolved
interface MetricsContext {
  FCP: number | null;
  TTFB: number | null;
  CLS: number | null;
  INP: number | null;
  navigations: NavigationEvent[];
  startNavigation: (path: string) => void;
}

const MetricsContext = createContext<MetricsContext | null>(null);

interface NavigationEvent {
  path: string;
  start: number;
}

export function MetricsProvider({ children }: { children: React.ReactNode }) {
  const [navigations, setNavigations] = useState<NavigationEvent[]>([]);
  const [FCP, setFCP] = useState<number | null>(null);
  const [TTFB, setTTFB] = useState<number | null>(null);
  const [CLS, setCLS] = useState<number | null>(null);
  const [INP, setINP] = useState<number | null>(null);

  const fcpLoaded = useRef(false);
  const pathname = usePathname();

  useEffect(() => {
    onFCP((metric) => setFCP((prev) => (prev === null ? metric.value : prev)));
    onTTFB((metric) =>
      setTTFB((prev) => (prev === null ? metric.value : prev)),
    );
    onCLS((metric) => setCLS(metric.value), { reportAllChanges: true });
    onINP((metric) => setINP(metric.value), { reportAllChanges: true });
  }, []);

  // biome-ignore lint/correctness/useExhaustiveDependencies: intended
  useEffect(() => {
    if (FCP === null || fcpLoaded.current) return;

    fcpLoaded.current = true;
    createSpanAction(
      pathname,
      Math.round(performance.timeOrigin),
      Math.round(performance.timeOrigin + FCP),
    );
  }, [FCP]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: intended
  useEffect(() => {
    if (!pathname) return;

    for (const navigation of navigations) {
      if (navigation.path === pathname) {
        const navigationEnd = performance.now();
        const fcp = Math.round(navigationEnd - navigation.start);
        setFCP(fcp);
        setTTFB(null);

        fcpLoaded.current = true;
        createSpanAction(
          navigation.path,
          Math.round(performance.timeOrigin + navigation.start),
          Math.round(performance.timeOrigin + navigationEnd),
        );
        setNavigations([]);
      }
    }
  }, [pathname]);

  return (
    <MetricsContext
      value={{
        FCP,
        TTFB,
        CLS,
        INP,
        navigations,
        startNavigation: (path: string) => {
          setNavigations([...navigations, { path, start: performance.now() }]);
        },
      }}
    >
      {children}
    </MetricsContext>
  );
}

export function useMetricsContext(): MetricsContext {
  const context = useContext(MetricsContext);
  if (!context) {
    throw new Error("useMetricsContext must be used within an MetricsProvider");
  }
  return context;
}
