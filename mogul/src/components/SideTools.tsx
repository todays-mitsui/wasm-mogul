import classNames from "classnames";
import type { JSX } from "solid-js";
import styles from "./SideTools.module.css";
import Context from "./SideTools/Context";
import Settings from "./SideTools/Settings";
import ToolBox, { useToolBox } from "./SideTools/ToolBox";
import { sideTools } from "~/signals";

interface Props {
  class?: string | string[];
}

export default function SideTools(props: Props): JSX.Element {
  return (
    <nav class={classNames(styles.sideTools, props.class)}>
      <ToolBox
        boxName="context"
        boxTitle="Context"
        open={sideTools.context.isOpen()}
      >
        <Context />
      </ToolBox>
      <ToolBox
        boxName="settings"
        boxTitle="Settings"
        open={sideTools.settings.isOpen()}
      >
        <Settings />
      </ToolBox>
    </nav>
  );
}
