import { createStore } from 'vuex'
import axios from 'axios'

export default createStore({
    state: {
        port: "/api",

        total_post: 420,
    },
    getters: {
        getTotalPost: state => state.total_post,
    },
    actions: {
        updCommonInfo({state}) {
            return new Promise(() => {
                axios({
                    url: state.port + '/common/all',
                    method: 'get', 
                  }).then(res => {
                    state.total_post = res.data
                  }).catch(err => {
                    console.log(err.response);
                  });
            })
        }
    },
})
