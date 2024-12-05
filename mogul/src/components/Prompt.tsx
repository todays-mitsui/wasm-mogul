import classNames from "classnames";
import type { JSX } from "solid-js";
import { type Command, parseCommand, runCommand } from "~/service/command";
import { commandStr, setCommandStr } from "~/signals";
import styles from "./Prompt.module.css";

interface Props {
  class?: string | string[];
}

export default function Prompt(props: Props): JSX.Element {
  const className =
    props.class == null
      ? []
      : Array.isArray(props.class)
        ? props.class
        : [props.class];

  const onSubmit: JSX.EventHandler<HTMLFormElement, SubmitEvent> = (event) => {
    event.preventDefault();
    console.info({ commandStr: commandStr() });
    let command: Command;
    try {
      command = parseCommand(commandStr());
    } catch (err) {
      console.error(err);
      return;
    }
    console.info({ command });
    runCommand(command);
    setCommandStr("");
  };

  return (
    <div class={classNames(...className, styles.prompt)}>
      <form onSubmit={onSubmit}>
        <div>
          <button class={styles.button} type="button" title="Random Spell">
            Random
          </button>
          <button class={styles.button} type="submit">
            Run
          </button>
        </div>
        <input
          value={commandStr()}
          onInput={(event) => setCommandStr(event.target.value)}
        />
      </form>
    </div>
  );
}
