export function insertMapAtIndex<K, V>(index: number, key: K, value: V, map: Map<K, V>) {
    const arr = Array.from(map);
    arr.splice(index, 0, [key, value]);
    return new Map(arr);
  }