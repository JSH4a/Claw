import { createApp } from 'vue'
import App from './App.vue'
import mitt from 'mitt';

const pathEmitter = mitt();
const resultSortTypeEmitter = mitt();

const app = createApp(App);

app.config.globalProperties.pathEmitter = pathEmitter;
app.config.globalProperties.resultSortTypeEmitter = resultSortTypeEmitter;

app.mount("#app");
