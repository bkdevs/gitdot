import { useEffect, useState } from "react";

export function useTypewriter(text: string, speed = 25, loopPauseMs?: number) {
  const [displayed, setDisplayed] = useState("");

  useEffect(() => {
    setDisplayed("");
    if (!text) return;
    let i = 0;
    let timer: ReturnType<typeof setTimeout> | null = null;
    const tick = () => {
      i++;
      setDisplayed(text.slice(0, i));
      if (i < text.length) {
        timer = setTimeout(tick, speed);
      } else if (loopPauseMs !== undefined) {
        timer = setTimeout(() => {
          i = 0;
          setDisplayed("");
          timer = setTimeout(tick, speed);
        }, loopPauseMs);
      }
    };
    timer = setTimeout(tick, speed);
    return () => {
      if (timer) clearTimeout(timer);
    };
  }, [text, speed, loopPauseMs]);

  return displayed;
}
