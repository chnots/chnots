import { type HTMLMotionProps, motion } from "framer-motion";
import { forwardRef } from "react";
import {
  defaultTransition,
  fadeInUp,
  scaleIn,
  staggerContainer,
  staggerItem,
} from "@/lib/animations";

type AnimatedDivProps = HTMLMotionProps<"div">;

const AnimatedDiv = motion.div;

const AnimatedPage = forwardRef<HTMLDivElement, AnimatedDivProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        initial="initial"
        animate="animate"
        exit="exit"
        variants={fadeInUp}
        transition={defaultTransition}
        className={className}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);
AnimatedPage.displayName = "AnimatedPage";

const AnimatedScale = forwardRef<HTMLDivElement, AnimatedDivProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        initial="initial"
        animate="animate"
        exit="exit"
        variants={scaleIn}
        transition={defaultTransition}
        className={className}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);
AnimatedScale.displayName = "AnimatedScale";

const StaggerContainer = forwardRef<HTMLDivElement, AnimatedDivProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        initial="initial"
        animate="animate"
        exit="exit"
        variants={staggerContainer}
        className={className}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);
StaggerContainer.displayName = "StaggerContainer";

const StaggerItem = forwardRef<HTMLDivElement, AnimatedDivProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={staggerItem}
        transition={defaultTransition}
        className={className}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);
StaggerItem.displayName = "StaggerItem";

const AnimatedList = forwardRef<
  HTMLDivElement,
  AnimatedDivProps & { staggerDelay?: number }
>(({ className, children, staggerDelay = 0.05, ...props }, ref) => {
  return (
    <motion.div
      ref={ref}
      initial="initial"
      animate="animate"
      exit="exit"
      variants={{
        initial: {},
        animate: {
          transition: {
            staggerChildren: staggerDelay,
          },
        },
        exit: {
          transition: {
            staggerChildren: 0.02,
            staggerDirection: -1,
          },
        },
      }}
      className={className}
      {...props}
    >
      {children}
    </motion.div>
  );
});
AnimatedList.displayName = "AnimatedList";

const AnimatedListItem = forwardRef<HTMLDivElement, AnimatedDivProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={{
          initial: { opacity: 0, y: 8, scale: 0.98 },
          animate: {
            opacity: 1,
            y: 0,
            scale: 1,
            transition: {
              type: "spring",
              stiffness: 400,
              damping: 25,
            },
          },
          exit: {
            opacity: 0,
            y: -4,
            scale: 0.98,
            transition: {
              duration: 0.15,
            },
          },
        }}
        className={className}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);
AnimatedListItem.displayName = "AnimatedListItem";

export {
  AnimatedDiv,
  AnimatedPage,
  AnimatedScale,
  StaggerContainer,
  StaggerItem,
  AnimatedList,
  AnimatedListItem,
};
