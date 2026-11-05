import { useAtomValue } from "jotai";
import "./App.css";
import LoadWasm from "./components/features/LoadWasm/LoadWasm";
import DesignScreen from "./screens/Design/DesignScreen";
import { designEngineAtom } from "./store/renderer";
import BackendEventSync from "./components/features/BackendEventSync/BackendEventSync";

function App() {
  const wasmLoaded = useAtomValue(designEngineAtom);

  return (
    <>
      {wasmLoaded && <DesignScreen />}
      <BackendEventSync />
      <LoadWasm />
    </>
  );
}

export default App;
