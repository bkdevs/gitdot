"use client";

import { useReportWebVitals } from "next/web-vitals";

const logWebVitals = (metric: any) => {
  console.log(metric);
};

export function WebVitals() {
  useReportWebVitals(logWebVitals);

  return null;
}
