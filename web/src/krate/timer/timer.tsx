import { useCallback, useEffect, useState } from "react";

import { Button } from "@/common/component/ui/button";

const Timer = () => {
  const [time, setTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);

  // 格式化时间为 HH:MM:SS
  const formatTime = useCallback((seconds: number): string => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    return [
      hours.toString().padStart(2, "0"),
      minutes.toString().padStart(2, "0"),
      secs.toString().padStart(2, "0"),
    ].join(":");
  }, []);

  // 处理开始/暂停
  const handleStartPause = useCallback(() => {
    setIsRunning((prev) => !prev);
  }, []);

  // 处理重置
  const handleReset = useCallback(() => {
    setTime(0);
  }, []);

  // 计时器逻辑
  useEffect(() => {
    let intervalId: NodeJS.Timeout;

    if (isRunning) {
      intervalId = setInterval(() => {
        setTime((prev) => prev + 1);
      }, 1000);
    }

    return () => {
      if (intervalId) clearInterval(intervalId);
    };
  }, [isRunning]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen text-black p-4">
      <div className="flex-1 flex items-center justify-center w-full">
        <div className="text-center">
          <h1
            className={`text-9xl md:text-[20rem] font-bold tracking-tighter  transition-all duration-300 ease-in-out scale-100 text-black font-mono`}
          >
            {formatTime(time)}
          </h1>
        </div>
      </div>

      <div className="flex gap-4 mb-8">
        <Button
          onClick={handleStartPause}
          className="px-8 py-4 rounded-full text-xl font-semibold bg-emerald-500 hover:bg-emerald-600 transition-colors"
          aria-label={isRunning ? "Pause timer" : "Start timer"}
          onSelect={() => {}}
        >
          {isRunning ? "Pause" : "Start"}
        </Button>

        <Button
          onClick={handleReset}
          disabled={time === 0}
          className={`px-8 py-4 rounded-full text-xl font-semibold transition-colors ${
            time === 0
              ? "bg-gray-500 cursor-not-allowed opacity-50"
              : "bg-amber-500 hover:bg-amber-600"
          }`}
          aria-label="Reset timer"
          onSelect={() => {}}
        >
          Reset
        </Button>
      </div>
    </div>
  );
};

export default Timer;
