import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { library } from "@fortawesome/fontawesome-svg-core";

import {
  faPlus,
  faLock
} from "@fortawesome/free-solid-svg-icons";

import {
  faCircleXmark
} from "@fortawesome/free-regular-svg-icons";

library.add(
  faPlus,
  faLock,
  faCircleXmark
)

createApp(App)
  .component("font-awesome-icon", FontAwesomeIcon)
  .mount("#app");
