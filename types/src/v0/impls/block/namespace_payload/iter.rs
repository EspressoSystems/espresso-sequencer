use crate::{Index, Iter, NsIndex, NsIter, Payload, TxIndex};

impl Index {
    pub fn ns(&self) -> &NsIndex {
        &self.ns_index
    }
    pub fn tx(&self) -> &TxIndex {
        &self.tx_index
    }
}

// TODO don't impl `PartialOrd`
// It's needed only for `QueryablePayload` trait:
// https://github.com/EspressoSystems/hotshot-query-service/issues/639
impl PartialOrd for Index {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(_other))
    }
}
// TODO don't impl `Ord`
// It's needed only for `QueryablePayload` trait:
// https://github.com/EspressoSystems/hotshot-query-service/issues/639
impl Ord for Index {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        unimplemented!()
    }
}

impl<'a> Iter<'a> {
    pub fn new(block: &'a Payload) -> Self {
        Self {
            ns_iter: NsIter::new(&block.ns_table().len()).peekable(),
            tx_iter: None,
            block,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(ns_index) = self.ns_iter.peek() else {
                break None; // ns_iter consumed
            };

            if let Some(tx_index) = self
                .tx_iter
                .get_or_insert_with(|| self.block.ns_payload(ns_index).iter())
                .next()
            {
                break Some(Index {
                    ns_index: ns_index.clone(),
                    tx_index,
                });
            }

            self.tx_iter = None; // unset `tx_iter`; it's consumed for this namespace
            self.ns_iter.next();
        }
    }
}
