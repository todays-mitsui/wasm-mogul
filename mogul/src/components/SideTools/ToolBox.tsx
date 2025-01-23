import { type JSX, splitProps } from "solid-js";
import styles from "./ToolBox.module.css";
import { type SideTools, sideTools } from "~/signals";
import classNames from "classnames";

interface ToolBoxProps {
  class?: string;
  boxName: SideTools;
  boxTitle: string;
  open: boolean;
  onClick?: () => void;
  children?: JSX.Element;
  [prop: string]: any;
}

export default function ToolBox(props: ToolBoxProps): JSX.Element {
  const [, remainingProps] = splitProps(props, [
    "class",
    "boxName",
    "boxTitle",
    "open",
    "onClick",
    "children",
  ]);
  const { boxName } = props;

  const onClick: JSX.EventHandler<HTMLDetailsElement, MouseEvent> = (event) => {
    event.preventDefault();
    sideTools.toggle(boxName);
    console.log({ boxName });
    props.onClick?.();
  };

  return (
    <details
      class={classNames(props.class, styles.toolBox)}
      onClick={onClick}
      open={sideTools.isOpen(boxName)}
      {...remainingProps}
    >
      <summary class={styles.summary} title={props.boxTitle}>
        {props.boxName}
      </summary>
      <div class={styles.inner}>{props.children}</div>
    </details>
  );
}
