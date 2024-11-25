// import '../global.css';
import "./index.css";
import { clientOnly } from "@solidjs/start";
import Header from "~/components/Header";

const Prompt = clientOnly(() => import("../components/Prompt"));
const Console = clientOnly(() => import("../components/Console"));

export default function Page() {
  return (
    <main>
      <Header />
      <Console />
      <Prompt />
    </main>
  );
}
