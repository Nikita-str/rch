import { createRouter, createWebHashHistory } from 'vue-router'
import App from '../App.vue'
import Tmp from '../Tmp.vue'
import PageNotFound from '../components/PageNotFound.vue'

const routes = [
    { 
        path: '/', 
        name: 'Rch',
        meta: {
          title: "Rch",
        },
        component: App 
    },
    { 
        path: '/tmp', 
        name: 'TMPch',
        meta: {
          title: "TMPch",
        },
        component: Tmp 
    },
    
    { 
      path: '/:pathMatch(.*)*', 
      component: PageNotFound,
    },
]

export default createRouter({
    history: createWebHashHistory(),
    routes,
})

