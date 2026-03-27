"use client";

import type { WebhookResource } from "gitdot-api";
import { useState } from "react";
import { createWebhookAction, deleteWebhookAction } from "@/actions/webhook";

export function RepositorySettingsWebhooks({
  owner,
  repo,
  webhooks: initialWebhooks,
}: {
  owner: string;
  repo: string;
  webhooks: WebhookResource[];
}) {
  const [webhooks, setWebhooks] = useState(initialWebhooks);
  const [showForm, setShowForm] = useState(false);
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleCreate(formData: FormData) {
    setPending(true);
    setError(null);
    const result = await createWebhookAction(owner, repo, formData);
    setPending(false);
    if ("error" in result) {
      setError(result.error);
      return;
    }
    setWebhooks((prev) => [result.webhook, ...prev]);
    setShowForm(false);
  }

  async function handleDelete(webhookId: string) {
    const result = await deleteWebhookAction(owner, repo, webhookId);
    if ("error" in result) {
      setError(result.error);
      return;
    }
    setWebhooks((prev) => prev.filter((w) => w.id !== webhookId));
  }

  return (
    <div className="flex flex-col w-full">
      <div className="flex items-center justify-between border-b px-2 h-9">
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Webhooks
        </h3>
        <button
          type="button"
          className="text-xs text-foreground hover:bg-muted border-l border-border flex items-center h-9 pl-2 cursor-pointer"
          onClick={() => setShowForm(!showForm)}
        >
          {showForm ? "Cancel" : "Add webhook"}
        </button>
      </div>

      {error && (
        <div className="px-2 py-1 text-xs text-destructive">{error}</div>
      )}

      {showForm && (
        <form
          action={handleCreate}
          className="flex flex-col gap-2 px-2 py-2 border-b"
        >
          <input
            name="url"
            type="url"
            placeholder="https://example.com/webhook"
            required
            className="text-sm px-2 py-1 border border-border rounded bg-background"
          />
          <input
            name="secret"
            type="text"
            placeholder="Secret"
            required
            className="text-sm px-2 py-1 border border-border rounded bg-background"
          />
          <div className="flex items-center gap-2">
            <label className="flex items-center gap-1 text-xs">
              <input
                type="checkbox"
                name="events"
                value="push"
                defaultChecked
              />
              push
            </label>
          </div>
          <button
            type="submit"
            disabled={pending}
            className="text-xs px-2 py-1 bg-foreground text-background rounded cursor-pointer disabled:opacity-50 self-start"
          >
            {pending ? "Creating..." : "Create"}
          </button>
        </form>
      )}

      {webhooks.length === 0 && !showForm && (
        <div className="px-2 py-2 text-xs text-muted-foreground">
          No webhooks configured.
        </div>
      )}

      {webhooks.map((webhook) => (
        <div
          key={webhook.id}
          className="flex items-center justify-between px-2 py-2 border-b h-9"
        >
          <div className="flex items-center gap-2 min-w-0">
            <span className="text-sm truncate">{webhook.url}</span>
            <span className="text-xs text-muted-foreground">
              {webhook.events.join(", ")}
            </span>
          </div>
          <button
            type="button"
            className="text-xs text-destructive hover:bg-muted border-l border-border flex items-center h-9 pl-2 cursor-pointer"
            onClick={() => handleDelete(webhook.id)}
          >
            Delete
          </button>
        </div>
      ))}
    </div>
  );
}
