<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Average packed unsigned 8-bit integers in a and b, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set)."><meta name="keywords" content="rust, rustlang, rust-lang, _mm256_maskz_avg_epu8"><title>_mm256_maskz_avg_epu8 in core::arch::x86_64 - Rust</title><link rel="stylesheet" type="text/css" href="../../../normalize1.58.1.css"><link rel="stylesheet" type="text/css" href="../../../rustdoc1.58.1.css" id="mainThemeStyle"><link rel="stylesheet" type="text/css" href="../../../light1.58.1.css"  id="themeStyle"><link rel="stylesheet" type="text/css" href="../../../dark1.58.1.css" disabled ><link rel="stylesheet" type="text/css" href="../../../ayu1.58.1.css" disabled ><script id="default-settings" ></script><script src="../../../storage1.58.1.js"></script><script src="../../../crates1.58.1.js"></script><script defer src="../../../main1.58.1.js"></script>
    <noscript><link rel="stylesheet" href="../../../noscript1.58.1.css"></noscript><link rel="alternate icon" type="image/png" href="../../../favicon-16x161.58.1.png"><link rel="alternate icon" type="image/png" href="../../../favicon-32x321.58.1.png"><link rel="icon" type="image/svg+xml" href="../../../favicon1.58.1.svg"><style type="text/css">#crate-search{background-image:url("../../../down-arrow1.58.1.svg");}</style></head><body class="rustdoc fn"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="sidebar-menu" role="button">&#9776;</div><a href='../../../core/index.html'><div class='logo-container rust-logo'><img src='../../../rust-logo1.58.1.png' alt='logo'></div></a><div class="sidebar-elems"><h2 class="location">Other items in<br><a href="../../index.html">core</a>::<wbr><a href="../index.html">arch</a>::<wbr><a href="index.html">x86_64</a></h2><div id="sidebar-vars" data-name="_mm256_maskz_avg_epu8" data-ty="fn" data-relpath=""></div><script defer src="sidebar-items.js"></script></div></nav><div class="theme-picker"><button id="theme-picker" aria-label="Pick another theme!" aria-haspopup="menu" title="themes"><img width="18" height="18" alt="Pick another theme!" src="../../../brush1.58.1.svg"></button><div id="theme-choices" role="menu"></div></div><nav class="sub"><form class="search-form"><div class="search-container"><div><select id="crate-search"><option value="All crates">All crates</option></select><input class="search-input" name="search" autocomplete="off" spellcheck="false" placeholder="Click or press ‘S’ to search, ‘?’ for more options…" type="search"></div><button type="button" id="help-button" title="help">?</button><a id="settings-menu" href="../../../settings.html" title="settings"><img width="18" height="18" alt="Change settings" src="../../../wheel1.58.1.svg"></a></div></form></nav><section id="main" class="content"><h1 class="fqn"><span class="in-band">Function <a href="../../index.html">core</a>::<wbr><a href="../index.html">arch</a>::<wbr><a href="index.html">x86_64</a>::<wbr><a class="fn" href="#">_mm256_maskz_avg_epu8</a><button id="copy-path" onclick="copy_path(this)" title="Copy item path to clipboard"><img src="../../../clipboard1.58.1.svg" width="19" height="18" alt="Copy item path"></button></span><span class="out-of-band"><span id="render-detail"><a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">[<span class="inner">&#x2212;</span>]</a></span><a class="srclink" href="../../../src/core/up/up/stdarch/crates/core_arch/src/x86/avx512bw.rs.html#4888-4892" title="goto source code">[src]</a></span></h1><div class="docblock item-decl"><pre class="rust fn"><code>pub unsafe fn _mm256_maskz_avg_epu8(<br>&nbsp;&nbsp;&nbsp;&nbsp;k: <a class="type" href="../x86/type.__mmask32.html" title="type core::arch::x86::__mmask32">__mmask32</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;a: <a class="struct" href="../x86/struct.__m256i.html" title="struct core::arch::x86::__m256i">__m256i</a>, <br>&nbsp;&nbsp;&nbsp;&nbsp;b: <a class="struct" href="../x86/struct.__m256i.html" title="struct core::arch::x86::__m256i">__m256i</a><br>) -&gt; <a class="struct" href="../x86/struct.__m256i.html" title="struct core::arch::x86::__m256i">__m256i</a></code></pre></div><div class="item-info"><div class="stab unstable"><span class="emoji">🔬</span> This is a nightly-only experimental API. (<code>stdsimd</code>&nbsp;<a href="https://github.com/rust-lang/rust/issues/48556">#48556</a>)</div><div class="stab portability">This is supported on <strong>x86-64 and target feature <code>avx512bw,avx512vl</code></strong> only.</div></div><details class="rustdoc-toggle top-doc" open><summary class="hideme"><span>Expand description</span></summary><div class="docblock"><p>Average packed unsigned 8-bit integers in a and b, and store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).</p>
