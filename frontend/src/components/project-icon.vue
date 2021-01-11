<template>
  <rust-icon v-if="lang === 'rust'" class="project-icon" />
  <godot-icon v-else-if="lang === 'godot'" class="project-icon" />
  <deno-icon v-else-if="lang === 'deno'" class="project-icon" />
  <javascript-icon v-else class="project-icon" />
</template>

<script>
  import RustIcon from '@/components/icons/rust';
  import GodotIcon from '@/components/icons/godot';
  import DenoIcon from '@/components/icons/deno';
  import JavascriptIcon from '@/components/icons/javascript';

  export default {
    components: {
      RustIcon,
      GodotIcon,
      DenoIcon,
      JavascriptIcon,
    },
    props: {
      lang: String,
    }
  };
</script>

<style lang="scss">
  .project__icon {
    align-self: center;
    color: #3a3a3a;
    width: 1.5rem;
    height: 1.5rem;
  }
</style>
