import { createRouter, createWebHashHistory } from 'vue-router'
import App from '../App.vue'
import Tmp from '../Tmp.vue'
import PageNotFound from '../components/PageNotFound.vue'
import Board from '../components/Board.vue'

const maxBoardNameLen = 16;

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
        path: '/tmp/', 
        name: 'TMPch',
        meta: {
          title: "TMPch",
        },
        component: Tmp 
    },

    {
      path: '/:pathMatch([a-z]{1,' + maxBoardNameLen + '})/',
      component: Board,
    },

    { 
      path: '/:pathMatch(.*)*', 
      component: PageNotFound,
    },
]

export default createRouter({
    history: createWebHashHistory(),
    routes,
    sensitive: true,
})

