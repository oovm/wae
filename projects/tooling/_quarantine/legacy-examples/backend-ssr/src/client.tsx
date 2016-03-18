import { signal } from "@wae/client";

const ready = signal(true);

export function App() {
  return <main>SSR client takeover skeleton: {String(ready.value)}</main>;
}
