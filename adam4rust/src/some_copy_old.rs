// use cats_utils::Dict;
#[path = "./dict.rs"]
mod dict;
use std::{cmp::max, iter::zip};

use dict::Dict;

// import math;
// import torch;
// from .opti // import Optimizer;

struct Param<T> {
    dtype: T,
}

struct SelfAdam<T> {
    defaults: T,
    param_groups: T,
    state: T,
}

#[inline]
pub fn inside<T: std::cmp::PartialEq>(to_compate: &T, rtgze: Vec<T>) -> bool {
    for e in rtgze {
        if e == *to_compate {
            return true;
        }
    }
    return false;
}

/// Are `T` and `U` are the same type?
const fn isinstance<T, U>(_: T, _: U) -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>()
}
// supposed to return a unique identifier among simultaneously existing objects
fn id<T>(some: T) -> isize {
    todo! {}
}

///     /// Implements Adam algorithm.
///     It has been proposed in `Adam: A Method for Stochastic Optimization`_.
///     The implementation of the L2 penalty follows changes proposed in
///     `Decoupled Weight Decay Regularization`_.
///     Arguments{
///         params (iterable): iterable of parameters to optimize || dicts defining{
///             parameter groups
/// }
///         lr (float, optional): learning rate (default: 1e-3)
///         betas (Tuple[float, float], optional): coefficients used for computing{
///             running averages of gradient && its square (default: (0.9, 0.999))
/// }
///         eps (float, optional): term added to the denominator to improve{
///             numerical stability (default: 1e-8)
/// }
///         weight_decay (float, optional): weight decay (L2 penalty) (default: 0)
///         amsgrad (boolean, optional): whether to use the AMSGrad variant of this{
///             algorithm // from the paper `On the Convergence of Adam && Beyond`_
///             (default: false)
/// }
/// }
///     .. _Adam\: A Method for Stochastic Optimization{
///         https:/arxiv.org/abs/1412.6980
/// }
///     .. _Decoupled Weight Decay Regularization{
///         https:/arxiv.org/abs/1711.05101
/// }
///     .. _On the Convergence of Adam && Beyond{
///         https:/openreview.net/forum?id=ryQu7f-RZ
/// }
impl<T> SelfAdam<T> {
    // fn new(params, lr=1e-3, betas=(0.9, 0.999), eps=1e-8, weight_decay=0, amsgrad=false)
    fn new(
        params: Dict<&str, T>,
        lr: f64,
        betas: (f64, f64),
        eps: f64,
        weight_decay: f64,
        amsgrad: bool,
    ) -> Self {
        if !0.0 <= lr {
            panic!("ValueError(\"Invalid learning rate: {}\")", lr);
        }
        if !0.0 <= eps {
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
        if !0.0 <= weight_decay {
            panic!(
                "ValueError(\"Invalid weight_decay value: {}\")",
                weight_decay
            );
        }
        // defaults = dict(
        //     lr = lr,
        //     betas = betas,
        //     eps = eps,
        //     weight_decay = weight_decay,
        //     amsgrad = amsgrad,
        // );
        let mut defaults = Dict::new();
        defaults["lr"] = lr;
        // defaults["betas"] = betas;
        defaults["betas1"] = betas.0;
        defaults["betas2"] = betas.1;
        defaults["eps"] = eps;
        defaults["weight_decay"] = weight_decay;
        defaults["amsgrad"] = amsgrad as f64;
        // super(Adam, self).new(params, defaults);
        // torch._C._log_api_usage_once("python.optimizer");
        // self.defaults = defaults;
        // if isinstance(params, torch.Tensor) {
        //     panic!(
        //         r#"TypeError("params argument given to the optimizer should be an iterable of Tensors || dicts, but got " +torch.typename(params))"#
        //     );
        // }
        let state = Dict::new();
        // let param_groups = [];
        let param_groups = vec![params];
        // should never happened ?
        if param_groups.len() == 0 {
            panic!(r#"ValueError("optimizer got an empty parameter list")"#);
        }
        // if !isinstance(param_groups[0], dict) {
        //     // param_groups = [stringify! {"params": param_groups}];
        //     let mut temp = Dict::new();
        //     temp["params"] = param_groups;
        //     param_groups = vec![temp];
        // }
        for param_group in param_groups {
            self.add_param_group(param_group);
        }
    }
    fn __setstate__<U, V>(self, state: Dict<U, V>) {
        // super(Adam, self).__setstate__(state);
        self.__dict__.update(state);
        for group in self.param_groups {
            group.setdefault("amsgrad", false);
        }
    }
    fn __getstate__<'a>(self) -> Dict<&'a str, T> {
        // return stringify! { "defaults": self.defaults, "state": self.state, "param_groups": self.param_groups, };
        let mut temp_dict = Dict::new();
        temp_dict["defaults"] = self.defaults;
        temp_dict["state"] = self.state;
        temp_dict["param_groups"] = self.param_groups;
        return temp_dict;
    }
    fn __repr__(self) {
        let format_string = self.__class__.__name__ + " (";
        for (i, group) in self.param_groups {
            format_string += "\n";
            format_string += "Parameter Group stringify!{0}\n".format(i);
            for key in group.keys() {
                if key != "params" {
                    format_string += " stringify!{0}: stringify!{1}\n".format(key, group[key]);
                }
            }
        }
        format_string += ")";
        return format_string;
    }

    ///         /// Returns the state of the optimizer as a :class:`dict`.
    ///         It contains two entries
    ///         * state - a dict holding current optimization state. Its content{
    ///             differs between optimizer classes.
    /// }
    ///         * param_groups - a dict containing all parameter groups
    fn state_dict<'a>(self) -> Dict<&'a str, T> {
        // Save order indices instead of Tensors;
        let mut param_mappings = Dict::new();
        let mut start_index = 0;
        // fn pack_group<'b, U>(group: Dict<&'b str, U>) {
        //     // nonlocal start_index;
        //     // packed = stringify!{k: v for ( k, v) in group.items() if k != "params"};
        //     let temp_packed_dict = Dict::new();
        //     for (k, v) in group.items() {
        //         if k != "params" {
        //             temp_packed_dict[k] = v;
        //         }
        //     }
        //     let packed = temp_packed_dict;
        //     // param_mappings.update(stringify!{id(p): i for ( i, p) in enumerate(group["params"], start_index);
        //     // if id(p) !in param_mappings});
        //     let temp_param_mappings_dict = Dict::new();
        //     for (i, p) in (group["params"], &mut start_index) {
        //         if !inside(id(p), param_mappings) {
        //             temp_param_mappings_dict[id(p)] = i;
        //         }
        //     }
        //     param_mappings.update(temp_param_mappings_dict);
        //     // packed["params"] = [param_mappings[id(p)] for p in group["params"]];
        //     temp_packed_list = [];
        //     for p in group["params"] {
        //         temp_packed_list.append(param_mappings[id(p)]);
        //     }
        //     packed["params"] = temp_packed_list;
        //     start_index += len(packed["params"]);
        //     return packed;
        // };
        // param_groups = [pack_group(g) for g in self.param_groups];
        let mut temp_param_groups_list = vec![];
        for g in self.param_groups {
            // temp_param_groups_list.append(pack_group(g));
            temp_param_groups_list.append({
                let group = g;
                // nonlocal start_index;
                // packed = stringify!{k: v for ( k, v) in group.items() if k != "params"};
                let temp_packed_dict = Dict::new();
                for (k, v) in group.items() {
                    if k != "params" {
                        temp_packed_dict[k] = v;
                    }
                }
                let mut packed = temp_packed_dict;
                // param_mappings.update(stringify!{id(p): i for ( i, p) in enumerate(group["params"], start_index);
                // if id(p) !in param_mappings});
                let temp_param_mappings_dict = Dict::new();
                for (i, p) in (group["params"], &mut start_index) {
                    if !inside(id(p), param_mappings) {
                        temp_param_mappings_dict[id(p)] = i;
                    }
                }
                param_mappings.update(temp_param_mappings_dict);
                // packed["params"] = [param_mappings[id(p)] for p in group["params"]];
                let mut temp_packed_list: Vec<T> = vec![];
                for p in group["params"] {
                    temp_packed_list.append(param_mappings[id(p)]);
                }
                packed["params"] = temp_packed_list;
                start_index += packed["params"].len();
                return packed;
            });
        }
        let param_groups = temp_param_groups_list;
        // Remap state to use order indices as keys;
        // packed_state = stringify!{(param_mappings[id(k)] if isinstance(k, torch.Tensor) else k): v;
        // for ( k, v) in self.state.items()};
        let mut temp_packed_state_dict = Dict::new();
        for (k, v) in self.state.items() {
            let temp_key = None;
            if isinstance(k, torch.Tensor) {
                temp_key = param_mappings[id(k)];
            } else {
                temp_key = k;
            }
            temp_packed_state_dict[temp_key] = v;
        }
        let packed_state = temp_packed_state_dict;
        // return stringify! { "state": packed_state, "param_groups": param_groups, };
        let mut temp_dict = Dict::new();
        temp_dict["state"] = packed_state;
        temp_dict["param_groups"] = param_groups;
        return temp_dict;
    }
    ///         /// Loads the optimizer state.
    ///         Arguments{
    ///             state_dict (dict): optimizer state. Should be an object returned{
    ///                 // from a call to :meth:`state_dict`.
    /// }
    /// }
    fn load_state_dict(self, state_dict: Dict<&str, T>) {
        // deepcopy, to be consistent with module API;
        // state_dict = deepcopy(state_dict);
        // Validate the state_dict;
        let groups = self.param_groups;
        let saved_groups = state_dict["param_groups"];
        if groups.len() != saved_groups.len() {
            panic!(r#"ValueError("loaded state dict has a different number of parameter groups")"#);
        }
        // param_lens = (len(g["params"]) for g in groups);
        let mut temp_param_lens_list = [];
        for g in groups {
            temp_param_lens_list.append(g["params"].len());
        }
        let param_lens = temp_param_lens_list;
        // saved_lens = (len(g["params"]) for g in saved_groups);
        let mut temp_saved_lens_list = [];
        for g in saved_groups {
            temp_saved_lens_list.append(g["params"].len());
        }
        param_lens = temp_saved_lens_list;
        //         if inside( any(p_len != s_len for p_len, s_len,zip(param_lens, saved_lens))){
        //             panic!(r#"ValueError("loaded state dict contains a parameter group that doesn"t match the size of optimizer"s group")"#);
        // }
        // // Update the state;
        // id_map = stringify!{old_id: p for ( old_id, p) in // zip(chain.from_iterable((g["params"] for g in saved_groups)), // chain.from_iterable((g["params"] for g in groups)))};
        let mut temp_dict_map = Dict::new();
        //         for ( old_id, p) in zip(chain.from_iterable((g["params"] for g in saved_groups)), chain.from_iterable((g["params"] for g in groups))){
        //             temp_dict_map[old_id] = p;
        // }
        let id_map = temp_dict_map;
        /// Make a deep copy of value, casting all tensors to device of param.
        fn cast<T, U>(param: Param<T>, value: U) {
            if isinstance(value, torch.Tensor) {
                // Floating-point types are a bit special here. They are the only ones;
                // that are assumed to always match the type of params.;
                if param.is_floating_point() {
                    value = value.to(param.dtype);
                }
                value = value.to(param.device);
                return value;
            } else if isinstance(value, Dict::<&str, _>::new()) {
                // return stringify!{k: cast(param, v) for ( k, v) in value.items()};
                let mut temp_dict = Dict::new();
                for (k, v) in value.items() {
                    temp_dict[k] = cast(param, v);
                }
                return temp_dict;
            } else if isinstance(value, container_abcs.Iterable) {
                // return type(value)(cast(param, v) for ( v) in value);
                // may !work the same as the one before so for the moment it"s commented
                let mut temp_tuple = vec![];
                for v in value {
                    temp_tuple.append(cast(param, v));
                }
                return temp_tuple as U;
            } else {
                return value;
            }
        }
        // Copy state assigned to params (and cast tensors to appropriate types).;
        // State that != assigned to params == copied as == (needed for;
        // backward compatibility).;
        let state = Dict::new();
        for (k, v) in state_dict["state"].items() {
            if inside(k, id_map) {
                let param = id_map[k];
                state[param] = cast(param, v);
            } else {
                state[k] = v;
            }
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
            temp_list.append(update_group(g, ng));
        }
        let param_groups = temp_list;
        // self.__setstate__(stringify! {"state": state, "param_groups": param_groups});
        let mut temp_dict = Dict::new();
        temp_dict["state"] = state;
        temp_dict["param_groups"] = param_groups;
        self.__setstate__(temp_dict);
    }
    /// Clears the gradients of all optimized :class:`torch.Tensor` s.
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
    // @torch.no_grad();
    ///         /// Performs a single optimization step.
    ///         Arguments{
    ///             closure (callable, optional): A closure that reevaluates the model{
    ///                 && returns the loss.
    /// }
    /// }
    fn step<A, B>(self, closure: fn(A) -> B) {
        let mut loss = None;
        //         if closure != None{
        //             with torch.enable_grad(){
        //                 loss = closure();
        // }
        // }
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
                let mut state = self.state[p];
                // state = self.state[p];
                // State initialization;
                if state.len() == 0 {
                    state["step"] = 0;
                    // old(step) = 0;
                    // Exponential moving average of gradient values;
                    state["exp_avg"] = torch.zeros_like(p, memory_format = torch.preserve_format);
                    // old(exp_avg) = [0.;?];
                    // Exponential moving average of squared gradient values;
                    state["exp_avg_sq"] =
                        torch.zeros_like(p, memory_format = torch.preserve_format);
                    // old(exp_avg_sq) = [0.;?];
                    if amsgrad {
                        // Maintains max of all exp. moving avg. of sq. grad. values;
                        state["max_exp_avg_sq"] =
                            torch.zeros_like(p, memory_format = torch.preserve_format);
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
                let bias_correction1 = 1 - beta1 * *state["step"];
                // bias_correction1 = 1 - beta1 ^ step;
                let bias_correction2 = 1 - beta2 * *state["step"];
                // bias_correction2 = 1 - beta2 ^ step;
                if group["weight_decay"] != 0 {
                    // grad = grad.add(p, alpha = group["weight_decay"]);
                    // grad = grad + weight_decay * p;
                    grad = grad + group["weight_decay"] * p;
                }
                // Decay the first && second moment running average coefficient;
                // exp_avg.mul_(beta1).add_(grad, alpha = 1 - beta1);
                exp_avg *= beta1;
                exp_avg += (1 - beta1) * grad;
                // exp_avg_sq
                //     .mul_(beta2)
                //     .addcmul_(grad, grad, value = 1 - beta2);
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
    ///         /// Add a param group to the :class:`Optimizer` s `param_groups`.
    ///         This can be useful when fine tuning a pre-trained network as frozen layers can be made
    ///         trainable && added to the :class:`Optimizer` as training progresses.
    ///         Arguments{
    ///             param_group (dict): Specifies what Tensors should be optimized along with group
    ///             specific optimization options.
    /// }
    fn add_param_group<A>(self, param_group: Dict<&str, A>) {
        // assert!(isinstance(param_group, dict), "param group must be a dict");
        let params = param_group["params"];
        // if isinstance(params, torch.Tensor) {
        //     param_group["params"] = [params];
        // } else if isinstance(params, set) {
        //     panic!(
        //         r#"TypeError("optimizer parameters need to be organized in ordered collections, but the ordering of tensors in sets will change between runs. Please use a list instead.")"#
        //     );
        // } else {
        //     param_group["params"] = list(params);
        // }
        param_group["params"] = vec![params];
        // for param in param_group["params"] {
        //     if !isinstance(param, torch.Tensor) {
        //         panic!(
        //             r#"TypeError("optimizer can only optimize Tensors, but one of the params == " + torch.typename(param))"#
        //         );
        //     }
        //     if !param.is_leaf {
        //         panic!(r#"ValueError("can"t optimize a non-leaf Tensor")"#);
        //     }
        // }
        for (name, default) in self.defaults.items() {
            // if !inside(default == required && name, param_group) {
            //     panic!(
            //         r#"ValueError("parameter group didn"t specify a value of required optimization parameter " + name)"#
            //     );
            // } else {
            //     param_group.setdefault(name, default);
            // }
            param_group.setdefault(name, default);
        }
        let param_set = set();
        for group in self.param_groups {
            param_set.update(set(group["params"]));
        }
        if !param_set.isdisjoint(set(param_group["params"])) {
            panic!(r#"ValueError("some parameters appear in more than one parameter group")"#);
        }
        self.param_groups.append(param_group);
    }
}
