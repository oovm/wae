import type { ViewNode } from "./types.js";

declare global {
  namespace JSX {
    type Element = ViewNode;
    type ElementType = string | ((props: Record<string, unknown>) => ViewNode | null);

    interface IntrinsicElements {
      [elemName: string]: Record<string, unknown>;
    }

    interface ElementChildrenAttribute {
      children: unknown;
    }
  }
}

export {};
