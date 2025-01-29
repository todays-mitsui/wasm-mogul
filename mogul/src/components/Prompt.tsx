import classNames from "classnames";
import { type JSX, createEffect, createSignal, splitProps } from "solid-js";
import { type Command, parseCommand, runCommand } from "~/service/command";
import { randomSpell } from "~/service/randomSpell";
import {
  addCommandHistory,
  commandHistory,
  commandStr,
  setCommandStr,
} from "~/signals";
import styles from "./Prompt.module.css";

const [historyIndex, setHistoryIndex] = createSignal(-1);

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
    setHistoryIndex(-1);
    runCommand(command);
    setCommandStr("");
  };

  // Enter キー押下で (改行ではなく) コマンドを実行する
  // ただし Shift, Ctrl, Command が同時に押下されていれば改行する
  const onEnter: JSX.EventHandler<HTMLTextAreaElement, KeyboardEvent> = (
    event,
  ) => {
    if (
      event.key === "Enter" &&
      !event.shiftKey &&
      !event.ctrlKey &&
      !event.metaKey
    ) {
      event.preventDefault();
      event.currentTarget.form?.dispatchEvent(new Event("submit"));
    }
  };

  const onRandomSpell: JSX.EventHandler<HTMLButtonElement, MouseEvent> = () => {
    setCommandStr(randomSpell());
  };

  return (
    <div class={classNames(...className, styles.prompt)}>
      <form onSubmit={onSubmit}>
        <div class={styles.buttons}>
          <button
            class={classNames(styles.button, styles.randomSpell)}
            type="button"
            title="Random Spell"
            onClick={onRandomSpell}
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
          onKeyDown={onEnter}
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
          const history = props.history[props.history.length - 1 - newIndex];
          setText(history);
          setCommandStr(history); // 密結合！
          event.preventDefault();
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
        const history = props.history[props.history.length - 1 - newIndex];
        setText(history);
        setCommandStr(history); // 密結合！
        event.preventDefault();
      } else if (currentLine === lastLine && historyIndex() === 0) {
        setHistoryIndex(-1);
        setText(typingText());
        setCommandStr(typingText()); // 密結合！
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
  ref?: HTMLTextAreaElement;
  [key: string]: any;
}

function ElasticInput(props: ElasticInputProps): JSX.Element {
  const [, remainingProps] = splitProps(props, [
    "value",
    "onInput",
    "rows",
    "ref",
  ]);

  let textareaRef: HTMLTextAreaElement | undefined;
  const value = () => {
    if (textareaRef != null) {
      setTimeout(() => {
        textareaRef.style.height = "auto";
        textareaRef.style.height = `${textareaRef.scrollHeight}px`;
      }, 0);
    }
    return props.value;
  };

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
      value={value()}
      ref={textareaRef}
      onInput={onInput}
      rows={props.rows ?? 1}
      {...remainingProps}
    />
  );
}
