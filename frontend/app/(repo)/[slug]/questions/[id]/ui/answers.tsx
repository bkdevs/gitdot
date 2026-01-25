import { Comments } from "./comments"
import { VoteBox } from "./vote-box"

export function Answers() {
  return (
    <div className="flex flex-col w-full">

      <div className="flex justify-between items-center pl-4 py-8 max-w-4xl">
        <span className="text-lg font-medium">2 Answers</span>
        <div className="flex items-center gap-2 text-sm">
          <span className="text-muted-foreground">Sorted by:</span>
          <select className="bg-transparent border border-border rounded px-2 py-1">
            <option>Highest score</option>
            <option>Date modified</option>
            <option>Date created</option>
          </select>
        </div>
      </div>

      <div className="flex flex-col gap-16">
        <Answer />
        <Answer/>
      </div>
    </div>
  )
}


function Answer() {
  return (
    <div className="flex pl-8 max-w-4xl">
      <VoteBox score={12} />

      <div className="flex-1 ml-8">
        <div className="text-sm">
          <p className="mb-4">
            This is a known issue with the latest OAuth library update. The
            token validation endpoint changed their response format.
          </p>
          <p className="mb-4">
            You need to update your validateToken function to handle the new
            response structure:
          </p>
          <pre className="bg-muted p-3 rounded text-sm overflow-x-auto mb-4">{`export async function validateToken(token: string) {
            const response = await fetch('/api/validate', {
            headers: { Authorization: \`Bearer \${token}\` }
            });
            const data = await response.json();
            // New format wraps user in a 'data' object
            return data.data.user;
            }`}</pre>
          <p>
            This fixed the issue for me in production. Make sure to also
            clear your Redis cache if you're caching tokens.
          </p>
        </div>
        <div className="flex justify-between text-sm mt-8">
          <div className="flex gap-4">
            <span>
              <span className="text-muted-foreground">Answered</span> 2
              hours ago
            </span>
          </div>
          <div className="flex gap-2 text-muted-foreground">
            <span className="underline">janedoe</span>
            <span>5,678</span>
          </div>
        </div>

        <div className="mt-2 mb-2 border border-gray-100" />
        <Comments />
      </div>
    </div>

  )
}
