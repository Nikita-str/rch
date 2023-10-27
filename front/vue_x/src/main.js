import { createApp, h } from 'vue'

import store from './js/store'
import router from './js/router'

import Render from "./Render.vue"

import './assets/main.css'
import './assets/post_elems.css'

const app = createApp({
    render: ()=>h(Render)
})
app.use(store)
app.use(router)
app.mount('#app')

