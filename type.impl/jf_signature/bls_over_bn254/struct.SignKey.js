(function() {
    var type_impls = Object.fromEntries([["hotshot",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-SignKey\" class=\"impl\"><a href=\"#impl-Clone-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; SignKey</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-SignKey\" class=\"impl\"><a href=\"#impl-Debug-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-SignKey\" class=\"impl\"><a href=\"#impl-Default-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; SignKey</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-SignKey\" class=\"impl\"><a href=\"#impl-Hash-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut __H</a>)<div class=\"where\">where\n    __H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/hash/mod.rs.html#235-237\">Source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;[Self], state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-SignKey\" class=\"impl\"><a href=\"#impl-PartialEq-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;SignKey) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/cmp.rs.html#261\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PrivateSignatureKey-for-SignKey\" class=\"impl\"><a href=\"#impl-PrivateSignatureKey-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl PrivateSignatureKey for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_bytes\" class=\"method trait-impl\"><a href=\"#method.to_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">to_bytes</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u8.html\">u8</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Vec&lt;u8&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Serialize the private key into bytes</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_bytes\" class=\"method trait-impl\"><a href=\"#method.from_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">from_bytes</a>(bytes: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;SignKey, <a class=\"struct\" href=\"https://docs.rs/anyhow/1.0.95/anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Deserialize the private key from bytes <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_tagged_base64\" class=\"method trait-impl\"><a href=\"#method.to_tagged_base64\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">to_tagged_base64</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;TaggedBase64, Tb64Error&gt;</h4></section></summary><div class='docblock'>Serialize the private key into TaggedBase64 blob. <a>Read more</a></div></details></div></details>","PrivateSignatureKey","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SignKey\" class=\"impl\"><a href=\"#impl-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.generate\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">generate</a>&lt;R&gt;(prng: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut R</a>) -&gt; SignKey<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html\" title=\"trait rand_core::CryptoRng\">CryptoRng</a> + <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>,</div></h4></section></summary><div class=\"docblock\"><p>Signature Key generation function</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_bytes\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_bytes</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u8.html\">u8</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Vec&lt;u8&gt;\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Explicit calls to serialize a <code>SignKey</code> into bytes.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_bytes\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">from_bytes</a>(bytes: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u8.html\">u8</a>]) -&gt; SignKey</h4></section></summary><div class=\"docblock\"><p>Deserialize <code>SignKey</code> from bytes.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_tagged_base64\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_tagged_base64</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;TaggedBase64, Tb64Error&gt;</h4></section></summary><div class=\"docblock\"><p>Explicit calls to serialize a <code>SignKey</code> into <code>TaggedBase64</code>.</p>\n</div></details></div></details>",0,"hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26TaggedBase64%3E-for-SignKey\" class=\"impl\"><a href=\"#impl-TryFrom%3C%26TaggedBase64%3E-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a TaggedBase64&gt; for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Tb64Error</h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(\n    tb: &amp;'a TaggedBase64,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;SignKey, &lt;SignKey as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a TaggedBase64&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" title=\"type core::convert::TryFrom::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a TaggedBase64>","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3CTaggedBase64%3E-for-SignKey\" class=\"impl\"><a href=\"#impl-TryFrom%3CTaggedBase64%3E-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;TaggedBase64&gt; for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Tb64Error</h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(\n    tb: TaggedBase64,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;SignKey, &lt;SignKey as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;TaggedBase64&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" title=\"type core::convert::TryFrom::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<TaggedBase64>","hotshot::types::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Zeroize-for-SignKey\" class=\"impl\"><a href=\"#impl-Zeroize-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Zeroize for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.zeroize\" class=\"method trait-impl\"><a href=\"#method.zeroize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">zeroize</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Zero out this object from memory using Rust intrinsics which ensure the\nzeroization operation is not “optimized away” by the compiler.</div></details></div></details>","Zeroize","hotshot::types::BLSPrivKey"],["<section id=\"impl-Eq-for-SignKey\" class=\"impl\"><a href=\"#impl-Eq-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for SignKey</h3></section>","Eq","hotshot::types::BLSPrivKey"],["<section id=\"impl-StructuralPartialEq-for-SignKey\" class=\"impl\"><a href=\"#impl-StructuralPartialEq-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for SignKey</h3></section>","StructuralPartialEq","hotshot::types::BLSPrivKey"]]],["hotshot_types",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-SignKey\" class=\"impl\"><a href=\"#impl-Clone-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; SignKey</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-SignKey\" class=\"impl\"><a href=\"#impl-Debug-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-SignKey\" class=\"impl\"><a href=\"#impl-Default-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; SignKey</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-SignKey\" class=\"impl\"><a href=\"#impl-Hash-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut __H</a>)<div class=\"where\">where\n    __H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/hash/mod.rs.html#235-237\">Source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;[Self], state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-SignKey\" class=\"impl\"><a href=\"#impl-PartialEq-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;SignKey) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/cmp.rs.html#261\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SignKey\" class=\"impl\"><a href=\"#impl-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.generate\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">generate</a>&lt;R&gt;(prng: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut R</a>) -&gt; SignKey<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html\" title=\"trait rand_core::CryptoRng\">CryptoRng</a> + <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a>,</div></h4></section></summary><div class=\"docblock\"><p>Signature Key generation function</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_bytes\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_bytes</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u8.html\">u8</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Vec&lt;u8&gt;\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Explicit calls to serialize a <code>SignKey</code> into bytes.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_bytes\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">from_bytes</a>(bytes: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u8.html\">u8</a>]) -&gt; SignKey</h4></section></summary><div class=\"docblock\"><p>Deserialize <code>SignKey</code> from bytes.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_tagged_base64\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_tagged_base64</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;TaggedBase64, Tb64Error&gt;</h4></section></summary><div class=\"docblock\"><p>Explicit calls to serialize a <code>SignKey</code> into <code>TaggedBase64</code>.</p>\n</div></details></div></details>",0,"hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26TaggedBase64%3E-for-SignKey\" class=\"impl\"><a href=\"#impl-TryFrom%3C%26TaggedBase64%3E-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a TaggedBase64&gt; for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Tb64Error</h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(\n    tb: &amp;'a TaggedBase64,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;SignKey, &lt;SignKey as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'a TaggedBase64&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" title=\"type core::convert::TryFrom::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&'a TaggedBase64>","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3CTaggedBase64%3E-for-SignKey\" class=\"impl\"><a href=\"#impl-TryFrom%3CTaggedBase64%3E-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;TaggedBase64&gt; for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Tb64Error</h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(\n    tb: TaggedBase64,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;SignKey, &lt;SignKey as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;TaggedBase64&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.TryFrom.html#associatedtype.Error\" title=\"type core::convert::TryFrom::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<TaggedBase64>","hotshot_types::signature_key::BLSPrivKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Zeroize-for-SignKey\" class=\"impl\"><a href=\"#impl-Zeroize-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Zeroize for SignKey</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.zeroize\" class=\"method trait-impl\"><a href=\"#method.zeroize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">zeroize</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Zero out this object from memory using Rust intrinsics which ensure the\nzeroization operation is not “optimized away” by the compiler.</div></details></div></details>","Zeroize","hotshot_types::signature_key::BLSPrivKey"],["<section id=\"impl-Eq-for-SignKey\" class=\"impl\"><a href=\"#impl-Eq-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for SignKey</h3></section>","Eq","hotshot_types::signature_key::BLSPrivKey"],["<section id=\"impl-StructuralPartialEq-for-SignKey\" class=\"impl\"><a href=\"#impl-StructuralPartialEq-for-SignKey\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for SignKey</h3></section>","StructuralPartialEq","hotshot_types::signature_key::BLSPrivKey"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[19400,17233]}