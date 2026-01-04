import './assets/tailwind.css'

import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import Register from './components/Register.vue' // Assuming you have a Register component
import Login from './components/Login.vue'

const routes = [
  { path: '/', redirect: '/login' }, // Default to login
  { path: '/register', component: Register },
  { path: '/login', component: Login }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

createApp(App).use(router).mount('#app')
