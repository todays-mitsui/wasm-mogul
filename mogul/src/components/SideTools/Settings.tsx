import type { JSX } from "solid-js";
import styles from "./Settings.module.css";
import { displayStyle, setDisplayStyle, sideTools } from "~/signals";
import { resetContext, clearContext } from "~/service/context";

export default function Settings(): JSX.Element {
  return (
    <>
      <fieldset class={styles.fieldset}>
        <legend>Expr Display Style</legend>
        <label>
          <input
            type="radio"
            name="tuber_display_style"
            value="ECMAScript"
            checked={displayStyle() === "EcmaScript"}
            onClick={() => setDisplayStyle("EcmaScript")}
          />
          <span>
            ECMAScript Style
            <small>
              e.g. <code>(x =&gt; f(x))(y)</code>
            </small>
          </span>
        </label>
        <label>
          <input
            type="radio"
            name="tuber_display_style"
            value="Lazy_K"
            checked={displayStyle() === "LazyK"}
            onClick={() => setDisplayStyle("LazyK")}
          />
          <span>
            Lazy_K Style
            <small>
              e.g. <code>`λx.`fx y</code>
            </small>
          </span>
        </label>
      </fieldset>
      <fieldset class={styles.fieldset}>
        <legend>Reset Context</legend>
        <ul>
          <li>
            <button
              type="button"
              name="tuber_reset_context"
              onClick={() => {
                if (confirm("Are you sure you want to reset the context?")) {
                  resetContext();
                  sideTools.open("context");
                }
              }}
            >
              Reset to default
            </button>
          </li>
          <li>
            <button
              type="button"
              name="tuber_clear_context"
              onClick={() => {
                if (confirm("Are you sure you want to clear all functions?")) {
                  clearContext();
                  sideTools.open("context");
                }
              }}
            >
              Clear all functions
            </button>
          </li>
        </ul>
      </fieldset>
    </>
  );
}
