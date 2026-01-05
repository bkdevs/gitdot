export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="p-6 max-w-5xl mx-auto">
      <div className="flex items-start gap-4 mb-4">
        <h1 className="text-2xl font-semibold flex-1">
          Fix authentication bug in login flow
          <span className="text-muted-foreground ml-2">#{params.id}</span>
        </h1>
      </div>

      <div className="flex items-center gap-2 mb-6 text-sm">
        <span className="px-2 py-1 bg-green-500/10 text-green-600 dark:text-green-400 rounded-full border border-green-500/20">
          Open
        </span>
        <span className="text-muted-foreground">
          <strong>user123</strong> opened this issue 2 days ago · 3 comments
        </span>
      </div>

      <div className="border rounded-lg">
        <div className="border-b p-4 bg-muted/30">
          <div className="flex items-center gap-2 text-sm">
            <strong>user123</strong>
            <span className="text-muted-foreground">commented 2 days ago</span>
          </div>
        </div>
        <div className="p-4">
          <p className="text-sm mb-4">
            The authentication flow is broken when users try to log in with
            social providers. Getting a 401 error on callback.
          </p>
          <div className="bg-muted/50 p-3 rounded text-sm font-mono">
            Error: Invalid token received from OAuth provider
          </div>
        </div>
      </div>

      <div className="mt-4 border rounded-lg">
        <div className="border-b p-4 bg-muted/30">
          <div className="flex items-center gap-2 text-sm">
            <strong>developer456</strong>
            <span className="text-muted-foreground">commented 1 day ago</span>
          </div>
        </div>
        <div className="p-4 text-sm">
          <p>Looking into this. Seems like the token validation is failing.</p>
        </div>
      </div>
    </div>
  );
}