<p><a href="https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_maskz_avg_epu8&amp;expand=396">Intel’s documentation</a></p>
</div></details></section><section id="search" class="content hidden"></section><div id="rustdoc-vars" data-root-path="../../../" data-current-crate="core" data-search-index-js="../../../search-index1.58.1.js" data-search-js="../../../search1.58.1.js"></div>
</body></html>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      <!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="API documentation for the Rust `powerpc` mod in crate `core`."><meta name="keywords" content="rust, rustlang, rust-lang, powerpc"><title>core::arch::powerpc - Rust</title><link rel="stylesheet" type="text/css" href="../../../normalize1.41.0.css"><link rel="stylesheet" type="text/css" href="../../../rustdoc1.41.0.css" id="mainThemeStyle"><link rel="stylesheet" type="text/css" href="../../../dark1.41.0.css"><link rel="stylesheet" type="text/css" href="../../../light1.41.0.css" id="themeStyle"><script src="../../../storage1.41.0.js"></script><noscript><link rel="stylesheet" href="../../../noscript1.41.0.css"></noscript><link rel="shortcut icon" href="../../../favicon1.41.0.ico"><style type="text/css">#crate-search{background-image:url("../../../down-arrow1.41.0.svg");}</style></head><body class="rustdoc mod"><!--[if lte IE 8]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="sidebar-menu">&#9776;</div><a href='../../../core/index.html'><div class='logo-container'><img src='../../../rust-logo1.41.0.png' alt='logo'></div></a><p class='location'>Module powerpc</p><div class="sidebar-elems"><div class="block items"><ul><li><a href="#structs">Structs</a></li><li><a href="#functions">Functions</a></li></ul></div><p class='location'><a href='../../index.html'>core</a>::<wbr><a href='../index.html'>arch</a></p><script>window.sidebarCurrent = {name: 'powerpc', ty: 'mod', relpath: '../'};</script><script defer src="../sidebar-items.js"></script></div></nav><div class="theme-picker"><button id="theme-picker" aria-label="Pick another theme!"><img src="../../../brush1.41.0.svg" width="18" alt="Pick another theme!"></button><div id="theme-choices"></div></div><script src="../../../theme1.41.0.js"></script><nav class="sub"><form class="search-form"><div class="search-container"><div><select id="crate-search"><option value="All crates">All crates</option></select><input class="search-input" name="search" disabled autocomplete="off" spellcheck="false" placeholder="Click or press ‘S’ to search, ‘?’ for more options…" type="search"></div><a id="settings-menu" href="../../../settings.html"><img src="../../../wheel1.41.0.svg" width="18" alt="Change settings"></a></div></form></nav><section id="main" class="content"><h1 class='fqn'><span class='out-of-band'><span class='since' title='Stable since Rust version '></span><span id='render-detail'><a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">[<span class='inner'>&#x2212;</span>]</a></span><a class='srclink' href='../../../src/core/up/stdarch/crates/core_arch/src/mod.rs.html#95-97' title='goto source code'>[src]</a></span><span class='in-band'>Module <a href='../../index.html'>core</a>::<wbr><a href='../index.html'>arch</a>::<wbr><a class="mod" href=''>powerpc</a></span></h1><div class='stability'><div class='stab unstable'><span class='emoji'>🔬</span> This is a nightly-only experimental API. (<code>stdsimd</code>&nbsp;<a href="https://github.com/rust-lang/rust/issues/27731">#27731</a>)</div><div class='stab portability'>This is supported on <strong>PowerPC</strong> only.</div></div><div class='docblock'><p>Platform-specific intrinsics for the <code>PowerPC</code> platform.</p>
<p>See the <a href="../index.html">module documentation</a> for more details.</p>
</div><h2 id='structs' class='section-header'><a href="#structs">Structs</a></h2>
<table><tr class='unstable module-item'><td><a class="struct" href="struct.vector_bool_long.html" title='core::arch::powerpc::vector_bool_long struct'>vector_bool_long</a></td><td class='docblock-short'><span class="stab unstable">Experimental</span><span class="stab portability">PowerPC</span><p>PowerPC-specific 128-bit wide vector mask of two elements</p>
</td></tr><tr class='unstable module-item'><td><a class="struct" href="struct.vector_double.html" title='core::arch::powerpc::vector_double struct'>vector_double</a></td><td class='docblock-short'><span class="stab unstable">Experimental</span><span class="stab portability">PowerPC</span><p>PowerPC-specific 128-bit wide vector of two packed <code>f64</code></p>
</td></tr><tr class='unstable module-item'><td><a class="struct" href="struct.vector_signed_long.html" title='core::arch::powerpc::vector_signed_long struct'>vector_signed_long</a></td><td class='docblock-short'><span class="stab unstable">Experimental</span><span class="stab portability">PowerPC</span><p>PowerPC-specific 128-bit wide vector of two packed <code>i64</code></p>
</td></tr><tr class='unstable module-item'><td><a class="struct" href="struct.vector_unsigned_long.html" title='core::arch::powerpc::vector_unsigned_long struct'>vector_unsigned_long</a></td><td class='docblock-short'><span class="stab unstable">Experimental</span><span class="stab portability">PowerPC</span><p>PowerPC-specific 128-bit wide vector of two packed <code>u64</code></p>
</td></tr></table><h2 id='functions' class='section-header'><a href="#functions">Functions</a></h2>
<table><tr class='unstable module-item'><td><a class="fn" href="fn.trap.html" title='core::arch::powerpc::trap fn'>trap</a><a title='unsafe function' href='#'><sup>⚠</sup></a></td><td class='docblock-short'><span class="stab unstable">Experimental</span><span class="stab portability">PowerPC</span><p>Generates the trap instruction <code>TRAP</code></p>
</td></tr><tr class='unstable module-item'><td><a class="fn" href="fn.vec_xxpermdi.html" title='core::arch::powerpc::vec_xxpermdi fn'>vec_xxpermdi</a><a title='unsafe function' href='#'><sup>⚠</sup></a></td><td class='docblock-short'><span class="stab unstable">Experimental</span><span class="stab portability">PowerPC and <code>vsx</code></span><p>Vector permute.</p>
</td></tr></table></section><section id="search" class="content hidden"></section><section class="footer"></section><script>window.rootPath = "../../../";window.currentCrate = "core";</script><script src="../../../aliases1.41.0.js"></script><script src="../../../main1.41.0.js"></script><script defer src="../../../search-index1.41.0.js"></script></body></html>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           a
   �нm�B�O�                   @   s  d Z ddlZddlZddlZddlZddlZddlZddlZddlmZm	Z	 ddl
