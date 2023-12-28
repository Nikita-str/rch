import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'
import { nextTick } from 'vue'
import { boardUrlCalc } from './board_fns'
import App from '../App.vue'
import PageNotFound from '../components/PageNotFound.vue'
import PageAwait from '../components/PageAwait.vue'
import Board from '../components/Board.vue'
import Thread from '../components/Thread.vue'
import Post from '../components/Post.vue'
import PostingForm from '../components/PostingForm.vue'
import DraggablePostingForm from '../components/DraggablePostingForm.vue'
import PicCloseView from '../components/files/pics/PicCloseView.vue'

import SaveLoad from '../components/ctrl/SaveLoad.vue'
import Shutdown from '../components/ctrl/Shutdown.vue'
import AddBoard from '../components/ctrl/AddBoard.vue'
import DelPost from '../components/ctrl/DelPost.vue'

import { BOARD_NAME as SETT_BOARD_NAME, BOARD_POSTFIX as SETT_BOARD_POSTFIX, BOARD_FULLNAME as SETT_BOARD_FULLNAME } from './settings'

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
const maxThrNLen = 16;

const routes = [
    { 
        path: '/',
        name: SETT_BOARD_FULLNAME,
        meta: {
          title: SETT_BOARD_FULLNAME,
          titleType: "main",
        },
        component: App 
    },

    {
      path: '/:pathMatch([a-z]{1,' + maxBoardNameLen + '})/',
      component: Board,
      meta: {
        title: "доска",
        titleType: "board:prefix",
      },
    },
    {
      path: '/:pathMatch([a-z]{1,' + maxBoardNameLen + '})/catalog/',
      component: Board,
      meta: {
        title: "каталог", 
        titleType: "board:prefix",
      },
    },
    {
      path: '/:pathMatch([a-z]{1,' + maxBoardNameLen + '})/:pathMatch([0-9]{1,' + maxThrNLen + '})',
      component: Thread,
      meta: {
        title: "тредик", 
        titleType: "board:prefix",
      },
    },

    { 
      path: '/~~page~~/await/', 
      component: PageAwait,
      meta: {
        title: "аа??", 
        titleType: "?",
      },
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
      path: '/~~page~~/posting-form/open/', 
      component: PostingForm,
      props: {
        boardUrl: 'b',
        opPostN: null,
      },
    },
    { 
      path: '/~~page~~/posting-form/common/', 
      component: PostingForm,
      props: {
        boardUrl: 'b',
        opPostN: 244,
      },
    },
    { 
      path: '/~~page~~/posting-form/draggable/', 
      component: DraggablePostingForm,
      props: {
        boardUrl: 'b',
        opPostN: null,
      },
    },
  


    {
      path: '/~~page~~/img-view/real/', 
      component: PicCloseView,
      props: {
        img_path: '9.jpg',
        expected_w: 1024,
        expected_h: 1024,
      },
    },
    {
      path: '/~~page~~/img-view/named/', 
      component: PicCloseView,
      props: {
        name: 'jtheyrsdgxh65erdyhj654weesd5tyhtvweyjnvrsertvhg.jfif',
        img_path: '9.jpg',
        expected_w: 1024,
        expected_h: 1024,
      },
    },
    {
      path: '/~~page~~/img-view/unkn/', 
      component: PicCloseView,
      props: {
        img_path: '9.jpg',
      },
    },
    {
      path: '/~~page~~/img-view/incorrect/',
      component: PicCloseView,
      props: {
        img_path: '9.jpg',
        expected_w: 512,
        expected_h: 512,
      },
    },
    {
      path: '/~~page~~/img-view/big/',
      component: PicCloseView,
      props: {
        img_path: 'test_01_big.png',
        expected_w: 3000,
        expected_h: 6000,
      },
    },
    {
      path: '/~~page~~/img-view/small/',
      component: PicCloseView,
      props: {
        img_path: '9_c.webp',
      },
    },
    

    {
      path: '/~~page~~/~~ctrl~~/full-save/', 
      component: SaveLoad,
      props: {
        isSave: true,
      },
    },
    {
      path: '/~~page~~/~~ctrl~~/full-load/', 
      component: SaveLoad,
      props: {
        isSave: false,
      },
    },
    {
      path: '/~~page~~/~~ctrl~~/add-board/', 
      component: AddBoard,
    },
    {
      path: '/~~page~~/~~ctrl~~/del-post/', 
      component: DelPost,
    },
    {
      path: '/~~page~~/~~ctrl~~/shutdown/', 
      component: Shutdown,
    },


    { 
      path: '/:pathMatch(.*)*', 
      component: PageNotFound,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
    sensitive: true,
})

router.afterEach((to) => {
  const BOARD_NAME = `${SETT_BOARD_NAME}:${SETT_BOARD_POSTFIX}`
  const POSTFIX = ` (${BOARD_NAME})`
  nextTick(() => {
      let bUrl = boardUrlCalc(to.path)

      if (to.meta.titleType == "main") {
          document.title = to.meta.title
        } else if (to.meta.titleType == "board:prefix") {
          document.title = `/${bUrl}/ - ` + to.meta.title + POSTFIX
      } else if (to.meta.titleType == "?") {
          document.title = to.meta.title + POSTFIX
      } else {
          const UNKN_TITLE = 'Нечто метахтоничное но на UwU борде';
          document.title = UNKN_TITLE + POSTFIX;
      }
  });
});

export default router