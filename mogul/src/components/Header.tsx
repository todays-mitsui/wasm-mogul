import { A } from "@solidjs/router";

export default function Header() {
  return (
    <header>
      <h1>ski Mogul</h1>
      <ul>
        <li>
          <a href="https://github.com/todays-mitsui/wasm-mogul" target="_blank">
            repo
          </a>
        </li>
        <li>
          <A href="/doc">doc</A>
        </li>
      </ul>
    </header>
  );
}
