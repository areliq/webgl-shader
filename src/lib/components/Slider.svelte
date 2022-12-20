<div class="container">
  <div class="image">
    <slot />
  </div>
  <div class="indicator">
    <button class="cursor prev" disabled={current < 1} on:click|preventDefault={update(current - 1)}>&lt;</button>
    {#each Array(num) as _, idx}
    {@const active = idx === current}
    <button class="selector" class:active on:click|preventDefault={update(idx)}></button>
    {/each}
    <button class="cursor next" disabled={current >= num - 1} on:click|preventDefault={update(current + 1)}>&gt;</button>
  </div>
</div>

<style>
  .container {
    background-color: #fff;
    border-radius: 4px;
    padding: 20px;
    box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .indicator {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-around;
    gap: 20px;
    margin-top: 20px;
  }
  .selector {
    height: 16px;
    width: 48px;
    border-radius: 10%;
    border: 3px solid var(--color-theme-2);
    background-color: transparent;
  }
  .indicator .active {
    background-color: var(--color-theme-2);
    transition-property: background-color;
    transition-duration: 0.6s;
  }
  .cursor {
    height: 32px;
    width: 32px;
    color: #fff;
    background-color: var(--color-theme-2);
    font-weight: bolder;
    margin: auto;
    border: none;
    border-radius: 4px;
  }
  /* .prev:active {
    transform: translate(-4px, 0);
  }
  .next:active {
    transform: translate(4px, 0);
  }
  .prev:disabled {
    transform: translate(4px, 0);
  }
  .next:disabled {
    transform: translate(-4px, 0);
  } */
  .cursor:hover {
    filter:brightness(1.2);
    transition-property: filter;
    transition-duration: 0.4s;
  }
  .cursor:disabled {
    filter:brightness(0.8);
    cursor: not-allowed;
    transition-property: filter;
    transition-duration: 0.4s;
  }
</style>

<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let num: number = 3;
  export let current: number = 0;
  // export let prev = (idx: number) => () => { console.log('prev: ' + idx) }
  // export let next = (idx: number) => () => { console.log('next: ' + idx) }
  // export let select = (idx: number) => () => { console.log('select: ' + idx) }

  const update = (idx: number) => () => {
    if (current < 0 || num <= current) { return };
    current = idx; 
    dispatch('slide', { next: current }); 
  };
</script>
