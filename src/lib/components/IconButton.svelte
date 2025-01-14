<script>
  let { icon, ...props } = $props();
</script>

<button {...props} type="button" aria-label="hello" class="btn">
  {#if icon == "resize"}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="icon"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"
      />
    </svg>
  {:else if icon == "close"}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="icon"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M6 18 18 6M6 6l12 12"
      />
    </svg>
  {/if}
</button>

<style>
  .icon {
    width: 14px;
    height: 14px;
  }
  .btn {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 8px;
    background-color: #444;
    border-radius: 100%;
  }
  .btn:hover {
    background-color: #222;
  }
  .btn:active {
    background-color: rgba(182, 20, 20, 0.979);
  }
</style>
