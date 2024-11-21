import { createSignal } from "solid-js";
import { parseExpr } from '../../../ski3/pkg/index';

export default function ParseExpr() {
  const [inputStr, setInputStr] = createSignal('');
  const expr = () => {
    try {
      return parseExpr(inputStr());
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div>
      <input value={inputStr()} onInput={(event) => setInputStr(event.target.value)} />
      <div>
        <pre><code>{JSON.stringify(expr(), null, 2)}</code></pre>
      </div>
    </div>
  );
}