mZmZmZ dZe�dej�ZG dd	� d	ej�Zd
d� Zdd� Zdd� Zdd� Zdd� ZG dd� de�Ze� Zddd�Zejdfdd�Zdejejfdd�Z dejejfdd�Z!dS ) zLoading unittests.�    N)�fnmatch�fnmatchcase�   )�case�suite�utilTz[_a-z]\w*\.py$c                       s,   e Zd ZdZ� fdd�Z� fdd�Z�  ZS )�_FailedTestNc                    s   || _ tt| ��|� d S �N)�
_exception�superr   �__init__)�selfZmethod_name�	exception��	__class__� �?D:/a/_temp/msys/msys64/mingw64/lib/python3.9/unittest/loader.pyr      s    z_FailedTest.__init__c                    s*   |� j krtt� ��|�S � fdd�}|S )Nc                      s
   � j �d S r	   )r
   r   �r   r   r   �testFailure!   s    z,_FailedTest.__getattr__.<locals>.testFailure)�_testMethodNamer   r   �__getattr__)r   �namer   r   r   r   r      s    
z_FailedTest.__getattr__)�__name__�
__module__�__qualname__r   r   r   �__classcell__r   r   r   r   r      s   r   c                 C   s"   d| t �� f }t| t|�||�S )Nz#Failed to import test module: %s
%s)�	traceback�
format_exc�_make_failed_test�ImportError)r   �
suiteClass�messager   r   r   �_make_failed_import_test&   s    �r"   c                 C   s   dt �� f }t| |||�S )NzFailed to call load_tests:
%s)r   r   r   )r   r   r    r!   r   r   r   �_make_failed_load_tests+   s    �r#   c                 C   s   t | |�}||f�|fS r	   )r   )�
methodnamer   r    r!   �testr   r   r   r   0   s    
r   c                 C   s<   t �t|��dd� �}| |i}tdt jf|�}||| �f�S )Nc                 S   s   d S r	   r   r   r   r   r   �testSkipped5   s    z'_make_skipped_test.<locals>.testSkippedZModuleSkipped)r   �skip�str�type�TestCase)r$   r   r    r&   �attrsZ	TestClassr   r   r   �_make_skipped_test4   s
    
