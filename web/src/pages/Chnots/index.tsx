import clsx from "clsx";
import { MarkdownChnot } from "../../components/ChnotElem/MarkdownChnot";
import useResponsiveWidth from "../../hooks/useResponsiveWidth";
import ChnotList from "@/components/ChnotList";

const Chnots = () => {
  const { md } = useResponsiveWidth();

  return (
    <section className="w-full max-w-5xl min-h-full flex flex-col justify-start items-center sm:pt-3 md:pt-6 pb-8">
      <div
        className={clsx(
          "w-full flex flex-row justify-start items-start px-4 sm:px-6 gap-4"
        )}
      >
        <div className={clsx(md ? "w-[calc(100%-15rem)]" : "w-full")}>
          <MarkdownChnot className="mb-2" />
          <ChnotList />
        </div>
      </div>
    </section>
  );
};

export default Chnots;
