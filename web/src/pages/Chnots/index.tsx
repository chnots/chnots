import clsx from "clsx";
import ChnotList from "@/components/ChnotList";
import ChnotView, { ChnotViewMode } from "@/components/ChnotView";

const Chnots = () => {
  return (
    <section className="w-full max-w-5xl min-h-full flex flex-col justify-start items-center sm:pt-3 md:pt-6 pb-8">
      <div
        className={clsx(
          "w-full flex flex-row justify-start items-start px-4 sm:px-6 gap-4"
        )}
      >
        <div className={clsx("w-[calc(100%-15rem)]")}>
          <ChnotView
            className="mb-2"
            viewMode={ChnotViewMode.Editor}
            createInput={true}
          />
          <ChnotList />
        </div>
      </div>
    </section>
  );
};

export default Chnots;
