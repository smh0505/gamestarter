import { createApp } from "vue";
import App from "./App.vue";
import File from "./File.vue";
import "./style.css";

if (document.querySelector("#app")) createApp(App).mount("#app");
if (document.querySelector("#file")) createApp(File).mount("#file");
