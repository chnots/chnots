import Icon from "./icon";

const AddButton = ({ onAdd }: { onAdd: () => void }) => {
  return (
    <div
      className="rounded-2xl flex flex-row items-center justify-center hover:bg-gray-100 hover:border-gray-400 hov px-2 py-1 space-x-2 hover:cursor-pointer"
      onClick={onAdd}
    >
      <Icon.PlusSquare />
      <div>New</div>
    </div>
  );
};

export default AddButton;
