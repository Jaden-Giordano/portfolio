<template>
  <div class="playground">
    <canvas ref="canvas" :height="height" :width="width" />
  </div>
</template>

<script>
  import { onMounted, ref } from '@vue/composition-api';

  export default {
    setup: () => {
      const width = ref(document.body.clientWidth);
      const height = ref(document.body.clientHeight - 8);
      const canvas = ref(null);

    window.addEventListener("resize", () => {
      width.value = document.body.clientWidth;
      height.value = document.body.clientHeight - 8;
    });

      const worker = new Worker('../playground-worker.js', { type: 'module' });

      onMounted(async () => {
        const offscreen = canvas.value.transferControlToOffscreen();
        worker.postMessage({ message: 'LOAD', canvas: offscreen }, [offscreen]);
      });

      return {
        canvas,
        height,
        width,
      };
    },
  }
</script>

<style lang="scss">
.playground {
  box-sizing: border-box;
  height: 100%;
  width: 100%;
}
</style>
