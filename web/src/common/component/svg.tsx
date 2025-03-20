const KSVG = ({ inner, className }: { inner: string; className?: string }) => {
  return (
    <svg
      dangerouslySetInnerHTML={{ __html: inner }}
      className={className ?? "size-8"}
    ></svg>
  );
};

export default KSVG;
