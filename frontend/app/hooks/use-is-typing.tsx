import { useEffect, useState } from "react";

export const useIsTyping = (...values: string[]) => {
  const [typing, setTyping] = useState(false);

  useEffect(() => {
    setTyping(true);

    const timeout = setTimeout(() => {
      setTyping(false);
    }, 150);

    return () => clearTimeout(timeout);
  }, [...values]);

  return typing;
};
