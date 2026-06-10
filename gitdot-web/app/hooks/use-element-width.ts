import { type RefObject, useEffect, useState } from "react";

/**
 * Track the rendered width (in px) of a referenced element, updating on resize
 * via a ResizeObserver. Returns 0 until the element is mounted and measured.
 */
export const useElementWidth = <T extends HTMLElement>(
  ref: RefObject<T | null>,
): number => {
  const [width, setWidth] = useState(0);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;

    const update = () => setWidth(el.clientWidth);
    update();

    const observer = new ResizeObserver(update);
    observer.observe(el);
    return () => observer.disconnect();
  }, [ref]);

  return width;
};
