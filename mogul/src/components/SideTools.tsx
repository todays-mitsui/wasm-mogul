import classNames from "classnames";
import type { JSX } from "solid-js";
import styles from "./SideTools.module.css";
import Context from "./SideTools/Context";
import Settings from "./SideTools/Settings";
import ToolBox from "./SideTools/ToolBox";
import { sideTools } from "~/signals";
import SettingsIcon from "./Icon/SettingsIcon";

interface Props {
  class?: string | string[];
}

export default function SideTools(props: Props): JSX.Element {
  return (
    <nav class={classNames(styles.sideTools, props.class)}>
      <ToolBox
        class={styles.context}
        boxName="context"
        boxTitle="Context"
        open={sideTools.isOpen("context")}
        onClick={() => sideTools.toggle("context")}
      >
        <Context />
      </ToolBox>
      <ToolBox
        boxName="settings"
        boxTitle="Settings"
        summary={<SettingsIcon fill="var(--color-text)" />}
        open={sideTools.isOpen("settings")}
        onClick={() => sideTools.toggle("settings")}
      >
        <Settings />
      </ToolBox>
    </nav>
  );
}
