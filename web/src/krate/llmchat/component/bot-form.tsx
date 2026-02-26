import type React from "react";
import { useEffect, useRef, useState } from "react";
import KSVG from "@/common/component/svg";
import { Button as KButton } from "@/common/component/ui/button";
import { Input } from "@/common/component/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/common/component/ui/select";
import { Textarea } from "@/common/component/ui/textarea";
import type {
  LLMChatBot,
  LLMChatBotBodyAI,
  LLMChatBotBodyOpenAIV1,
} from "@/krate/llmchat/po";
import {
  getProviderDefaultBaseUrl,
  PRESET_CONFIGS,
  PROVIDER_OPTIONS,
} from "@/krate/llmchat/provider-manager";
import { genTID, type TID } from "@/lib/id_util";
import { detectSVG } from "@/lib/svg-utils";

const isLegacyBody = (body: unknown): body is LLMChatBotBodyOpenAIV1 => {
  return (
    typeof body === "object" &&
    body !== null &&
    "url" in body &&
    "token" in body &&
    "model_name" in body &&
    !("provider" in body)
  );
};

const migrateLegacyToAI = (
  legacy: LLMChatBotBodyOpenAIV1,
): LLMChatBotBodyAI => {
  return {
    provider: "openai-compatible",
    base_url: legacy.url,
    api_key: legacy.token,
    model_name: legacy.model_name,
  };
};

