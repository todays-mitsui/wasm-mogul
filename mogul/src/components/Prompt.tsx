import { type JSX } from "solid-js";
import { commandStr, setCommandStr } from "~/signals";
import { type Command, parseCommand, runCommand } from "~/service/command";
import styles from "./Prompt.module.css";

export default function Prompt(): JSX.Element {
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
  };

  return (
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
  );
}
