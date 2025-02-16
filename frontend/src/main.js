import "./styles.css";
import "vuetify/styles";

import { createApp } from "vue";
import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi";
import "@mdi/font/css/materialdesignicons.css";

import App from "./App.vue";

const vuetify = createVuetify({
    theme: {
        defaultTheme: "dark",
    },
    icons: {
        defaultSet: "mdi",
        aliases,
        sets: {
            mdi,
        },
    },
});

createApp(App).use(vuetify).mount('#app');
