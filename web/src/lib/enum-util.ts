
/**
 * https://stackoverflow.com/questions/17380845/how-do-i-convert-a-string-to-enum-in-typescript
 * @param enm enum
 * @param value 
 * @returns 
 */
export const enumFromStringValue = <T>(
  enm: { [s: string]: T },
  value?: string,
  def?: T
): T | undefined => {
  if (!value) {
    return def;
  }
  return (Object.values(enm) as unknown as string[]).includes(value)
    ? (value as unknown as T)
    : def;
};