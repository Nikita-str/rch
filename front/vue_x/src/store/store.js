import { createStore } from 'vuex'

export default createStore({
    state: {
        total_post: 420,
    },
    getters: {
        getTotalPost: state => state.total_post,
    },
    actions: {
    },
})
