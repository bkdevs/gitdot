"use client";

import { useState } from "react";
import type { OrganizationResponse, UserResponse } from "@/lib/dto";

export function CreateMigrationForm({
  user,
  organizations,
}: {
  user: UserResponse;
  organizations: OrganizationResponse[];
}) {
  const [destination, setDestination] = useState(user.name);

  return (
    <>
      <h1 className="text-lg font-medium border-b border-border pb-2 mb-4">
        Start new migration
      </h1>
      <form className="space-y-4">
        <div className="flex flex-col gap-1">
          <label
            htmlFor="destination"
            className="text-xs text-muted-foreground"
          >
            Destination
          </label>
          <select
            id="destination"
            name="destination"
            value={destination}
            onChange={(e) => setDestination(e.target.value)}
            className="w-full p-2 text-sm bg-background border border-border rounded outline-none"
          >
            <option value={user.name}>{user.name} (personal)</option>
            {organizations.map((org) => (
              <option key={org.id} value={org.name}>
                {org.name}
              </option>
            ))}
          </select>
        </div>
      </form>
    </>
  );
}
