import { usePathname } from "next/navigation";
import { useReportWebVitals } from "next/web-vitals";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";

// note: we have multiple navigations here as it's possible for a navigation to be cancelled
// in which case we append both entries, but only report the latency to the one resolved
interface MetricsContext {
  currentPageLoad: number | null;
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
  const [currentPageLoad, setCurrentPageLoad] = useState<number | null>(null);

  const pathname = usePathname();

  // biome-ignore lint/correctness/useExhaustiveDependencies: intended
  const handleWebVitals = useCallback((metric: any) => {
    if (metric.name === "FCP" && currentPageLoad === null) {
      setCurrentPageLoad(metric.value);
    } else {
      console.log(metric);
    }
  }, []);
  useReportWebVitals(handleWebVitals);

  // biome-ignore lint/correctness/useExhaustiveDependencies: intended
  useEffect(() => {
    if (!pathname) return;
    for (const navigation of navigations) {
      if (navigation.path === pathname) {
        setCurrentPageLoad(performance.now() - navigation.start);
        setNavigations([]);
      }
    }
  }, [pathname]);

  return (
    <MetricsContext
      value={{
        currentPageLoad,
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
