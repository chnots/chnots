import KSVG from "@/common/component/svg";
import { LLMChatTemplate } from "@/store/llmchat";
import React, { useState } from "react";
import { v4 } from "uuid";

const AddTemplate = ({
  template,
  onSubmit,
  onClose,
}: {
  template?: LLMChatTemplate;
  onSubmit: (data: LLMChatTemplate) => Promise<boolean>;
  onClose: () => void;
}) => {
  const [formData, setFormData] = useState<{
    name?: string;
    prompt?: string;
    svg_logo?: string;
  }>({
    name: template?.name,
    prompt: template?.prompt,
    svg_logo: template?.svg_logo,
  });

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (formData?.name?.trim() === "" || formData?.prompt?.trim() === "") {
      alert("Please fill in all fields.");
      return;
    }

    const toInsert: LLMChatTemplate = {
      id: template ? template.id : v4(),
      name: formData.name!,
      prompt: formData.prompt!,
      svg_logo: formData.svg_logo,
      insert_time: new Date(),
    };

    const submiResult = await onSubmit(toInsert);
    if (submiResult) {
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/10 backdrop-blur-sm">
      <div className="bg-white rounded-lg shadow-lg p-6 w-150 relative">
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label
              htmlFor="name"
              className="block text-gray-700 font-bold mb-2"
            >
              Name
            </label>
            <input
              type="text"
              id="name"
              name="name"
              value={formData.name ?? ""}
              onChange={handleInputChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              required
              tabIndex={0}
              aria-label="Template Name"
            />
          </div>
          <div className="mb-4">
            <label
              htmlFor="svg_logo"
              className="block text-gray-700 font-bold mb-2"
            >
              Svg Logo Url
            </label>
            <div className="flex flex-row space-x-2 items-center">
              {formData.svg_logo && <KSVG inner={formData.svg_logo} />}
              <textarea
                id="svg_logo"
                name="svg_logo"
                value={formData.svg_logo ?? ""}
                onChange={handleInputChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                aria-label="Template Name"
              />
            </div>
          </div>
          <div className="mb-4">
            <label
              htmlFor="prompt"
              className="block text-gray-700 font-bold mb-2"
            >
              Prompt
            </label>
            <textarea
              id="prompt"
              name="prompt"
              value={formData.prompt ?? ""}
              onChange={handleInputChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              required
              aria-label="Template Prompt"
            />
          </div>
          <div className="flex flex-row justify-center space-x-4">
            <button
              type="submit"
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              aria-label="Submit Template"
            >
              Submit
            </button>
            <button
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              aria-label="Close"
              onClick={onClose}
            >
              Close
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default AddTemplate;
