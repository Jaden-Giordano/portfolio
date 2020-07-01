import Vue from "vue";
import App from "./App.vue";

import wasm from '../webgl.wasm';

Vue.config.productionTip = false;

new Vue({
  render: h => h(App)
}).$mount("#app");
