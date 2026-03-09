import { HvigorNode, HvigorPlugin } from "@ohos/hvigor";
import { OhosHarContext, OhosPluginId, Target } from "@ohos/hvigor-ohos-plugin";
import { spawnSync } from "child_process";
import { dirname, resolve } from "path";

const PLATFORM_ARM32 = "ohos-arm";
const PLATFORM_ARM64 = "ohos-arm64";
const PLATFORM_X86_64 = "ohos-x64";

const PLATFORMS = [PLATFORM_ARM32, PLATFORM_ARM64, PLATFORM_X86_64];

export function cargokitHvigorPlugin(
  cargokit_mainifest_dir: string,
): HvigorPlugin {
  if (!cargokit_mainifest_dir) {
    throw new Error("cargokit_mainifest_dir is required");
  }

  return {
    pluginId: "cargokitHvigorPlugin",
    apply(node: HvigorNode) {
      console.log(">>>>> cargokitHvigorPlugin <<<<<");

      const context = node.getContext(
        OhosPluginId.OHOS_HAR_PLUGIN,
      ) as OhosHarContext;

      context?.targets((target) => {
        registerCargokitTask(node, target, cargokit_mainifest_dir);
      });
    },
  };
}

function registerCargokitTask(
  node: HvigorNode,
  target: Target,
  cargokit_mainifest_dir: string,
) {
  const targetName = target.getTargetName();

  node.registerTask({
    name: `${targetName}@cargokitTask`,
    run: () => {
      const executableName =
        process.platform === "win32"
          ? "run_build_tool.cmd"
          : "run_build_tool.sh";
      const currentDir = dirname(__filename);
      const executablePath = resolve(
        currentDir,
        "..",
        "..",
        "..",
        executableName,
      );

      if (process.platform !== "win32") {
        spawnSync("chmod", ["+x", executablePath], { stdio: "inherit" });
      }

      const dir = node.getNodeDir().filePath;
      const outputDir = resolve(
        dir,
        "build",
        targetName,
        "intermediates",
        "libs",
        targetName,
      );
      const buildDir = resolve(dir, "build");

      const rootNodeDirPath = node.getParentNode()?.nodeDir.filePath;

      if (!rootNodeDirPath) {
        throw new Error("rootNodeDirPath is required");
      }

      const cargokitManifestPath = resolve(
        rootNodeDirPath,
        cargokit_mainifest_dir,
      );

      const result = spawnSync(executablePath, ["build-hvigor"], {
        stdio: "inherit",
        env: {
          ...process.env,
          CARGOKIT_ROOT_PROJECT_DIR: rootNodeDirPath,
          CARGOKIT_TOOL_TEMP_DIR: `${buildDir}/build_tool`,
          CARGOKIT_TARGET_PLATFORMS: PLATFORMS.join(","),
          CARGOKIT_MANIFEST_DIR: cargokitManifestPath,
          CARGOKIT_CONFIGURATION: "release",
          CARGOKIT_TARGET_TEMP_DIR: `${buildDir}/cargokit`,
          CARGOKIT_OUTPUT_DIR: outputDir,
        },
      });

      if (result.status !== 0) {
        throw new Error(`cargokitTask failed with status ${result.status}`);
      }
    },
    dependencies: [`${targetName}@ProcessLibs`],
    postDependencies: [`${targetName}@DoNativeStrip`],
  });
}
