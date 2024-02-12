import { createRouter, createWebHashHistory } from 'vue-router'

import Home from "./pages/Home.vue";
import Vm from "./pages/Vm.vue";
import New from './pages/New.vue'

const routes = [
  { path: "/", component: Home },
  { path: "/vms", component: Vm },
  { path: "/new", component: New },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;


