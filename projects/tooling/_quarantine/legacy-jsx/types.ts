/** 共享视图类型（避免 jsx-runtime ↔ index 循环依赖）。 */

export type NodeId = string;

export type ComponentType = (props: Record<string, unknown>) => ViewNode | null;

export type ViewNode = {
  type: string | ComponentType;
  props: Record<string, unknown>;
  key?: string;
  children?: ViewNode[];
};
