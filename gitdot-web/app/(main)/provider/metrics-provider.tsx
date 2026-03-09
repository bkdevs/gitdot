import { usePathname } from "next/navigation";
import { useReportWebVitals } from "next/web-vitals";
import { createContext, useContext, useEffect, useState } from "react";

interface MetricsContext {
  navigations: NavigationEvent[];
  startNavigation: (path: string) => void;
}

const MetricsContext = createContext<MetricsContext | null>(null);

interface NavigationEvent {
  path: string;
  start: number;
}

// module scoped to keep a stable reference
const logWebVitals = (metric: any) => {
  console.log(metric);
};

export function MetricsProvider({ children }: { children: React.ReactNode }) {
  const [navigations, setNavigations] = useState<NavigationEvent[]>([]);
  const pathname = usePathname();

  useReportWebVitals(logWebVitals);

  // biome-ignore lint/correctness/useExhaustiveDependencies: intended, only run on pathname updates
  useEffect(() => {
    if (!pathname) return;
    for (const navigation of navigations) {
      if (navigation.path === pathname) {
        console.log(
          `${pathname} took ${performance.now() - navigation.start}ms`,
        );
        setNavigations([]);
      }
    }
  }, [pathname]);

  return (
    <MetricsContext
      value={{
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
