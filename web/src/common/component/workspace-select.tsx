import Icon from "./icon";
import { useWorkspaceStore } from "@/store/workspace";
import * as RadixDropmenu from "@radix-ui/react-dropdown-menu";
import KButton from "./kbutton";

const WorkspaceIcon = ({
  name,
  className,
}: {
  name?: string;
  className?: string;
}) => {
  if (name === "public") {
    return <Icon.BookKey className={className} />;
  } else if (name === "work") {
    return <Icon.BriefcaseBusiness className={className} />;
  } else if (name === "private") {
    return <Icon.BookLock className={className} />;
  } else {
    return <Icon.Dice1 className={className} />;
  }
};

export const WorkspaceSelect = ({
  onSelect,
  currentWorkspace,
  className,
  menuClassName,
}: {
  onSelect: (workspace: string) => void;
  currentWorkspace: string;
  className?: string;
  menuClassName?: string;
}) => {
  const { workspaces } = useWorkspaceStore();
  return (
    <RadixDropmenu.Root>
      <RadixDropmenu.Trigger asChild>
        <KButton className={menuClassName}>
          <WorkspaceIcon
            name={currentWorkspace}
            className={className}
          ></WorkspaceIcon>
        </KButton>
      </RadixDropmenu.Trigger>

      <RadixDropmenu.Portal>
        <RadixDropmenu.Content
          className="RadixDropmenuContent z-20"
          sideOffset={5}
        >
          {workspaces().map((e) => (
            <RadixDropmenu.Item
              className="p-2"
              key={e.name}
              onClick={() => {
                onSelect(e.name);
              }}
            >
              {e.name}
            </RadixDropmenu.Item>
          ))}
        </RadixDropmenu.Content>
      </RadixDropmenu.Portal>
    </RadixDropmenu.Root>
  );
};
