<template>
  <div class="container">
    <home />
  </div>
</template>

<script>
  import Home from '@/pages/home';

  export default {
    components: {
      Home,
    },
  };
</script>

<style lang="scss">
  .container {
    background-color: #202020;
    height: 100%;
    width: 100%;
  }
</style>
