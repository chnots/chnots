import * as React from "react";

type DebounceProps = {
  handleDebounce: (value: string) => void;
  debounceTimeout: number;
};

export default function DebounceInput(props: DebounceProps) {
  const { handleDebounce, debounceTimeout, ...rest } = props;

  const timerRef = React.useRef<ReturnType<typeof setTimeout> | undefined>(
    undefined
  );

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => {
      handleDebounce(event.target.value);
    }, debounceTimeout);
  };

  return <input {...rest} onChange={handleChange} />;
}