const LLMChatBotBodyAIForm = ({
  bodyRef,
}: {
  bodyRef: React.MutableRefObject<LLMChatBotBodyAI>;
}) => {
  const body = bodyRef.current;
  const [formData, setFormData] = useState<LLMChatBotBodyAI>({
    provider: body?.provider ?? "openai",
    base_url: body?.base_url ?? "",
    api_key: body?.api_key ?? "",
    model_name: body?.model_name ?? "",
  });

  useEffect(() => {
    bodyRef.current = formData;
  }, [formData, bodyRef]);

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleProviderChange = (provider: string) => {
    const defaultUrl = getProviderDefaultBaseUrl(
      provider as LLMChatBotBodyAI["provider"],
    );
    setFormData((prev) => ({
      ...prev,
      provider: provider as LLMChatBotBodyAI["provider"],
      base_url: prev.base_url || defaultUrl,
    }));
  };

  const applyPreset = (presetKey: string) => {
    const preset = PRESET_CONFIGS[presetKey];
    if (preset) {
      setFormData((prev) => ({
        ...prev,
        ...preset,
        api_key: prev.api_key,
      }));
    }
  };

  return (
    <div className="w-full">
      <div className="mb-4">
        <span className="block mb-2 text-sm font-medium">Preset Configs</span>
        <div className="flex flex-wrap gap-2">
          {Object.keys(PRESET_CONFIGS).map((key) => (
            <KButton
              key={key}
              type="button"
              variant="outline"
              size="sm"
              onClick={() => applyPreset(key)}
              className="text-xs"
            >
              {key.charAt(0).toUpperCase() + key.slice(1)}
            </KButton>
          ))}
        </div>
      </div>

      <div className="mb-4">
        <span className="block mb-2">Provider</span>
        <Select value={formData.provider} onValueChange={handleProviderChange}>
          <SelectTrigger className="w-full">
            <SelectValue placeholder="Select provider" />
          </SelectTrigger>
          <SelectContent>
            {PROVIDER_OPTIONS.map((option) => (
              <SelectItem key={option.value} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <div className="mb-4">
        <label htmlFor="base_url" className="block mb-2">
          Base URL
        </label>
        <Input
          type="text"
          id="base_url"
          name="base_url"
          value={formData.base_url ?? ""}
          onChange={handleInputChange}
          className="w-full px-3 py-2 border-b border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
          tabIndex={0}
          aria-label="Base URL"
        />
      </div>

      <div className="mb-4">
        <label htmlFor="api_key" className="block mb-2">
          API Key
        </label>
        <Input
          type="password"
          id="api_key"
          name="api_key"
          value={formData.api_key ?? ""}
          onChange={handleInputChange}
          className="w-full px-3 py-2 border-b border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          aria-label="API Key"
        />
      </div>

      <div className="mb-4">
        <label htmlFor="model_name" className="block mb-2">
          Model Name
        </label>
        <Input
          id="model_name"
          name="model_name"
          value={formData.model_name ?? ""}
          onChange={handleInputChange}
          className="w-full px-3 py-2 border-b border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500"
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
  onDelete,
}: {
  bot?: LLMChatBot;
  onSubmit: (data: LLMChatBot) => Promise<boolean>;
  onClose: () => void;
  onDelete?: (bot: LLMChatBot) => Promise<boolean>;
}) => {
  const [formData, setFormData] = useState<{
    name: string;
    svg_logo?: string;
  }>({
    name: bot?.name ?? "",
    svg_logo: bot?.svg_logo,
  });

  const [botId, setBotId] = useState<TID>(bot?.otid ?? genTID());

  const parseInitialBody = (): LLMChatBotBodyAI => {
    if (!bot?.body) {
      return {
        provider: "openai",
        base_url: "",
        api_key: "",
        model_name: "",
      };
    }
    const parsed = JSON.parse(bot.body);
    if (isLegacyBody(parsed)) {
      return migrateLegacyToAI(parsed);
    }
    return parsed as LLMChatBotBodyAI;
  };

  const bodyRef = useRef<LLMChatBotBodyAI>(parseInitialBody());

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    setFormData({ ...formData, [e.target.name]: e.target.value });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (
      formData?.name?.trim() === "" ||
      !bodyRef.current ||
      bodyRef.current.base_url.trim() === "" ||
      bodyRef.current.model_name.trim() === ""
    ) {
      alert("Please fill in all required fields.");
      return;
    }

    const body: LLMChatBotBodyAI = {
      provider: bodyRef.current.provider,
      base_url: bodyRef.current.base_url,
      api_key: bodyRef.current.api_key,
      model_name: bodyRef.current.model_name,
    };

    const toInsert: LLMChatBot = {
      otid: botId,
      name: formData.name,
      svg_logo:
        formData.svg_logo && detectSVG(formData.svg_logo)
          ? formData.svg_logo
          : "",
      body: JSON.stringify(body),
      tid: genTID(),
    };

    const submitResult = await onSubmit(toInsert);
    if (submitResult) {
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/10 backdrop-blur-sm">
      <div className="bg-white rounded-lg shadow-lg p-6 w-150 relative">
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="name" className="block mb-2">
              Name
            </label>
            <Input
              type="text"
              id="name"
              name="name"
              value={formData.name ?? ""}
              onChange={handleInputChange}
              className="w-full px-3 py-2 border-b focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              required
              tabIndex={0}
              aria-label="Bot Name"
            />
          </div>
          <div className="mb-4">
            <label htmlFor="svg_logo" className="block mb-2">
              Svg Logo
            </label>
            <div className="flex flex-row space-x-2 items-center">
              {formData.svg_logo && <KSVG src={formData.svg_logo} />}
              <Textarea
                id="svg_logo"
                name="svg_logo"
                value={formData.svg_logo ?? ""}
                onChange={handleInputChange}
                className="w-full px-3 py-2 border-b border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                aria-label="Bot Logo"
              />
            </div>
          </div>
          <LLMChatBotBodyAIForm bodyRef={bodyRef} />
          <div className="flex flex-row justify-center space-x-4">
            <KButton className="p-2" type="submit" aria-label="Submit Template">
              Submit
            </KButton>
            <KButton className="p-2" aria-label="Close" onClick={onClose}>
              Close
            </KButton>
            <KButton
              className="p-2"
              aria-label="Duplicate"
              onClick={() => {
                setFormData((prev) => {
                  return { ...prev, name: `${prev.name} -- Clone` };
                });
                setBotId(genTID());
              }}
              type="button"
            >
              Duplicate
            </KButton>
            {bot && onDelete && (
              <KButton
                className="p-2"
                variant="destructive"
                aria-label="Delete"
                onClick={async () => {
                  if (confirm("Are you sure you want to delete this bot?")) {
                    const result = await onDelete(bot);
                    if (result) {
                      onClose();
                    }
                  }
                }}
                type="button"
              >
                Delete
              </KButton>
            )}
          </div>
        </form>
      </div>
    </div>
  );
};

export default BotForm;
