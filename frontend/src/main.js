import Vue from "vue";
import App from "./App.vue";

import wasm from '@portfolio/webgl';

Vue.config.productionTip = false;

new Vue({
  render: h => h(App)
}).$mount("#app");
