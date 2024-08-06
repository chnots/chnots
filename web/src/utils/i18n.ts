// Represents a typed translation function.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type TypedT = (key: any, params?: Record<string, any>) => string;

export const useTranslate = (): TypedT => {
  return (s) => s;
};
