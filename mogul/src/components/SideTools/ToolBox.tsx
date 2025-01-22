import {
  type JSX,
  Show,
  children,
  splitProps,
  onMount,
  createSignal,
} from "solid-js";
import styles from "./ToolBox.module.css";
import { useEffect } from "storybook/internal/preview-api";
import { type SideTools, sideTools } from "~/signals";

interface ToolBoxProps {
  boxName: SideTools;
  boxTitle: string;
  open: boolean;
  onClick?: () => void;
  children?: JSX.Element;
  [prop: string]: any;
}

export default function ToolBox(props: ToolBoxProps): JSX.Element {
  const [, remainingProps] = splitProps(props, [
    "boxName",
    "boxTitle",
    "open",
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
      class={styles.toolBox}
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

interface UseToolBoxProps {
  open: boolean;
}

export function useToolBox(props: UseToolBoxProps) {
  const [isOpen, setIsOpen] = createSignal(props.open);

  function ToolBox(props: ToolBoxProps): JSX.Element {
    const [, remainingProps] = splitProps(props, ["name", "title", "children"]);

    let detailsRef: HTMLDetailsElement | undefined;
    useEffect(() => {
      if (detailsRef != null) {
        detailsRef.open = isOpen();
      }
    });

    return (
      <details ref={detailsRef} class={styles.toolBox} {...remainingProps}>
        <summary class={styles.summary} title={props.boxTitle}>
          {props.boxName}
        </summary>
        <div class={styles.inner}>{props.children}</div>
      </details>
    );
  }

  return {
    ToolBox,
    isOpen,
    openToolbox: () => setIsOpen(true),
    closeToolBox: () => setIsOpen(false),
  };
}
