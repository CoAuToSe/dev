>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>];

<span class="ident">v</span>.<span class="ident">iter</span>().<span class="ident">for_each</span>(<span class="op">|</span><span class="ident">x</span><span class="op">|</span> <span class="macro">println</span><span class="macro">!</span>(<span class="string">&quot;{}&quot;</span>, <span class="ident">x</span>));
<span class="comment">// or</span>
<span class="kw">for</span> <span class="ident">x</span> <span class="kw">in</span> <span class="kw-2">&amp;</span><span class="ident">v</span> {
    <span class="macro">println</span><span class="macro">!</span>(<span class="string">&quot;{}&quot;</span>, <span class="ident">x</span>);
}<a class="test-arrow" target="_blank" href="https://play.rust-lang.org/?code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Alet%20v%20%3D%20vec!%5B1%2C%202%2C%203%2C%204%2C%205%5D%3B%0A%0Av.iter().for_each(%7Cx%7C%20println!(%22%7B%7D%22%2C%20x))%3B%0A%2F%2F%20or%0Afor%20x%20in%20%26v%20%7B%0A%20%20%20%20println!(%22%7B%7D%22%2C%20x)%3B%0A%7D%0A%7D&amp;edition=2018">Run</a></pre></div>
<p>Another common way to evaluate an iterator is to use the <a href="trait.Iterator.html#method.collect"><code>collect</code></a>
method to produce a new collection.</p>
<h1 id="infinity" class="section-header"><a href="#infinity">Infinity</a></h1>
<p>Iterators do not have to be finite. As an example, an open-ended range is
an infinite iterator:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered">
<span class="kw">let</span> <span class="ident">numbers</span> <span class="op">=</span> <span class="number">0</span>..;<a class="test-arrow" target="_blank" href=