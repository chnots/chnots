const KInput = ({ onChange }: { onChange: (input: string) => void }) => {
  return (
    <div className="w-full p-2 bg-transparent rounded border border-gray-200">
      <input
        type="text"
        className="bg-transparent w-full h-full outline-none"
        placeholder="search"
        onChange={(value) => onChange(value.target.value)}
      />
    </div>
  );
};

export default KInput;
