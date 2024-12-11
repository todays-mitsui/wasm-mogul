import classNames from "classnames";
import { type JSX, createEffect, createSignal, splitProps } from "solid-js";
import { type Command, parseCommand, runCommand } from "~/service/command";
import {
  commandStr,
  setCommandStr,
  commandHistory,
  addCommandHistory,
} from "~/signals";
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
    addCommandHistory(commandStr());
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
        <HistoryInput
          class={styles.input}
          value={commandStr()}
          placeholder={focus() ? "" : "_"}
          onFocusIn={() => setFocus(true)}
          onFocusOut={() => setFocus(false)}
          onInput={(event) => setCommandStr(event.target.value)}
          history={commandHistory()}
        />
      </form>
    </div>
  );
}

// ========================================================================== //

type TextareaKeyDownEvent = KeyboardEvent & {
  currentTarget: HTMLTextAreaElement;
  target: HTMLTextAreaElement;
};

interface HistoryInputProps extends ElasticInputProps {
  history: string[];
  value: string;
  onInput?: JSX.EventHandler<HTMLTextAreaElement, TextareaInputEvent>;
  onKeyDown?: JSX.EventHandler<HTMLTextAreaElement, TextareaKeyDownEvent>;
}

export function HistoryInput(props: HistoryInputProps): JSX.Element {
  const [, remainingProps] = splitProps(props, [
    "value",
    "history",
    "onInput",
    "onKeyDown",
  ]);

  const [text, setText] = createSignal("");
  const [historyIndex, setHistoryIndex] = createSignal(-1);
  const [typingText, setTypingText] = createSignal(""); // 入力中未確定テキストを逃がしておくための signal

  createEffect(() => {
    setText(props.value);
  });

  const onInput: JSX.EventHandler<HTMLTextAreaElement, TextareaInputEvent> = (
    event,
  ) => {
    setText(event.target.value);
    props.onInput?.(event);
  };

  const onKeyDown: JSX.EventHandler<
    HTMLTextAreaElement,
    TextareaKeyDownEvent
  > = (event) => {
    if (event.key === "ArrowUp") {
      // カーソル位置と1行目の判定
      const textarea = event.target;
      const cursorPosition = textarea.selectionStart;
      const currentLine = textarea.value
        .substring(0, cursorPosition)
        .split("\n").length;

      if (currentLine === 1) {
        if (historyIndex() < 0) {
          setTypingText(text());
        }
        if (historyIndex() < props.history.length - 1) {
          const newIndex = historyIndex() + 1;
          setHistoryIndex(newIndex);
          setText(props.history[props.history.length - 1 - newIndex]);
          event.preventDefault(); // デフォルト動作を防止
        }
      }
    }

    if (event.key === "ArrowDown") {
      const textarea = event.target;
      const cursorPosition = textarea.selectionStart;
      const currentLine = textarea.value
        .substring(0, cursorPosition)
        .split("\n").length;
      const lastLine = textarea.value.split("\n").length;

      if (currentLine === lastLine && historyIndex() > 0) {
        const newIndex = historyIndex() - 1;
        setHistoryIndex(newIndex);
        setText(props.history[props.history.length - 1 - newIndex]);
        event.preventDefault();
      } else if (currentLine === lastLine && historyIndex() === 0) {
        setHistoryIndex(-1);
        setText(typingText());
        event.preventDefault();
      }
    }

    props.onKeyDown?.(event);
  };

  return (
    <ElasticInput
      value={text()}
      onInput={onInput}
      onKeyDown={onKeyDown}
      {...remainingProps}
    />
  );
}

// ========================================================================== //

type TextareaInputEvent = InputEvent & {
  currentTarget: HTMLTextAreaElement;
  target: HTMLTextAreaElement;
};

interface ElasticInputProps {
  value: string;
  onInput?: JSX.EventHandler<HTMLTextAreaElement, TextareaInputEvent>;
  rows?: number;
  [key: string]: any;
}

function ElasticInput(props: ElasticInputProps): JSX.Element {
  const [, remainingProps] = splitProps(props, ["value", "onInput", "rows"]);

  const onInput: JSX.EventHandler<HTMLTextAreaElement, TextareaInputEvent> = (
    event,
  ) => {
    const target = event.target;
    target.style.height = "auto";
    target.style.height = `${target.scrollHeight}px`;

    props.onInput?.(event);
  };

  return (
    <textarea
      value={props.value}
      onInput={onInput}
      rows={props.rows ?? 1}
      {...remainingProps}
    />
  );
}
