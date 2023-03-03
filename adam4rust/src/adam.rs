use std::{any::Any, collections::HashMap, hash::Hash};

// import math;
// use Math;
// import torch;
// use Torch;
// from .opti // import Optimizer;
pub mod util;
// use crate::util::Group;

mod opti;
use opti::Optimizer;
// pub mod dict;
// use dict::Dict;*
use cats_utils::Dict;

pub struct Adam<T, U, V, W, X>
where
    U: Eq + PartialEq + Hash,
    V: Eq + PartialEq + Hash,
{
    optimizer: Optimizer<T, U, V, W, X>,
}

struct State<T> {
    step: T,
    exp_avg: T,
    exp_avg_sq: T,
    max_exp_avg_sq: T,
}

///     /// Implements Adam algorithm.
///     It has been proposed in `Adam: A Method for Stochastic Optimization`_.
///     The implementation of the L2 penalty follows changes proposed in
///     `Decoupled Weight Decay Regularization`_.
///     Arguments{
///         params (iterable): iterable of parameters to optimize || dicts defining{
///             parameter groups}
///         lr (float, optional): learning rate (default: 1e-3)
///         betas (Tuple[float, float], optional): coefficients used for computing{
///             running averages of gradient && its square (default: (0.9, 0.999))}
///         eps (float, optional): term added to the denominator to improve{
///             numerical stability (default: 1e-8)}
///         weight_decay (float, optional): weight decay (L2 penalty) (default: 0)
///         amsgrad (boolean, optional): whether to use the AMSGrad variant of this{
///             algorithm // from the paper `On the Convergence of Adam && Beyond`_
///             (default: false)}}
///     .. _Adam\: A Method for Stochastic Optimization{
///         https://arxiv.org/abs/1412.6980}
///     .. _Decoupled Weight Decay Regularization{
///         https://arxiv.org/abs/1711.05101}
///     .. _On the Convergence of Adam && Beyond{
///         https://openreview.net/forum?id=ryQu7f-RZ}
impl<T, U, W, X> Adam<T, U, &'static str, W, X>
where
    U: Eq + PartialEq + Hash,
    // V: Eq + PartialEq + Hash,
    // where
    //     T: std::iter::Iterator,
{
    pub fn new(// self,
        // /*  params,*/ lr: f64,
        // betas: (f64, f64),
        // eps: f64,
        // weight_decay: f64,
        // amsgrad: bool,
    ) {
        let lr = 1e-3;
        let betas = (0.9, 0.999);
        let eps = 1e-8;
        let weight_decay = 0.;
        let amsgrad = false;
        if !(0.0 <= lr) {
            panic!("Invalid learning rate: stringify!{}", lr);
        }
        if !(0.0 <= eps) {
            panic!("Invalid epsilon value: stringify!{}", eps);
        }
        if !(0.0 <= betas.0 && betas.0 < 1.0) {
            panic!("Invalid beta parameter at index 0: stringify!{}", betas.0);
        }
        if !(0.0 <= betas.1 && betas.1 < 1.0) {
            panic!("Invalid beta parameter at index 1: stringify!{}", betas.1);
        }
        if !(0.0 <= weight_decay) {
            panic!("Invalid weight_decay value: stringify!{}", weight_decay);
        }
        // let defaults = dict(
        //     lr = lr,
        //     betas = betas,
        //     eps = eps,
        //     weight_decay = weight_decay,
        //     amsgrad = amsgrad,
        // );
        let mut defaults: Dict<_> = Dict::new();
        defaults.insert("lr", lr);
        defaults.insert("betas", betas);
        defaults.insert("eps", eps);
        defaults.insert("weight_decay", weight_decay);
        defaults.insert("amsgrad", amsgrad);
        // let temp: &f64 = defaults.get::<f64>("lr").unwrap();
        // println!("{:?}", temp);
        // super(Adam, self).new(params, defaults);
    }
    fn __setstate__(self /*state*/) {
        // self.optimizer.__setstate__(state);
        // for group in self.optimizer.param_groups {
        //     group.setdefault("amsgrad", false);
        // }
        match self.optimizer.param_groups.get::<bool>("amsgrad") {
            Ok(_) => (),
            Err(_) => self.optimizer.param_groups.insert("amsgrad", false),
        }
    }
    /// Performs a single optimization step.
    ///         Arguments{
    ///             closure (callable, optional): A closure that reevaluates the model{
    ///                 && returns the loss.}}
    // @torch.no_grad();
    fn step(self, closure: Option<T>) -> Option<T> {
        let loss = None;
        // if !closure.is_none() {
        //     if Torch.enable_grad() {
        //         loss = Torch.closure();
        //     }
        // }
        for group in self.optimizer.param_groups {
            for p in group.params {
                if p.grad == None {
                    continue;
                }
                let grad = p.grad;
                if grad.is_sparse {
                    panic!(
                        r#"RuntimeError("Adam does !support sparse gradients, please consider SparseAdam instead")"#
                    );
                }
                let amsgrad = group.get("amsgrad");
                let state = self.optimizer.state.get(p);
                // State initialization;
                if state.len() == 0 {
                    // state["step"] = 0;
                    state.get_mut::<usize>("step").unwrap() = 0;
                    // Exponential moving average of gradient values;
                    // state["exp_avg"] = Torch.zeros_like(p, memory_format = torch.preserve_format);
                    // state["exp_avg"] = Torch.zeros_like(p, Torch.preserve_format);
                    // Exponential moving average of squared gradient values;
                    // state["exp_avg_sq"] =
                    //     // Torch.zeros_like(p, memory_format = torch.preserve_format);
                    //     Torch.zeros_like(p,  Torch.preserve_format);
                    if amsgrad {
                        // Maintains max of all exp. moving avg. of sq. grad. values;
                        // state["max_exp_avg_sq"] =
                        //     // Torch.zeros_like(p, memory_format = torch.preserve_format);
                        //     Torch.zeros_like(p,  Torch.preserve_format);
                    }
                }
                let (exp_avg, exp_avg_sq) = (state["exp_avg"], state["exp_avg_sq"]);
                // if amsgrad {
                //     max_exp_avg_sq = state["max_exp_avg_sq"];
                // }
                let (beta1, beta2) = group["betas"];
                state["step"] += 1;
                let bias_correction1 = 1 - beta1 * *state["step"];
                let bias_correction2 = 1 - beta2 * *state["step"];
                if group.get("weight_decay") != 0. {
                    // grad = grad.add(p, alpha = group["weight_decay"]);
                    grad = grad.add(p, group["weight_decay"]);
                }
                // Decay the first && second moment running average coefficient;
                // exp_avg.mul_(beta1).add_(grad, alpha = 1 - beta1);
                exp_avg.mul_(beta1).add_(grad, 1 - beta1);
                exp_avg_sq
                    .mul_(beta2)
                    // .addcmul_(grad, grad, value = 1 - beta2);
                    .addcmul_(grad, grad, 1 - beta2);
                let denom;
                if amsgrad {
                    let max_exp_avg_sq = state["max_exp_avg_sq"];
                    // Maintains the maximum of all 2nd moment running avg. till now;
                    // Torch.max(max_exp_avg_sq, exp_avg_sq, out = max_exp_avg_sq);
                    Torch.max(max_exp_avg_sq, exp_avg_sq, max_exp_avg_sq);
                    // Use the max. for normalizing running avg. of gradient;
                    denom =
                        (max_exp_avg_sq.sqrt() / Math.sqrt(bias_correction2)).add_(group["eps"]);
                } else {
                    denom = (exp_avg_sq.sqrt() / Math.sqrt(bias_correction2)).add_(group["eps"]);
                }
                let step_size = group["lr"] / bias_correction1;
                // p.addcdiv_(exp_avg, denom, value = -step_size);
                p.addcdiv_(exp_avg, denom, -step_size);
            }
        }
        return loss;
    }
}
