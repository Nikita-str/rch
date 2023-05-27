import { rand_n_arr } from './fns'; 

export function storage_circle_value(arr_key, index_key, circle_qty) {
    const ARR_KEY = arr_key;
    const INDEX_KEY = index_key;
    const CIRLCE_QTY = circle_qty;

    let arr;
    let storage_value = sessionStorage.getItem(ARR_KEY);
    if (storage_value) {
        arr = JSON.parse(storage_value);
    } else {
        arr = rand_n_arr(CIRLCE_QTY, 1);
        sessionStorage.setItem(ARR_KEY, JSON.stringify((rand_n_arr(CIRLCE_QTY, 1))));
    };

    let ret = 0;
    let arr_index = sessionStorage.getItem(INDEX_KEY);
    if (arr_index) {
        ret = JSON.parse(arr_index);
        ret += 1;
        if (ret == arr.length) {
            ret = 0;
        }
    }
    
    sessionStorage.setItem(INDEX_KEY, JSON.stringify(ret));
    // console.log(arr, ret)
    return arr[ret]
  }