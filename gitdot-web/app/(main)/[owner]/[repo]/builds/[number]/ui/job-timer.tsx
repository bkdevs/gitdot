"use client";

import { useEffect, useState } from "react";
import { formatDuration } from "@/util";

export function JobTimer({
  createdAt,
  updatedAt,
  running,
}: {
  createdAt: Date;
  updatedAt: Date;
  running: boolean;
}) {
  const [now, setNow] = useState(() => Date.now());

  useEffect(() => {
    if (!running) return;
    const id = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(id);
  }, [running]);

  const elapsed = running
    ? now - createdAt.getTime()
    : updatedAt.getTime() - createdAt.getTime();

  return <>{formatDuration(elapsed)}</>;
}
