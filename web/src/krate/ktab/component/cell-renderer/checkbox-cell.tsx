import type { CellRendererProps } from "./types";

export const CheckboxCell = ({
  value,
  onCommit,
  readonly,
}: CellRendererProps) => {
  const checked = !!value && value !== 0 && value !== "0" && value !== "false";

  return (
    <div className="flex items-center justify-center w-full min-h-[28px]">
      <input
        type="checkbox"
        checked={checked}
        disabled={readonly}
        onChange={(e) => {
          onCommit(e.target.checked);
        }}
        className="h-4 w-4 rounded border-border accent-primary cursor-pointer"
      />
    </div>
  );
};
