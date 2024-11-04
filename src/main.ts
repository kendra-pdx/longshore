import { createApp } from "vue";
import App from "./App.vue";
import "./main.css"
import PrimeVue from "primevue/config"
import { primeConfig } from "./prime.ts";
import "primeicons/primeicons.css";
import { router } from "./router.ts";

const app = createApp(App)
app.use(PrimeVue, primeConfig)
app.use(router)
app.mount("#app");
