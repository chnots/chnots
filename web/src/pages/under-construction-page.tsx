import Icon from "@/components/icon";

const UnderConstructionPage = () => {
  return (
    <section className="@container w-full max-w-5xl min-h-[100svh] flex flex-row justify-start items-center sm:pt-3 md:pt-6 pb-8">
      <div className="w-full px-4 grow flex flex-row justify-center items-center sm:px-6">
        <Icon.Construction size={144} className="m-5" />
        <p className="mt-4 text-[1rem] font-mono dark:text-gray-300">
          This page is under construction.
        </p>
      </div>
    </section>
  );
};

export default UnderConstructionPage;
