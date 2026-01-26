import Markdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

export function MarkdownBody({ content }: { content: string }) {
  return (
    <Markdown
      remarkPlugins={[remarkGfm]}
      components={{
        h1: ({ node, ...props }) => (
          <h1 className="text-3xl font-bold mb-4 border-b pb-2" {...props} />
        ),
        h2: ({ node, ...props }) => (
          <h2 className="text-xl font-semibold mb-3" {...props} />
        ),
        h3: ({ node, ...props }) => (
          <h3 className="text-lg font-medium mb-2" {...props} />
        ),

        p: ({ node, ...props }) => (
          <p className="leading-relaxed text-sm mb-4" {...props} />
        ),
        a: ({ node, ...props }) => (
          <a className="text-sm underline underline-offset-4 decoration-1 hover:decoration-2 transition-all" {...props} />
        ),
        blockquote: ({ node, ...props }) => (
          <blockquote className="text-sm border-l-4 border-current pl-4 italic my-4 opacity-80" {...props} />
        ),

        ul: ({ node, ...props }) => (
          <ul className="list-disc list-outside ml-6 mb-4 space-y-1" {...props} />
        ),
        ol: ({ node, ...props }) => (
          <ol className="list-decimal list-outside ml-6 mb-4 space-y-1" {...props} />
        ),
        li: ({ node, ...props }) => (
          <li className="text-sm" {...props} />
        ),

        code: ({ node, className, children, ...props }) => {
          return <code className="bg-black/5 dark:bg-white/10 px-1.5 py-0.5 rounded font-mono text-sm" {...props}>
            {children}
          </code>
        },

        table: ({ node, ...props }) => (
          <div className="text-sm overflow-x-auto mb-6">
            <table className="min-w-full divide-y divide-current border border-current/20" {...props} />
          </div>
        ),
        th: ({ node, ...props }) => (
          <th className="px-3 py-3.5 text-left text-sm font-semibold bg-black/5 dark:bg-white/5" {...props} />
        ),
        td: ({ node, ...props }) => (
          <td className="px-3 py-4 text-sm border-t border-current/10" {...props} />
        ),

        img: ({ node, ...props }) => (
          <img className="rounded-xl my-8 mx-auto max-w-full h-auto" {...props} alt={props.alt || ''} />
        ),
        hr: () => <hr className="my-8 border-t border-current/20" />,
      }}
    >
      {content}
    </Markdown>
  )
}
