import { ChevronDown, ChevronUp } from "lucide-react";
import { QuestionHeader } from "./ui/question-header";

export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="flex flex-col w-full">
      <QuestionHeader />
      <div className="px-4 py-4 max-w-4xl mx-auto">
        <div className="flex flex-row gap-4 items-center">
          <div className="flex flex-col">
            <h1 className="text-xl font-medium mb-1">
              How do I fix authentication bug in login flow?
            </h1>
          </div>
        </div>

        <div className="flex gap-4">
          <div className="flex-1">
            <div className="text-sm">
              <p className="mb-4">
                The authentication flow is broken when users try to log in with
                social providers. Getting a 401 error on callback.
              </p>
              <pre className="bg-muted p-3 rounded text-sm overflow-x-auto">
                Error: Invalid token received from OAuth provider
              </pre>
            </div>
          </div>
        </div>

        <div className="flex items-center gap-2 mt-4 pt-4 border-t">
          <button className="text-muted-foreground hover:text-foreground">
            <ChevronUp className="w-5 h-5" />
          </button>
          <span className="text-sm">42</span>
          <button className="text-muted-foreground hover:text-foreground">
            <ChevronDown className="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  );
}
