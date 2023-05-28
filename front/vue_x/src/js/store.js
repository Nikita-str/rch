import { createStore } from 'vuex'
import axios from 'axios'

function setPropsByName(to, from, props) {
    for (const prop of props) {
        to[prop] = from[prop]
      }
}

const boardName = {
    name: null,
};

export default createStore({
    state: {
        port: "/api",

        total_post: null,
        open_board: null,
        speed_post: null,

        pop_boards: null,

        existed_boards_name: new Map(),
        unexisted_boards: new Set(),
    },


    getters: {
        getPort: state => state.port,

        getTotalPost: state => state.total_post,
        getOpenBoardAmount: state => state.open_board,
        getSpeedPost: state => state.speed_post,

        getPopBoards: state => state.pop_boards,

        getBoardName: (state) => (board_url) => {
            let board_name = state.existed_boards_name.get(board_url);
            if (board_name) { 
                var ret = Object.create(boardName)
                ret.name = board_name
                return ret
            }
            if (state.unexisted_boards.has(board_url)) {
                return Object.create(boardName)
            }
            return null
        },
    },


    mutations: {
        updCommonAll(state, obj) {
            console.log("HERE#updCommonAll:START")
            console.log(obj)
            setPropsByName(state, obj, ['total_post', 'open_board', 'speed_post'])
            console.log("HERE#updCommonAll:END")
        },
        updPopBoards(state, obj) {
            obj.sort((a, b) => (a.boards.length == b.boards.length) ? a.tag.localeCompare(b.tag) : b.boards.length - a.boards.length)
            console.log("pop boards obj", obj)
            state.pop_boards = obj
        },
        addExistenceBoard(state, { board_url, board_name }) {
            if (board_name) { state.existed_boards_name.set(board_url, board_name) }
            else { state.unexisted_boards.add(board_url) }
        },
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
        /*
        getIsBoardExist({ getters, commit }, board_url) {
            let params = new URLSearchParams([['board_url', board_url]])
            return axios({
                url: getters.getPort + '/board/is_exist',
                method: 'get',
                params,
            }).then(res => {
                const is_exist = res.data
                commit('addExistenceBoard', { board_url, is_exist, })
                return res.data
            }).catch(err => { console.log(err.response) });
        },
        */
        getReqBoardName({ getters, commit }, board_url) {
            let params = new URLSearchParams([['board_url', board_url]])
            return axios({
                url: getters.getPort + '/board/name',
                method: 'get',
                params,
            }).then(res => {
                const board_name = res.data
                commit('addExistenceBoard', { board_url, board_name, })
                let ret = Object.create(boardName)
                ret.name = res.data
                return ret
            }).catch(err => { console.log(err.response) });
        },
    },
})
