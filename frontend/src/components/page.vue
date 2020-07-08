<template>
  <div class="page">
    <playground />
    <div class="overlay">
      <slot />
    </div>
  </div>
</template>

<script>
  import Playground from '@/components/playground';

  export default {
    components: {
      Playground,
    },
  };
</script>

<style lang="scss">
  .page {
    height: 100%;
    position: relative;
    width: 100%;

    .overlay {
      background: transparent;
      height: 100%;
      position: absolute;
      top: 0;
      width: 100%;
    }
  }
</style>
