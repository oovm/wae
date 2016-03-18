import { signal } from "@wae/client";
import { Button, WaeProvider } from "@wae/ui";

const count = signal(0);

export function App() {
  return (
    <WaeProvider>
      <main>
        <h1>{{name}}</h1>
        <Button onPress={() => count.value++}>{String(count.value)}</Button>
      </main>
    </WaeProvider>
  );
}

export default App;