r,   c                 C   s*   | � � �d�r| d d� S tj�| �d S )Nz	$py.classi����r   )�lower�endswith�os�path�splitext)r0   r   r   r   �_jython_aware_splitext<   s    r2   c                       s�   e Zd ZdZdZeej�ZdZ	e
jZdZ� fdd�Zdd� Zdd�d	d
�Zd!dd�Zd"dd�Zdd� Zd#dd�Zdd� Zdd� Zdd� Zdd� Zd$dd�Zd%dd �Z�  ZS )&�
TestLoaderz�
    This class is responsible for loading tests according to various criteria
    and returning them wrapped in a TestSuite
    r%   Nc                    s    t t| ���  g | _t� | _d S r	   )r   r3   r   �errors�set�_loading_packagesr   r   r   r   r   M   s    zTestLoader.__init__c                 C   sF   t |tj�rtd��| �|�}|s2t|d�r2dg}| �t||��}|S )z;Return a suite of all test cases contained in testCaseClasszYTest cases should not be derived from TestSuite. Maybe you meant to derive from TestCase?ZrunTest)�
issubclassr   �	TestSuite�	TypeError�getTestCaseNames�hasattrr    �map)r   �testCaseClassZtestCaseNamesZloaded_suiter   r   r   �loadTestsFromTestCaseT   s    
z TestLoader.loadTestsFromTestCase��patternc             
   O   s8  t |�dksd|v r,t�dt� |�dd� t |�dkrRt |�d }td�|���t |�dkrxt|�d }td�|���g }t|�D ]4}t	||�}t
|t�r�t|tj�r�|�| �|�� q�t	|dd�}	| �|�}|	du�r4z|	| ||�W S  t�y2 }
 z0t|j|
| j�\}}| j�|� |W  Y d}
~
S d}
~
0 0 |S )	z>Return a suite of all test cases contained in the given moduler   Zuse_load_testsz(use_load_tests is deprecated and ignoredNr   zCloadTestsFromModule() takes 1 positional argument but {} were givenz=loadTestsFromModule() got an unexpected keyword argument '{}'�
load_tests)�len�warnings�warn�DeprecationWarning�popr9   �format�sorted�dir�getattr�
isinstancer)   r7   r   r*   �appendr>   r    �	Exceptionr#   r   r4   )r   �moduler@   �args�kwsZ	complaint�testsr   �objrA   �e�
error_case�error_messager   r   r   �loadTestsFromModuleb   s8    �



