<script lang="ts">
	import clsx from "clsx";

    type TypographyProps = {
        variant: "div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
        weight: "normal" | "medium" | "semi-bold" | "bold";
        fontSize: "sm" | "base" | "lg" | "xl" | "2xl";
        class?: string;
    }

    export let variant: TypographyProps["variant"];
    export let weight: TypographyProps["weight"];
    export let fontSize: TypographyProps["fontSize"];
    export let classname: TypographyProps["class"] = "";

</script>


<svelte:element this={variant} class={clsx(classname, `text-${fontSize} font-${weight}`)}>
    <slot />
</svelte:element>

<style>
    .text-sm {
        font-size: 12px;
        line-height: 16px;
    }
    .text-base {
        font-size: 14px;
        line-height: 20px;
    }
    .text-lg {
        font-size: 16px;
        line-height: 24px;
    }
    .text-xl {
        font-size: 18px;
        line-height: 28px;
    }
    .text-2xl {
        font-size: 20px;
        line-height: 30px;
    }
    .font-normal {
        font-weight: 400;
    }
    .font-medium {
        font-weight: 500;
    }
    .font-semi-bold {
        font-weight: 600;
    }
    .font-bold {
        font-weight: 700;
    }
</style>