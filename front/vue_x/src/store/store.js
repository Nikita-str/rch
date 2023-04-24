import { createStore } from 'vuex'
import axios from 'axios'

export default createStore({
    state: {
        port: "/api",

        total_post: 420,
    },


    getters: {
        getPort: state => state.port,
        getTotalPost: state => state.total_post,
    },


    mutations: {
        updCommonAll(state, obj) {
            state.total_post = obj.total_post
            console.log("HERE#updCommonAll:END|" + state.total_post)
        }
    },


    actions: {
        updCommonInfo({ getters, commit }) {
            return axios({
                url: getters.getPort + '/common/all',
                method: 'get',
            }).then(res => {
                commit('updCommonAll', {total_post: res.data})
            }).catch(err => { console.log(err.response) });
        }
    },
})
