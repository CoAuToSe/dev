// use cats_utils::Dict;
#[path = "./dict.rs"]
mod dict;
use std::{cmp::max, iter::zip};

use dict::Dict;

// import math;
// import torch;
// from .opti // import Optimizer;

struct Param<f64> {
    dtype: f64,
}

struct SelfAdam<'a, f64> {
    defaults: Dict<&'a str, f64>,
    param_groups: Dict<&'a str, Dict<&'a str, f64>>,
    state: Dict<&'a str, Dict<&'a str, f64>>,
}

#[inline]
pub fn inside<f64: std::cmp::PartialEq>(to_compate: &f64, rtgze: Vec<f64>) -> bool {
    for e in rtgze {
        if e == *to_compate {
            return true;
        }
    }
    return false;
}

/// Are `T` and `U` are the same type?
#[inline]
const fn isinstance<T, U>(_: T, _: U) -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>()
}
// supposed to return a unique identifier among simultaneously existing objects
fn id<T>(some: T) -> isize {
    todo! {}
}

impl<'life> SelfAdam<'life, f64>
// where
//     f64: Default,
{
    // fn new(params, lr=1e-3, betas=(0.9, 0.999), eps=1e-8, weight_decay=0, amsgrad=false)
    fn new(
        params: Dict<&str, f64>,
        lr: f64,
        betas: (f64, f64),
        eps: f64,
        weight_decay: f64,
        amsgrad: bool,
    ) -> Self {
        if !(0.0 <= lr) {
            panic!("ValueError(\"Invalid learning rate: {}\")", lr);
        }
        if !(0.0 <= eps) {
            panic!("ValueError(\"Invalid epsilon value: {}\")", eps);
        }
        if !(0.0 <= betas.0 && betas.0 < 1.0) {
            panic!(
                "ValueError(\"Invalid beta parameter at index 0: {}\")",
                betas.0
            );
        }
        if !(0.0 <= betas.1 && betas.1 < 1.0) {
            panic!(
                "ValueError(\"Invalid beta parameter at index 1: {}\")",
                betas.1
            );
        }
        if !(0.0 <= weight_decay) {
            panic!(
                "ValueError(\"Invalid weight_decay value: {}\")",
                weight_decay
            );
        }
        let mut defaults = Dict::new();
        defaults["lr"] = lr as f64;
        defaults["betas1"] = betas.0 as f64;
        defaults["betas2"] = betas.1 as f64;
        defaults["eps"] = eps as f64;
        defaults["weight_decay"] = weight_decay as f64;
        defaults["amsgrad"] = amsgrad as usize as f64;
        let state = Dict::new();
        // let param_groups = [];
        let param_groups = Dict::new();
        param_groups["params"] = params;
        // let param_groups = vec![params];
        if param_groups.len() == 0 {
            // should never happened ?
            panic!(r#"ValueError("optimizer got an empty parameter list")"#);
        }
        for param_group in param_groups {
            let params = param_group["params"];
            for (name, default) in defaults.items() {
                param_group.setdefault(name, default);
            }
            param_groups.push(param_group);
        }
        SelfAdam {
            defaults,
            param_groups,
            state,
        }
    }

    fn __setstate__<'a>(self, state: Dict<&'a str, Dict<&'a str, Dict<&'a str, f64>>>) {
        // super(Adam, self).__setstate__(state);
        self.__dict__.update(state);
        for group in self.param_groups {
            group.setdefault("amsgrad", false);
        }
    }

    // fn __getstate__<'a>(self) -> Dict<&'a str, Dict<&'a str, Dict<&'a str, f64>>> {
    //     // return stringify! { "defaults": self.defaults, "state": self.state, "param_groups": self.param_groups, };
    //     let mut temp_dict = Dict::new();
    //     temp_dict["defaults"] = self.defaults;
    //     temp_dict["state"] = self.state;
    //     temp_dict["param_groups"] = self.param_groups;
    //     return temp_dict;
    // }

    fn state_dict<'a>(self) -> Dict<&'a str, Dict<&'a str, Dict<&'a str, f64>>> {
        let mut param_mappings = Dict::new();
        let mut start_index = 0;
        let mut temp_param_groups_list: Vec<Dict<&str, Vec<f64>>> = vec![];
        for g in self.param_groups {
            temp_param_groups_list.push({
                let group = g;
                let temp_packed_dict = Dict::new();
                for (k, v) in group.items() {
                    if *k != "params" {
                        temp_packed_dict[*k] = *v;
                    }
                }
                let mut packed = temp_packed_dict;
                let temp_param_mappings_dict = Dict::new();
                for (i, p) in (group["params"], &mut start_index) {
                    if !inside(&id(p), param_mappings.keys()) {
                        temp_param_mappings_dict[id(p)] = i;
                    }
                }
                param_mappings.update(temp_param_mappings_dict);
                let mut temp_packed_list: Vec<f64> = vec![];
                for p in group["params"] {
                    temp_packed_list.push(param_mappings[id(p)]);
                }
                packed["params"] = temp_packed_list;
                start_index += packed["params"].len();
                packed
            });
        }
        let param_groups = temp_param_groups_list;
        let mut temp_packed_state_dict = Dict::new();
        for (k, v) in self.state.items() {
            temp_packed_state_dict[*k] = *v;
        }
        let packed_state = temp_packed_state_dict;
        let mut temp_dict = Dict::new();
        temp_dict["state"] = packed_state;
        temp_dict["param_groups"] = param_groups;
        return temp_dict;
    }

    fn load_state_dict<'a>(self, state_dict: Dict<&'a str, Dict<&'a str, Dict<&'a str, f64>>>) {
        let groups = self.param_groups;
        let saved_groups = state_dict["param_groups"];
        if groups.len() != saved_groups.len() {
            panic!(r#"ValueError("loaded state dict has a different number of parameter groups")"#);
        }
        // let mut temp_param_lens_list = vec![];
        // for g in groups {
        //     temp_param_lens_list.push(g["params"].len());
        // }
        // let mut param_lens = temp_param_lens_list;
        // let mut temp_saved_lens_list = vec![];
        // for g in saved_groups {
        //     temp_saved_lens_list.push(g["params"].len());
        // }
        // param_lens = temp_saved_lens_list;
        let state: Dict<&str, f64> = Dict::new();
        for (k, v) in state_dict["state"]["state"].items() {
            state[*k] = *v;
        }
        // Update parameter groups, setting their "params" value;
        fn update_group<'a, A, B>(
            group: Dict<&str, A>,
            new_group: Dict<&str, B>,
        ) -> Dict<&'a str, B> {
            new_group["params"] = group["params"];
            return new_group;
        };
        // param_groups = [update_group(g, ng) for ( g, ng) in zip(groups, saved_groups)];
        let mut temp_list = vec![];
        for (g, ng) in zip(groups, saved_groups) {
            temp_list.push(update_group(g, ng));
        }
        let param_groups = temp_list;
        // self.__setstate__(stringify! {"state": state, "param_groups": param_groups});
        let mut temp_dict = Dict::new();
        temp_dict["state"] = state;
        temp_dict["param_groups"] = param_groups;
        self.__setstate__(temp_dict);
    }

    /// Clears the gradients of all optimized :class:`torch.f64ensor` s.
    fn zero_grad(self) {
        for group in self.param_groups {
            for p in group["params"] {
                if p.grad != None {
                    p.grad.detach_();
                    p.grad.zero_();
                }
            }
        }
    }
    fn step<A, B>(self, closure: Option<fn(A) -> B>) -> Option<fn(A) -> B> {
        let mut loss = None;
        for group in self.param_groups {
            for p in group["params"] {
                if p.grad == None {
                    continue; // if the weight doesn"t have a gradiant, skip it;
                }
                let grad = p.grad;
                // grad = p.grad;
                if grad.is_sparse {
                    panic!(
                        r#"RuntimeError("Adam does !support sparse gradients, please consider SparseAdam instead")"#
                    );
                }
                let amsgrad = group["amsgrad"];
                // amsgrad = old(amsgrad);
                let mut state: Dict<&str, f64> = self.state[p.to_string().as_str()];
                // state = self.state[p];
                // State initialization;
                if state.len() == 0 {
                    state["step"] = 0 as f64;
                    // old(step) = 0;
                    // Exponential moving average of gradient values;
                    // state["exp_avg"] = torch.zeros_like(p, memory_format = torch.preserve_format);
                    state["exp_avg"] = 0. as f64;
                    // old(exp_avg) = [0.;?];
                    // Exponential moving average of squared gradient values;
                    // state["exp_avg_sq"] = torch.zeros_like(p, memory_format = torch.preserve_format);
                    state["exp_avg_sq"] = 0. as f64;
                    // old(exp_avg_sq) = [0.;?];
                    if amsgrad as bool {
                        // Maintains max of all exp. moving avg. of sq. grad. values;
                        // state["max_exp_avg_sq"] = torch.zeros_like(p, memory_format = torch.preserve_format);
                        state["max_exp_avg_sq"] = 0. as f64;
                        // old(max_exp_avg_sq) = [0.;?];
                    }
                }
                let (exp_avg, exp_avg_sq) = (state["exp_avg"], state["exp_avg_sq"]);
                // (exp_avg, exp_avg_sq) = old(exp_avg, exp_avg_sq);
                let max_exp_avg_sq;
                if amsgrad as bool {
                    max_exp_avg_sq = state["max_exp_avg_sq"];
                    // max_exp_avg_sq = old(max_exp_avg_sq);
                }
                let (beta1, beta2) = group["betas"];
                // (beta1, beta2) = (0.9, 0.999);
                state["step"] += 1;
                let bias_correction1 = 1 - beta1.pow(state["step"]);
                // bias_correction1 = 1 - beta1 ^ step;
                let bias_correction2 = 1 - beta2.pow(state["step"]);
                // bias_correction2 = 1 - beta2 ^ step;
                if group["weight_decay"] != 0. {
                    // grad = grad.add(p, alpha = group["weight_decay"]);
                    // grad = grad + weight_decay * p;
                    grad = grad + group["weight_decay"] * p;
                }
                // Decay the first && second moment running average coefficient;
                // exp_avg.mul_(beta1).add_(grad, alpha = 1 - beta1);
                exp_avg *= beta1;
                exp_avg += (1 - beta1) * grad;
                // exp_avg_sq.mul_(beta2).addcmul_(grad, grad, value = 1 - beta2);
                exp_avg_sq *= beta2;
                exp_avg_sq += (1 - beta2) * grad * grad;
                let denom;
                if amsgrad as bool {
                    // Maintains the maximum of all 2nd moment running avg. till now;
                    // torch.max(max_exp_avg_sq, exp_avg_sq, out = max_exp_avg_sq);
                    max_exp_avg_sq = max(max_exp_avg_sq, exp_avg_sq);
                    // Use the max. for normalizing running avg. of gradient;
                    denom = (max_exp_avg_sq.sqrt() / bias_correction2.sqrt()).add_(group["eps"]);
                    // denom = (max_exp_avg_sq/bias_correction2) ^ (1/2) + eps;
                } else {
                    denom = (exp_avg_sq.sqrt() / bias_correction2.sqrt()).add_(group["eps"]);
                    // denom = (exp_avg_sq/bias_correction2) ^ (1/2) + eps;
                }
                let step_size = group["lr"] / bias_correction1;
                // p.addcdiv_(exp_avg, denom, value = -step_size);
                p += (-step_size) * (exp_avg / denom);
            }
        }
        return loss;
    }
}
