import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { defaultContext, renderExpr, parseExpr, Reducer } from '../../ski3/pkg/index.js';

createRoot(document.getElementById("root")!).render(
	<StrictMode>
		<App />
	</StrictMode>,
);

const context = defaultContext();
console.log(context);
Object.defineProperty(window, 'mogulContext', {
  value: context,
  writable: false,
});

function reduce(exprStr: string) {
	const expr = parseExpr(exprStr);
	return {
		[Symbol.iterator]() {
			return new Reducer(context, expr);
		}
	}
}

Object.defineProperty(window, 'reduce', {
  value: reduce,
  writable: false,
});


Object.defineProperty(window, 'mogulParseExpr', {
  value: parseExpr,
  writable: false,
});

Object.defineProperty(window, 'MogulReducer', {
  value: Reducer,
  writable: false,
});

Object.defineProperty(window, 'mogulRenderExpr', {
  value: renderExpr,
  writable: false,
});
