"use client";

export function UserActions() {
  const actions: { label: string; onClick: () => void }[] = [
    {
      label: "new repo",
      onClick: () => window.dispatchEvent(new CustomEvent("openNewRepo")),
    },
    {
      label: "new org",
      onClick: () => window.dispatchEvent(new CustomEvent("openNewOrg")),
    },
    {
      label: "new migration",
      onClick: () => window.dispatchEvent(new CustomEvent("openMigrateRepo")),
    },
    {
      label: "settings",
      onClick: () => window.dispatchEvent(new CustomEvent("openSettings")),
    },
  ];

  return (
    <div className="flex flex-col items-start">
      <p className="font-semibold text-sm mb-0.5">actions</p>
      {actions.map((action) => (
        <button
          key={action.label}
          type="button"
          onClick={action.onClick}
          className="text-xs underline decoration-transparent hover:decoration-current transition-colors duration-200 cursor-pointer"
        >
          {action.label}
        </button>
      ))}
    </div>
  );
}
