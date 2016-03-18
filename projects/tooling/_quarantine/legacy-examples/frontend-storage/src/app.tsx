import { signal } from "@wae/client";

const label = signal("frontend-storage");

export function App() {
  return (
    <main class="page">
      <h1>{label.value}</h1>
      <p>验证矩阵骨架。</p>
    </main>
  );
}
