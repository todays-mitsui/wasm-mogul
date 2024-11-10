import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { loadWasmModule } from '../../ski2/loadWasmModule.ts';

createRoot(document.getElementById("root")!).render(
	<StrictMode>
		<App />
	</StrictMode>,
);

console.log({ loadWasmModule });

const wasmModule = await loadWasmModule();
console.log({ wasmModule });

const { parseExpr, formatExpr } = wasmModule;
console.log({ parseExpr, formatExpr });

const expr = parseExpr("```sxyz");
console.log({ expr });

const formattedExpr = formatExpr(expr, "ECMAScript");
console.log({ formattedExpr });
