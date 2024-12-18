import classNames from "classnames";
import { type JSX, splitProps } from "solid-js";
import styles from "./Button.module.css";

interface ButtonProps {
  class?: string;
  children?: JSX.Element;
  [key: string]: any;
}

export default function Button(props: ButtonProps): JSX.Element {
  const [, remainingProps] = splitProps(props, ["children"]);

  return (
    <button class={classNames(styles.button, props.class)} {...remainingProps}>
      {props.children}
    </button>
  );
}
