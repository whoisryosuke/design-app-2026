import { useEffect, useRef } from "react";
import { useAtomValue } from "jotai";
import { designEngineAtom } from "../../../store/renderer";

const CANVAS_NAME = "preview";

const DesignCanvas = () => {
  const designEngine = useAtomValue(designEngineAtom);
  // Generation ID for the current renderer in this "mount"
  const creationIdRef = useRef(0);

  useEffect(() => {
    let cancelled = false;
    const myId = ++creationIdRef.current;
    let didCreate = false;

    // This handles creating the `DesignRenderer` inside `DesignEngine` using the provided canvas.
    // To protect against this un-mounting + re-mounting quickly, we use a "generation" system
    // (aka an index we increment as an "ID") to keep track of what renderer was created.
    async function startRenderer() {
      if (!designEngine) return;

      try {
        // Create renderer - this returns true if created successfully.
        const created = await designEngine.create_render(CANVAS_NAME);

        // Generation mis-match? Means newer render was created.
        // @TODO: Might not be necessary here...
        if (myId !== creationIdRef.current) return;

        // Store creation flag locally in `useEffect` so un-mount callback can see it.
        didCreate = created;

        // Did we unmount? Stop the renderer if it was created.
        // And immediately return - we can assume component will re-mount if needed.
        if (cancelled) {
          if (didCreate) {
            try {
              designEngine.stop_renderer();
            } catch (e) {
              console.error("Error stopping renderer", e);
            }
          }
          return;
        }

        // Failed to create? Bail early because we can't start render loop.
        if (!created) return;

        // Start render loop
        designEngine.start_renderer();
        console.log("Render loop started");
      } catch (e) {
        console.error("Error creating renderer", e);
      }
    }

    if (designEngine) startRenderer();

    return () => {
      // Cancel the `async` promise that's possibly still running
      cancelled = true;

      // Only stop if this mount actually owns the current renderer.
      if (myId === creationIdRef.current && didCreate) {
        try {
          designEngine?.stop_renderer();
        } catch (e) {
          console.error("Error stopping renderer", e);
        }
      }
    };
  }, [designEngine]);

  return (
    <div>
      <canvas id={CANVAS_NAME} width={800} height={600} />
    </div>
  );
};

export default DesignCanvas;
