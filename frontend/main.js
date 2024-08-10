import { createApp } from 'vue'
import App from './App.vue'
import mitt from 'mitt';

const pathEmitter = mitt();
const app = createApp(App);
app.config.globalProperties.pathEmitter = pathEmitter;
app.mount("#app");
