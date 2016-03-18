import type { ViewNode } from "@wae/client";
import "@wae/client";

export type Theme = {
  colorScheme?: "light" | "dark" | "system";
  colors?: Record<string, string>;
  components?: Record<string, Record<string, unknown>>;
};

export function createTheme(partial: Theme = {}): Theme {
  return {
    colorScheme: "system",
    ...partial,
  };
}

export function WaeProvider(props: {
  theme?: Theme;
  children?: unknown;
}): ViewNode {
  return {
    type: "wae-provider",
    props: { theme: props.theme ?? createTheme() },
    children: Array.isArray(props.children)
      ? (props.children as ViewNode[])
      : props.children
        ? [props.children as ViewNode]
        : [],
  };
}
