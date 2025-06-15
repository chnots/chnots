import KSVG from "@/common/component/svg";
import { Button } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import { Textarea } from "@/common/component/ui/textarea";
import { LLMChatTemplate } from "@/krate/llmchat/po";
import { genTID } from "@/lib/id_util";
import React, { useState } from "react";

const TemplateForm = ({
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
      tid: template ? template.tid : genTID(),
      name: formData.name!,
      prompt: formData.prompt!,
      svg_logo: formData.svg_logo,
    };

    const submiResult = await onSubmit(toInsert);
    if (submiResult) {
      onClose();
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <div className="mb-4">
        <label htmlFor="name" className="block text-gray-700 font-bold mb-2">
          Name
        </label>
        <Input
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
          Svg Logo Data
        </label>
        <div className="flex flex-row space-x-2 items-center">
          <div className="border rounded-md">
            <KSVG inner={formData.svg_logo ?? ""} />
          </div>

          <Textarea
            id="svg_logo"
            name="svg_logo"
            value={formData.svg_logo ?? ""}
            onChange={handleInputChange}
            aria-label="Template Name"
          />
        </div>
      </div>
      <div className="mb-4">
        <label htmlFor="prompt" className="block text-gray-700 font-bold mb-2">
          Prompt
        </label>
        <Textarea
          id="prompt"
          name="prompt"
          value={formData.prompt ?? ""}
          onChange={handleInputChange}
          required
          aria-label="Template Prompt"
        />
      </div>
      <div className="flex flex-row justify-center space-x-4">
        <Button type="submit" aria-label="Submit Template">
          Submit
        </Button>
        <Button aria-label="Close" onClick={onClose}>
          Close
        </Button>
      </div>
    </form>
  );
};

export default TemplateForm;
