import React, { useState } from "react";

import { Button } from "@/common/component/ui/button";
import {
  CommandDialog,
  CommandInput,
  CommandList,
  CommandItem,
  CommandEmpty,
} from "@/common/component/ui/command";
import CommonText from "@/krate/commontext";

const PageComponent: React.FC = () => {
  const [isCommandOpen, setIsCommandOpen] = useState<boolean>(false);
  const [output, setOutput] = useState<string>(""); // State for command output

  // Handle opening the command dialog
  const handleOpenCommand = () => {
    setIsCommandOpen(true);
  };

  // Handle closing the command dialog
  const handleCloseCommand = () => {
    setIsCommandOpen(false);
  };

  // Handle selecting a command from the list
  const handleSelectCommand = (command: string) => {
    setOutput((prev) => prev + `${command} executed.\n`); // Example output update
    // In a real scenario, you might run the command and set output based on result
  };

  // Format current date to "mm-dd weekday"
  const formatDate = (): string => {
    const date = new Date();
    const month = (date.getMonth() + 1).toString().padStart(2, "0"); // mm
    const day = date.getDate().toString().padStart(2, "0"); // dd
    const weekday = date.toLocaleDateString("en-US", { weekday: "long" }); // full weekday name
    return `${month}-${day} ${weekday}`;
  };

  // Example commands list
  const commands = [
    { id: "1", label: "Execute Task 1", value: "task1" },
    { id: "2", label: "Execute Task 2", value: "task2" },
    { id: "3", label: "Show Help", value: "help" },
  ];

  return (
    <div className="flex flex-col h-screen">
      {/* Top: Editor */}
      <div className="flex-1">
        <CommonText />
      </div>

      {/* Bottom: Buttons and Date */}
      <div className="flex items-center justify-between p-4 border-t">
        <div className="flex gap-2">
          {/* Button 1: Open Command Dialog */}
          <Button
            onClick={handleOpenCommand}
            aria-label="Open command menu"
            tabIndex={0}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                handleOpenCommand();
              }
            }}
          >
            Button 1
          </Button>
          {/* Button 2: Use Buton2 component */}
          {/*           <Buton2 />
           */}{" "}
        </div>

        {/* Current Date */}
        <span className="text-sm text-gray-600" aria-label="Current date">
          {formatDate()}
        </span>
      </div>

      {/* Command Dialog with Output */}
      <CommandDialog open={isCommandOpen} onOpenChange={setIsCommandOpen}>
        <CommandInput placeholder="Type a command or search..." />
        <CommandList>
          <CommandEmpty>No commands found.</CommandEmpty>
          {commands.map((command) => (
            <CommandItem
              key={command.id}
              value={command.value}
              onSelect={() => handleSelectCommand(command.label)}
            >
              {command.label}
            </CommandItem>
          ))}
        </CommandList>
        {/* Output Box */}
        <div className="p-4 border-t">
          <label htmlFor="output" className="sr-only">
            Command output
          </label>
          <textarea
            id="output"
            value={output}
            readOnly
            className="w-full h-32 p-2 text-sm border rounded-md resize-none"
            aria-label="Command output"
          />
        </div>
      </CommandDialog>
    </div>
  );
};

export default PageComponent;
