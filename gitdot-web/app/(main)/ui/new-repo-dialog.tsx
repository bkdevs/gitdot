"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { ChevronDown } from "lucide-react";
import { useRouter } from "next/navigation";
import { useActionState, useEffect, useState } from "react";
import { OrgImage } from "@/(main)/[owner]/ui/org/org-image";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { useUserContext } from "@/(main)/provider/user";
import { createRepositoryAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";

export function NewRepoDialog() {
  const { user, memberships } = useUserContext();
  const router = useRouter();

  const [open, setOpen] = useState(false);
  const [owner, setOwner] = useState("");
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [visibility, setVisibility] = useState<"public" | "private">("public");
  const [readme, setReadme] = useState(false);
  const [gitignore, setGitignore] = useState(false);
  const [license, setLicense] = useState(false);

  useEffect(() => {
    if (!open) {
      setName("");
      setDescription("");
      setVisibility("public");
      setReadme(false);
      setGitignore(false);
      setLicense(false);
    }
  }, [open]);

  const ownerType = owner === user?.name ? "user" : "organization";
  const selectedMembership = memberships?.find((m) => m.org_name === owner);

  const [state, formAction, isPending] = useActionState(
    createRepositoryAction,
    null,
  );

  useEffect(() => {
    if (state && "repository" in state) {
      setOpen(false);
      router.push(`/${state.repository.owner}/${state.repository.name}`);
    }
  }, [state, router]);

  useEffect(() => {
    const handle = (e: Event) => {
      if (!user) return;
      const detail = (e as CustomEvent<{ owner?: string }>).detail;
      setOwner(detail?.owner ?? user.name);
      setOpen(true);
    };
    window.addEventListener("openNewRepo", handle);
    return () => window.removeEventListener("openNewRepo", handle);
  }, [user]);

  const isValid = name.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-xl min-w-xl border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>New repository</DialogTitle>
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <div className="flex">
            <div className="flex flex-col w-2/3 border-r border-border">
              <div className="group flex flex-col gap-1 p-2 border-b border-border">
                <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
                  <span className="text-foreground/40 select-none"># </span>
                  Name
                </p>
                <input
                  type="text"
                  id="repo-name"
                  name="repo-name"
                  placeholder="my-next-repo"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  className="w-full text-sm bg-background outline-none"
                  disabled={isPending}
                />
              </div>
              <div className="group flex flex-col flex-1 gap-1 p-2 border-b border-border">
                <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
                  <span className="text-foreground/40 select-none"># </span>
                  Description
                </p>
                <textarea
                  id="repo-description"
                  name="repo-description"
                  placeholder="what it does and what it will do...."
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  className="w-full flex-1 text-sm bg-background outline-none resize-none"
                  disabled={isPending}
                />
              </div>
            </div>
            <div className="flex flex-col gap-2 w-1/3 p-2 border-b border-border">
              <div className="pb-4">
                <h2 className="text-sm font-medium dark:font-normal">
                  New repository
                </h2>
                <p className="text-xs text-muted-foreground leading-normal">
                  A new home for your code and its history. Have a repo already?{" "}
                  <button
                    type="button"
                    className="appearance-none p-0 m-0 bg-transparent border-0 underline hover:text-foreground transition-colors cursor-pointer"
                  >
                    Import it.
                  </button>
                </p>
              </div>
              <div className="flex flex-col gap-1 mt-1 text-xs">
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground">Owner:</span>
                  <DropdownMenu>
                    <DropdownMenuTrigger
                      disabled={isPending}
                      className="flex items-center gap-1.5 hover:text-muted-foreground transition-colors cursor-pointer"
                    >
                      {selectedMembership ? (
                        <OrgImage
                          orgId={selectedMembership.organization_id}
                          px={14}
                        />
                      ) : (
                        <UserImage userId={user?.id} px={14} />
                      )}
                      {owner}
                      <ChevronDown className="size-3" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="min-w-32">
                      {user && (
                        <DropdownMenuItem
                          className="text-xs"
                          onClick={() => setOwner(user.name)}
                        >
                          <UserImage userId={user.id} px={14} />
                          {user.name}
                        </DropdownMenuItem>
                      )}
                      {memberships?.map((m) => (
                        <DropdownMenuItem
                          key={m.organization_id}
                          className="text-xs"
                          onClick={() => setOwner(m.org_name)}
                        >
                          <OrgImage orgId={m.organization_id} px={14} />
                          {m.org_name}
                        </DropdownMenuItem>
                      ))}
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground">Visibility:</span>
                  <DropdownMenu>
                    <DropdownMenuTrigger
                      disabled={isPending}
                      className="flex items-center gap-1 hover:text-muted-foreground transition-colors cursor-pointer"
                    >
                      {visibility}
                      <ChevronDown className="size-3" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="min-w-20">
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setVisibility("public")}
                      >
                        public
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setVisibility("private")}
                      >
                        private
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground">README:</span>
                  <DropdownMenu>
                    <DropdownMenuTrigger
                      disabled={isPending}
                      className="flex items-center gap-1 hover:text-muted-foreground transition-colors cursor-pointer"
                    >
                      {readme ? "yes" : "no"}
                      <ChevronDown className="size-3" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="min-w-20">
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setReadme(true)}
                      >
                        yes
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setReadme(false)}
                      >
                        no
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground">.gitignore:</span>
                  <DropdownMenu>
                    <DropdownMenuTrigger
                      disabled={isPending}
                      className="flex items-center gap-1 hover:text-muted-foreground transition-colors cursor-pointer"
                    >
                      {gitignore ? "yes" : "no"}
                      <ChevronDown className="size-3" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="min-w-20">
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setGitignore(true)}
                      >
                        yes
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setGitignore(false)}
                      >
                        no
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-muted-foreground">LICENSE:</span>
                  <DropdownMenu>
                    <DropdownMenuTrigger
                      disabled={isPending}
                      className="flex items-center gap-1 hover:text-muted-foreground transition-colors cursor-pointer"
                    >
                      {license ? "yes" : "no"}
                      <ChevronDown className="size-3" />
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end" className="min-w-20">
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setLicense(true)}
                      >
                        yes
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        className="text-xs"
                        onClick={() => setLicense(false)}
                      >
                        no
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              </div>
            </div>
          </div>
          <div className="flex items-center justify-between h-7">
            <input type="hidden" name="visibility" value={visibility} />
            <input type="hidden" name="owner" value={owner} />
            <input type="hidden" name="owner_type" value={ownerType} />
            <input type="hidden" name="repo-description" value={description} />
            <span
              className={cn(
                "pl-2 text-xs truncate",
                state && "error" in state
                  ? "text-red-500"
                  : "text-muted-foreground",
              )}
            >
              {state && "error" in state
                ? state.error
                : "Create a new repository"}
            </span>
            <div className="flex items-center h-full">
              <button
                type="reset"
                onClick={() => setOpen(false)}
                className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || isPending}
                className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed cursor-pointer"
              >
                {isPending ? "Creating..." : "Create"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