�zTestLoader.loadTestsFromModulec                 C   s`  |� d�}d\}}|du r�|dd� }|r�zd�|�}t|�}W q�W q& ty�   |�� }t|| j�\}}|s�| j�|� | Y S Y q&0 q&|dd� }|}	|D ]�}
z|	t	|	|
� }}	W q� t
�yV } z�t	|	dd�du�r|du�r| j�|� |W  Y d}~  S t|
|| jdt�� f �\}}| j�|� |W  Y d}~  S W Y d}~q�d}~0 0 q�t|	tj��rr| �|	�S t|	t��r�t|	tj��r�| �|	�S t|	tj��r�t|t��r�t|tj��r�|d }||�}tt	||�tj��s| �|g�S nt|	tj��r|	S t|	��rP|	� }t|tj��r$|S t|tj��r>| �|g�S td|	|f ��ntd	|	 ��dS )
aS  Return a suite of all test cases given a string specifier.

        The name may resolve either to a module, a test case class, a
        test method within a test case class, or a callable object which
        returns a TestCase or TestSuite instance.

        The method optionally resolves the names relative to a given module.
        �.)NNNr   �__path__zFailed to access attribute:
%s�����z"calling %s returned %s, not a testz$don't know how to make test from: %s)�split�join�
__import__r   rF   r"   r    r4   rL   rJ   �AttributeErrorr   r   r   rK   �types�
ModuleTyperV   r)   r7   r   r*   r>   �FunctionTyper   r8   �callabler9   )r   r   rN   �partsrT   rU   Z
parts_copy�module_nameZnext_attributerR   �part�parentrS   �instr%   r   r   r   �loadTestsFromName�   sz    	

����,

�
�
�zTestLoader.loadTestsFromNamec                    s   � �fdd�|D �}�� |�S )z�Return a suite of all test cases found using the given sequence
        of string specifiers. See 'loadTestsFromName()'.
        c                    s   g | ]}�� |� ��qS r   )rg   )�.0r   �rN   r   r   r   �
<listcomp>�   �    z1TestLoader.loadTestsFromNames.<locals>.<listcomp>)r    )r   �namesrN   Zsuitesr   ri   r   �loadTestsFromNames�   s    zTestLoader.loadTestsFromNamesc                    s>   � �fdd�}t t|t����}� jr:|jt�� j�d� |S )zLReturn a sorted sequence of method names found within testCaseClass
        c                    sZ   | � �j�sdS t�| �}t|�s&dS d�j�j| f � �jd u pXt� fdd��jD ��S )NFz%s.%s.%sc                 3   s   | ]}t � |�V  qd S r	   )r   )rh   r@   �ZfullNamer   r   �	<genexpr>�   rk   zKTestLoader.getTestCaseNames.<locals>.shouldIncludeMethod.<locals>.<genexpr>)�
startswith�testMethodPrefixrJ   ra   r   r   �testNamePatterns�any)�attrnameZtestFunc�r   r=   rn   r   �shouldIncludeMethod�   s    

�
�z8TestLoader.getTestCaseNames.<locals>.shouldIncludeMethod)�key)�list�filterrI   �sortTestMethodsUsing�sort�	functools�
cmp_to_key)r   r=   rv   ZtestFnNamesr   ru   r   r:   �   s
    zTestLoader.getTestCaseNames�test*.pyc                 C   sD  d}|du r| j dur| j }n|du r.d}|}tj�|�}|tjvrRtj�d|� || _ d}d}g }tj�tj�|��r�tj�|�}||kr�tj�tj�|d�� }�njzt	|� W n t
y�   d}Y �nF0 tj| }|�d�d }	ztj�tj�|j��}W n� t�y�   z
|j}
W n t�y2   d}
Y n0 |
�r�|
jdu �r�|
jdu�r�d}|jD ]P}|�sv|�|��sv�q\|�|j�dtjj��d | _ |�| j||dd�� �q\n*|jtjv �r�td�d�ntd	�|��d�Y n0 |�r|�s| �|	�| _ tj�|� ntj�|� |�r$t
d
| ��|�s:t| �||��}| � |�S )a%  Find and return all test modules from the specified start
        directory, recursing into subdirectories to find them and return all
        tests found within them. Only test files that match the pattern will
        be loaded. (Using shell style pattern matching.)

        All test modules must be importable from the top level of the project.
        If the start directory is not the top level directory then the top
        level directory must be specified separately.

        If a test package name (directory with '__init__.py') matches the
        pattern then the package will be checked for a 'load_tests' function. If
        this exists then it will be called with (loader, tests, pattern) unless
        the package has already had load_tests called from the same discovery
        invocation, in which case the package module object is not scanned for
        tests - this ensures that when a package uses discover to further
        discover child tests that infinite recursion does not happen.

        If load_tests exists then discovery does *not* recurse into the package,
        load_tests is responsible for loading all tests in the package.

        The pattern is deliberately not stored as a loader attribute so that
        packages can continue discovery themselves. top_level_dir is stored so
        load_tests does not need to pass this argument in to loader.discover().

        Paths are sorted before being imported to ensure reproducible execution
        order even on filesystems with non-alphabetical ordering like ext3/4.
        FNTr   �__init__.pyrW   )�	namespacez2Can not use builtin modules as dotted module namesz$don't know how to discover from {!r}z%Start directory is not importable: %r)!�_top_level_dirr/   r0   �abspath�sys�insert�isdir�isfiler[   r\   r   �modulesrZ   �dirname�__file__r]   �__spec__�loader�submodule_search_locationsrX   rp   r   �replace�sep�extend�_find_tests�builtin_module_namesr9   rG   � _get_directory_containing_module�removerx   r    )r   �	start_dirr@   Ztop_level_dirZset_implicit_topZis_not_importable�is_namespacerQ   Z
the_moduleZtop_part�specr0   r   r   r   �discover�   s�    

�


�
���
������zTestLoader.discoverc                 C   sR   t j| }tj�|j�}tj�|��� �d�rBtj�	tj�	|��S tj�	|�S d S )Nr   )
r�   r�   r/   r0   r�   r�   �basenamer-   rp   r�   )r   rc   rN   �	full_pathr   r   r   r�   `  s
    
