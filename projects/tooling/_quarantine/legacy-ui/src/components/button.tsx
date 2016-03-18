import type { ViewNode } from "@wae/client";
import "@wae/client";

export type ButtonProps = {
  variant?: "primary" | "secondary" | "ghost" | "danger";
  size?: "sm" | "md" | "lg";
  loading?: boolean;
  disabled?: boolean;
  onPress?: () => void;
  onClick?: () => void;
  children?: unknown;
};

export function Button(props: ButtonProps): ViewNode {
  const {
    variant = "primary",
    size = "md",
    loading = false,
    disabled = false,
    onPress,
    onClick,
    children,
  } = props;
  return {
    type: "button",
    props: {
      class: `wae-btn wae-btn-${variant} wae-btn-${size}`,
      disabled: disabled || loading,
      onClick: () => {
        onPress?.();
        onClick?.();
      },
    },
    children: [
      {
        type: "text",
        props: { value: String(children ?? "") },
        children: [],
      },
    ],
  };
}
