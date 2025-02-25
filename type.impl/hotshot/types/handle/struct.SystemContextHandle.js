(function() {
    var type_impls = Object.fromEntries([["sequencer",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ConsensusApi%3CTYPES,+I%3E-for-SystemContextHandle%3CTYPES,+I,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hotshot/lib.rs.html#957-958\">Source</a><a href=\"#impl-ConsensusApi%3CTYPES,+I%3E-for-SystemContextHandle%3CTYPES,+I,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;TYPES, I, V&gt; <a class=\"trait\" href=\"hotshot_types/traits/consensus_api/trait.ConsensusApi.html\" title=\"trait hotshot_types::traits::consensus_api::ConsensusApi\">ConsensusApi</a>&lt;TYPES, I&gt; for <a class=\"struct\" href=\"hotshot/types/handle/struct.SystemContextHandle.html\" title=\"struct hotshot::types::handle::SystemContextHandle\">SystemContextHandle</a>&lt;TYPES, I, V&gt;<div class=\"where\">where\n    TYPES: <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>,\n    I: <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeImplementation.html\" title=\"trait hotshot_types::traits::node_implementation::NodeImplementation\">NodeImplementation</a>&lt;TYPES&gt;,\n    V: <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.Versions.html\" title=\"trait hotshot_types::traits::node_implementation::Versions\">Versions</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.total_nodes\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hotshot/lib.rs.html#960\">Source</a><a href=\"#method.total_nodes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"hotshot_types/traits/consensus_api/trait.ConsensusApi.html#tymethod.total_nodes\" class=\"fn\">total_nodes</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/core/num/nonzero/struct.NonZero.html\" title=\"struct core::num::nonzero::NonZero\">NonZero</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Total number of nodes in the network. Also known as <code>n</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.builder_timeout\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hotshot/lib.rs.html#964\">Source</a><a href=\"#method.builder_timeout\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"hotshot_types/traits/consensus_api/trait.ConsensusApi.html#tymethod.builder_timeout\" class=\"fn\">builder_timeout</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a></h4></section></summary><div class='docblock'>The maximum amount of time a leader can wait to get a block from a builder.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.send_event\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hotshot/lib.rs.html#956\">Source</a><a href=\"#method.send_event\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"hotshot_types/traits/consensus_api/trait.ConsensusApi.html#tymethod.send_event\" class=\"fn\">send_event</a>&lt;'life0, 'async_trait&gt;(\n    &amp;'life0 self,\n    event: <a class=\"struct\" href=\"hotshot_types/event/struct.Event.html\" title=\"struct hotshot_types::event::Event\">Event</a>&lt;TYPES&gt;,\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.unit.html\">()</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'async_trait&gt;&gt;<div class=\"where\">where\n    'life0: 'async_trait,\n    <a class=\"struct\" href=\"hotshot/types/handle/struct.SystemContextHandle.html\" title=\"struct hotshot::types::handle::SystemContextHandle\">SystemContextHandle</a>&lt;TYPES, I, V&gt;: 'async_trait,</div></h4></section></summary><div class='docblock'>Notify the system of an event within <code>hotshot-consensus</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.public_key\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hotshot/lib.rs.html#973\">Source</a><a href=\"#method.public_key\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"hotshot_types/traits/consensus_api/trait.ConsensusApi.html#tymethod.public_key\" class=\"fn\">public_key</a>(&amp;self) -&gt; &amp;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.SignatureKey\" title=\"type hotshot_types::traits::node_implementation::NodeType::SignatureKey\">SignatureKey</a></h4></section></summary><div class='docblock'>Get a reference to the public key.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.private_key\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/hotshot/lib.rs.html#977\">Source</a><a href=\"#method.private_key\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"hotshot_types/traits/consensus_api/trait.ConsensusApi.html#tymethod.private_key\" class=\"fn\">private_key</a>(\n    &amp;self,\n) -&gt; &amp;&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.SignatureKey\" title=\"type hotshot_types::traits::node_implementation::NodeType::SignatureKey\">SignatureKey</a> as <a class=\"trait\" href=\"hotshot_types/traits/signature_key/trait.SignatureKey.html\" title=\"trait hotshot_types::traits::signature_key::SignatureKey\">SignatureKey</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/signature_key/trait.SignatureKey.html#associatedtype.PrivateKey\" title=\"type hotshot_types::traits::signature_key::SignatureKey::PrivateKey\">PrivateKey</a></h4></section></summary><div class='docblock'>Get a reference to the private key.</div></details></div></details>","ConsensusApi<TYPES, I>","sequencer::context::Consensus"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SystemContextHandle%3CTYPES,+I,+V%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#77-78\">Source</a><a href=\"#impl-SystemContextHandle%3CTYPES,+I,+V%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;TYPES, I, V&gt; <a class=\"struct\" href=\"hotshot/types/handle/struct.SystemContextHandle.html\" title=\"struct hotshot::types::handle::SystemContextHandle\">SystemContextHandle</a>&lt;TYPES, I, V&gt;<div class=\"where\">where\n    TYPES: <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>,\n    I: <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeImplementation.html\" title=\"trait hotshot_types::traits::node_implementation::NodeImplementation\">NodeImplementation</a>&lt;TYPES&gt; + 'static,\n    V: <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.Versions.html\" title=\"trait hotshot_types::traits::node_implementation::Versions\">Versions</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_task\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#81\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.add_task\" class=\"fn\">add_task</a>&lt;S&gt;(&amp;mut self, task_state: S)<div class=\"where\">where\n    S: <a class=\"trait\" href=\"hotshot_task/task/trait.TaskState.html\" title=\"trait hotshot_task::task::TaskState\">TaskState</a>&lt;Event = <a class=\"enum\" href=\"hotshot_task_impls/events/enum.HotShotEvent.html\" title=\"enum hotshot_task_impls::events::HotShotEvent\">HotShotEvent</a>&lt;TYPES&gt;&gt; + 'static,</div></h4></section></summary><div class=\"docblock\"><p>Adds a hotshot consensus-related task to the <code>SystemContextHandle</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event_stream\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#92\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.event_stream\" class=\"fn\">event_stream</a>(&amp;self) -&gt; impl Stream&lt;Item = <a class=\"struct\" href=\"hotshot_types/event/struct.Event.html\" title=\"struct hotshot_types::event::Event\">Event</a>&lt;TYPES&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>obtains a stream to expose to the user</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.send_external_message\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#102-106\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.send_external_message\" class=\"fn\">send_external_message</a>(\n    &amp;self,\n    msg: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt;,\n    recipients: <a class=\"enum\" href=\"hotshot_types/message/enum.RecipientList.html\" title=\"enum hotshot_types::message::RecipientList\">RecipientList</a>&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.SignatureKey\" title=\"type hotshot_types::traits::node_implementation::NodeType::SignatureKey\">SignatureKey</a>&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://docs.rs/anyhow/1.0.96/anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Message other participants with a serialized message from the application\nReceivers of this message will get an <code>Event::ExternalMessageReceived</code> via\nthe event stream.</p>\n<h5 id=\"errors\"><a class=\"doc-anchor\" href=\"#errors\">§</a>Errors</h5>\n<p>Errors if serializing the request fails, or the request fails to be sent</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.request_proposal\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#139-143\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.request_proposal\" class=\"fn\">request_proposal</a>(\n    &amp;self,\n    view: &lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.View\" title=\"type hotshot_types::traits::node_implementation::NodeType::View\">View</a>,\n    leaf_commitment: Commitment&lt;<a class=\"struct\" href=\"hotshot_types/data/struct.Leaf2.html\" title=\"struct hotshot_types::data::Leaf2\">Leaf2</a>&lt;TYPES&gt;&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"hotshot_types/message/struct.Proposal.html\" title=\"struct hotshot_types::message::Proposal\">Proposal</a>&lt;TYPES, <a class=\"struct\" href=\"hotshot_types/data/struct.QuorumProposalWrapper.html\" title=\"struct hotshot_types::data::QuorumProposalWrapper\">QuorumProposalWrapper</a>&lt;TYPES&gt;&gt;, <a class=\"struct\" href=\"https://docs.rs/anyhow/1.0.96/anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>&gt;&gt;, <a class=\"struct\" href=\"https://docs.rs/anyhow/1.0.96/anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Request a proposal from the all other nodes.  Will block until some node\nreturns a valid proposal with the requested commitment.  If nobody has the\nproposal this will block forever</p>\n<h5 id=\"errors-1\"><a class=\"doc-anchor\" href=\"#errors-1\">§</a>Errors</h5>\n<p>Errors if signing the request for proposal fails</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.event_stream_known_impl\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#212\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.event_stream_known_impl\" class=\"fn\">event_stream_known_impl</a>(&amp;self) -&gt; Receiver&lt;<a class=\"struct\" href=\"hotshot_types/event/struct.Event.html\" title=\"struct hotshot_types::event::Event\">Event</a>&lt;TYPES&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>HACK so we can know the types when running tests…\nthere are two cleaner solutions:</p>\n<ul>\n<li>make the stream generic and in nodetypes or nodeimpelmentation</li>\n<li>type wrapper</li>\n</ul>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.internal_event_stream_sender\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#218\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.internal_event_stream_sender\" class=\"fn\">internal_event_stream_sender</a>(&amp;self) -&gt; Sender&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;<a class=\"enum\" href=\"hotshot_task_impls/events/enum.HotShotEvent.html\" title=\"enum hotshot_task_impls::events::HotShotEvent\">HotShotEvent</a>&lt;TYPES&gt;&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>HACK so we can create dependency tasks when running tests</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.internal_event_stream_receiver_known_impl\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#229\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.internal_event_stream_receiver_known_impl\" class=\"fn\">internal_event_stream_receiver_known_impl</a>(\n    &amp;self,\n) -&gt; Receiver&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;<a class=\"enum\" href=\"hotshot_task_impls/events/enum.HotShotEvent.html\" title=\"enum hotshot_task_impls::events::HotShotEvent\">HotShotEvent</a>&lt;TYPES&gt;&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>HACK so we can know the types when running tests…\nthere are two cleaner solutions:</p>\n<ul>\n<li>make the stream generic and in nodetypes or nodeimpelmentation</li>\n<li>type wrapper</li>\n</ul>\n<p>NOTE: this is only used for sanity checks in our tests</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decided_state\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#237\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.decided_state\" class=\"fn\">decided_state</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.ValidatedState\" title=\"type hotshot_types::traits::node_implementation::NodeType::ValidatedState\">ValidatedState</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the last decided validated state of the <a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a> instance.</p>\n<h5 id=\"panics\"><a class=\"doc-anchor\" href=\"#panics\">§</a>Panics</h5>\n<p>If the internal consensus is in an inconsistent state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.state\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#248\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.state\" class=\"fn\">state</a>(\n    &amp;self,\n    view: &lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.View\" title=\"type hotshot_types::traits::node_implementation::NodeType::View\">View</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.ValidatedState\" title=\"type hotshot_types::traits::node_implementation::NodeType::ValidatedState\">ValidatedState</a>&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the validated state from a given <code>view</code>.</p>\n<p>Returns the requested state, if the <a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a> is tracking this view. Consensus\ntracks views that have not yet been decided but could be in the future. This function may\nreturn <a href=\"https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html#variant.None\" title=\"variant core::option::Option::None\"><code>None</code></a> if the requested view has already been decided (but see\n<a href=\"hotshot/types/handle/struct.SystemContextHandle.html#method.decided_state\" title=\"method hotshot::types::handle::SystemContextHandle::decided_state\"><code>decided_state</code></a>) or if there is no path for the requested\nview to ever be decided.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decided_leaf\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#256\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.decided_leaf\" class=\"fn\">decided_leaf</a>(&amp;self) -&gt; <a class=\"struct\" href=\"hotshot_types/data/struct.Leaf2.html\" title=\"struct hotshot_types::data::Leaf2\">Leaf2</a>&lt;TYPES&gt;</h4></section></summary><div class=\"docblock\"><p>Get the last decided leaf of the <a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a> instance.</p>\n<h5 id=\"panics-1\"><a class=\"doc-anchor\" href=\"#panics-1\">§</a>Panics</h5>\n<p>If the internal consensus is in an inconsistent state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_decided_leaf\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#266\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.try_decided_leaf\" class=\"fn\">try_decided_leaf</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"hotshot_types/data/struct.Leaf2.html\" title=\"struct hotshot_types::data::Leaf2\">Leaf2</a>&lt;TYPES&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Tries to get the most recent decided leaf, returning instantly\nif we can’t acquire the lock.</p>\n<h5 id=\"panics-2\"><a class=\"doc-anchor\" href=\"#panics-2\">§</a>Panics</h5>\n<p>Panics if internal consensus is in an inconsistent state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.submit_transaction\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#278-281\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.submit_transaction\" class=\"fn\">submit_transaction</a>(\n    &amp;self,\n    tx: &lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.Transaction\" title=\"type hotshot_types::traits::node_implementation::NodeType::Transaction\">Transaction</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"hotshot_types/error/enum.HotShotError.html\" title=\"enum hotshot_types::error::HotShotError\">HotShotError</a>&lt;TYPES&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Submits a transaction to the backing <a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a> instance.</p>\n<p>The current node broadcasts the transaction to all nodes on the network.</p>\n<h5 id=\"errors-2\"><a class=\"doc-anchor\" href=\"#errors-2\">§</a>Errors</h5>\n<p>Will return a <a href=\"hotshot_types/error/enum.HotShotError.html\" title=\"enum hotshot_types::error::HotShotError\"><code>HotShotError</code></a> if some error occurs in the underlying\n<a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a> instance.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#287\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.consensus\" class=\"fn\">consensus</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;RwLock&lt;<a class=\"struct\" href=\"hotshot_types/consensus/struct.Consensus.html\" title=\"struct hotshot_types::consensus::Consensus\">Consensus</a>&lt;TYPES&gt;&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the underlying consensus state for this <a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.shut_down\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#292\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.shut_down\" class=\"fn\">shut_down</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Shut down the inner hotshot and wait until all background threads are closed.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.next_view_timeout\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#315\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.next_view_timeout\" class=\"fn\">next_view_timeout</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u64.html\">u64</a></h4></section></summary><div class=\"docblock\"><p>return the timeout for a view of the underlying <code>SystemContext</code></p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.leader\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#324-328\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.leader\" class=\"fn\">leader</a>(\n    &amp;self,\n    view_number: &lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.View\" title=\"type hotshot_types::traits::node_implementation::NodeType::View\">View</a>,\n    epoch_number: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.Epoch\" title=\"type hotshot_types::traits::node_implementation::NodeType::Epoch\">Epoch</a>&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.SignatureKey\" title=\"type hotshot_types::traits::node_implementation::NodeType::SignatureKey\">SignatureKey</a>, <a class=\"struct\" href=\"https://docs.rs/anyhow/1.0.96/anyhow/struct.Error.html\" title=\"struct anyhow::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Wrapper for <code>HotShotConsensusApi</code>’s <code>leader</code> function</p>\n<h5 id=\"errors-3\"><a class=\"doc-anchor\" href=\"#errors-3\">§</a>Errors</h5>\n<p>Returns an error if the leader cannot be calculated</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.public_key\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#341\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.public_key\" class=\"fn\">public_key</a>(&amp;self) -&gt; &lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.SignatureKey\" title=\"type hotshot_types::traits::node_implementation::NodeType::SignatureKey\">SignatureKey</a></h4></section></summary><div class=\"docblock\"><p>Wrapper to get this node’s public key</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.external_channel_sender\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#348\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.external_channel_sender\" class=\"fn\">external_channel_sender</a>(&amp;self) -&gt; Sender&lt;<a class=\"struct\" href=\"hotshot_types/event/struct.Event.html\" title=\"struct hotshot_types::event::Event\">Event</a>&lt;TYPES&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the sender side of the external event stream for testing purpose</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.internal_channel_sender\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#355\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.internal_channel_sender\" class=\"fn\">internal_channel_sender</a>(&amp;self) -&gt; Sender&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;<a class=\"enum\" href=\"hotshot_task_impls/events/enum.HotShotEvent.html\" title=\"enum hotshot_task_impls::events::HotShotEvent\">HotShotEvent</a>&lt;TYPES&gt;&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Get the sender side of the internal event stream for testing purpose</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.cur_view\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#360\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.cur_view\" class=\"fn\">cur_view</a>(&amp;self) -&gt; &lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.View\" title=\"type hotshot_types::traits::node_implementation::NodeType::View\">View</a></h4></section></summary><div class=\"docblock\"><p>Wrapper to get the view number this node is on.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.cur_epoch\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#366\">Source</a><h4 class=\"code-header\">pub async fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.cur_epoch\" class=\"fn\">cur_epoch</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&lt;TYPES as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html\" title=\"trait hotshot_types::traits::node_implementation::NodeType\">NodeType</a>&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeType.html#associatedtype.Epoch\" title=\"type hotshot_types::traits::node_implementation::NodeType::Epoch\">Epoch</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Wrapper to get the epoch number this node is on.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.storage\" class=\"method\"><a class=\"src rightside\" href=\"src/hotshot/types/handle.rs.html#374\">Source</a><h4 class=\"code-header\">pub fn <a href=\"hotshot/types/handle/struct.SystemContextHandle.html#tymethod.storage\" class=\"fn\">storage</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;RwLock&lt;&lt;I as <a class=\"trait\" href=\"hotshot_types/traits/node_implementation/trait.NodeImplementation.html\" title=\"trait hotshot_types::traits::node_implementation::NodeImplementation\">NodeImplementation</a>&lt;TYPES&gt;&gt;::<a class=\"associatedtype\" href=\"hotshot_types/traits/node_implementation/trait.NodeImplementation.html#associatedtype.Storage\" title=\"type hotshot_types::traits::node_implementation::NodeImplementation::Storage\">Storage</a>&gt;&gt;</h4></section></summary><div class=\"docblock\"><p>Provides a reference to the underlying storage for this <a href=\"hotshot/struct.SystemContext.html\" title=\"struct hotshot::SystemContext\"><code>SystemContext</code></a>, allowing access to\nhistorical data</p>\n</div></details></div></details>",0,"sequencer::context::Consensus"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[34153]}