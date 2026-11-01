import { Stack } from "../../../styled-system/jsx";
import DesignCanvas from "../../components/features/DesignCanvas/DesignCanvas";
import DesignSidebar from "../../components/features/DesignSidebar/DesignSidebar";

type Props = {};

const DesignScreen = (props: Props) => {
  return (
    <Stack flexDirection="row">
      <DesignSidebar />
      <DesignCanvas />
    </Stack>
  );
};

export default DesignScreen;
