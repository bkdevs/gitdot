"use client";

import { useState } from "react";
import { createRepositoryAction } from "@/actions";
import { Button } from "@/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
} from "@/ui/dialog";
import { Input } from "@/ui/input";

export default function CreateRepoButton() {
  const [open, setOpen] = useState(false);
  const [owner, setOwner] = useState("");
  const [repoName, setRepoName] = useState("");
  const [visibility, setVisibility] = useState("public");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    setLoading(true);

    const formData = new FormData();
    formData.set("visibility", visibility);

    const result = await createRepositoryAction(owner, repoName, formData);

    setLoading(false);

    if (result?.error) {
      setError(result.error);
    } else {
      setOpen(false);
      setOwner("");
      setRepoName("");
      setVisibility("public");
    }
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">New Repository</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogTitle>Create a new repository</DialogTitle>
        <DialogDescription>
          A repository contains all project files, including the revision
          history.
        </DialogDescription>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4 mt-2">
          <div className="flex flex-col gap-2">
            <label htmlFor="owner" className="text-sm font-medium">
              Owner
            </label>
            <Input
              id="owner"
              value={owner}
              onChange={(e) => setOwner(e.target.value)}
              placeholder="username"
              required
            />
          </div>
          <div className="flex flex-col gap-2">
            <label htmlFor="repo-name" className="text-sm font-medium">
              Repository name
            </label>
            <Input
              id="repo-name"
              value={repoName}
              onChange={(e) => setRepoName(e.target.value)}
              placeholder="my-awesome-project"
              required
            />
          </div>
          <div className="flex flex-col gap-2">
            <label htmlFor="visibility" className="text-sm font-medium">
              Visibility
            </label>
            <select
              id="visibility"
              value={visibility}
              onChange={(e) => setVisibility(e.target.value)}
              className="h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm"
            >
              <option value="public">Public</option>
              <option value="private">Private</option>
            </select>
          </div>
          {error && <p className="text-sm text-destructive">{error}</p>}
          <div className="flex justify-end gap-2 mt-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setOpen(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={loading || !owner || !repoName}>
              {loading ? "Creating..." : "Create repository"}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
