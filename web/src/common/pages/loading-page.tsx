import Icon from "@/common/component/icon";

function LoadingPage() {
  return (
    <div className="fixed w-full h-full flex flex-row justify-center items-center">
      <div className="w-80 max-w-full h-full flex flex-col justify-center items-center">
        <Icon.Loader className="animate-spin dark:text-gray-200" />
      </div>
    </div>
  );
}

export default LoadingPage;
