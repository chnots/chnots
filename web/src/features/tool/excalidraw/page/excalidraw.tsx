import ExcalidrawContainer from "../component/excalidraw-container";

import "@excalidraw/excalidraw/index.css";

import * as TExcalidraw from "@excalidraw/excalidraw";

const ExcalidrawPage = () => {
  const { Excalidraw } = TExcalidraw;

  return (
    <ExcalidrawContainer
      useCustom={() => {}}
      excalidrawLib={TExcalidraw}
    >
      <Excalidraw />
    </ExcalidrawContainer>
  );
};

export default ExcalidrawPage;
