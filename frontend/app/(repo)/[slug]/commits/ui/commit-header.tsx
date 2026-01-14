import type { RepositoryCommit } from "@/lib/dto";
import { timeAgo } from "@/util";

export default function CommitHeader({ commit }: { commit: RepositoryCommit }) {
  return (
    <div className="px-2 py-3">
      <div className="flex items-center gap-2 text-xs text-gray-600 mb-2">
        <span>{commit.author}</span>
        <span>•</span>
        <span>{timeAgo(new Date(commit.date))}</span>
        <span>•</span>
        <span className="font-mono">{commit.sha.slice(0, 7)}</span>
      </div>
      <div className="text-sm font-semibold mb-1">
        {commit.message.split("\n")[0]}
      </div>
      <div className="text-sm text-gray-700 leading-relaxed">
        {commit.message.split("\n").slice(1).join("\n")}
      </div>
    </div>
  );
}
