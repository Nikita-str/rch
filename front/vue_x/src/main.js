import { createApp, h } from 'vue'

import store from './js/store'
import router from './js/router'

import Render from "./Render.vue"

import drag from "v-drag"

import './assets/main.css'

const app = createApp({
    render: ()=>h(Render)
})
app.use(store)
app.use(router)
app.use(drag)
app.mount('#app')

