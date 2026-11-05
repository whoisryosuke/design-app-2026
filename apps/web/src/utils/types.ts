import type { DesignCommand, DesignEvent } from "design-app-renderer-types";

type DesignEventType = DesignEvent["type"];

export type DesignEventPayload<T extends DesignEventType> = Extract<
  DesignEvent,
  { type: T }
>["payload"];

// type DesignCommandType = DesignCommand["type"];

// export type DesignCommandPayload<T extends DesignCommandType> = Extract<
//   DesignEvent,
//   { type: T }
// >["payload"];
