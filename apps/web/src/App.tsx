import { useAtomValue } from "jotai";
import "./App.css";
import LoadWasm from "./components/features/LoadWasm/LoadWasm";
import DesignScreen from "./screens/Design/DesignScreen";
import { designEngineAtom } from "./store/renderer";

function App() {
  const wasmLoaded = useAtomValue(designEngineAtom);

  // const handleCreateLayer = () => {
  //   const width = Math.round(Math.random() * 420);
  //   const height = Math.round(Math.random() * 420);
  //   dispatch({
  //     AddLayer: {
  //       layer: {
  //         name: "Test",
  //         kind: "Rect",
  //         position: [0, 0],
  //         width,
  //         height,
  //         opacity: 1.0,
  //         parent: null,
  //         visible: true,
  //         items: [],
  //       },
  //     },
  //   });
  // };

  return (
    <>
      {wasmLoaded && <DesignScreen />}
      <LoadWasm />
    </>
  );
}

export default App;
