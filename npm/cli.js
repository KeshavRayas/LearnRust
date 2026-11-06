#!/usr/bin/env node
"use strict";

const { spawnSync } = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");

const repoRoot = path.resolve(__dirname, "..");
const rustlabDir = path.join(repoRoot, "rustlab");
const contentSource = path.join(rustlabDir, "content");
const isWindows = process.platform === "win32";
const cargoBin = path.join(os.homedir(), ".cargo", "bin");
const rustlabBin = path.join(cargoBin, isWindows ? "rustlab.exe" : "rustlab");
const home = process.env.RUSTLAB_HOME || defaultHome();
const contentDest = path.join(home, "content");
const projectsDir = path.join(os.homedir(), "rustlab-projects");
const env = {
  ...process.env,
  PATH: `${cargoBin}${path.delimiter}${process.env.PATH || ""}`,
};

function defaultHome() {
  if (isWindows) {
    return path.join(
      process.env.LOCALAPPDATA || path.join(os.homedir(), "AppData", "Local"),
      "rustlab",
    );
  }
  if (process.platform === "darwin") {
    return path.join(os.homedir(), "Library", "Application Support", "rustlab");
  }
  return path.join(
    process.env.XDG_DATA_HOME || path.join(os.homedir(), ".local", "share"),
    "rustlab",
  );
}

function has(command) {
  const result = spawnSync(command, ["--version"], { env, stdio: "ignore" });
  return !result.error && result.status === 0;
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    env,
    stdio: "inherit",
    ...options,
  });
  if (result.error || result.status !== 0) {
    process.exit(result.status || 1);
  }
}

function main() {
  console.log("rustlab setup\n");

  if (!has("cargo")) {
    console.error("Rust is not installed.");
    console.error("Install it from https://rustup.rs, then run this command again.");
    process.exit(1);
  }

  console.log("Building rustlab...");
  run(
    "cargo",
    [
      "install",
      "--locked",
      "--path",
      path.join(rustlabDir, "crates", "rustlab-cli"),
      "--force",
    ],
    { cwd: rustlabDir },
  );

  console.log("Installing course content...");
  fs.mkdirSync(home, { recursive: true });
  fs.rmSync(contentDest, { recursive: true, force: true });
  fs.cpSync(contentSource, contentDest, { recursive: true });

  const runtimeEnv = { ...env, RUSTLAB_HOME: home, RUSTLAB_CONTENT: contentDest };
  run(rustlabBin, ["doctor"], { env: runtimeEnv });

  fs.mkdirSync(projectsDir, { recursive: true });
  const projectDir = path.join(projectsDir, "hello-rust");
  if (fs.existsSync(projectDir)) {
    console.log(`Project already exists: ${projectDir}`);
  } else {
    run(
      rustlabBin,
      ["start", "hello-rust", "--dir", projectDir],
      { env: runtimeEnv },
    );
  }

  console.log("\nSetup complete.");
  console.log(`Project: ${projectDir}`);
  console.log(`Next:\n  cd "${projectDir}"\n  rustlab check`);
  console.log("\nIf `rustlab` is not found in a new terminal, add this folder to PATH:");
  console.log(`  ${cargoBin}`);
}

main();
