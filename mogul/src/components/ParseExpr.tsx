import { createSignal } from "solid-js";
import { type Expr, parseExpr } from "../../../ski3/pkg/index";
import styles from "./ParseExpr.module.css";

export default function ParseExpr() {
  const [inputStr, setInputStr] = createSignal("");
  const [parsedExpr, setParsedExpr] = createSignal<Expr | null>(null);
  const onClick = () => {
    try {
      const expr = parseExpr(inputStr());
      setParsedExpr(expr);
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div class={styles.parse_expr}>
      <input
        value={inputStr()}
        onInput={(event) => setInputStr(event.target.value)}
      />
      <button onClick={onClick}>Parse</button>
      <div>
        <pre>
          <code>{JSON.stringify(parsedExpr(), null, 2)}</code>
        </pre>
      </div>
    </div>
  );
}
