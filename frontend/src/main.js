import Vue from "vue";
import VueCompositionApi from '@vue/composition-api';
import App from "./App.vue";

import './style.scss';

Vue.config.productionTip = false;

Vue.use(VueCompositionApi);

const worker = new Worker('./playground-worker.js', { type: 'module' });

new Vue({
  render: h => h(App)
}).$mount("#app");
