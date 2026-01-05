export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="p-6 max-w-5xl mx-auto">
      <div className="flex items-start gap-4 mb-4">
        <h1 className="text-2xl font-semibold flex-1">
          feat: Add user authentication system
          <span className="text-muted-foreground ml-2">#{params.id}</span>
        </h1>
      </div>

      <div className="flex items-center gap-2 mb-6 text-sm">
        <span className="px-2 py-1 bg-green-500/10 text-green-600 dark:text-green-400 rounded-full border border-green-500/20">
          Open
        </span>
        <span className="text-muted-foreground">
          <strong>contributor789</strong> wants to merge 12 commits into{" "}
          <code className="px-1.5 py-0.5 bg-muted rounded">main</code> from{" "}
          <code className="px-1.5 py-0.5 bg-muted rounded">feature/auth</code>
        </span>
      </div>

      <div className="border rounded-lg mb-4">
        <div className="border-b p-4 bg-muted/30">
          <div className="flex items-center gap-2 text-sm">
            <strong>contributor789</strong>
            <span className="text-muted-foreground">commented 3 days ago</span>
          </div>
        </div>
        <div className="p-4 text-sm">
          <p className="mb-3">
            This PR implements a complete authentication system with:
          </p>
          <ul className="list-disc list-inside space-y-1 text-muted-foreground">
            <li>JWT-based authentication</li>
            <li>OAuth provider integration</li>
            <li>Session management</li>
            <li>Password reset functionality</li>
          </ul>
        </div>
      </div>

      <div className="flex gap-4 mb-4">
        <div className="flex-1 border rounded-lg p-4">
          <div className="text-sm font-semibold mb-2">Commits</div>
          <div className="text-2xl font-semibold">12</div>
        </div>
        <div className="flex-1 border rounded-lg p-4">
          <div className="text-sm font-semibold mb-2">Files changed</div>
          <div className="text-2xl font-semibold">24</div>
        </div>
        <div className="flex-1 border rounded-lg p-4">
          <div className="text-sm font-semibold mb-2">Contributors</div>
          <div className="text-2xl font-semibold">2</div>
        </div>
      </div>

      <div className="border rounded-lg bg-green-500/5 border-green-500/20 p-4">
        <div className="flex items-center gap-2 text-sm text-green-600 dark:text-green-400">
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 16 16">
            <title>Checkmark</title>
            <path d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z" />
          </svg>
          <span className="font-semibold">All checks have passed</span>
        </div>
      </div>
    </div>
  );
}
