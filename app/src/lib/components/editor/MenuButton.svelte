<script lang="ts">
  import clsx from "clsx";

  export let isActive = false;
  export let classname = "";
</script>

<button {...$$restProps} on:click class={clsx(classname, "hover:bg-gray-200 px-2 py-1 rounded-md",
	"flex place-items-center", "menu-button", isActive ? "bg-gray-200" : ""
)}>
  <slot />
</button>

<style>
    .menu-button {
        margin: 2px;
    }
</style>
