import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import { Analytics } from 'vercel-analytics-solid/solidstart';

export default function App() {
  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Title>skiMogul - Lambda Calculus</Title>
          <Analytics />
          <Suspense>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
