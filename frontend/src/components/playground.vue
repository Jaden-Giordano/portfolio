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

      onMounted(async () => {
        // Use OffscreenCanvas if browser supports it.
        if (canvas.value.transferControlToOffscreen) {
          const worker = new Worker('../playground-worker.js', { type: 'module' });
          const offscreen = canvas.value.transferControlToOffscreen();
          worker.postMessage({ message: 'LOAD', canvas: offscreen }, [offscreen]);
        } else {
          import('@portfolio/webgl').then(module => {
            const gl = canvas.value.getContext('webgl');
            const client = new module.FolioClient(gl);

            const render = () => {
              client.update();
              client.render();
              requestAnimationFrame(render);
            };
            requestAnimationFrame(render);
          });
        }
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
