import "./App.css";
import DesignScreen from "./screens/Design/DesignScreen";

function App() {
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

  return <DesignScreen />;
}

export default App;
