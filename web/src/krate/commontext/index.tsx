import React, { useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { html } from "@codemirror/lang-html";
import { css } from "@codemirror/lang-css";

// Define language options with type
interface LanguageOption {
  value: string;
  label: string;
}

const languageOptions: LanguageOption[] = [
  { value: "javascript", label: "JavaScript" },
  { value: "typescript", label: "TypeScript" },
  { value: "html", label: "HTML" },
  { value: "css", label: "CSS" },
];

// Map language to corresponding CodeMirror extension
const getExtension = (lang: string) => {
  switch (lang) {
    case "javascript":
    case "typescript":
      return javascript();
    case "html":
      return html();
    case "css":
      return css();
    default:
      return javascript(); // Default to JavaScript
  }
};

const CommonText: React.FC = () => {
  const [code, setCode] = useState<string>("");
  const [language, setLanguage] = useState<string>("javascript");

  const handleCopyAll = () => {
    navigator.clipboard.writeText(code).catch((err) => {
      console.error("Failed to copy text: ", err);
    });
  };

  const handleLanguageChange = (
    event: React.ChangeEvent<HTMLSelectElement>,
  ) => {
    setLanguage(event.target.value);
  };

  const handleKeyDownOnCopy = (
    event: React.KeyboardEvent<HTMLButtonElement>,
  ) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      handleCopyAll();
    }
  };

  const handleKeyDownOnSelect = (
    event: React.KeyboardEvent<HTMLSelectElement>,
  ) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      // No action needed beyond default select behavior, but prevent default to avoid double trigger
    }
  };

  return (
    <div className="flex flex-col border border-gray-300 rounded-lg overflow-hidden">
      {/* Status Bar */}
      <div className="flex items-center justify-between bg-gray-100 p-2 border-b border-gray-300">
        <div className="flex items-center space-x-2">
          <select
            value={language}
            onChange={handleLanguageChange}
            onKeyDown={handleKeyDownOnSelect}
            aria-label="Select programming language"
            className="px-3 py-1 border border-gray-300 rounded-md bg-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            {languageOptions.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        </div>
        <button
          onClick={handleCopyAll}
          onKeyDown={handleKeyDownOnCopy}
          aria-label="Copy all code"
          className="px-3 py-1 bg-blue-500 text-white text-sm rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          Copy All
        </button>
      </div>

      {/* CodeMirror Editor */}
      <CodeMirror
        value={code}
        onChange={setCode}
        extensions={[getExtension(language)]}
        basicSetup={{
          lineNumbers: true,
          highlightActiveLine: true,
          bracketMatching: true,
        }}
        theme="light"
        className="flex-1 w-full h-full"
      />
    </div>
  );
};

export default CommonText;
