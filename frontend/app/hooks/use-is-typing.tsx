import { useEffect, useState } from "react";

export const useIsTyping = (value: string, delay = 150) => {
  const [typing, setTyping] = useState(false);

  useEffect(() => {
    setTyping(true);

    const timeout = setTimeout(() => {
      setTyping(false);
    }, delay);

    return () => clearTimeout(timeout);
  }, [value, delay]);

  return typing;
};
