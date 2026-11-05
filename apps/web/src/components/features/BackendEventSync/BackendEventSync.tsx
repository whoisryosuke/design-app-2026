import { useAtomValue } from "jotai";
import React, { useEffect, useRef } from "react";
import { designEngineAtom } from "../../../store/renderer";
import type { DesignEvent } from "design-app-renderer-types";
import { handleBackendEvent } from "../../../services/backend-sync";

type Props = {};

const BackendEventSync = (props: Props) => {
  const engine = useAtomValue(designEngineAtom);
  const isCallbackAttached = useRef(false);

  const syncEvents = (event: DesignEvent) => {
    console.log("got event", event);
    handleBackendEvent(event);
  };

  useEffect(() => {
    if (engine && !isCallbackAttached.current) {
      engine.setup_communication_callback(syncEvents);
      isCallbackAttached.current = true;
    }

    return () => {
      if (engine && isCallbackAttached.current) {
        engine.stop_communication_callback();
        isCallbackAttached.current = false;
      }
    };
  }, [engine]);

  return <></>;
};

export default BackendEventSync;
