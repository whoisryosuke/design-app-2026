import { Stack } from "../../../styled-system/jsx";
import DesignCanvas from "../../components/features/DesignCanvas/DesignCanvas";
import DesignSidebar from "../../components/features/DesignSidebar/DesignSidebar";
import EditorSidebar from "../../components/features/EditorSidebar/EditorSidebar";

type Props = {};

const DesignScreen = (props: Props) => {
  return (
    <Stack flexDirection="row" flex={1} gap="0">
      <DesignSidebar />
      <DesignCanvas />
      <EditorSidebar />
    </Stack>
  );
};

export default DesignScreen;
