"use client";

import { createRunnerAction } from "@/actions";
import type { UserResponse } from "@/lib/dto";
import Link from "@/ui/link";
import { useActionState, useState } from "react";

export default function CreateRunnerForm({
  user,
}: {
  user: UserResponse;
}) {
  const [name, setName] = useState("");
  const [state, formAction, isPending] = useActionState(
    createRunnerAction,
    null,
  );

  const ownerName = user.name;
  const ownerType = "user";

  const showToken = state && "token" in state;

  return (
    <>
      <h1 className="text-lg font-medium border-b border-border pb-2 mb-4">
        Register new runner
      </h1>
      {showToken ? (
        <div className="space-y-4">
          <p className="text-sm text-muted-foreground">
            Runner created. Copy the token below to register your runner.
          </p>
          <div className="flex flex-col gap-1">
            <label
              htmlFor="runner-token"
              className="text-xs text-muted-foreground"
            >
              Token
            </label>
            <input
              id="runner-token"
              type="text"
              readOnly
              value={state.token}
              className="w-full p-2 text-sm bg-muted border border-border rounded font-mono"
            />
          </div>
          <Link
            href="/settings/runners"
            className="text-sm text-primary hover:underline"
            prefetch={true}
          >
            Back to runners
          </Link>
        </div>
      ) : (
        <form action={formAction} className="space-y-4">
          <input type="hidden" name="owner_name" value={ownerName} />
          <input type="hidden" name="owner_type" value={ownerType} />
          <div className="flex flex-col gap-1">
            <label htmlFor="name" className="text-xs text-muted-foreground">
              Name
            </label>
            <input
              type="text"
              id="name"
              name="name"
              placeholder="my-runner"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full p-2 text-sm bg-background border border-border rounded outline-none"
              disabled={isPending}
            />
          </div>
          <div className="flex flex-col gap-1">
            <label
              htmlFor="owner_name"
              className="text-xs text-muted-foreground"
            >
              Owner
            </label>
            <input
              type="text"
              id="owner_name"
              name="owner_name"
              value={ownerName}
              className="w-full p-2 text-sm bg-muted border border-border rounded outline-none cursor-not-allowed"
              disabled={true}
            />
          </div>
          <div className="flex flex-col gap-1">
            <label
              htmlFor="owner_type"
              className="text-xs text-muted-foreground"
            >
              Owner type
            </label>
            <select
              id="owner_type"
              name="owner_type"
              value={ownerType}
              className="w-full p-2 text-sm bg-muted border border-border rounded outline-none cursor-not-allowed"
              disabled={true}
            >
              <option value="user">User</option>
            </select>
          </div>
          {state && "error" in state && (
            <p className="text-sm text-red-500">{state.error}</p>
          )}
          <button
            type="submit"
            disabled={!name.trim() || !ownerName.trim() || isPending}
            className="px-3 py-2 text-sm bg-primary text-primary-foreground rounded disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isPending ? "Creating..." : "Create runner"}
          </button>
        </form>
      )}
    </>
  );
}