z+TestLoader._get_directory_containing_modulec                 C   sh   || j krdS ttj�|��}tj�|| j �}tj�|�rBJ d��|�d�rTJ d��|�tjj	d�}|S )NrW   zPath must be within the projectz..)
r�   r2   r/   r0   �normpath�relpath�isabsrp   r�   r�   )r   r0   Z_relpathr   r   r   r   �_get_name_from_pathl  s    
zTestLoader._get_name_from_pathc                 C   s   t |� tj| S r	   )r\   r�   r�   )r   r   r   r   r   �_get_module_from_namex  s    z TestLoader._get_module_from_namec                 C   s
   t ||�S r	   )r   )r   r0   r�   r@   r   r   r   �_match_path|  s    zTestLoader._match_pathFc           
   
   c   s�   | � |�}|dkrD|| jvrD| �|||�\}}|dur<|V  |sDdS tt�|��}|D ]�}tj�||�}	| �|	||�\}}|dur�|V  |rV| � |	�}| j�|� z$| �	|	||�E dH  W | j�
|� qV| j�
|� 0 qVdS )z/Used by discovery. Yields test suites it loads.rW   N)r�   r6   �_find_test_pathrH   r/   �listdirr0   r[   �addr�   �discard)
r   r�   r@   r�   r   rQ   Zshould_recurse�pathsr0   r�   r   r   r   r�   �  s.    
��
zTestLoader._find_testsc              
   C   s�  t j�|�}t j�|��rTt�|�s(dS | �|||�s:dS | �|�}z| �|�}W nf t	j
y� } zt||| j�dfW  Y d}~S d}~0    t|| j�\}}	| j�|	� |df Y S 0 t j�t|d|��}
tt j�|
��}tt j�|��}|�� |�� k�r>t j�|�}tt j�|��}t j�|�}d}t||||f ��| j||d�dfS �n,t j�|��r||�s�t j�t j�|d���s�dS d}d}| �|�}z| �|�}W nh t	j
�y� } zt||| j�dfW  Y d}~S d}~0    t|| j�\}}	| j�|	� |df Y S 0 t|dd�}| j�|� zD| j||d�}|du�rV|dfW | j�|� S |d	fW | j�|� S | j�|� 0 ndS dS )
z�Used by discovery.

        Loads tests from a single file, or a directories' __init__.py when
        passed the directory.

        Returns a tuple (None_or_tests_from_file, should_recurse).
        )NFFNr�   zW%r module incorrectly imported from %r. Expected %r. Is this module globally installed?r?   r   rA   T)r/   r0   r�   r�   �VALID_MODULE_NAME�matchr�   r�   r�   r   ZSkipTestr,   r    r"   r4   rL   r�   rJ   r2   �realpathr-   r�   r   rV   r�   r[   r6   r�   r�   )r   r�   r@   r�   r�   r   rN   rS   rT   rU   Zmod_filer�   Zfullpath_noextZ
