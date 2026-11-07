import { defineConfig } from "vitest/config";
import { playwright } from "@vitest/browser-playwright"; // Vitest 4

export default defineConfig({
  test: {
    include: ["tests/**/*.test.ts"],
    browser: {
      enabled: true,
      provider: playwright(), // Vitest 3: provider: 'playwright'
      instances: [{ browser: "chromium" }],
    },
  },
});
