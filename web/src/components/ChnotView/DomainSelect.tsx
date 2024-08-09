import { Select, Option } from "@mui/joy";
import Icon from "../Icon";
import { useDomainStore } from "@/store/v1/domain";

export const DomainSelect = () => {
  const domainStore = useDomainStore();

  const handleMemoVisibilityChange = (domain: string) => {
    domainStore.changeDomain(domain);
  };

  const domainIcon = (name: string, className: string, size: string) => {
    if (name === "public") {
      return <Icon.Globe2 className={className} size={size} />;
    } else if (name === "work") {
      return <Icon.BriefcaseBusiness className={className} size={size} />;
    } else {
      return <Icon.Notebook className={className} size={size} />;
    }
  };

  return (
    <div
      className="relative flex flex-row justify-start items-center border border-gray-300 rounded-lg mr-4"
      onFocus={(e) => e.stopPropagation()}
    >
      <Select
        variant="plain"
        value={domainStore.current.name}
        startDecorator={domainIcon(domainStore.current.name, "text-xs", "20px")}
        onChange={(_, domain) => {
          if (domain) {
            handleMemoVisibilityChange(domain);
          }
        }}
      >
        {domainStore.domains().map((item) => {
          return (
            <Option
              key={item.name}
              value={item.name}
              className="whitespace-nowrap"
            >
              {item.name}
            </Option>
          );
        })}
      </Select>
    </div>
  );
};
