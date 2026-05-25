"use client";

import { createContext, useContext, useState } from "react";

const TimezoneContext = createContext<string>("UTC");

export function TimezoneProvider({
  timezone,
  children,
}: {
  timezone: string;
  children: React.ReactNode;
}) {
  const [tz] = useState(timezone);

  return (
    <TimezoneContext.Provider value={tz}>{children}</TimezoneContext.Provider>
  );
}

export function useTimezone(): string {
  return useContext(TimezoneContext);
}
