import ParseExpr from "~/components/ParseExpr";
import "./index.css";

export default function Page() {
  return (
    <main>
      <h1>Hello world!</h1>
      <ParseExpr />
      <p>
        Visit{" "}
        <a href="https://solidjs.com" target="_blank">
          solidjs.com
        </a>{" "}
        to learn how to build Solid apps.
      </p>
    </main>
  );
}
