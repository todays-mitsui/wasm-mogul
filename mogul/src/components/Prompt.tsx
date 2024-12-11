import classNames from "classnames";
import { type JSX, createSignal } from "solid-js";
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

  const [focus, setFocus] = createSignal(false);

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
        <div class={styles.buttons}>
          <button
            class={classNames(styles.button, styles.randomSpell)}
            type="button"
            title="Random Spell"
          >
            Random
          </button>
          <button
            class={classNames(styles.button, styles.submit)}
            type="submit"
          >
            Run
          </button>
        </div>
        <input
          class={styles.input}
          value={commandStr()}
          placeholder={focus() ? "" : "_"}
          onFocusIn={() => setFocus(true)}
          onFocusOut={() => setFocus(false)}
          onInput={(event) => setCommandStr(event.target.value)}
        />
      </form>
    </div>
  );
}
