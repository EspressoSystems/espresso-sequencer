(function() {
    var type_impls = Object.fromEntries([["hotshot_state_prover",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CanonicalDeserialize-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-CanonicalDeserialize-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; CanonicalDeserialize for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize_with_mode\" class=\"method trait-impl\"><a href=\"#method.deserialize_with_mode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">deserialize_with_mode</a>&lt;R&gt;(\n    reader: R,\n    compress: Compress,\n    validate: Validate,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;VerKey&lt;P&gt;, SerializationError&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div></h4></section></summary><div class='docblock'>The general deserialize method that takes in customization flags.</div></details><section id=\"method.deserialize_compressed\" class=\"method trait-impl\"><a href=\"#method.deserialize_compressed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">deserialize_compressed</a>&lt;R&gt;(reader: R) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, SerializationError&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div></h4></section><section id=\"method.deserialize_compressed_unchecked\" class=\"method trait-impl\"><a href=\"#method.deserialize_compressed_unchecked\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">deserialize_compressed_unchecked</a>&lt;R&gt;(\n    reader: R,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, SerializationError&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div></h4></section><section id=\"method.deserialize_uncompressed\" class=\"method trait-impl\"><a href=\"#method.deserialize_uncompressed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">deserialize_uncompressed</a>&lt;R&gt;(reader: R) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, SerializationError&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div></h4></section><section id=\"method.deserialize_uncompressed_unchecked\" class=\"method trait-impl\"><a href=\"#method.deserialize_uncompressed_unchecked\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">deserialize_uncompressed_unchecked</a>&lt;R&gt;(\n    reader: R,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, SerializationError&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,</div></h4></section></div></details>","CanonicalDeserialize","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CanonicalSerialize-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-CanonicalSerialize-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; CanonicalSerialize for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize_with_mode\" class=\"method trait-impl\"><a href=\"#method.serialize_with_mode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">serialize_with_mode</a>&lt;W&gt;(\n    &amp;self,\n    writer: W,\n    compress: Compress,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, SerializationError&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div></h4></section></summary><div class='docblock'>The general serialize method that takes in customization flags.</div></details><section id=\"method.serialized_size\" class=\"method trait-impl\"><a href=\"#method.serialized_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">serialized_size</a>(&amp;self, compress: Compress) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a></h4></section><section id=\"method.serialize_compressed\" class=\"method trait-impl\"><a href=\"#method.serialize_compressed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">serialize_compressed</a>&lt;W&gt;(&amp;self, writer: W) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, SerializationError&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div></h4></section><section id=\"method.compressed_size\" class=\"method trait-impl\"><a href=\"#method.compressed_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">compressed_size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a></h4></section><section id=\"method.serialize_uncompressed\" class=\"method trait-impl\"><a href=\"#method.serialize_uncompressed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">serialize_uncompressed</a>&lt;W&gt;(&amp;self, writer: W) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, SerializationError&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div></h4></section><section id=\"method.uncompressed_size\" class=\"method trait-impl\"><a href=\"#method.uncompressed_size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">uncompressed_size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a></h4></section></div></details>","CanonicalSerialize","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Clone-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; VerKey&lt;P&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.82.0/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Debug-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, __f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Default-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; VerKey&lt;P&gt;</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.82.0/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Deserialize%3C'de%3E-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de, P&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.214/serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(\n    __deserializer: __D,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;VerKey&lt;P&gt;, &lt;__D as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.214/serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"https://docs.rs/serde/1.0.214/serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Display-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Display-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Display.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Display.html#tymethod.fmt\">Read more</a></div></details></div></details>","Display","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26SignKey%3CF%3E%3E-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-From%3C%26SignKey%3CF%3E%3E-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;SignKey&lt;F&gt;&gt; for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig&lt;ScalarField = F&gt;,\n    F: PrimeField,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(sk: &amp;SignKey&lt;F&gt;) -&gt; VerKey&lt;P&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&SignKey<F>>","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CAffine%3CP%3E%3E-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-From%3CAffine%3CP%3E%3E-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Affine&lt;P&gt;&gt; for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(point: Affine&lt;P&gt;) -&gt; VerKey&lt;P&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Affine<P>>","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FromStr-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-FromStr-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/str/traits/trait.FromStr.html\" title=\"trait core::str::traits::FromStr\">FromStr</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Err\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Err\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.82.0/core/str/traits/trait.FromStr.html#associatedtype.Err\" class=\"associatedtype\">Err</a> = Tb64Error</h4></section></summary><div class='docblock'>The associated error which can be returned from parsing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_str\" class=\"method trait-impl\"><a href=\"#method.from_str\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/str/traits/trait.FromStr.html#tymethod.from_str\" class=\"fn\">from_str</a>(s: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.str.html\">str</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;VerKey&lt;P&gt;, &lt;VerKey&lt;P&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/str/traits/trait.FromStr.html\" title=\"trait core::str::traits::FromStr\">FromStr</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.82.0/core/str/traits/trait.FromStr.html#associatedtype.Err\" title=\"type core::str::traits::FromStr::Err\">Err</a>&gt;</h4></section></summary><div class='docblock'>Parses a string <code>s</code> to return a value of this type. <a href=\"https://doc.rust-lang.org/1.82.0/core/str/traits/trait.FromStr.html#tymethod.from_str\">Read more</a></div></details></div></details>","FromStr","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Hash-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;H&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.82.0/src/core/hash/mod.rs.html#235-237\">source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;[Self], state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.82.0/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;VerKey&lt;P&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.82.0/src/core/cmp.rs.html#261\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Serialize-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(\n    &amp;self,\n    __serializer: __S,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&lt;__S as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, &lt;__S as <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>&gt;::<a class=\"associatedtype\" href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __S: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"https://docs.rs/serde/1.0.214/serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Tagged-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Tagged-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; Tagged for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.tag\" class=\"method trait-impl\"><a href=\"#method.tag\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">tag</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a></h4></section></div></details>","Tagged","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ToFields%3CFp%3CMontBackend%3CFrConfig,+4%3E,+4%3E%3E-for-VerKey%3CEdwardsConfig%3E\" class=\"impl\"><a href=\"#impl-ToFields%3CFp%3CMontBackend%3CFrConfig,+4%3E,+4%3E%3E-for-VerKey%3CEdwardsConfig%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl ToFields&lt;Fp&lt;MontBackend&lt;FrConfig, 4&gt;, 4&gt;&gt; for VerKey&lt;EdwardsConfig&gt;</h3></section></summary><div class=\"docblock\"><p>Hashable representation of a key\nNOTE: commitment is only used in light client contract.\nFor this application, we needs only hash the Schnorr verification key.</p>\n</div><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.SIZE\" class=\"associatedconstant trait-impl\"><a href=\"#associatedconstant.SIZE\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">SIZE</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.usize.html\">usize</a> = 2usize</h4></section></summary><div class='docblock'>The number of field elements needed to represent the given struct.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_fields\" class=\"method trait-impl\"><a href=\"#method.to_fields\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">to_fields</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;Fp&lt;MontBackend&lt;FrConfig, 4&gt;, 4&gt;&gt;</h4></section></summary><div class='docblock'>Convert the given struct into a list of field elements.</div></details></div></details>","ToFields<Fp<MontBackend<FrConfig, 4>, 4>>","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3C%26TaggedBase64%3E-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-TryFrom%3C%26TaggedBase64%3E-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;TaggedBase64&gt; for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Tb64Error</h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(\n    t: &amp;TaggedBase64,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;VerKey&lt;P&gt;, &lt;VerKey&lt;P&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;TaggedBase64&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html#associatedtype.Error\" title=\"type core::convert::TryFrom::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<&TaggedBase64>","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TryFrom%3CTaggedBase64%3E-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-TryFrom%3CTaggedBase64%3E-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;TaggedBase64&gt; for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = Tb64Error</h4></section></summary><div class='docblock'>The type returned in the event of a conversion error.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_from\" class=\"method trait-impl\"><a href=\"#method.try_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html#tymethod.try_from\" class=\"fn\">try_from</a>(\n    t: TaggedBase64,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;VerKey&lt;P&gt;, &lt;VerKey&lt;P&gt; as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;TaggedBase64&gt;&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.TryFrom.html#associatedtype.Error\" title=\"type core::convert::TryFrom::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Performs the conversion.</div></details></div></details>","TryFrom<TaggedBase64>","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Valid-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Valid-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; Valid for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.check\" class=\"method trait-impl\"><a href=\"#method.check\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">check</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, SerializationError&gt;</h4></section><section id=\"method.batch_check\" class=\"method trait-impl\"><a href=\"#method.batch_check\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">batch_check</a>&lt;'a&gt;(\n    batch: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = &amp;'a VerKey&lt;P&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, SerializationError&gt;<div class=\"where\">where\n    VerKey&lt;P&gt;: 'a,</div></h4></section></div></details>","Valid","hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F, P&gt; VerKey&lt;P&gt;<div class=\"where\">where\n    F: RescueParameter,\n    P: TECurveConfig&lt;BaseField = F&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.internal\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">internal</a>(&amp;self) -&gt; &amp;Projective&lt;P&gt;</h4></section></summary><div class=\"docblock\"><p>Get the internal of verifying key, namely a curve Point</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.verify\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">verify</a>&lt;B&gt;(\n    &amp;self,\n    msg: &amp;[&lt;P as CurveConfig&gt;::BaseField],\n    sig: &amp;Signature&lt;P&gt;,\n    csid: B,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.unit.html\">()</a>, SignatureError&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u8.html\">u8</a>]&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Signature verification function</p>\n</div></details></div></details>",0,"hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.randomize_with\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">randomize_with</a>&lt;F&gt;(&amp;self, randomizer: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.reference.html\">&amp;F</a>) -&gt; VerKey&lt;P&gt;<div class=\"where\">where\n    F: PrimeField,\n    P: TECurveConfig&lt;ScalarField = F&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Return a randomized verification key.</p>\n</div></details></div></details>",0,"hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.to_affine\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">to_affine</a>(&amp;self) -&gt; Affine&lt;P&gt;</h4></section></summary><div class=\"docblock\"><p>Convert the verification key into the affine form.</p>\n</div></details></div></details>",0,"hotshot_state_prover::mock_ledger::SchnorrVerKey"],["<section id=\"impl-Eq-for-VerKey%3CP%3E\" class=\"impl\"><a href=\"#impl-Eq-for-VerKey%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for VerKey&lt;P&gt;<div class=\"where\">where\n    P: TECurveConfig,</div></h3></section>","Eq","hotshot_state_prover::mock_ledger::SchnorrVerKey"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[39605]}