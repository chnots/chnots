import KSVG from "@/common/component/svg";
import { LLMChatBot, LLMChatBotBodyOpenAIV1 } from "@/store/llmchat";
import React, { RefObject, useEffect, useRef, useState } from "react";
import { v4 } from "uuid";

const LLMChatBotBodyOpenAIV1Body = ({
  bodyRef,
}: {
  bodyRef: RefObject<LLMChatBotBodyOpenAIV1 | null>;
}) => {
  const body = bodyRef.current;
  const [formData, setFormData] = useState<{
    model_name?: string;
    token?: string;
    url?: string;
  }>({
    model_name: body?.model_name,
    token: body?.token,
    url: body?.url,
  });

  useEffect(() => {
    bodyRef.current = {
      model_name: formData.model_name ?? "",
      token: formData.token ?? "",
      url: formData.url ?? "",
    };
  }, [formData, bodyRef]);

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  return (
    <div className="w-full">
      <div className="mb-4">
        <label htmlFor="url" className="block text-gray-700 font-bold mb-2">
          Url
        </label>
        <input
          type="text"
          id="url"
          name="url"
          value={formData.url ?? ""}
          onChange={handleInputChange}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
          tabIndex={0}
          aria-label="Template Name"
        />
      </div>
      <div className="mb-4">
        <label htmlFor="token" className="block text-gray-700 font-bold mb-2">
          Token
        </label>
        <input
          id="token"
          name="token"
          value={formData.token ?? ""}
          onChange={handleInputChange}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
          aria-label="Token"
        />
      </div>
      <div className="mb-4">
        <label
          htmlFor="model_name"
          className="block text-gray-700 font-bold mb-2"
        >
          Model Name
        </label>
        <input
          id="model_name"
          name="model_name"
          value={formData.model_name ?? ""}
          onChange={handleInputChange}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
          aria-label="Model Name"
        />
      </div>
    </div>
  );
};

const BotForm = ({
  bot,
  onSubmit,
  onClose,
}: {
  bot?: LLMChatBot;
  onSubmit: (data: LLMChatBot) => Promise<boolean>;
  onClose: () => void;
}) => {
  const [formData, setFormData] = useState<{
    name?: string;
    svg_logo?: string;
  }>({
    name: bot?.name,
    svg_logo: bot?.svg_logo,
  });
  const body = bot?.body
    ? (JSON.parse(bot?.body) as LLMChatBotBodyOpenAIV1)
    : null;

  const bodyRef = useRef<LLMChatBotBodyOpenAIV1>(body);

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (
      formData?.name?.trim() === "" ||
      !bodyRef.current ||
      bodyRef.current.url.trim() === "" ||
      bodyRef.current.token.trim() === "" ||
      bodyRef.current.model_name.trim() === ""
    ) {
      console.log(
        "all fields",
        formData?.name?.trim(),
        bodyRef.current?.url,
        bodyRef.current?.token,
        bodyRef.current?.model_name
      );
      alert("Please fill in all fields.");
      return;
    }

    const body: LLMChatBotBodyOpenAIV1 = {
      url: bodyRef.current?.url,
      token: bodyRef.current?.token,
      model_name: bodyRef.current?.model_name,
    };

    const toInsert: LLMChatBot = {
      id: bot ? bot.id : v4(),
      name: formData.name!!,
      svg_logo: formData.svg_logo,
      insert_time: new Date(),
      body: JSON.stringify(body),
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
              aria-label="Bot Name"
            />
          </div>
          <div className="mb-4">
            <label
              htmlFor="svg_logo"
              className="block text-gray-700 font-bold mb-2"
            >
              Svg Logo
            </label>
            <div className="flex flex-row space-x-2 items-center">
              {formData.svg_logo && <KSVG inner={formData.svg_logo} />}
              <textarea
                id="svg_logo"
                name="svg_logo"
                value={formData.svg_logo ?? ""}
                onChange={handleInputChange}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                aria-label="Bot Logo"
              />
            </div>
          </div>
          <LLMChatBotBodyOpenAIV1Body bodyRef={bodyRef} />
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

export default BotForm;
