import { Select, Option } from "@mui/joy";
import Icon from "./Icon";
import { useDomainStore } from "@/store/v1/domain";

export const DomainIcon = ({
  name,
  className,
}: {
  name: string;
  className?: string;
}) => {
  if (name === "public") {
    return <Icon.Globe2 className={className} />;
  } else if (name === "work") {
    return <Icon.BriefcaseBusiness className={className} />;
  } else {
    return <Icon.Notebook className={className} />;
  }
};

export const DomainSelect = ({
  domainName,
  handleDomainChange,
}: {
  domainName: string;
  handleDomainChange: (domain: string) => void;
}) => {
  const domainStore = useDomainStore();

  return (
    <div
      className="relative flex flex-row justify-start items-center border border-gray-300 rounded-lg mr-4"
      onFocus={(e) => e.stopPropagation()}
    >
      <Select
        variant="plain"
        value={domainName}
        startDecorator={<DomainIcon name={domainName} />}
        onChange={(_, domain) => {
          if (domain) {
            handleDomainChange(domain);
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
