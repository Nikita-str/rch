import { createRouter, createWebHashHistory } from 'vue-router'
import App from '../App.vue'
import Tmp from '../Tmp.vue'
import PageNotFound from '../components/PageNotFound.vue'
import PageAwait from '../components/PageAwait.vue'
import Board from '../components/Board.vue'
import Post from '../components/Post.vue'
import PostingForm from '../components/PostingForm.vue'

const POST_LONG_MSG = 'The characters allowed in a URI are either reserved or unreserved'
+' (or a percent character as part of a percent-encoding).'
+' Reserved characters are those characters that sometimes have special meaning.'
+' For example, forward slash characters are used to separate different parts of a URL'
+' (or more generally, a URI). Unreserved characters have no such meanings.'
+' Using percent-encoding, reserved characters are represented using special character sequences.'
+' The sets of reserved and unreserved characters and the circumstances under which certain'
+' reserved characters have special meaning have changed slightly with<br/><br/>'
+' each revision of specifications that govern URIs and URI schemes. ';
const POST_LONG_REPLIES = [
  3000111462, 3000111549, 3000111757, 3000111820, 3000111991, 
  3000112177, 3000112489, 3000112545, 3000112895, 3000113137, 
  3000113311, 3000113371, 3000113698, 3000113755, 3000114032, 
  3000114223, 3000114300, 3000114588, 3000114858, 3000114964, 
];


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
      path: '/~~page~~/await/', 
      component: PageAwait,
    },



    /*
    # python:
    def gen_replies(init, n):
        from random import randint as rand_n
        for ind in range(0, n):
            init += rand_n(50, 400)
            print(init, end = ', ')
        print()
    */
    { 
      path: '/~~page~~/post/', 
      component: Post,
      props: {
        msg: 'И так проверим как <b>жирный текст</b> и... перенос<br/> и... <s><i>наклоненно зачеркнутый</i></s>',
        msgDate: '07:48:28 27.05.23',
        msgBoardN: '3000111256',
        msgThrN: '2',
        msgWho: 'Анон',
        msgReplies: [3000111297, 3000111422, 3000111527],
      }
    },
    { 
      path: '/~~page~~/post/~micro/', 
      component: Post,
      props: {
        msg: '&gt///&lt',
        msgDate: '07:48:28 27.05.23',
        msgBoardN: '3000111256',
        msgThrN: '2',
        msgWho: 'Анон',
        msgReplies: POST_LONG_REPLIES,
      }
    },
    { 
      path: '/~~page~~/post/~long/', 
      component: Post,
      props: {
        msg: POST_LONG_MSG,
        msgDate: '07:48:28 27.05.23',
        msgBoardN: '3000111256',
        msgThrN: '2',
        msgWho: 'Анон',
        msgReplies: POST_LONG_REPLIES,
      },
    },
    { 
      path: '/~~page~~/post/~micro-no-reply/', 
      component: Post,
      props: {
        msg: '&gt///&lt',
        msgDate: '07:48:28 27.05.23',
        msgBoardN: '3000111256',
        msgThrN: '2',
        msgWho: 'Анон',
      }
    },
    { 
      path: '/~~page~~/post/~long-no-reply/', 
      component: Post,
      props: {
        msg: POST_LONG_MSG,
        msgDate: '07:48:28 27.05.23',
        msgBoardN: '3000111256',
        msgThrN: '2',
        msgWho: 'Анон',
      }
    },



    
    { 
      path: '/~~page~~/posting-form/', 
      component: PostingForm,
      props: {
        boardUrl: 'b',
        isNewThr: true,
      },
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

