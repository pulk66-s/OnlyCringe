import { createApp } from 'vue'
import App from './App.vue'
import * as VueRouter from 'vue-router'
import HomePage from "./pages/HomePage.vue"

const router = VueRouter.createRouter({
    history: VueRouter.createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'Home',
            component: HomePage
        },
    ]
})

const app = createApp(App)
app.use(router).mount('#app')
app.mount('app')