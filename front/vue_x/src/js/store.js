import { createStore } from 'vuex'
import axios from 'axios'
import { trim } from './fns'

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
        getReq_Board_Name({ getters, commit }, board_url) {
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
        
        getReq_Board_ThrsLoad({ getters }, {board_url, from, to}) {
            let params = new URLSearchParams([['board_url', board_url], ['from', from], ['to', to]])
            return axios({
                url: getters.getPort + '/board/thrs_load',
                method: 'get',
                params,
            }).then(res => {
                return res.data
            }).catch(err => { console.log(err.response) });
        },        
        postReq_Board_CtlgLoad({ getters }, {board_url, known_n}) {
            let params = new URLSearchParams([['board_url', board_url]])
            let data = { known_n, }
            console.log('postReq_Board_CtlgLoad', data)
            return axios({
                url: getters.getPort + '/board/ctlg_load',
                method: 'post',
                params,
                data,
            }).then(res => {
                return res.data
            }).catch(err => { console.log(err.response) });
        },
          
          
        postReq_Board_ThrNew({ getters }, data) {
            if (data.post_header !== null) {
                data.post_header = trim(data.post_header, ' ')
                if (data.post_header.length == 0) {
                    data.post_header = null
                }
            }
            return axios({
                url: getters.getPort + '/board/thr_new',
                method: 'post',
                data,
            }).then(res => {
                return res.data
            }).catch(err => { console.log('POST board/thr_new', err.response) });
        },
        postReq_Thread_PostNew({ getters }, data) {
            if (data.post_header !== null) {
                console.log('[WARN][POST:thread/post_new]: post_header isnt empty', data.post_header)
            }
            data.post_header = null

            return axios({
                url: getters.getPort + '/thread/post_new',
                method: 'post',
                data,
            }).then(res => {
                return res.data
            }).catch(err => { console.log('POST:thread/post_new', err.response) });
        },
    },
})
