use std::collections::{/* LinkedList, */ VecDeque};

use crate::thread::ThreadOpN;

const INITIAL_RATE: f32 = 100.;
const STD_RATE_COEF: f32 = 5.;

const SEC_IN_MIN: usize = 60;
const SEC_IN_H: usize = 60 * SEC_IN_MIN;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
struct Rate<T> {
    rate: f32,
    value: T,
}
impl<T> Rate<T> {
    pub fn new(value: T) -> Self {
        Self {
            rate: INITIAL_RATE,
            value,
        }
    }

    pub fn inc_rate(&mut self, post_rate: f32, thr_qty: usize) {
        self.rate += post_rate * (thr_qty as f32)
    }
    pub fn dec_rate(&mut self, post_rate: f32) {
        self.rate -= post_rate
    }
    pub fn upd_rate(&mut self, post_rate: f32, thr_qty: usize, increase: bool) {
        if increase { self.inc_rate(post_rate, thr_qty) } 
        else { self.dec_rate(post_rate) }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct ThreadUsageRate {
    rates: VecDeque<Rate<ThreadOpN>>,
}
impl ThreadUsageRate {
    pub fn new() -> Self {
        Self {
            rates: VecDeque::new(),
        }
    }

    pub(in crate) fn top_n(&self, n: usize) -> Option<ThreadOpN> {
        let len = self.rates.len();
        if len <= n { None }
        else {
            Some(self.rates[len - n - 1].value)
        }
    }

    pub(in crate) fn post_rate(post_n: usize, dt_sec: f32) -> f32 { post_rate(post_n, dt_sec) }
    
    pub(in crate) fn add_new(&mut self, thr_op_n: ThreadOpN) {
        let post_rate = Self::post_rate(0, 0.);
        self.rates.iter_mut().for_each(|x|x.dec_rate(post_rate));
        
        let rate = Rate::new(thr_op_n);
        let index = self.rates.partition_point(|x|x.rate <= rate.rate);
        self.rates.insert(index, rate);
    }

    pub(in crate) fn remove_by_thr_n(&mut self, thr_op_n: ThreadOpN) {
        if let Some(index) = self.rates.iter().position(|rate|rate.value == thr_op_n) {
            self.rates.remove(index);
        }
    }

    /// # panic
    /// * if `thr_op_n` is unkn
    pub(in crate) fn upd_rates(&mut self, thr_op_n: ThreadOpN, post_rate: f32) {
        // maybe we can do it faster (? by using LinkedList ?) but mheww...

        let mut cur_ind = None;
        let mut cur_rate = None;
        let mut insert_ind = None;
        let thr_qty = self.rates.len();

        for (ind, rate) in self.rates.iter_mut().enumerate() {
            let cur = rate.value == thr_op_n;
            rate.upd_rate(post_rate, thr_qty, cur);
            
            if cur {
                cur_ind = Some(ind);
                cur_rate = Some(rate.rate);
            } else if let Some(x_rate) = cur_rate {
                if insert_ind.is_none() {
                    if rate.rate > x_rate {
                        insert_ind = Some(ind);
                    }
                }
            }
        }

        // slow ops:
        if let Some(index) = cur_ind {
            let cur = self.rates.remove(index).unwrap();
            if let Some(index) = insert_ind {
                self.rates.insert(index, cur)
            } else {
                self.rates.push_back(cur)
            }
        } else {
            panic!("[ALGO ERROR]: `thr_op_n` is unkn")
        }
    }

    
    pub fn thrs_rate(&self) -> Vec<u64> {
        self.rates.iter().map(|x|x.value.into()).collect()
    }
}

fn post_rate(post_n: usize, dt_sec: f32) -> f32 {
    let dt_sec = if dt_sec <= 0. { 0. } 
    else if dt_sec >= (SEC_IN_H as f32) { SEC_IN_H as f32 }
    else { dt_sec };

    let dt = dt_sec / (SEC_IN_H as f32);
    
    // dt in [0; 1]
    // time_coef =
    //     * if dt < 5min => `3 - (x^2 * 60*60/25)` /* from 3 to 2 */ 
    //     * else => `2 - (x-5min)^2 * 1.785` /* from 2 to 0.5 */
    let time_coef = if dt_sec <= 5. * (SEC_IN_MIN as f32) {
        const DT_COEF: f32 = 60. * 60. * (1. / 5.) * (1. / 5.);
        3. - (dt * dt * DT_COEF) 
    } else {
        const DT_COEF: f32 = 1.785;
        let dt = dt - 5. / 60.;
        2. - (dt * dt * DT_COEF)
    };

    const SIGNIFICANCY_N: usize = 10;
    let n = (post_n / SIGNIFICANCY_N) + 1;
    let post_n_coef = 1. / (n as f32);

    post_n_coef * time_coef * STD_RATE_COEF
}