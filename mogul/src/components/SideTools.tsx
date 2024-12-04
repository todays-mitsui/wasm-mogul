import classNames from "classnames";
import type { JSX } from "solid-js";
import styles from "./SideTools.module.css";

interface Props {
  class?: string | string[];
}

export default function SideTools(props: Props): JSX.Element {
  const className =
    props.class == null
      ? []
      : Array.isArray(props.class)
        ? props.class
        : [props.class];

  return <nav class={classNames(...className, styles.side_tools)}>A</nav>;
}
