<template>
  <div class="playground">
    <canvas :height="height" id="canvas" :width="width" />
  </div>
</template>

<script>
  import { ref } from '@vue/composition-api';
  import uid from 'uid';

  export default {
    setup: () => {
      const width = ref(document.body.clientWidth);
      const height = ref(document.body.clientHeight - 8);

      window.addEventListener('resize', () => {
        width.value = document.body.clientWidth;
        height.value = document.body.clientHeight - 8;
      });

      import('@portfolio/webgl').then((folio) => {
        const client = new folio.FolioClient();

        setInterval(() => {
          client.update();
          client.render();
        }, 100);
      });

      return {
        height,
        id: uid(8),
        width,
      }
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
