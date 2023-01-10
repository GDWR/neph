import {createApp} from "vue";
import "./style.css";
import {createRouter, createWebHashHistory} from "vue-router";
import App from "./App.vue";
import Overview from "./views/Overview.vue";
import Namespaces from "./views/Namespaces.vue";
import Nodes from "./views/Nodes.vue";
import Pods from "./views/Pods.vue";
import {library} from '@fortawesome/fontawesome-svg-core'
import {fas} from '@fortawesome/free-solid-svg-icons'
import {FontAwesomeIcon} from '@fortawesome/vue-fontawesome'
import Settings from "./views/Settings.vue";
import Pod from "./views/Pod.vue";


const routes = [
    {path: '/', component: Overview},
    {path: '/namespaces', component: Namespaces},
    {path: '/nodes', component: Nodes},
    {path: '/pods', component: Pods},
    {path: '/pod/:uid', component: Pod},
    {path: '/settings', component: Settings},
];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

library.add(fas);

createApp(App)
    .component('font-awesome-icon', FontAwesomeIcon)
    .use(router)
    .mount("#app");
