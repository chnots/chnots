import type React from "react";
import { useState } from "react";
import { Button } from "./button";
import { Sheet, SheetClose, SheetContent, SheetTrigger } from "./sheet";

const FullScreenEditor = ({ children }: { children: React.ReactNode }) => {
  const [isOpen, setIsOpen] = useState<boolean>(false);

  return (
    <Sheet open={isOpen} onOpenChange={setIsOpen}>
      <SheetTrigger asChild>
        <Button
          aria-label="Open full-screen editor"
          onClick={() => setIsOpen(true)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setIsOpen(true);
            }
          }}
          tabIndex={0}
        >
          Edit
        </Button>
      </SheetTrigger>

      <SheetContent side="bottom" className="p-0 w-screen h-screen">
        <div>
          <SheetClose />
        </div>
        {children}
      </SheetContent>
    </Sheet>
  );
};

export default FullScreenEditor;
