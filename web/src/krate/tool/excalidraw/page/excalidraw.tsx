import { SaveState } from "@/common/types";
import ExcalidrawContainer from "../component/excalidraw-container";

import "@excalidraw/excalidraw/index.css";


const ExcalidrawPage = () => {
  return <ExcalidrawContainer onSetSaveState={function(state: SaveState): void {
    throw new Error("Function not implemented.");
  } }></ExcalidrawContainer>;
};

export default ExcalidrawPage;
