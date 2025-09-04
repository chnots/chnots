import KSVG from "@/common/component/svg";
import { Button } from "@/common/component/ui/button";
import { DialogClose, DialogFooter } from "@/common/component/ui/dialog";
import { Input } from "@/common/component/ui/input";
import { Label } from "@/common/component/ui/label";
import { Textarea } from "@/common/component/ui/textarea";
import { LLMChatTemplate } from "@/krate/llmchat/po";
import { genTID } from "@/lib/id_util";
import { detectSVG } from "@/lib/svg-utils";
import React, { useState } from "react";
import { useLLMChatStore } from "../store";

const TemplateForm = ({
  template,
  onSubmit,
}: {
  template?: LLMChatTemplate;
  onSubmit: (data: LLMChatTemplate) => Promise<boolean>;
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
  const { refreshTemplates } = useLLMChatStore();

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
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
      otid: template ? template.otid : genTID(),
      name: formData.name!,
      prompt: formData.prompt!,
      svg_logo:
        formData.svg_logo && detectSVG(formData.svg_logo)
          ? formData.svg_logo
          : "",
      tid: genTID(),
    };

    const submiResult = await onSubmit(toInsert);
    if (submiResult) {
      refreshTemplates();
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <div className="mb-4">
        <Label htmlFor="name" className="block mb-2">
          Name
        </Label>
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
        <Label htmlFor="svg_logo" className="block mb-2">
          Svg Logo Data
        </Label>
        <div className="flex flex-row space-x-2 items-center">
          <div className="border rounded-md">
            <KSVG src={formData.svg_logo ?? ""} />
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
        <Label htmlFor="prompt" className="block mb-2">
          Prompt
        </Label>
        <Textarea
          id="prompt"
          name="prompt"
          value={formData.prompt ?? ""}
          onChange={handleInputChange}
          required
          aria-label="Template Prompt"
        />
      </div>
      <DialogFooter>
        <DialogClose asChild>
          <Button variant="outline">Cancel</Button>
        </DialogClose>
        <Button type="submit" aria-label="Submit Template">
          Submit
        </Button>
      </DialogFooter>
    </form>
  );
};

export default TemplateForm;
