import './assets/clear-styles.css'
import './assets/tailwind.css'
import { createApp } from "vue";
import App from "./App.vue";
import '@mdi/font/css/materialdesignicons.css';
// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import './assets/main.css'
import router from './router';

const vuetify = createVuetify({
    components,
    directives,
})
createApp(App).use(vuetify).use(router).mount("#app");
