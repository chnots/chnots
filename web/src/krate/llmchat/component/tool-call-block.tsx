import { ChevronDown, ChevronRight, Loader2, X } from "lucide-react";
import { useState } from "react";
import { Button } from "@/common/component/ui/button";

type ToolCallBlockProps = {
  toolName: string;
  toolCallId: string;
  state:
    | "input-streaming"
    | "input-available"
    | "approval-requested"
    | "approval-responded"
    | "output-available"
    | "output-error"
    | "output-denied";
  input?: unknown;
  output?: unknown;
  errorText?: string;
  approved?: boolean;
  onApprove?: (toolCallId: string) => void;
  onDeny?: (toolCallId: string) => void;
};

const JsonPreview = ({
  data,
  collapsed = false,
}: {
  data: unknown;
  collapsed?: boolean;
}) => {
  const [open, setOpen] = useState(!collapsed);
  const json = JSON.stringify(data, null, 2);

  return (
    <div className="text-xs">
      <button
        type="button"
        className="flex items-center gap-1 text-muted-foreground hover:text-foreground"
        onClick={() => setOpen((p) => !p)}
      >
        {open ? (
          <ChevronDown className="h-3 w-3" />
        ) : (
          <ChevronRight className="h-3 w-3" />
        )}
        <span className="font-mono">{open ? "Hide" : "Show"}</span>
      </button>
      {open && (
        <pre className="mt-1 max-h-40 overflow-auto rounded-md bg-muted/50 p-2 font-mono text-xs">
          {json}
        </pre>
      )}
    </div>
  );
};

export function ToolCallBlock({
  toolName,
  toolCallId,
  state,
  input,
  output,
  errorText,
  onApprove,
  onDeny,
}: ToolCallBlockProps) {
  const [expanded, setExpanded] = useState(false);

  const stateLabel: Record<ToolCallBlockProps["state"], string> = {
    "input-streaming": "Calling...",
    "input-available": "Called",
    "approval-requested": "Awaiting Approval",
    "approval-responded": "Approved",
    "output-available": "Completed",
    "output-error": "Error",
    "output-denied": "Denied",
  };

  const stateIcon =
    state === "input-streaming" ? (
      <Loader2 className="h-3 w-3 animate-spin" />
    ) : state === "output-error" || state === "output-denied" ? (
      <X className="h-3 w-3 text-destructive" />
    ) : null;

  return (
    <div className="my-2 rounded-lg border bg-muted/30">
      <button
        type="button"
        className="flex w-full items-center gap-2 px-3 py-2 text-sm"
        onClick={() => setExpanded((p) => !p)}
      >
        {expanded ? (
          <ChevronDown className="h-3 w-3" />
        ) : (
          <ChevronRight className="h-3 w-3" />
        )}
        <span className="font-mono font-medium">{toolName}</span>
        <span className="flex items-center gap-1 text-xs text-muted-foreground">
          {stateIcon}
          {stateLabel[state]}
        </span>
      </button>

      {expanded && (
        <div className="border-t px-3 py-2 space-y-2">
          {input !== undefined && input !== null && (
            <div>
              <div className="text-xs font-medium text-muted-foreground mb-1">
                Input
              </div>
              <JsonPreview data={input} />
            </div>
          )}

          {output !== undefined && output !== null && (
            <div>
              <div className="text-xs font-medium text-muted-foreground mb-1">
                Output
              </div>
              <JsonPreview data={output} collapsed />
            </div>
          )}

          {errorText && (
            <div className="rounded-md bg-destructive/10 p-2 text-xs text-destructive">
              {errorText}
            </div>
          )}

          {state === "approval-requested" && (
            <div className="flex gap-2 pt-1">
              {onApprove && (
                <Button size="sm" onClick={() => onApprove(toolCallId)}>
                  Approve
                </Button>
              )}
              {onDeny && (
                <Button
                  size="sm"
                  variant="destructive"
                  onClick={() => onDeny(toolCallId)}
                >
                  Deny
                </Button>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
