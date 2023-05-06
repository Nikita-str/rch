import { createStore } from 'vuex'
import axios from 'axios'

function setPropsByName(to, from, props) {
    for (const prop of props) {
        to[prop] = from[prop]
      }
}

export default createStore({
    state: {
        port: "/api",

        total_post: null,
        open_board: null,
        speed_post: null,

        pop_boards: null,
    },


    getters: {
        getPort: state => state.port,

        getTotalPost: state => state.total_post,
        getOpenBoardAmount: state => state.open_board,
        getSpeedPost: state => state.speed_post,

        getPopBoards: state => state.pop_boards,
    },


    mutations: {
        updCommonAll(state, obj) {
            console.log("HERE#updCommonAll:START")
            console.log(obj)
            setPropsByName(state, obj, ['total_post', 'open_board', 'speed_post'])
            console.log("HERE#updCommonAll:END")
        },
        updPopBoards(state, obj) {
            console.log("pop boards obj", obj)
            state.pop_boards = obj
        }
    },


    actions: {
        updCommonInfo({ getters, commit }) {
            return axios({
                url: getters.getPort + '/common/all',
                method: 'get',
            }).then(res => {
                commit('updCommonAll', res.data)
            }).catch(err => { console.log(err.response) });
        },
        updPopBards({ getters, commit }) {
            return axios({
                url: getters.getPort + '/common/pop_boards',
                method: 'get',
            }).then(res => {
                commit('updPopBoards', res.data)
            }).catch(err => { console.log(err.response) });
        },
    },
})
