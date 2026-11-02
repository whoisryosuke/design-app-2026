const { execSync } = require("child_process");
const path = require("path");
const { styleText } = require("node:util");

const packageDir = path.resolve(__dirname, "../../../");

const commands = [
  "cargo build -p renderer --target wasm32-unknown-unknown",
  "wasm-bindgen target/wasm32-unknown-unknown/debug/renderer.wasm --target web --out-dir crates/renderer/pkg",
];

try {
  for (const cmd of commands) {
    console.log(
      styleText(["blueBright", "bold", "bgBlue"], `⚙️ Running: ${cmd}`),
    );
    // stdio: 'inherit' allows you to see the cargo output in real-time
    execSync(cmd, {
      stdio: "inherit",
      cwd: packageDir,
    });
  }
  console.log(
    styleText(["greenBright", "bold", "bgGreen"], "✅ WASM Build Successful!"),
  );
} catch (error) {
  console.error(styleText(["redBright", "bold", "bRed"], "❌ Build failed."));
  console.log(error);
  process.exit(1);
}
