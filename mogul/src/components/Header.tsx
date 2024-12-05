import { A } from "@solidjs/router";
import styles from "./Header.module.css";

export default function Header() {
  return (
    <header class={styles.header}>
      <h1>
        <A href="/">ski Mogul</A>
      </h1>
      <ul>
        <li>
          <a
            href="https://github.com/todays-mitsui/wasm-mogul"
            target="_blank"
            rel="noreferrer"
          >
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
