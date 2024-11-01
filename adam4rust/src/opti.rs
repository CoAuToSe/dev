// from collections // import defaultdict;
// from torch._six // import container_abcs;
// import torch;
// use Torch;

use std::hash::Hash;

// use crate::util::Paze;
use cats_utils::Dict;
// from copy // import deepcopy;
// from itertools // import chain;

// mod util;
// use util::*;
// use crate::util::Group;

struct _RequiredParameter {}

/// Singleton impl representing a required parameter for an Optimizer.
impl _RequiredParameter {
    fn __repr__(self) -> &'static str {
        return "<required parameter>";
    }
}
// fn aze() {
//     required = _RequiredParameter::();
// }

pub struct Optimizer<T, U, V, W, X>
where
    U: Eq + PartialEq + Hash,
    V: Eq + PartialEq + Hash,
{
    pub param_groups: Dict<V>,
    pub state: Dict<U>,
    pub defaults: T,
    pub params: X,
    tmp: W,
}

// ///     /// Base impl for all optimizers.
// ///     .. warning{
// ///         Parameters need to be specified as collections that have a deterministic
// ///         ordering that == consistent between runs. Examples of objects that don"t
// ///         satisfy those properties are sets && iterators over values of dictionaries.}
// ///     Arguments{
// ///         params (iterable): an iterable of :class:`torch.Tensor` s or{
// ///             :class:`dict` s. Specifies what Tensors should be optimized.}
// ///         defaults: (dict): a dict containing default values of optimization{
// ///             options (used when a parameter group doesn"t specify them).}}
// impl<T, U, V, W, X> Optimizer<T, U, V, W, X> {
//     fn new(self, params: X, defaults: T) {
//         Torch._C._log_api_usage_once("python.optimizer");
//         self.defaults = defaults;
//         if isinstance(params, Torch.Tensor) {
//             panic!(
//                 r#"TypeError("params argument given to the optimizer should be an iterable of Tensors || dicts, but got " +torch.typename(params))"#
//             );
//         }
//         self.state = defaultdict(dict);
//         self.param_groups = [];
//         let param_groups = list(params);
//         if param_groups.len() == 0 {
//             panic!(r#"ValueError("optimizer got an empty parameter list")"#);
//         }
//         if !isinstance(param_groups[0], dict) {
//             param_groups = [stringify! {"params": param_groups}];
//         }
//         for param_group in param_groups {
//             self.add_param_group(param_group);
//         }
//     }
//     fn __getstate__(self) -> &'static str {
//         return stringify! { "defaults": self.defaults, "state": self.state, "param_groups": self.param_groups, };
//     }
//     // fn __setstate__(/*self, state)*/) {
//     //     self.__dict__.update(state);
//     // }
//     fn __repr__(self) {
//         let format_string = self.__class__.__name__ + " (";
//         for (i, group) in enumerate(self.param_groups) {
//             format_string += "\n";
//             format_string += "Parameter Group stringify!{0}\n".format(i);
//             for key in group.keys().sort() {
//                 if key != "params" {
//                     format_string += " stringify!{0}: stringify!{1}\n".format(key, group[key]);
//                 }
//             }
//         }
//         format_string += ")";
//         return format_string;
//     }
//     ///         /// Returns the state of the optimizer as a :class:`dict`.
//     ///         It contains two entries
//     ///         * state - a dict holding current optimization state. Its content{
//     ///             differs between optimizer classes.}
//     ///         * param_groups - a dict containing all parameter groups
//     fn state_dict(self) -> &'static str {
//         // Save order indices instead of Tensors;
//         let mut param_mappings = stringify! {};
//         let mut start_index = 0;
//         fn pack_group(/*group)*/) {
//             // nonlocal start_index;
//             let packed = stringify! {k: v for ( k, v) in group.items() if k != "params"};
//             param_mappings.update(
//                 stringify! {id(p): i for ( i, p) in enumerate(group["params"], start_index)
//                 if id(p) !in param_mappings},
//             );
//             // packed["params"] = [param_mappings[id(p)] for p in group["params"]]; =
//             packed["params"] = {
//                 let mut temp = vec![];
//                 for p in group["params"] {
//                     temp.push(param_mappings[id(p)])
//                 }
//                 temp
//             };
//             start_index += packed["params"].len();
//             return packed;
//         }
//         // param_groups = [pack_group(g) for g in self.param_groups]; =
//         let param_groups = {
//             let mut temp = vec![];
//             for g in self.param_groups {
//                 temp.push(pack_group(g))
//             }
//             temp
//         };
//         // Remap state to use order indices as keys;
//         packed_state = stringify! {(param_mappings[id(k)] if isinstance(k, torch.Tensor) else k): v
//         for ( k, v) in self.state.items()};
//         return stringify! { "state": packed_state, "param_groups": param_groups, };
//     }