module_dir�mod_nameZexpected_dir�msgrA   rQ   �packager   r   r   r�   �  s�    

&
�
�
�
�
���
&
�
��zTestLoader._find_test_path)N)N)r~   N)F)F)r   r   r   �__doc__rq   �staticmethodr   �three_way_cmprz   rr   r   r8   r    r�   r   r>   rV   rg   rm   r:   r�   r�   r�   r�   r�   r�   r�   r   r   r   r   r   r3   B   s&   
(
N

n
"r3   c                 C   s&   t � }||_| |_||_|r"||_|S r	   )r3   rz   rq   rr   r    )�prefix�	sortUsingr    rr   r�   r   r   r   �_makeLoader�  s    r�   c                 C   s   t |||d��| �S )N)rr   )r�   r:   )r=   r�   r�   rr   r   r   r   r:   �  s    r:   r%   c                 C   s   t |||��| �S r	   )r�   r>   )r=   r�   r�   r    r   r   r   �	makeSuite�  s    �r�   c                 C   s   t |||��| �S r	   )r�   rV   )rN   r�   r�   r    r   r   r   �findTestCases  s    �r�   )NN)"r�   r/   �rer�   r   r^   r|   rC   r   r   � r   r   r   Z
__unittest�compile�
IGNORECASEr�   r*   r   r"   r#   r   r,   r2   �objectr3   ZdefaultTestLoaderr�   r�   r:   r8   r�   r�   r   r   r   r   �<module>   s<      /
	�
�                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          page    ,132
        title   strncat - append n chars of string1 to string2
;***
;strncat.asm - append n chars of string to new string
;
;       Copyright (c) Microsoft Corporation. All rights reserved.
;
;Purpose:
;       defines strncat() - appends n characters of string onto
;       end of other string
;
;*******************************************************************************

        .xlist
        include cruntime.inc
        .list

page
;***
;char *strncat(front, back, count) - append count chars of back onto front
;
;Purpose:
;       Appends at most count characters of the string back onto the
;       end of front, and ALWAYS terminates with a null character.
;       If count is greater than the length of back, the length of back
;       is used instead.  (Unlike strncpy, this routine does not pad out
;       to count characters).
;
;       Algorithm:
;       char *
;       strncat (front, back, count)
;           char *front, *back;
;           unsigned count;
;       {
;           char *start = front;
;
;           while (*front++)
;               ;
;           front--;
;           while (count--)
;               if (!(*front++ = *back++))
;                   return(start);
;           *front = '\0';
;           return(start);
;       }
;
;Entry:
;       char *   front - string to append onto
;       char *   back  - string to append
;       unsigned count - count of max characters to append
;
;Exit:
;       returns a pointer to string appended onto (front).
;
;Uses:  ECX, EDX
;
;Exceptions:
;
;*******************************************************************************

    CODESEG

    public  strncat
strncat proc
;   front:ptr byte,
;   back:ptr byte,
;   count:IWORD

        .FPO    ( 0, 3, 0, 0, 0, 0 )

        mov     ecx,[esp + 0ch]     ; ecx = count
        push    edi                 ; preserve edi
        test    ecx,ecx
        jz      finish              ; leave if count is zero

        mov     edi,[esp + 8]       ; edi -> front string
        push    esi                 ; preserve esi
        test    edi,3               ; is string aligned on dword (4 bytes)
        push    ebx                 ; preserve ebx
        je      short find_end_of_front_string_loop

        ; simple byte loop until string is aligned

front_misaligned:
        mov     al,byte ptr [edi]
        add     edi,1
        test    al,al
        je      short start_byte_3
        test    edi,3
        jne     short front_misaligned

find_end_of_front_string_loop:
        mov     eax,dword ptr [edi] ; read dword (4 bytes)
        mov     edx,7efefeffh
        add     edx,eax
        xor     eax,-1
        xor     eax,edx
        add     edi,4
        test    eax,81010100h
        je      short find_end_of_front_string_loop

