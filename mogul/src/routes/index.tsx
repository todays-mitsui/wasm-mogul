// import '../global.css';
import "./index.css";
import { clientOnly } from "@solidjs/start";
import Header from "~/components/Header";
import styles from "~/routes/index.module.css";
import { Analytics } from 'vercel-analytics-solid/solidstart';

const SideTools = clientOnly(() => import("~/components/SideTools"));
const Prompt = clientOnly(() => import("~/components/Prompt"));
const Console = clientOnly(() => import("~/components/Console"));

export default function Page() {
  return (
    <>
      <Analytics />
      <Header />
      <main class={styles.main}>
        <SideTools class={styles.sideTools} />
        <Console class={styles.console} />
        <Prompt class={styles.prompt} />
      </main>
    </>
  );
}
