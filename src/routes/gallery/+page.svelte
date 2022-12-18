<script lang="ts">
  import Slider from "$lib/components/Slider.svelte"

//   const num = 3
//   let current = 0;
//   const prev = (idx: number) => () => { if (current > 0) {current--} console.log(current) };
//   const next = (idx: number) => () => { if (current < num - 1) {current++} console.log(current) };
//   const select = (idx: number) => () => { current = idx; console.log(current) };

//   const slider = {
//     num, current, prev, next, select
//   }
</script>

<Slider num={5} on:slide={event => console.log(event.detail.next)}>
  <canvas />
</Slider>

<style>
    canvas {
      width: 420px;
      height: 340px;
      display: block;
      background-color: aquamarine;
    }
</style>
