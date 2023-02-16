import math
import torch
from .opti import Optimizer


class SelfAdam:
    r"""Implements Adam algorithm.

    It has been proposed in `Adam: A Method for Stochastic Optimization`_.
    The implementation of the L2 penalty follows changes proposed in
    `Decoupled Weight Decay Regularization`_.

    Arguments:
        params (iterable): iterable of parameters to optimize or dicts defining
            parameter groups
        lr (float, optional): learning rate (default: 1e-3)
        betas (Tuple[float, float], optional): coefficients used for computing
            running averages of gradient and its square (default: (0.9, 0.999))
        eps (float, optional): term added to the denominator to improve
            numerical stability (default: 1e-8)
        weight_decay (float, optional): weight decay (L2 penalty) (default: 0)
        amsgrad (boolean, optional): whether to use the AMSGrad variant of this
            algorithm from the paper `On the Convergence of Adam and Beyond`_
            (default: False)

    .. _Adam\: A Method for Stochastic Optimization:
        https://arxiv.org/abs/1412.6980
    .. _Decoupled Weight Decay Regularization:
        https://arxiv.org/abs/1711.05101
    .. _On the Convergence of Adam and Beyond:
        https://openreview.net/forum?id=ryQu7f-RZ
    """

    def __init__(self, params, lr=1e-3, betas=(0.9, 0.999), eps=1e-8,
                 weight_decay=0, amsgrad=False):
        if not 0.0 <= lr:
            raise ValueError("Invalid learning rate: {}".format(lr))
        if not 0.0 <= eps:
            raise ValueError("Invalid epsilon value: {}".format(eps))
        if not 0.0 <= betas[0] < 1.0:
            raise ValueError("Invalid beta parameter at index 0: {}".format(betas[0]))
        if not 0.0 <= betas[1] < 1.0:
            raise ValueError("Invalid beta parameter at index 1: {}".format(betas[1]))
        if not 0.0 <= weight_decay:
            raise ValueError("Invalid weight_decay value: {}".format(weight_decay))
        defaults = dict(lr=lr, betas=betas, eps=eps,
                        weight_decay=weight_decay, amsgrad=amsgrad)
        # super(Adam, self).__init__(params, defaults)
        torch._C._log_api_usage_once("python.optimizer")
        self.defaults = defaults

        if isinstance(params, torch.Tensor):
            raise TypeError("params argument given to the optimizer should be "
                            "an iterable of Tensors or dicts, but got " +
                            torch.typename(params))

        self.state = defaultdict(dict)
        self.param_groups = []

        param_groups = list(params)
        if len(param_groups) == 0:
            raise ValueError("optimizer got an empty parameter list")
        if not isinstance(param_groups[0], dict):
            param_groups = [{'params': param_groups}]

        for param_group in param_groups:
            self.add_param_group(param_group)

    def __setstate__(self, state):
        # super(Adam, self).__setstate__(state)
        self.__dict__.update(state)
        for group in self.param_groups:
            group.setdefault('amsgrad', False)

    def __getstate__(self):
        return {
            'defaults': self.defaults,
            'state': self.state,
            'param_groups': self.param_groups,
        }
        
    def __repr__(self):
        format_string = self.__class__.__name__ + ' ('
        for i, group in enumerate(self.param_groups):
            format_string += '\n'
            format_string += 'Parameter Group {0}\n'.format(i)
            for key in sorted(group.keys()):
                if key != 'params':
                    format_string += '    {0}: {1}\n'.format(key, group[key])
        format_string += ')'
        return format_string
    
    def state_dict(self):
        r"""Returns the state of the optimizer as a :class:`dict`.

        It contains two entries:

        * state - a dict holding current optimization state. Its content
            differs between optimizer classes.
        * param_groups - a dict containing all parameter groups
        """
        # Save order indices instead of Tensors
        param_mappings = {}
        start_index = 0

        def pack_group(group):
            nonlocal start_index
            # packed = {k: v for k, v in group.items() if k != 'params'}
            temp_packed_dict = {}
            for k, v in group.items():
                if k != 'params':
                    temp_packed_dict[k] = v
            packed = temp_packed_dict
            
            # param_mappings.update({id(p): i for i, p in enumerate(group['params'], start_index)
            #                        if id(p) not in param_mappings})
            temp_param_mappings_dict = {}
            for i, p in enumerate(group['params'], start_index):
                if id(p) not in param_mappings:
                    temp_param_mappings_dict[id(p)] = i
            param_mappings.update(temp_param_mappings_dict)
            
            # packed['params'] = [param_mappings[id(p)] for p in group['params']]
            temp_packed_list = []
            for p in group['params']:
                temp_packed_list.append(param_mappings[id(p)])
            packed['params'] = temp_packed_list
            
            start_index += len(packed['params'])
            return packed
        
        # param_groups = [pack_group(g) for g in self.param_groups]
        temp_param_groups_list = []
        for g in self.param_groups:
            temp_param_groups_list.append(pack_group(g))
        param_groups = temp_param_groups_list
        
        # Remap state to use order indices as keys
        # packed_state = {(param_mappings[id(k)] if isinstance(k, torch.Tensor) else k): v
        #                 for k, v in self.state.items()}
        temp_packed_state_dict = {}
        for k, v in self.state.items():
            temp_key = None
            if isinstance(k, torch.Tensor):
                temp_key = param_mappings[id(k)] 
            else:
                temp_key = k
            temp_packed_state_dict[temp_key] = v
        packed_state = temp_packed_state_dict
        
        return {
            'state': packed_state,
            'param_groups': param_groups,
        }
        
    def load_state_dict(self, state_dict):
        r"""Loads the optimizer state.

        Arguments:
            state_dict (dict): optimizer state. Should be an object returned
                from a call to :meth:`state_dict`.
        """
        # deepcopy, to be consistent with module API
        state_dict = deepcopy(state_dict)
        # Validate the state_dict
        groups = self.param_groups
        saved_groups = state_dict['param_groups']

        if len(groups) != len(saved_groups):
            raise ValueError("loaded state dict has a different number of "
                             "parameter groups")
            
        # param_lens = (len(g['params']) for g in groups)
        temp_param_lens_list = []
        for g in groups:
            temp_param_lens_list.append(len(g['params']))
        param_lens = temp_param_lens_list
        
        # saved_lens = (len(g['params']) for g in saved_groups)
        temp_saved_lens_list = []
        for g in saved_groups:
            temp_saved_lens_list.append(len(g['params']))
        param_lens = temp_saved_lens_list
        
        if any(p_len != s_len for p_len, s_len in zip(param_lens, saved_lens)):
            raise ValueError("loaded state dict contains a parameter group "
                             "that doesn't match the size of optimizer's group")

        # # Update the state
        # id_map = {old_id: p for old_id, p in
        #           zip(chain.from_iterable((g['params'] for g in saved_groups)),
        #               chain.from_iterable((g['params'] for g in groups)))}
        temp_dict_map = {}
        for old_id, p in zip(chain.from_iterable((g['params'] for g in saved_groups)), chain.from_iterable((g['params'] for g in groups))):
            temp_dict_map[old_id] = p
        id_map = temp_dict_map

        def cast(param, value):
            r"""Make a deep copy of value, casting all tensors to device of param."""
            if isinstance(value, torch.Tensor):
                # Floating-point types are a bit special here. They are the only ones
                # that are assumed to always match the type of params.
                if param.is_floating_point():
                    value = value.to(param.dtype)
                value = value.to(param.device)
                return value
            elif isinstance(value, dict):
                # return {k: cast(param, v) for k, v in value.items()}
                temp_dict = {}
                for k, v in value.items():
                    temp_dict[k] = cast(param,v)
                return temp_dict
            elif isinstance(value, container_abcs.Iterable):
                return type(value)(cast(param, v) for v in value)
                """may not work the same as the one before so for the moment it's commented"""
                # temp_tuple = []
                # for v in value:
                #     temp_tuple.append(cast(param,v))
                # return type(value)(temp_tuple)
            else:
                return value

        # Copy state assigned to params (and cast tensors to appropriate types).
        # State that is not assigned to params is copied as is (needed for
        # backward compatibility).
        state = defaultdict(dict)
        for k, v in state_dict['state'].items():
            if k in id_map:
                param = id_map[k]
                state[param] = cast(param, v)
            else:
                state[k] = v

        # Update parameter groups, setting their 'params' value
        def update_group(group, new_group):
            new_group['params'] = group['params']
            return new_group
        
        # param_groups = [update_group(g, ng) for g, ng in zip(groups, saved_groups)]
        temp_list = []
        for g, ng in zip(groups, saved_groups):
            temp_list.append(update_group(g, ng))
        param_groups = temp_list
        
        self.__setstate__({'state': state, 'param_groups': param_groups})


    def zero_grad(self):
        r"""Clears the gradients of all optimized :class:`torch.Tensor` s."""
        for group in self.param_groups:
            for p in group['params']:
                if p.grad is not None:
                    p.grad.detach_()
                    p.grad.zero_()
    @torch.no_grad()
    def step(self, closure=None):
        """Performs a single optimization step.

        Arguments:
            closure (callable, optional): A closure that reevaluates the model
                and returns the loss.
        """
        loss = None
        if closure is not None:
            with torch.enable_grad():
                loss = closure()

        for group in self.param_groups:
            for p in group['params']:
                if p.grad is None:
                    continue # if the weight doesn't have a gradiant, skip it
                grad = p.grad
                # grad = p.grad
                if grad.is_sparse:
                    raise RuntimeError('Adam does not support sparse gradients, please consider SparseAdam instead')
                amsgrad = group['amsgrad']
                # amsgrad = old(amsgrad)

                state = self.state[p]
                # state = self.state[p]

                # State initialization
                if len(state) == 0:
                    state['step'] = 0
                    # old(step) = 0
                    # Exponential moving average of gradient values
                    state['exp_avg'] = torch.zeros_like(p, memory_format=torch.preserve_format)
                    # old(exp_avg) = [0.;?]
                    # Exponential moving average of squared gradient values
                    state['exp_avg_sq'] = torch.zeros_like(p, memory_format=torch.preserve_format)
                    # old(exp_avg_sq) = [0.;?]
                    if amsgrad:
                        # Maintains max of all exp. moving avg. of sq. grad. values
                        state['max_exp_avg_sq'] = torch.zeros_like(p, memory_format=torch.preserve_format)
                        # old(max_exp_avg_sq) = [0.;?]

                exp_avg, exp_avg_sq = state['exp_avg'], state['exp_avg_sq']
                # (exp_avg, exp_avg_sq) = old(exp_avg, exp_avg_sq)
                if amsgrad:
                    max_exp_avg_sq = state['max_exp_avg_sq']
                    # max_exp_avg_sq = old(max_exp_avg_sq)
                beta1, beta2 = group['betas']
                # (beta1, beta2) = (0.9, 0.999)

                state['step'] += 1
                bias_correction1 = 1 - beta1 ** state['step']
                # bias_correction1 = 1 - beta1 ^ step
                bias_correction2 = 1 - beta2 ** state['step']
                # bias_correction2 = 1 - beta2 ^ step

                if group['weight_decay'] != 0:
                    grad = grad.add(p, alpha=group['weight_decay'])
                    # grad = grad + weight_decay*p

                # Decay the first and second moment running average coefficient
                exp_avg.mul_(beta1).add_(grad, alpha=1 - beta1)
                # exp_avg *= beta1
                # exp_avg += (1 - beta1)*grad
                
                exp_avg_sq.mul_(beta2).addcmul_(grad, grad, value=1 - beta2)
                # exp_avg_sq *= beta2
                # exp_avg_sq += (1 - beta2)*grad*grad
                
                if amsgrad:
                    # Maintains the maximum of all 2nd moment running avg. till now
                    torch.max(max_exp_avg_sq, exp_avg_sq, out=max_exp_avg_sq)
                    # max_exp_avg_sq = max(max_exp_avg_sq, exp_avg_sq)
                    
                    # Use the max. for normalizing running avg. of gradient
                    denom = (max_exp_avg_sq.sqrt() / math.sqrt(bias_correction2)).add_(group['eps'])
                    # denom = (max_exp_avg_sq/bias_correction2) ^ (1/2) + eps
                else:
                    denom = (exp_avg_sq.sqrt() / math.sqrt(bias_correction2)).add_(group['eps'])
                    # denom = (exp_avg_sq/bias_correction2) ^ (1/2) + eps

                step_size = group['lr'] / bias_correction1

                p.addcdiv_(exp_avg, denom, value=-step_size)
                # p += (-step_size)* (exp_avg/denom)

        return loss

    def add_param_group(self, param_group):
        r"""Add a param group to the :class:`Optimizer` s `param_groups`.

        This can be useful when fine tuning a pre-trained network as frozen layers can be made
        trainable and added to the :class:`Optimizer` as training progresses.

        Arguments:
            param_group (dict): Specifies what Tensors should be optimized along with group
            specific optimization options.
        """
        assert isinstance(param_group, dict), "param group must be a dict"

        params = param_group['params']
        if isinstance(params, torch.Tensor):
            param_group['params'] = [params]
        elif isinstance(params, set):
            raise TypeError('optimizer parameters need to be organized in ordered collections, but '
                            'the ordering of tensors in sets will change between runs. Please use a list instead.')
        else:
            param_group['params'] = list(params)

        for param in param_group['params']:
            if not isinstance(param, torch.Tensor):
                raise TypeError("optimizer can only optimize Tensors, "
                                "but one of the params is " + torch.typename(param))
            if not param.is_leaf:
                raise ValueError("can't optimize a non-leaf Tensor")

        for name, default in self.defaults.items():
            if default is required and name not in param_group:
                raise ValueError("parameter group didn't specify a value of required optimization parameter " +
                                 name)
            else:
                param_group.setdefault(name, default)

        param_set = set()
        for group in self.param_groups:
            param_set.update(set(group['params']))

        if not param_set.isdisjoint(set(param_group['params'])):
            raise ValueError("some parameters appear in more than one parameter group")

        self.param_groups.append(param_group)
