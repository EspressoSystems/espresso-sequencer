(function() {
    var type_impls = Object.fromEntries([["espresso_types",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MerklizedState%3CSeqTypes,+%7B+Self::ARITY+%7D%3E-for-UniversalMerkleTree%3CFeeAmount,+Sha3Digest,+FeeAccount,+FEE_MERKLE_TREE_ARITY,+Sha3Node%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1025-1055\">Source</a><a href=\"#impl-MerklizedState%3CSeqTypes,+%7B+Self::ARITY+%7D%3E-for-UniversalMerkleTree%3CFeeAmount,+Sha3Digest,+FeeAccount,+FEE_MERKLE_TREE_ARITY,+Sha3Node%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl MerklizedState&lt;<a class=\"struct\" href=\"espresso_types/v0/struct.SeqTypes.html\" title=\"struct espresso_types::v0::SeqTypes\">SeqTypes</a>, { Self::ARITY }&gt; for <a class=\"type\" href=\"espresso_types/v0/v0_1/type.FeeMerkleTree.html\" title=\"type espresso_types::v0::v0_1::FeeMerkleTree\">FeeMerkleTree</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Key\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1026\">Source</a><a href=\"#associatedtype.Key\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Key</a> = &lt;UniversalMerkleTree&lt;<a class=\"struct\" href=\"espresso_types/v0/v0_1/struct.FeeAmount.html\" title=\"struct espresso_types::v0::v0_1::FeeAmount\">FeeAmount</a>, Sha3Digest, <a class=\"struct\" href=\"espresso_types/v0/v0_1/struct.FeeAccount.html\" title=\"struct espresso_types::v0::v0_1::FeeAccount\">FeeAccount</a>, FEE_MERKLE_TREE_ARITY, Sha3Node&gt; as MerkleTreeScheme&gt;::Index</h4></section><section id=\"associatedtype.Entry\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1027\">Source</a><a href=\"#associatedtype.Entry\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Entry</a> = &lt;UniversalMerkleTree&lt;<a class=\"struct\" href=\"espresso_types/v0/v0_1/struct.FeeAmount.html\" title=\"struct espresso_types::v0::v0_1::FeeAmount\">FeeAmount</a>, Sha3Digest, <a class=\"struct\" href=\"espresso_types/v0/v0_1/struct.FeeAccount.html\" title=\"struct espresso_types::v0::v0_1::FeeAccount\">FeeAccount</a>, FEE_MERKLE_TREE_ARITY, Sha3Node&gt; as MerkleTreeScheme&gt;::Element</h4></section><section id=\"associatedtype.T\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1028\">Source</a><a href=\"#associatedtype.T\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">T</a> = Sha3Node</h4></section><section id=\"associatedtype.Commit\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1029\">Source</a><a href=\"#associatedtype.Commit\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Commit</a> = &lt;UniversalMerkleTree&lt;<a class=\"struct\" href=\"espresso_types/v0/v0_1/struct.FeeAmount.html\" title=\"struct espresso_types::v0::v0_1::FeeAmount\">FeeAmount</a>, Sha3Digest, <a class=\"struct\" href=\"espresso_types/v0/v0_1/struct.FeeAccount.html\" title=\"struct espresso_types::v0::v0_1::FeeAccount\">FeeAccount</a>, FEE_MERKLE_TREE_ARITY, Sha3Node&gt; as MerkleTreeScheme&gt;::Commitment</h4></section><section id=\"associatedtype.Digest\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1030\">Source</a><a href=\"#associatedtype.Digest\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Digest</a> = Sha3Digest</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.state_type\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1032-1034\">Source</a><a href=\"#method.state_type\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">state_type</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Retrieves the name of the state being queried.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.header_state_commitment_field\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1036-1038\">Source</a><a href=\"#method.header_state_commitment_field\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">header_state_commitment_field</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Retrieves the field in the header containing the Merkle tree commitment\nfor the state implementing this trait.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.tree_height\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1040-1042\">Source</a><a href=\"#method.tree_height\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">tree_height</a>() -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Get the height of the tree</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert_path\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/espresso_types/v0/impls/state.rs.html#1044-1054\">Source</a><a href=\"#method.insert_path\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">insert_path</a>(\n    &amp;mut self,\n    key: Self::Key,\n    proof: &amp;MerkleProof&lt;Self::Entry, Self::Key, Self::T, { Self::ARITY }&gt;,\n) -&gt; <a class=\"type\" href=\"https://docs.rs/anyhow/1.0.97/anyhow/type.Result.html\" title=\"type anyhow::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Insert a forgotten path into the tree.</div></details></div></details>","MerklizedState<SeqTypes, { Self::ARITY }>","espresso_types::v0::v0_1::state::FeeMerkleCommitment"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[6347]}