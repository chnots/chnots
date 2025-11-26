import SVG, { type Props as SVGProps } from "react-inlinesvg";

const KSVG = ({ className, ...rest }: SVGProps) => {
  return <SVG className={className ?? "w-4 h-4"} {...rest}></SVG>;
};

export default KSVG;
