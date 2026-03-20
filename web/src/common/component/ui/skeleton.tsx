import { motion } from "framer-motion";
import { cn } from "@/lib/utils";

function Skeleton({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="skeleton"
      className={cn("relative overflow-hidden rounded-md bg-accent", className)}
      {...props}
    >
      <motion.div
        className="absolute inset-0 -translate-x-full bg-gradient-to-r from-transparent via-white/20 to-transparent"
        animate={{
          translateX: ["-100%", "200%"],
        }}
        transition={{
          duration: 1.5,
          repeat: Number.POSITIVE_INFINITY,
          ease: "linear",
        }}
      />
    </div>
  );
}

function SkeletonGroup({
  className,
  count = 3,
  ...props
}: React.ComponentProps<"div"> & { count?: number }) {
  return (
    <div className={cn("space-y-3", className)} {...props}>
      {Array.from({ length: count }).map((_, i) => (
        <motion.div
          key={i}
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{
            delay: i * 0.1,
            duration: 0.3,
          }}
        >
          <Skeleton
            className={cn(
              "h-4",
              i === 0 && "w-3/4",
              i === 1 && "w-full",
              i === 2 && "w-5/6",
            )}
          />
        </motion.div>
      ))}
    </div>
  );
}

export { Skeleton, SkeletonGroup };
