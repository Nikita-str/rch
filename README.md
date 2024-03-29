###### <sup><sub>hi! I mean sowwy for bad English.</sub></sup>

# About
Anonymous imageboard with [backend](/back/server/src/) in Rust and [frontend](/front/vue_x/src/) in Vue.

# Getting Started
You should have `cargo`(`Rust`), `npm` and `Python 3`<sup>*1</sup> installed.

First of all we need to build frontend: 

1. `cd ./front/vue_x/` : open [frontend folder](/front/vue_x/)
1. `npm install` : install dependencies
1.  `npm run build` : build frontend

Then build backend:

4. `cd ../../` : return to repo root folder
4. `cd ./back/server/` : open [backend folder](/back/server/)
4. `cargo run -r` : build and run backend

If you need you can also auto create some threads and posts:

7. `cd ./test/REST/` : open [backend REST test folder](/back/server/test/REST/)
8. for example `python ./create_a.py` : create a thread on `/a/` board and make some posts there (no pics) 

Backend config can be changed [here](/back/server/Config.toml).

###### <sup>*1</sup>: `Python 3` only used on frontend building and it's need only to save users' posted pics between rebuilding frontend. So if you don't need to save it then just remove in [frontend config file](/front/vue_x/package.json) `"prebuild"` and `"postbuild"` script commands. And after it there will be no `Python 3` dependencies.

### Admin Imageboard Control
You can do it on `/~~page~~/~~ctrl~~/`-pages. Currently for any action you should use single action pwd(sends hash of it) from file `back\server\saves\aux\single_pwds.txt`(path by default). Currently here you can save imageboard state, load its state, add board, del post, del thread, and shutdown.

### CLI
Currently there is only one run arg for save loading on start
* `--load <save_name>`
* `--load <save_name> --load--single-file` if imageboard state saved in single-file format
###### You can run imageboard with a presave where are few already boards added by:<br/> `cargo run -r -- --load STD_SAVE_A --load-single-file`

# How it looks
<img alt="/rp/ screen example" src="./docs/pics/screen_example_01.webp" title="/rp/ screen example" width="100%" />
<div style="text-align: center;"><sup><div style="color: gray; display: inline-block;">thread example</div></sup></div>

<br/>

<img alt="/rp/ pic view" src="./docs/pics/screen_example_02.webp" title="/rp/ pic view" width="100%" />
<div style="text-align: center;"><sup><div style="color: gray; display: inline-block;">pic view example</div></sup></div>

<br/>

And [preview.mp4](./docs/pics/preview_old_ver.mp4): server side delays are emulated here (run in dev mode)

# Some other docs
* [Text preprocessors](/back/server/src/preproc/preproc.md) : doc about `[D10]`, `>>` and so on
