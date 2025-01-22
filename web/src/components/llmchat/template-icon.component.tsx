import Icon from "../icon";

const componentMap = new Map([
  ["translate", <Icon.Languages />],
  ["develop", <Icon.Code2 />],
  ["common", <Icon.MessageSquare />],
]);

const LLMChatTemplateIcon = ({ name }: { name?: string }) => {
  const icon = componentMap.get(name ? name : "common");
  return icon ? icon : <Icon.MessageSquare />;
};

export default LLMChatTemplateIcon;