; found zero byte in the loop
        mov     eax,[edi - 4]
        test    al,al               ; is it byte 0
        je      short start_byte_0
        test    ah,ah               ; is it byte 1
        je      short start_byte_1
        test    eax,00ff0000h       ; is it byte 2
        je      short start_byte_2
        test    eax,0ff000000h      ; is it byte 3
        jne     short find_end_of_front_string_loop
                                    ; taken if bits 24-30 are clear and bit
                                    ; 31 is set
start_byte_3:
        sub     edi,1
        jmp     short copy_start
start_byte_2:
        sub     edi,2
        jmp     short copy_start
start_byte_1:
        sub     edi,3
        jmp     short copy_start
start_byte_0:
        sub     edi,4

; edi now points to the end of front string.

copy_start:
        mov     esi,[esp + 14h]     ; esi -> back string
        test    esi,3               ; is back string is dword aligned?
        jnz     back_misaligned

        mov     ebx,ecx             ; store count for tail loop

        shr     ecx,2
        jnz     short main_loop_entrance
        jmp     short tail_loop_start   ; 0 < counter < 4

; simple byte loop until back string is aligned

back_misaligned:
        mov     dl,byte ptr [esi]
        add     esi,1
        test    dl,dl
        je      short byte_0
        mov     [edi],dl
        add     edi,1
        sub     ecx,1
        jz      empty_counter
        test    esi,3
        jne     short back_misaligned
        mov     ebx,ecx             ; store count for tail loop
        shr     ecx,2               ; convert ecx to dword count
        jnz     short main_loop_entrance

tail_loop_start:
        mov     ecx,ebx
        and     ecx,3               ; ecx = count of leftover bytes after the
                                    ; dwords have been concatenated
        jz      empty_counter

tail_loop:
        mov     dl,byte ptr [esi]
        add     esi,1
        mov     [edi],dl
        add     edi,1
        test    dl,dl
        je      short finish1       ; '\0' was already copied
        sub     ecx,1
        jnz     tail_loop

empty_counter:
        mov     [edi],cl            ; cl=0;
finish1:
        pop     ebx
        pop     esi
finish:
        mov     eax,[esp + 8]       ; return in eax pointer to front string
        pop     edi
        ret                         ; _cdecl return


byte_0:
        mov     [edi],dl
        mov     eax,[esp + 10h]     ; return in eax pointer to front string
        pop     ebx
        pop     esi
        pop     edi
        ret                         ; _cdecl return


main_loop:                          ; edx contains first dword of back string
        mov     [edi],edx           ; store one more dword
        add     edi,4               ; kick pointer to front string

        sub     ecx,1
        jz      tail_loop_start
main_loop_entrance:
        mov     edx,7efefeffh
        mov     eax,dword ptr [esi] ; read 4 bytes

        add     edx,eax
        xor     eax,-1

        xor     eax,edx
        mov     edx,[esi]           ; it's in cache now

        add     esi,4               ; kick pointer to back string
        test    eax,81010100h

        je      short main_loop

; may be found zero byte in the loop
        test    dl,dl               ; is it byte 0
        je      short byte_0
        test    dh,dh               ; is it byte 1
        je      short byte_1
        test    edx,00ff0000h       ; is it byte 2
        je      short byte_2
        test    edx,0ff000000h      ; is it byte 3
        jne short main_loop         ; taken if bits 24-30 are clear and bit
                                    ; 31 is set
byte_3:
        mov     [edi],edx
        mov     eax,[esp + 10h]     ; return in eax pointer to front string
        pop     ebx
        pop     esi
        pop     edi
        ret                         ; _cdecl return

byte_2:
        mov     [edi],dx
        xor     edx,edx
        mov     eax,[esp + 10h]     ; return in eax pointer to front string
        mov     [edi + 2],dl
        pop     ebx
        pop     esi
        pop     edi
        ret                         ; _cdecl return

byte_1:
        mov     [edi],dx
        mov     eax,[esp + 10h]     ; return in eax pointer to front string
        pop     ebx
        pop     esi
        pop     edi
        ret                         ; _cdecl return

strncat endp

        end

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               INDX( 	 YVr           (      �                             ͉   	 x d     ̉    e������ eQj�]�m������e������       �              g l o b i n p u t 