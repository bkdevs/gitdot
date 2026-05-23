"use client";

import type { WebVitalEvent } from "gitdot-api";
import { useParams, usePathname } from "next/navigation";
import { useCallback, useEffect, useRef } from "react";
import type { Metric } from "web-vitals";
import { onCLS, onFCP, onINP, onLCP, onTTFB } from "web-vitals";

import { inferRouteTemplate } from "@/lib/route-template";

const BEACON_URL = "/api/metrics/web-vital";

export function MetricsProvider({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const params = useParams();

  const queueRef = useRef<WebVitalEvent[]>([]);
  const contextRef = useRef({ pathname, params });
  contextRef.current = { pathname, params };

  const flush = useCallback(() => {
    if (queueRef.current.length === 0) return;
    const body = JSON.stringify({ events: queueRef.current.splice(0) });
    navigator.sendBeacon(
      BEACON_URL,
      new Blob([body], { type: "application/json" }),
    );
  }, []);

  useEffect(() => {
    const push = (m: Metric) => {
      const { pathname, params } = contextRef.current;
      queueRef.current.push({
        event_time: Date.now(),
        name: m.name,
        value: m.value,
        rating: m.rating,
        metric_id: m.id,
        navigation_type: m.navigationType,
        route: inferRouteTemplate(pathname, params),
        path: pathname,
      });
    };

    onFCP(push);
    onTTFB(push);
    onLCP(push);
    onCLS(push, { reportAllChanges: true });
    onINP(push, { reportAllChanges: true });
  }, []);

  useEffect(() => {
    const onVisibility = () => {
      if (document.visibilityState === "hidden") flush();
    };
    document.addEventListener("visibilitychange", onVisibility);
    window.addEventListener("pagehide", flush);
    return () => {
      document.removeEventListener("visibilitychange", onVisibility);
      window.removeEventListener("pagehide", flush);
    };
  }, [flush]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: pathname is the trigger for cleanup-on-change; the body doesn't read it
  useEffect(() => {
    return flush;
  }, [pathname, flush]);

  return <>{children}</>;
}
