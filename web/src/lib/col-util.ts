export function arraysAreEqual<T>(
  arr1: T[],
  arr2: T[],
  compare?: (v1: T, v2: T) => boolean,
): boolean {
  if (arr1.length !== arr2.length) {
    return false;
  }

  for (let i = 0; i < arr1.length; i++) {
    if (arr1[i] !== arr2[i] || (compare && compare(arr1[i], arr2[i]))) {
      return false;
    }
  }

  return true;
}
