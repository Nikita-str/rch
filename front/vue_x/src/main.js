import { createApp, h } from 'vue'
import App from './App.vue'
import store from './store/store'
import router from './router/router'

import Render from "./Render.vue"

import './assets/main.css'

const app = createApp({
    render: ()=>h(Render)
})
app.use(store)
app.use(router)
app.mount('#app')

