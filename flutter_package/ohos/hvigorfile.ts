import { harTasks } from "@ohos/hvigor-ohos-plugin";
import { cargokitHvigorPlugin } from "../cargokit/hvigor/index";

export default {
  system: harTasks /* Built-in plugin of Hvigor. It cannot be modified. */,
  plugins: [
    cargokitHvigorPlugin("../native/hub"),
  ] /* Custom plugin to extend the functionality of Hvigor. */,
};
