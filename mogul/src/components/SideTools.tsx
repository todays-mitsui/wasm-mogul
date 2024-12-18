import classNames from "classnames";
import type { JSX } from "solid-js";
import styles from "./SideTools.module.css";
import Context from "./SideTools/Context";
import Settings from "./SideTools/Settings";
import ToolBox from "./SideTools/ToolBox";

interface Props {
  class?: string | string[];
}

export default function SideTools(props: Props): JSX.Element {
  return (
    <nav class={classNames(styles.sideTools, props.class)}>
      <ToolBox boxName="Context" boxTitle="Context">
        <Context />
      </ToolBox>
      <ToolBox boxName="Settings" boxTitle="Settings">
        <Settings />
      </ToolBox>
    </nav>
  );
}
