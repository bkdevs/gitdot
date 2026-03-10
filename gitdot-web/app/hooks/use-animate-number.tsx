import { useEffect, useRef, useState } from "react";

export function useAnimateNumber(
  value: number | null,
  duration = 500,
): number | null {
  const [display, setDisplay] = useState<number | null>(null);
  const prevRef = useRef<number | null>(null);
  const rafRef = useRef<number>(0);

  useEffect(() => {
    if (value === null) {
      prevRef.current = null;
      setDisplay(null);
      return;
    }
    if (prevRef.current !== null) {
      setDisplay(value);
      return;
    }

    const start = performance.now();
    const tick = (now: number) => {
      const elapsed = now - start;
      const progress = Math.min(elapsed / duration, 1);
      const eased = 1 - (1 - progress) ** 3;
      setDisplay(Math.round(eased * value));
      if (progress < 1) {
        rafRef.current = requestAnimationFrame(tick);
      }
    };
    rafRef.current = requestAnimationFrame(tick);
    prevRef.current = value;

    return () => cancelAnimationFrame(rafRef.current);
  }, [value, duration]);

  return display;
}
