import type { ComponentType, ViewNode } from "./types.js";

export type { ViewNode, ComponentType };

export function jsx(
  type: string | ComponentType,
  props: Record<string, unknown> | null,
  key?: string,
): ViewNode {
  const safeProps = props ?? {};
  const { children, ...rest } = safeProps;
  return {
    type,
    props: rest,
    key,
    children: normalizeChildren(children),
  };
}

export function jsxs(
  type: string | ComponentType,
  props: Record<string, unknown> | null,
  key?: string,
): ViewNode {
  return jsx(type, props, key);
}

export function Fragment(props: { children?: unknown }): ViewNode {
  return {
    type: "fragment",
    props: {},
    children: normalizeChildren(props.children),
  };
}

function normalizeChildren(children: unknown): ViewNode[] {
  if (children == null || children === false) return [];
  if (Array.isArray(children)) {
    return children.flatMap((child) => normalizeChildren(child));
  }
  if (typeof children === "object" && children !== null && "type" in (children as object)) {
    return [children as ViewNode];
  }
  return [
    {
      type: "text",
      props: { value: String(children) },
      children: [],
    },
  ];
}
