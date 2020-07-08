<template>
  <div class="playground">
    <canvas :height="height" id="canvas" :width="width" />
  </div>
</template>

<script>
  import uid from 'uid';

  export default {
    setup: () => ({
      height: window.innerHeight,
      id: uid(8),
      width: window.innerWidth,
    }),
  }
</script>

<style lang="scss">
  .playground {
    height: 100%;
    width: 100%;
  }
</style>
