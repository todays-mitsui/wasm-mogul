// import '../global.css';
import "./index.css";
import { clientOnly } from "@solidjs/start";
import Header from "~/components/Header";
import styles from "~/routes/index.module.css";

const SideTools = clientOnly(() => import("~/components/SideTools"));
const Prompt = clientOnly(() => import("~/components/Prompt"));
const Console = clientOnly(() => import("~/components/Console"));

export default function Page() {
  return (
    <>
      <Header />
      <main class={styles.main}>
        <SideTools class={styles.side_tools} />
        <Console class={styles.console} />
        <Prompt class={styles.prompt} />
      </main>
    </>
  );
}
