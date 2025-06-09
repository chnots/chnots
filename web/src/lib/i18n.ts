// Represents a typed translation function.
type TypedT = (key: any, params?: Record<string, any>) => string;

export const useTranslate = (): TypedT => {
  return (s) => s;
};
