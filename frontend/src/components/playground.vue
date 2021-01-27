<template>
  <div class="playground" ref="root">
    <canvas ref="canvas" width="100%" height="100%" />
  </div>
</template>

<script>
import { onMounted, ref } from "@vue/composition-api";

export default {
  setup: () => {
    const root = ref(null);
    const canvas = ref(null);

    window.addEventListener("resize", () => {
      canvas.width = root.value.offsetWidth;
      canvas.height = root.value.offsetHeight;
    });

    onMounted(async () => {
      canvas.value.width = root.value.offsetWidth;
      canvas.value.height = root.value.offsetHeight;

      // Use OffscreenCanvas if browser supports it.
      if (canvas.value.transferControlToOffscreen) {
        const worker = new Worker("../playground-worker.js", {
          type: "module",
        });
        const offscreen = canvas.value.transferControlToOffscreen();
        // Pass the OffscreenCanvas to the web worker.
        worker.postMessage({ message: "LOAD", canvas: offscreen }, [offscreen]);
      } else {
        import("@portfolio/webgl").then((module) => {
          const gl = canvas.value.getContext("webgl");
          let n = Math.floor(Math.random() * 3);
          const client = new module.FolioClient(gl, n);

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
      root,
    };
  },
};
</script>

<style lang="scss">
.playground {
  box-sizing: border-box;
  height: 100%;
}
</style>