//     ///         /// Loads the optimizer state.
//     ///         Arguments{
//     ///             state_dict (dict): optimizer state. Should be an object returned{
//     ///                 // from a call to :meth:`state_dict`.}}
//     fn load_state_dict(/*self, state_dict)*/) {
//         // deepcopy, to be consistent with module API;
//         let state_dict = deepcopy(state_dict);
//         // Validate the state_dict;
//         let groups = self.param_groups;
//         let saved_groups = state_dict["param_groups"];
//         if len(groups) != len(saved_groups) {
//             panic!(r#"ValueError("loaded state dict has a different number of parameter groups")"#);
//         }
//         // param_lens = (len(g["params"]) for g in groups);
//         // saved_lens = (len(g["params"]) for g in saved_groups);
//         for (p_len, s_len) in zip(param_lens, saved_lens) {
//             if p_len != s_len {
//                 panic!(
//                     r#"ValueError("loaded state dict contains a parameter group that doesn"t match the size of optimizer"s group")"#
//                 );
//             }
//         }
//         // Update the state;
//         let id_map = stringify! {old_id: p for ( old_id, p) in  zip(chain.from_iterable((g["params"] for g in saved_groups)), chain.from_iterable((g["params"] for g in groups)))};

//         /// Make a deep copy of value, casting all tensors to device of param.
//         fn cast(/*param, value)*/) {
//             if isinstance(value, torch.Tensor) {
//                 // Floating-point types are a bit special here. They are the only ones;
//                 // that are assumed to always match the type of params.;
//                 if param.is_floating_point() {
//                     value = value.to(param.dtype);
//                 }
//                 value = value.to(param.device);
//                 return value;
//             } else if isinstance(value, dict) {
//                 return stringify! {k: cast(param, v) for ( k, v) in value.items()};
//             } else if isinstance(value, container_abcs.Iterable) {
//                 // return type(value)(cast(param, v) for ( v) in value);
//             } else {
//                 return value;
//             }
//         }
//         // Copy state assigned to params (and cast tensors to appropriate types).;
//         // State that != assigned to params == copied as == (needed for;
//         // backward compatibility).;
//         state = defaultdict(dict);
//         for (k, v) in state_dict["state"].items() {
//             if inside(k, id_map) {
//                 param = id_map[k];
//                 // state[param] = cast(param, v);
//             } else {
//                 state[k] = v;
//             }
//         }
//         // Update parameter groups, setting their "params" value;
//         fn update_group(/*group, new_group)*/) {
//             new_group["params"] = group["params"];
//             return new_group;
//         }
//         // param_groups = [update_group(g, ng) for ( g, ng) in zip(groups, saved_groups)];
//         self.__setstate__(stringify! {"state": state, "param_groups": param_groups});
//     }
//     /// Clears the gradients of all optimized :class:`torch.Tensor` s.
//     fn zero_grad(self) {
//         for group/* : Group<_,_> */ in self.param_groups {
//             for p/* : Paze<_> */ in group["params"] {
//                 if p.grad != None {
//                     p.grad.detach_();
//                     p.grad.zero_();
//                 }
//             }
//         }
//     }
//     ///         /// Performs a single optimization step (parameter update).
//     ///         Arguments{
//     ///             closure (callable): A closure that reevaluates the model and{
//     ///                 returns the loss. Optional for most optimizers.}}
//     ///         .. note{
//     ///             Unless otherwise specified, this function should !modify the
//     ///             ``.grad`` field of the parameters.}
//     fn step(/*self, closure)*/) {
//         panic!(r#"NotImplementedError"#);
//     }
//     ///         /// Add a param group to the :class:`Optimizer` s `param_groups`.
//     ///         This can be useful when fine tuning a pre-trained network as frozen layers can be made
//     ///         trainable && added to the :class:`Optimizer` as training progresses.
//     ///         Arguments{
//     ///             param_group (dict): Specifies what Tensors should be optimized along with group
//     ///             specific optimization options.}
//     fn add_param_group(self, param_group: crate::util::Group<V, W>) {
//         assert!(isinstance(param_group, dict), "param group must be a dict");
//         let params = param_group["params"];
//         if isinstance(params, Torch.Tensor) {
//             param_group["params"] = [params];
//         } else if isinstance(params, set) {
//             panic!(
//                 r#"TypeError("optimizer parameters need to be organized in ordered collections, but the ordering of tensors in sets will change between runs. Please use a list instead.")"#
//             );
//         } else {
//             param_group["params"] = list(params);
//         }
//         for param in param_group["params"] {
//             if !isinstance(param, Torch.Tensor) {
//                 panic!(
//                     r#"TypeError("optimizer can only optimize Tensors, but one of the params == " + torch.typename(param))"#
//                 );
//             }
//             if !param.is_leaf {
//                 panic!(r#"ValueError("can"t optimize a non-leaf Tensor")"#);
//             }
//         }
//         for (name, default) in self.defaults.items() {
//             if default == "required" && !inside(name, param_group) {
//                 panic!(
//                     r#"ValueError("parameter group didn"t specify a value of required optimization parameter " + name)"#
//                 );
//             } else {
//                 param_group.setdefault(name, default);
//             }
//         }
//         let param_set = set();
//         for group in self.param_groups {
//             param_set.update(set(group["params"]));
//         }
//         if !param_set.isdisjoint(set(param_group["params"])) {
//             panic!(r#"ValueError("some parameters appear in more than one parameter group")"#);
//         }
//         self.param_groups.append(param_group);
//     }
// }
