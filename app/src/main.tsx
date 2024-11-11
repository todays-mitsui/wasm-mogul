import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { printContext } from '../../ski3/pkg/index.js';

createRoot(document.getElementById("root")!).render(
	<StrictMode>
		<App />
	</StrictMode>,
);

console.log({printContext});
console.log(printContext({
	i: {
		name: 'i',
		params: [],
		body: {Variable: {identifier: 'i'}}
	},
	k: {
		name: 'k',
		params: [ 'x', 'y' ],
		body: {Variable: {identifier: 'x'}}
	},
}));
