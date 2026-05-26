/// from excalidraw packages/common/src/utils.ts

export type ResolvablePromise<T> = Promise<T> & {
  resolve: [T] extends [undefined] ? (value?: T) => void : (value: T) => void;
  reject: (error: Error) => void;
};

export const resolvablePromise = <T>() => {
  let resolve: (value: T) => void;
  let reject: (error: Error) => void;
  const promise = new Promise<T>((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });
  const result = promise as ResolvablePromise<T>;
  // Conditional type requires assertion — runtime signature is correct
  result.resolve = resolve! as typeof result.resolve;
  result.reject = reject!;
  return result;
};
