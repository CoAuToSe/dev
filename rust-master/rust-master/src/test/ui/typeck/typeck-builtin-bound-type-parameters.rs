n</span>(<span class="ident">handle</span>) =&gt; <span class="kw">match</span> <span class="ident">handle</span>.<span class="ident">force</span>() {
                    <span class="ident">Leaf</span>(<span class="ident">leaf</span>) =&gt; <span class="kw">return</span> <span class="ident">GoDown</span>(<span class="ident">leaf</span>),
                    <span class="ident">Internal</span>(<span class="ident">internal</span>) =&gt; <span class="ident">internal</span>.<span class="ident">descend</span>(),
                },
            }
        }
    }

    <span class="doccomment">/// Descends to the nearest node where the edge matching the lower bound</span>
    <span class="doc