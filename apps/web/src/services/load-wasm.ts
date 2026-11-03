import init, { DesignEngine } from "design-app-renderer";

// Keep engine persisted as singleton in global context
let engineInstance: DesignEngine | null = null;
let initializationPromise: Promise<DesignEngine> | null = null;

// Initialize WASM and the DesignEngine backend
export const getDesignEngine = async (): Promise<DesignEngine> => {
  // If we already have the instance, return it immediately
  if (engineInstance) return engineInstance;

  // No initialization? Start it up.
  if (!initializationPromise) {
    initializationPromise = (async () => {
      // Initialize WASM module
      await init();

      // Then initialize our backend struct
      engineInstance = new DesignEngine();
      return engineInstance;
    })();
  }

  // If an initialization is ALREADY in progress, return the existing promise.
  // This prevents the "double-run" from starting a second fetch/init.
  return initializationPromise.then((instance) => {
    engineInstance = instance;
    return instance;
  });
};
