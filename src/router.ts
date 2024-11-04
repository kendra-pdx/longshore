import { createMemoryHistory, createRouter, RouteRecordRaw } from "vue-router";
import Builder from "./Builder.vue";
import Configure from "./Configure.vue";

const routes: RouteRecordRaw[] = [
    { name: "default", path: "/", redirect: "/builder" },
    { name: "builder", path: "/builder", component: Builder },
    { name: "configure", path: "/configure", component: Configure }
]
export const router = createRouter({
    routes,
    history: createMemoryHistory()
})