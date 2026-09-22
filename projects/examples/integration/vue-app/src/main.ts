import { createClient } from "@wae/client";
import { provideWae } from "@wae/adapter-vue";
import { createApp } from "vue";
import App from "./App.vue";

const client = createClient({ server: { baseUrl: "/api" } });
const app = createApp(App);
provideWae(app, client);
app.mount("#app");
