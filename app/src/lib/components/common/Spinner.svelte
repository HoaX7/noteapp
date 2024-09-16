<script lang="ts">
  import clsx from "clsx";
  export let className = "";
  export let size: "xxs" | "xs" | "sm" | "md" | "lg" | "xl" = "sm";

  const classMap = {
    xxs: {
      h: "h-4",
      w: "w-4",
    },
    xs: {
      h: "h-8",
      w: "w-8",
    },
    sm: {
      h: "h-12",
      w: "w-12",
    },
    md: {
      h: "h-16",
      w: "w-16",
    },
    lg: {
      h: "h-24",
      w: "w-24",
    },
    xl: {
      h: "h-34",
      w: "w-34",
    },
  };
</script>

<div
  class={clsx(
    "ease-linear rounded-full border-2 border-t-2 loader",
    classMap[size].h,
    classMap[size].w,
    className
  )}
></div>

<style>
  .loader {
    border-top-color: rgb(37 99 235 / 1);
    -webkit-animation: spinner 1s linear infinite;
    animation: spinner 1s linear infinite;
  }

  @keyframes spinner {
    0% {
      -webkit-transform: rotate(0deg);
      transform: rotate(0deg);
    }
    100% {
      -webkit-transform: rotate(360deg);
      transform: rotate(360deg);
    }
  }
</style>
