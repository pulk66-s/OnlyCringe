import { createApp } from 'vue'
import App from './App.vue'
import * as VueRouter from 'vue-router'

const router = VueRouter.createRouter({
    history: VueRouter.createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'Home',
            component: () => import('./pages/HomePage.vue')
        },
        {
            path: "/account/create",
            name: "CreateAccount",
            component: () => import("./pages/account/CreateAccountPage.vue")
        },
        {
            path: "/topics",
            name: "Topics",
            component: () => import("./pages/topics/HubPage.vue")
        }
    ]
})

const app = createApp(App)
app.use(router).mount('#app')
app.mount('app')