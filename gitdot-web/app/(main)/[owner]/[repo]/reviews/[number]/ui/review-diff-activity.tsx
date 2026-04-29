"use client";

import { AvatarBeam } from "@/ui/avatar-beam";

const MOCK_ACTIVITY = [
  { user: "pybae", action: "published diff #1", time: "2h ago" },
  { user: "mikkelk", action: "left 3 comments", time: "1h ago" },
  { user: "pybae", action: "published diff #2", time: "45m ago" },
  { user: "mikkelk", action: "approved diff #2", time: "10m ago" },
];

export function ReviewDiffActivity() {
  return (
    <div className="mx-16 px-1 py-4 flex flex-col gap-2">
      <div className="flex flex-col gap-3">
      {[...MOCK_ACTIVITY].reverse().map((item, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: mock data
        <div key={i} className="flex items-center gap-2">
          <AvatarBeam name={item.user} size={18} />
          <span className="text-xs font-mono text-muted-foreground">
            {item.user} {item.action}{" "}
            {item.time}
          </span>
        </div>
      ))}
      </div>
    </div>
  );
}
