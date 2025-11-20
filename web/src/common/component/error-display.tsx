import type React from 'react';

interface ErrorDisplayProps {
  description: string;
  errorDetails?: string | Error | null;
}

const getErrorMessage = (error: unknown): string => {
  if (!error) return 'No error details available';
  if (typeof error === 'string') return error;
  if (error instanceof Error) return error.message;
  return JSON.stringify(error, null, 2);
};

const ErrorDisplay: React.FC<ErrorDisplayProps> = ({ description, errorDetails }) => {
  if (!description && !errorDetails) return null;

  const message = errorDetails ? getErrorMessage(errorDetails) : null;

  const handleContainerClick = (e: React.MouseEvent) => {
    e.stopPropagation();
  };

  const handleContainerKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      (e.currentTarget as HTMLElement).blur();
    }
  };

  return (
    <div
      role="alert"
      aria-live="assertive"
      className="w-full h-full flex flex-col bg-red-50 border-1 border-dashed border-red-500 rounded-lg p-4"
      aria-label="Error notification"
      onClick={handleContainerClick}
      onKeyDown={handleContainerKeyDown}
    >
      <div className="flex-0 mb-3">
        <h3 className="font-bold text-red-700 text-lg">{description}</h3>
      </div>

      {message && (
        <div className="flex-1 min-h-0 overflow-hidden">
          <pre className="w-full h-full bg-white text-red-600 p-3 rounded border border-red-200 overflow-auto text-sm">
            {message}
          </pre>
        </div>
      )}
    </div>
  );
};

export default ErrorDisplay;
