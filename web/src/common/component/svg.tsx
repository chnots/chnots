const KSVG = ({ inner, className }: { inner: string; className?: string }) => {
  return (
    <svg
      dangerouslySetInnerHTML={{ __html: inner }}
      width="24px"
      height="24px"
      viewBox="0 0 24 24"
      className={className}
    ></svg>
  );
};

export default KSVG;
