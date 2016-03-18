import type { WaeRuntime } from "@wae/client";

export function attachWae(root: ParentNode, runtime: WaeRuntime): () => void {
  void root;
  void runtime;
  return () => {};
}
