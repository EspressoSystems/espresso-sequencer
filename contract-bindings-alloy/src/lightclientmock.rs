///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    type BaseField is uint256;
    type ScalarField is uint256;
    struct G1Point { BaseField x; BaseField y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BaseField(alloy::sol_types::private::primitives::aliases::U256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BaseField>
            for alloy::sol_types::private::primitives::aliases::U256
        {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl BaseField {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::primitives::aliases::U256) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::primitives::aliases::U256 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BaseField {
            type RustType = alloy::sol_types::private::primitives::aliases::U256;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BaseField {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScalarField(alloy::sol_types::private::primitives::aliases::U256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ScalarField>
            for alloy::sol_types::private::primitives::aliases::U256
        {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl ScalarField {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::primitives::aliases::U256) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::primitives::aliases::U256 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ScalarField {
            type RustType = alloy::sol_types::private::primitives::aliases::U256;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ScalarField {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct G1Point { BaseField x; BaseField y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        pub x: <BaseField as alloy::sol_types::SolType>::RustType,
        pub y: <BaseField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (BaseField, BaseField);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BaseField as alloy::sol_types::SolType>::RustType,
            <BaseField as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.x, value.y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    x: tuple.0,
                    y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BaseField as alloy_sol_types::SolType>::tokenize(&self.x),
                    <BaseField as alloy_sol_types::SolType>::tokenize(&self.y),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for G1Point {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 x,uint256 y)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BaseField as alloy_sol_types::SolType>::eip712_data_word(&self.x).0,
                    <BaseField as alloy_sol_types::SolType>::eip712_data_word(&self.y).0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BaseField as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.x)
                    + <BaseField as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <BaseField as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.x, out);
                <BaseField as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.y, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

    See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BN254Instance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BN254Instance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BN254Instance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IPlonkVerifier {
    struct PlonkProof { BN254.G1Point wire0; BN254.G1Point wire1; BN254.G1Point wire2; BN254.G1Point wire3; BN254.G1Point wire4; BN254.G1Point prodPerm; BN254.G1Point split0; BN254.G1Point split1; BN254.G1Point split2; BN254.G1Point split3; BN254.G1Point split4; BN254.G1Point zeta; BN254.G1Point zetaOmega; BN254.ScalarField wireEval0; BN254.ScalarField wireEval1; BN254.ScalarField wireEval2; BN254.ScalarField wireEval3; BN254.ScalarField wireEval4; BN254.ScalarField sigmaEval0; BN254.ScalarField sigmaEval1; BN254.ScalarField sigmaEval2; BN254.ScalarField sigmaEval3; BN254.ScalarField prodPermZetaOmegaEval; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IPlonkVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct PlonkProof { BN254.G1Point wire0; BN254.G1Point wire1; BN254.G1Point wire2; BN254.G1Point wire3; BN254.G1Point wire4; BN254.G1Point prodPerm; BN254.G1Point split0; BN254.G1Point split1; BN254.G1Point split2; BN254.G1Point split3; BN254.G1Point split4; BN254.G1Point zeta; BN254.G1Point zetaOmega; BN254.ScalarField wireEval0; BN254.ScalarField wireEval1; BN254.ScalarField wireEval2; BN254.ScalarField wireEval3; BN254.ScalarField wireEval4; BN254.ScalarField sigmaEval0; BN254.ScalarField sigmaEval1; BN254.ScalarField sigmaEval2; BN254.ScalarField sigmaEval3; BN254.ScalarField prodPermZetaOmegaEval; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PlonkProof {
        pub wire0: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub wire1: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub wire2: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub wire3: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub wire4: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub prodPerm: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub split0: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub split1: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub split2: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub split3: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub split4: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub zeta: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub zetaOmega: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        pub wireEval0: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub wireEval1: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub wireEval2: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub wireEval3: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub wireEval4: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub sigmaEval0: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub sigmaEval1: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub sigmaEval2: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub sigmaEval3: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub prodPermZetaOmegaEval: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::G1Point,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PlonkProof> for UnderlyingRustTuple<'_> {
            fn from(value: PlonkProof) -> Self {
                (
                    value.wire0,
                    value.wire1,
                    value.wire2,
                    value.wire3,
                    value.wire4,
                    value.prodPerm,
                    value.split0,
                    value.split1,
                    value.split2,
                    value.split3,
                    value.split4,
                    value.zeta,
                    value.zetaOmega,
                    value.wireEval0,
                    value.wireEval1,
                    value.wireEval2,
                    value.wireEval3,
                    value.wireEval4,
                    value.sigmaEval0,
                    value.sigmaEval1,
                    value.sigmaEval2,
                    value.sigmaEval3,
                    value.prodPermZetaOmegaEval,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PlonkProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    wire0: tuple.0,
                    wire1: tuple.1,
                    wire2: tuple.2,
                    wire3: tuple.3,
                    wire4: tuple.4,
                    prodPerm: tuple.5,
                    split0: tuple.6,
                    split1: tuple.7,
                    split2: tuple.8,
                    split3: tuple.9,
                    split4: tuple.10,
                    zeta: tuple.11,
                    zetaOmega: tuple.12,
                    wireEval0: tuple.13,
                    wireEval1: tuple.14,
                    wireEval2: tuple.15,
                    wireEval3: tuple.16,
                    wireEval4: tuple.17,
                    sigmaEval0: tuple.18,
                    sigmaEval1: tuple.19,
                    sigmaEval2: tuple.20,
                    sigmaEval3: tuple.21,
                    prodPermZetaOmegaEval: tuple.22,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PlonkProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PlonkProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.wire0),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.wire1),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.wire2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.wire3),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.wire4),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.prodPerm),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.split0),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.split1),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.split2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.split3),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.split4),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.zeta),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.zetaOmega),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.wireEval0),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.wireEval1),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.wireEval2),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.wireEval3),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.wireEval4),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.sigmaEval0),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.sigmaEval1),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.sigmaEval2),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.sigmaEval3),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(
                        &self.prodPermZetaOmegaEval,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PlonkProof {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PlonkProof {
            const NAME: &'static str = "PlonkProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PlonkProof(BN254.G1Point wire0,BN254.G1Point wire1,BN254.G1Point wire2,BN254.G1Point wire3,BN254.G1Point wire4,BN254.G1Point prodPerm,BN254.G1Point split0,BN254.G1Point split1,BN254.G1Point split2,BN254.G1Point split3,BN254.G1Point split4,BN254.G1Point zeta,BN254.G1Point zetaOmega,uint256 wireEval0,uint256 wireEval1,uint256 wireEval2,uint256 wireEval3,uint256 wireEval4,uint256 sigmaEval0,uint256 sigmaEval1,uint256 sigmaEval2,uint256 sigmaEval3,uint256 prodPermZetaOmegaEval)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(13);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.wire0).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.wire1).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.wire2).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.wire3).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.wire4).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.prodPerm)
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.split0).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.split1).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.split2).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.split3).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.split4).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.zeta).0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.zetaOmega)
                        .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.wireEval0,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.wireEval1,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.wireEval2,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.wireEval3,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.wireEval4,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.sigmaEval0,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.sigmaEval1,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.sigmaEval2,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.sigmaEval3,
                    )
                    .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                        &self.prodPermZetaOmegaEval,
                    )
                    .0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PlonkProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wire0,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wire1,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wire2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wire3,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wire4,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prodPerm,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.split0,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.split1,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.split2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.split3,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.split4,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.zeta,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.zetaOmega,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wireEval0,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wireEval1,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wireEval2,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wireEval3,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wireEval4,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigmaEval0,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigmaEval1,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigmaEval2,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigmaEval3,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prodPermZetaOmegaEval,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wire0,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wire1,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wire2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wire3,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wire4,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prodPerm,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.split0,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.split1,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.split2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.split3,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.split4,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.zeta, out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.zetaOmega,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wireEval0,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wireEval1,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wireEval2,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wireEval3,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wireEval4,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigmaEval0,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigmaEval1,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigmaEval2,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigmaEval3,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prodPermZetaOmegaEval,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IPlonkVerifier`](self) contract instance.

    See the [wrapper's documentation](`IPlonkVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IPlonkVerifierInstance<T, P, N> {
        IPlonkVerifierInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IPlonkVerifier`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IPlonkVerifier`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPlonkVerifierInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IPlonkVerifierInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPlonkVerifierInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IPlonkVerifierInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IPlonkVerifier`](self) contract instance.

        See the [wrapper's documentation](`IPlonkVerifierInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IPlonkVerifierInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPlonkVerifierInstance<T, P, N> {
            IPlonkVerifierInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IPlonkVerifierInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IPlonkVerifierInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library LightClient {
    struct LightClientState { uint64 viewNum; uint64 blockHeight; BN254.ScalarField blockCommRoot; }
    struct StakeTableState { uint256 threshold; BN254.ScalarField blsKeyComm; BN254.ScalarField schnorrKeyComm; BN254.ScalarField amountComm; }
    struct StateHistoryCommitment { uint64 l1BlockHeight; uint64 l1BlockTimestamp; uint64 hotShotBlockHeight; BN254.ScalarField hotShotBlockCommRoot; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod LightClient {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct LightClientState { uint64 viewNum; uint64 blockHeight; BN254.ScalarField blockCommRoot; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LightClientState {
        pub viewNum: u64,
        pub blockHeight: u64,
        pub blockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            BN254::ScalarField,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LightClientState> for UnderlyingRustTuple<'_> {
            fn from(value: LightClientState) -> Self {
                (value.viewNum, value.blockHeight, value.blockCommRoot)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LightClientState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    viewNum: tuple.0,
                    blockHeight: tuple.1,
                    blockCommRoot: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for LightClientState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for LightClientState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.viewNum,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.blockHeight,
                    ),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.blockCommRoot),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for LightClientState {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for LightClientState {
            const NAME: &'static str = "LightClientState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "LightClientState(uint64 viewNum,uint64 blockHeight,uint256 blockCommRoot)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.viewNum)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blockHeight)
                        .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                            &self.blockCommRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for LightClientState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.viewNum,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockHeight,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockCommRoot,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.viewNum,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockHeight,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockCommRoot,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct StakeTableState { uint256 threshold; BN254.ScalarField blsKeyComm; BN254.ScalarField schnorrKeyComm; BN254.ScalarField amountComm; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeTableState {
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
        pub blsKeyComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub schnorrKeyComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub amountComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            BN254::ScalarField,
            BN254::ScalarField,
            BN254::ScalarField,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StakeTableState> for UnderlyingRustTuple<'_> {
            fn from(value: StakeTableState) -> Self {
                (
                    value.threshold,
                    value.blsKeyComm,
                    value.schnorrKeyComm,
                    value.amountComm,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StakeTableState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    threshold: tuple.0,
                    blsKeyComm: tuple.1,
                    schnorrKeyComm: tuple.2,
                    amountComm: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StakeTableState {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StakeTableState {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.threshold,
                    ),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.blsKeyComm),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(
                        &self.schnorrKeyComm,
                    ),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(&self.amountComm),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StakeTableState {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StakeTableState {
            const NAME: &'static str = "StakeTableState";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StakeTableState(uint256 threshold,uint256 blsKeyComm,uint256 schnorrKeyComm,uint256 amountComm)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.threshold)
                        .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                            &self.blsKeyComm,
                        )
                        .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                            &self.schnorrKeyComm,
                        )
                        .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                            &self.amountComm,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeTableState {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.threshold,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blsKeyComm,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.schnorrKeyComm,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountComm,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.threshold,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blsKeyComm,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.schnorrKeyComm,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountComm,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct StateHistoryCommitment { uint64 l1BlockHeight; uint64 l1BlockTimestamp; uint64 hotShotBlockHeight; BN254.ScalarField hotShotBlockCommRoot; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StateHistoryCommitment {
        pub l1BlockHeight: u64,
        pub l1BlockTimestamp: u64,
        pub hotShotBlockHeight: u64,
        pub hotShotBlockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            BN254::ScalarField,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            u64,
            <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StateHistoryCommitment> for UnderlyingRustTuple<'_> {
            fn from(value: StateHistoryCommitment) -> Self {
                (
                    value.l1BlockHeight,
                    value.l1BlockTimestamp,
                    value.hotShotBlockHeight,
                    value.hotShotBlockCommRoot,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StateHistoryCommitment {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    l1BlockHeight: tuple.0,
                    l1BlockTimestamp: tuple.1,
                    hotShotBlockHeight: tuple.2,
                    hotShotBlockCommRoot: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StateHistoryCommitment {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StateHistoryCommitment {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.l1BlockHeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.l1BlockTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.hotShotBlockHeight,
                    ),
                    <BN254::ScalarField as alloy_sol_types::SolType>::tokenize(
                        &self.hotShotBlockCommRoot,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StateHistoryCommitment {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StateHistoryCommitment {
            const NAME: &'static str = "StateHistoryCommitment";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StateHistoryCommitment(uint64 l1BlockHeight,uint64 l1BlockTimestamp,uint64 hotShotBlockHeight,uint256 hotShotBlockCommRoot)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.l1BlockHeight)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.l1BlockTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hotShotBlockHeight,
                        )
                        .0,
                    <BN254::ScalarField as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hotShotBlockCommRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StateHistoryCommitment {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1BlockHeight,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.l1BlockTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hotShotBlockHeight,
                    )
                    + <BN254::ScalarField as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hotShotBlockCommRoot,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1BlockHeight,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.l1BlockTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hotShotBlockHeight,
                    out,
                );
                <BN254::ScalarField as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hotShotBlockCommRoot,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LightClient`](self) contract instance.

    See the [wrapper's documentation](`LightClientInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LightClientInstance<T, P, N> {
        LightClientInstance::<T, P, N>::new(address, provider)
    }
    /**A [`LightClient`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`LightClient`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LightClientInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LightClientInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LightClientInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > LightClientInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`LightClient`](self) contract instance.

        See the [wrapper's documentation](`LightClientInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> LightClientInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LightClientInstance<T, P, N> {
            LightClientInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > LightClientInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > LightClientInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BN254 {
    type BaseField is uint256;
    type ScalarField is uint256;
    struct G1Point {
        BaseField x;
        BaseField y;
    }
}

library IPlonkVerifier {
    struct PlonkProof {
        BN254.G1Point wire0;
        BN254.G1Point wire1;
        BN254.G1Point wire2;
        BN254.G1Point wire3;
        BN254.G1Point wire4;
        BN254.G1Point prodPerm;
        BN254.G1Point split0;
        BN254.G1Point split1;
        BN254.G1Point split2;
        BN254.G1Point split3;
        BN254.G1Point split4;
        BN254.G1Point zeta;
        BN254.G1Point zetaOmega;
        BN254.ScalarField wireEval0;
        BN254.ScalarField wireEval1;
        BN254.ScalarField wireEval2;
        BN254.ScalarField wireEval3;
        BN254.ScalarField wireEval4;
        BN254.ScalarField sigmaEval0;
        BN254.ScalarField sigmaEval1;
        BN254.ScalarField sigmaEval2;
        BN254.ScalarField sigmaEval3;
        BN254.ScalarField prodPermZetaOmegaEval;
    }
}

library LightClient {
    struct LightClientState {
        uint64 viewNum;
        uint64 blockHeight;
        BN254.ScalarField blockCommRoot;
    }
    struct StakeTableState {
        uint256 threshold;
        BN254.ScalarField blsKeyComm;
        BN254.ScalarField schnorrKeyComm;
        BN254.ScalarField amountComm;
    }
    struct StateHistoryCommitment {
        uint64 l1BlockHeight;
        uint64 l1BlockTimestamp;
        uint64 hotShotBlockHeight;
        BN254.ScalarField hotShotBlockCommRoot;
    }
}

interface LightClientMock {
    error AddressEmptyCode(address target);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error FailedInnerCall();
    error InsufficientSnapshotHistory();
    error InvalidAddress();
    error InvalidArgs();
    error InvalidHotShotBlockForCommitmentCheck();
    error InvalidInitialization();
    error InvalidMaxStateHistory();
    error InvalidProof();
    error MissingLastBlockInEpochUpdate();
    error NoChangeRequired();
    error NotInitializing();
    error OutdatedState();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error ProverNotPermissioned();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error WrongStakeTableUsed();

    event Initialized(uint64 version);
    event NewEpoch(uint64 epoch);
    event NewState(uint64 indexed viewNum, uint64 indexed blockHeight, BN254.ScalarField blockCommRoot);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PermissionedProverNotRequired();
    event PermissionedProverRequired(address permissionedProver);
    event Upgrade(address implementation);
    event Upgraded(address indexed implementation);

    constructor(LightClient.LightClientState genesis, LightClient.StakeTableState genesisStakeTableState, uint32 maxHistorySeconds, uint64 blocksPerEpoch);

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function _blocksPerEpoch() external view returns (uint64);
    function currentBlockNumber() external view returns (uint256);
    function currentEpoch() external view returns (uint64);
    function disablePermissionedProverMode() external;
    function epochFromBlockNumber(uint64 blockNum, uint64 blocksPerEpoch) external pure returns (uint64);
    function finalizedState() external view returns (uint64 viewNum, uint64 blockHeight, BN254.ScalarField blockCommRoot);
    function genesisStakeTableState() external view returns (uint256 threshold, BN254.ScalarField blsKeyComm, BN254.ScalarField schnorrKeyComm, BN254.ScalarField amountComm);
    function genesisState() external view returns (uint64 viewNum, uint64 blockHeight, BN254.ScalarField blockCommRoot);
    function getHotShotCommitment(uint256 hotShotBlockHeight) external view returns (BN254.ScalarField hotShotBlockCommRoot, uint64 hotshotBlockHeight);
    function getStateHistoryCount() external view returns (uint256);
    function getVersion() external pure returns (uint8 majorVersion, uint8 minorVersion, uint8 patchVersion);
    function initialize(LightClient.LightClientState memory _genesis, LightClient.StakeTableState memory _genesisStakeTableState, uint32 _stateHistoryRetentionPeriod, address owner, uint64 blocksPerEpoch) external;
    function isLastBlockInEpoch(uint64 blockHeight) external view returns (bool);
    function isPermissionedProverEnabled() external view returns (bool);
    function lagOverEscapeHatchThreshold(uint256 blockNumber, uint256 threshold) external view returns (bool);
    function newFinalizedState(LightClient.LightClientState memory newState, LightClient.StakeTableState memory nextStakeTable, IPlonkVerifier.PlonkProof memory proof) external;
    function owner() external view returns (address);
    function permissionedProver() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function renounceOwnership() external;
    function setBlocksPerEpoch(uint64 newBlocksPerEpoch) external;
    function setFinalizedState(LightClient.LightClientState memory state) external;
    function setHotShotDownSince(uint256 l1Height) external;
    function setHotShotUp() external;
    function setPermissionedProver(address prover) external;
    function setStateHistory(LightClient.StateHistoryCommitment[] memory _stateHistoryCommitments) external;
    function setstateHistoryRetentionPeriod(uint32 historySeconds) external;
    function stateHistoryCommitments(uint256) external view returns (uint64 l1BlockHeight, uint64 l1BlockTimestamp, uint64 hotShotBlockHeight, BN254.ScalarField hotShotBlockCommRoot);
    function stateHistoryFirstIndex() external view returns (uint64);
    function stateHistoryRetentionPeriod() external view returns (uint32);
    function transferOwnership(address newOwner) external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function votingStakeTableState() external view returns (uint256 threshold, BN254.ScalarField blsKeyComm, BN254.ScalarField schnorrKeyComm, BN254.ScalarField amountComm);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "genesis",
        "type": "tuple",
        "internalType": "struct LightClient.LightClientState",
        "components": [
          {
            "name": "viewNum",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockHeight",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockCommRoot",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      },
      {
        "name": "genesisStakeTableState",
        "type": "tuple",
        "internalType": "struct LightClient.StakeTableState",
        "components": [
          {
            "name": "threshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "blsKeyComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "schnorrKeyComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "amountComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      },
      {
        "name": "maxHistorySeconds",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "blocksPerEpoch",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "UPGRADE_INTERFACE_VERSION",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "_blocksPerEpoch",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentBlockNumber",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentEpoch",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "disablePermissionedProverMode",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "epochFromBlockNumber",
    "inputs": [
      {
        "name": "blockNum",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blocksPerEpoch",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "finalizedState",
    "inputs": [],
    "outputs": [
      {
        "name": "viewNum",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockHeight",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockCommRoot",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "genesisStakeTableState",
    "inputs": [],
    "outputs": [
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "blsKeyComm",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      },
      {
        "name": "schnorrKeyComm",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      },
      {
        "name": "amountComm",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "genesisState",
    "inputs": [],
    "outputs": [
      {
        "name": "viewNum",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockHeight",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockCommRoot",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getHotShotCommitment",
    "inputs": [
      {
        "name": "hotShotBlockHeight",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "hotShotBlockCommRoot",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      },
      {
        "name": "hotshotBlockHeight",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStateHistoryCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVersion",
    "inputs": [],
    "outputs": [
      {
        "name": "majorVersion",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minorVersion",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "patchVersion",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_genesis",
        "type": "tuple",
        "internalType": "struct LightClient.LightClientState",
        "components": [
          {
            "name": "viewNum",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockHeight",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockCommRoot",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      },
      {
        "name": "_genesisStakeTableState",
        "type": "tuple",
        "internalType": "struct LightClient.StakeTableState",
        "components": [
          {
            "name": "threshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "blsKeyComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "schnorrKeyComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "amountComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      },
      {
        "name": "_stateHistoryRetentionPeriod",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "blocksPerEpoch",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isLastBlockInEpoch",
    "inputs": [
      {
        "name": "blockHeight",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isPermissionedProverEnabled",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lagOverEscapeHatchThreshold",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "newFinalizedState",
    "inputs": [
      {
        "name": "newState",
        "type": "tuple",
        "internalType": "struct LightClient.LightClientState",
        "components": [
          {
            "name": "viewNum",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockHeight",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockCommRoot",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      },
      {
        "name": "nextStakeTable",
        "type": "tuple",
        "internalType": "struct LightClient.StakeTableState",
        "components": [
          {
            "name": "threshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "blsKeyComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "schnorrKeyComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "amountComm",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      },
      {
        "name": "proof",
        "type": "tuple",
        "internalType": "struct IPlonkVerifier.PlonkProof",
        "components": [
          {
            "name": "wire0",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "wire1",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "wire2",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "wire3",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "wire4",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "prodPerm",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "split0",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "split1",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "split2",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "split3",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "split4",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "zeta",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "zetaOmega",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "x",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              },
              {
                "name": "y",
                "type": "uint256",
                "internalType": "BN254.BaseField"
              }
            ]
          },
          {
            "name": "wireEval0",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "wireEval1",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "wireEval2",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "wireEval3",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "wireEval4",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "sigmaEval0",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "sigmaEval1",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "sigmaEval2",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "sigmaEval3",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          },
          {
            "name": "prodPermZetaOmegaEval",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "permissionedProver",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proxiableUUID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setBlocksPerEpoch",
    "inputs": [
      {
        "name": "newBlocksPerEpoch",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setFinalizedState",
    "inputs": [
      {
        "name": "state",
        "type": "tuple",
        "internalType": "struct LightClient.LightClientState",
        "components": [
          {
            "name": "viewNum",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockHeight",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockCommRoot",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setHotShotDownSince",
    "inputs": [
      {
        "name": "l1Height",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setHotShotUp",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPermissionedProver",
    "inputs": [
      {
        "name": "prover",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setStateHistory",
    "inputs": [
      {
        "name": "_stateHistoryCommitments",
        "type": "tuple[]",
        "internalType": "struct LightClient.StateHistoryCommitment[]",
        "components": [
          {
            "name": "l1BlockHeight",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "l1BlockTimestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "hotShotBlockHeight",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "hotShotBlockCommRoot",
            "type": "uint256",
            "internalType": "BN254.ScalarField"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setstateHistoryRetentionPeriod",
    "inputs": [
      {
        "name": "historySeconds",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stateHistoryCommitments",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "l1BlockHeight",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "l1BlockTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "hotShotBlockHeight",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "hotShotBlockCommRoot",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stateHistoryFirstIndex",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stateHistoryRetentionPeriod",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "upgradeToAndCall",
    "inputs": [
      {
        "name": "newImplementation",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "votingStakeTableState",
    "inputs": [],
    "outputs": [
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "blsKeyComm",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      },
      {
        "name": "schnorrKeyComm",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      },
      {
        "name": "amountComm",
        "type": "uint256",
        "internalType": "BN254.ScalarField"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewEpoch",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewState",
    "inputs": [
      {
        "name": "viewNum",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "blockHeight",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "blockCommRoot",
        "type": "uint256",
        "indexed": false,
        "internalType": "BN254.ScalarField"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PermissionedProverNotRequired",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PermissionedProverRequired",
    "inputs": [
      {
        "name": "permissionedProver",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgrade",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Upgraded",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AddressEmptyCode",
    "inputs": [
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967InvalidImplementation",
    "inputs": [
      {
        "name": "implementation",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC1967NonPayable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedInnerCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientSnapshotHistory",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidArgs",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHotShotBlockForCommitmentCheck",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidMaxStateHistory",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MissingLastBlockInEpochUpdate",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoChangeRequired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotInitializing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutdatedState",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ProverNotPermissioned",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnauthorizedCallContext",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UUPSUnsupportedProxiableUUID",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "WrongStakeTableUsed",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod LightClientMock {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523060805234801562000014575f80fd5b50604051620035bf380380620035bf833981016040819052620000379162000328565b6200004162000059565b6200004f848484846200010d565b50505050620003f0565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000aa5760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b03908116146200010a5780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b83516001600160401b031615158062000132575060208401516001600160401b031615155b806200014057506020830151155b806200014e57506040830151155b806200015c57506060830151155b806200016757508251155b806200017a5750610e108263ffffffff16105b806200018f57506301e133808263ffffffff16115b80620001a257506001600160401b038116155b15620001c1576040516350dd03f760e11b815260040160405180910390fd5b8351600980546020808801516001600160401b0390811668010000000000000000026001600160801b03199384169582169586178117909455604098890151600a819055600b8054909416909517909317909155600c92909255845160018190559185015160028190559585015160038190556060909501516004819055600592909255600695909555600793909355600892909255600d805463ffffffff909216600160a01b0263ffffffff60a01b199092169190911790555f8054919092166001600160401b031991909116179055565b604051606081016001600160401b0381118282101715620002c357634e487b7160e01b5f52604160045260245ffd5b60405290565b604051608081016001600160401b0381118282101715620002c357634e487b7160e01b5f52604160045260245ffd5b80516001600160401b03811681146200030f575f80fd5b919050565b805163ffffffff811681146200030f575f80fd5b5f805f808486036101208112156200033e575f80fd5b60608112156200034c575f80fd5b6200035662000294565b6200036187620002f8565b81526200037160208801620002f8565b60208201526040878101519082015294506080605f198201121562000394575f80fd5b506200039f620002c9565b60608681015182526080870151602083015260a0870151604083015260c0870151908201529250620003d460e0860162000314565b9150620003e56101008601620002f8565b905092959194509250565b6080516131a8620004175f395f818161113d0152818161116601526112e301526131a85ff3fe6080604052600436106101f1575f3560e01c8063826e41fc11610108578063b2424e3f1161009d578063d24d933d1161006d578063d24d933d146106f6578063e030330114610725578063f2fde38b14610744578063f567616014610763578063f9e50d1914610782575f80fd5b8063b2424e3f1461062e578063b5adea3c1461064c578063c23b9e9e146106a3578063c8e5e498146106db575f80fd5b806396c1ca61116100d857806396c1ca611461055e5780639fdb54a71461057d578063a1be8d52146105d2578063ad3cb1cc146105f1575f80fd5b8063826e41fc1461049c5780638584d23f146104c75780638da5cb5b1461050357806390c143901461053f575f80fd5b80633c23b6db1161018957806369cc6a041161015957806369cc6a0414610422578063715018a614610436578063757c37ad1461044a5780637667180814610469578063811f853f1461047d575f80fd5b80633c23b6db1461039d578063426d3194146103d95780634f1ef286146103fb57806352d1902d1461040e575f80fd5b80632d52aad6116101c45780632d52aad6146102e05780632f79889d1461030c578063313df7b11461034a578063378ec23b14610381575f80fd5b8063013fa5fc146101f557806302b592f3146102165780630625e19b146102735780630d8e6e2c146102b5575b5f80fd5b348015610200575f80fd5b5061021461020f366004612558565b610796565b005b348015610221575f80fd5b50610235610230366004612571565b610849565b60405161026a94939291906001600160401b039485168152928416602084015292166040820152606081019190915260800190565b60405180910390f35b34801561027e575f80fd5b506005546006546007546008546102959392919084565b60408051948552602085019390935291830152606082015260800161026a565b3480156102c0575f80fd5b5060408051600181525f602082018190529181019190915260600161026a565b3480156102eb575f80fd5b506102146102fa366004612571565b600f805460ff19166001179055601055565b348015610317575f80fd5b50600d5461033290600160c01b90046001600160401b031681565b6040516001600160401b03909116815260200161026a565b348015610355575f80fd5b50600d54610369906001600160a01b031681565b6040516001600160a01b03909116815260200161026a565b34801561038c575f80fd5b50435b60405190815260200161026a565b3480156103a8575f80fd5b506102146103b736600461259e565b5f805467ffffffffffffffff19166001600160401b0392909216919091179055565b3480156103e4575f80fd5b506001546002546003546004546102959392919084565b610214610409366004612646565b610892565b348015610419575f80fd5b5061038f6108b1565b34801561042d575f80fd5b506102146108cc565b348015610441575f80fd5b5061021461093a565b348015610455575f80fd5b506102146104643660046127d8565b61094b565b348015610474575f80fd5b50610332610c16565b348015610488575f80fd5b506102146104973660046129b1565b610c40565b3480156104a7575f80fd5b50600d546001600160a01b031615155b604051901515815260200161026a565b3480156104d2575f80fd5b506104e66104e1366004612571565b610d64565b604080519283526001600160401b0390911660208301520161026a565b34801561050e575f80fd5b507f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b0316610369565b34801561054a575f80fd5b50610332610559366004612a17565b610e8f565b348015610569575f80fd5b50610214610578366004612a48565b610eec565b348015610588575f80fd5b50600b54600c546105ac916001600160401b0380821692600160401b909204169083565b604080516001600160401b0394851681529390921660208401529082015260600161026a565b3480156105dd575f80fd5b506104b76105ec36600461259e565b610f75565b3480156105fc575f80fd5b50610621604051806040016040528060058152602001640352e302e360dc1b81525081565b60405161026a9190612a83565b348015610639575f80fd5b505f54610332906001600160401b031681565b348015610657575f80fd5b50610214610666366004612ab5565b8051600b805460208401516001600160401b03908116600160401b026001600160801b031990921693169290921791909117905560400151600c55565b3480156106ae575f80fd5b50600d546106c690600160a01b900463ffffffff1681565b60405163ffffffff909116815260200161026a565b3480156106e6575f80fd5b50610214600f805460ff19169055565b348015610701575f80fd5b50600954600a546105ac916001600160401b0380821692600160401b909204169083565b348015610730575f80fd5b506104b761073f366004612acf565b610fb7565b34801561074f575f80fd5b5061021461075e366004612558565b610fea565b34801561076e575f80fd5b5061021461077d366004612aef565b61102c565b34801561078d575f80fd5b50600e5461038f565b61079e6110d7565b6001600160a01b0381166107c55760405163e6c4247b60e01b815260040160405180910390fd5b600d546001600160a01b03908116908216036107f45760405163a863aec960e01b815260040160405180910390fd5b600d80546001600160a01b0319166001600160a01b0383169081179091556040519081527f8017bb887fdf8fca4314a9d40f6e73b3b81002d67e5cfa85d88173af6aa46072906020015b60405180910390a150565b600e8181548110610858575f80fd5b5f918252602090912060029091020180546001909101546001600160401b038083169350600160401b8304811692600160801b9004169084565b61089a611132565b6108a3826111d6565b6108ad8282611217565b5050565b5f6108ba6112d8565b505f8051602061317c83398151915290565b6108d46110d7565b600d546001600160a01b03161561091f57600d80546001600160a01b03191690556040517f9a5f57de856dd668c54dd95e5c55df93432171cbca49a8776d5620ea59c02450905f90a1565b60405163a863aec960e01b815260040160405180910390fd5b565b6109426110d7565b6109385f611321565b600d546001600160a01b0316151580156109705750600d546001600160a01b03163314155b1561098e576040516301474c8f60e71b815260040160405180910390fd5b600b5483516001600160401b0391821691161115806109c75750600b5460208401516001600160401b03600160401b9092048216911611155b156109e55760405163051c46ef60e01b815260040160405180910390fd5b6109f28360400151611391565b5f6109fb610c16565b60208501515f805492935091610a1a91906001600160401b0316610e8f565b9050610a27826001612bf1565b6001600160401b0316816001600160401b0316148015610a605750600b54610a5e90600160401b90046001600160401b0316610f75565b155b8015610a7457505f826001600160401b0316115b15610a9257604051637150de4560e01b815260040160405180910390fd5b610a9d826002612bf1565b6001600160401b0316816001600160401b031610610ace57604051637150de4560e01b815260040160405180910390fd5b610adb8460200151611391565b610ae88460400151611391565b610af58460600151611391565b610b00858585611401565b8451600b805460208801516001600160401b03818116600160401b026001600160801b03199093169416939093171790556040860151600c55610b4290610f75565b15610bac5783516005556020840151600655604084015160075560608401516008557f31eabd9099fdb25dacddd206abff87311e553441fc9d0fcdef201062d7e7071b610b90826001612bf1565b6040516001600160401b03909116815260200160405180910390a15b610bb7434287611559565b84602001516001600160401b0316855f01516001600160401b03167fa04a773924505a418564363725f56832f5772e6b8d0dbd6efce724dfe803dae68760400151604051610c0791815260200190565b60405180910390a35050505050565b600b545f80549091610c3b916001600160401b03600160401b90920482169116610e8f565b905090565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a008054600160401b810460ff1615906001600160401b03165f81158015610c845750825b90505f826001600160401b03166001148015610c9f5750303b155b905081158015610cad575080155b15610ccb5760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff191660011785558315610cf557845460ff60401b1916600160401b1785555b610cfe87611742565b610d06611753565b610d128a8a8a8961175b565b8315610d5857845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50505050505050505050565b600e80545f91829190610d78600183612c11565b81548110610d8857610d88612c24565b5f918252602090912060029091020154600160801b90046001600160401b03168410610dc757604051631856a49960e21b815260040160405180910390fd5b600d54600160c01b90046001600160401b03165b81811015610e885784600e8281548110610df757610df7612c24565b5f918252602090912060029091020154600160801b90046001600160401b03161115610e8057600e8181548110610e3057610e30612c24565b905f5260205f20906002020160010154600e8281548110610e5357610e53612c24565b905f5260205f2090600202015f0160109054906101000a90046001600160401b0316935093505050915091565b600101610ddb565b5050915091565b5f816001600160401b03165f03610ea757505f610ee6565b610eb18284612c4c565b6001600160401b03165f03610ed157610eca8284612c71565b9050610ee6565b610edb8284612c71565b610eca906001612bf1565b92915050565b610ef46110d7565b610e108163ffffffff161080610f1357506301e133808163ffffffff16115b80610f315750600d5463ffffffff600160a01b909104811690821611155b15610f4f576040516307a5077760e51b815260040160405180910390fd5b600d805463ffffffff909216600160a01b0263ffffffff60a01b19909216919091179055565b5f816001600160401b03165f03610f8d57505f919050565b5f54610fa2906001600160401b031683612c4c565b6001600160401b03161592915050565b919050565b600f545f9060ff16610fd257610fcd83836118d5565b610fe3565b8160105484610fe19190612c11565b115b9392505050565b610ff26110d7565b6001600160a01b03811661102057604051631e4fbdf760e01b81525f60048201526024015b60405180910390fd5b61102981611321565b50565b611037600e5f612276565b5f5b81518110156108ad57600e82828151811061105657611056612c24565b6020908102919091018101518254600181810185555f94855293839020825160029092020180549383015160408401516001600160401b03908116600160801b0267ffffffffffffffff60801b19928216600160401b026001600160801b031990971691909416179490941793909316178255606001519082015501611039565b336111097f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031690565b6001600160a01b0316146109385760405163118cdaa760e01b8152336004820152602401611017565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806111b857507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166111ac5f8051602061317c833981519152546001600160a01b031690565b6001600160a01b031614155b156109385760405163703e46dd60e11b815260040160405180910390fd5b6111de6110d7565b6040516001600160a01b03821681527ff78721226efe9a1bb678189a16d1554928b9f2192e2cb93eeda83b79fa40007d9060200161083e565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611271575060408051601f3d908101601f1916820190925261126e91810190612c96565b60015b61129957604051634c9c8ce360e01b81526001600160a01b0383166004820152602401611017565b5f8051602061317c83398151915281146112c957604051632a87526960e21b815260048101829052602401611017565b6112d38383611a2d565b505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109385760405163703e46dd60e11b815260040160405180910390fd5b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930080546001600160a01b031981166001600160a01b03848116918217845560405192169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a3505050565b7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018110806108ad5760405162461bcd60e51b815260206004820152601b60248201527f426e3235343a20696e76616c6964207363616c6172206669656c6400000000006044820152606401611017565b5f61140a611a82565b9050611414612294565b84516001600160401b0390811682526020808701805190921690830152604080870151908301526006546060830152600754608083015260085460a083015260055460c08301525161146590610f75565b1561149757602084015160e08201526040840151610100820152606084015161012082015283516101408201526114bb565b60065460e08201526007546101008201526008546101208201526005546101408201525b60405163fc8660c760e01b8152735fbdb2315678afecb367f032d93f642f64180aa39063fc8660c7906114f690859085908890600401612eb9565b602060405180830381865af4158015611511573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061153591906130e7565b611552576040516309bde33960e01b815260040160405180910390fd5b5050505050565b600e54158015906115ce5750600d54600e8054600160a01b830463ffffffff1692600160c01b90046001600160401b031690811061159957611599612c24565b5f9182526020909120600290910201546115c390600160401b90046001600160401b031684613106565b6001600160401b0316115b1561166157600d54600e80549091600160c01b90046001600160401b03169081106115fb576115fb612c24565b5f9182526020822060029091020180546001600160c01b031916815560010155600d8054600160c01b90046001600160401b031690601861163b83613126565b91906101000a8154816001600160401b0302191690836001600160401b03160217905550505b604080516080810182526001600160401b03948516815292841660208085019182528301518516848301908152929091015160608401908152600e80546001810182555f91909152935160029094027fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd81018054935194518716600160801b0267ffffffffffffffff60801b19958816600160401b026001600160801b03199095169690971695909517929092179290921693909317909155517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fe90910155565b61174a6120b1565b611029816120fa565b6109386120b1565b83516001600160401b031615158061177f575060208401516001600160401b031615155b8061178c57506020830151155b8061179957506040830151155b806117a657506060830151155b806117b057508251155b806117c25750610e108263ffffffff16105b806117d657506301e133808263ffffffff16115b806117e857506001600160401b038116155b15611806576040516350dd03f760e11b815260040160405180910390fd5b8351600980546020808801516001600160401b03908116600160401b026001600160801b03199384169582169586178117909455604098890151600a819055600b8054909416909517909317909155600c92909255845160018190559185015160028190559585015160038190556060909501516004819055600592909255600695909555600793909355600892909255600d805463ffffffff909216600160a01b0263ffffffff60a01b199092169190911790555f80549190921667ffffffffffffffff1991909116179055565b600e545f90438411806118e6575080155b806119305750600d54600e80549091600160c01b90046001600160401b031690811061191457611914612c24565b5f9182526020909120600290910201546001600160401b031684105b1561194e5760405163b0b4387760e01b815260040160405180910390fd5b5f808061195c600185612c11565b90505b816119f857600d54600160c01b90046001600160401b031681106119f85786600e828154811061199157611991612c24565b5f9182526020909120600290910201546001600160401b0316116119e65760019150600e81815481106119c6576119c6612c24565b5f9182526020909120600290910201546001600160401b031692506119f8565b806119f08161314b565b91505061195f565b81611a165760405163b0b4387760e01b815260040160405180910390fd5b85611a218489612c11565b11979650505050505050565b611a3682612102565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611a7a576112d38282612165565b6108ad6121d7565b611a8a6122b3565b620100008152600b60208201527f2faf5a113efd87d75818e63ff9a6170007f22c89bbc4a8bd0f2b48268757b0146040820151527f185aee05f8d3babfce67931f15db39e61f25f794a4134d7bee6e18c5ad1ec0576020604083015101527f0dccf5dcf667a37ca93b8d721091d8f3a8049b3d1e89a56d66e42751bbaf7b5e6060820151527f2cf10949fc5bfcecb3bc54dd4121e55807430f17f30498a7ea6a026070b191626020606083015101527f08d70e4e0184fe53bd566f0d7edc4cd7b0e339490973d0faec7dac2089f538e56080820151527ef665fe1fd110d37d1dea446e8400f06f06b9b58ab3df90fbae7c47ee5860416020608083015101527f087e14d71924ac0f2880adf0f106925e5a6fdd57d020bb3c8aa70fa9fc00ccf360a0820151527f01db7e3178b342f91d54fc972cee72569f429a393988ee43c289e2ed96077152602060a083015101527f196dd42d767201f7f196c42aaef485656046310f5083559592bd1313e16948b760c0820151527f17889680810aaabd1ff3ac4a6c5492100579e059170cd2b77e2b3da6d37cc246602060c083015101527f24935e7a77ac313fd3d60ff3f1a0a79ec32c7dc519b39da0acb2c49f367771cc60e0820151527f168e29425ef138cb6943c75352f33c190e5f1488eb54a9e11deb744da7fb6b2e602060e083015101527f1b58d558b5526453bd1028ca938c940bb89e723f7c35787c02f9f179ae9a0cea610100820151527f21afc121d91d9d1c17dafb9236bc9b872c5b43df064c0b1286012fb43a762324602061010083015101527f1047fc55794d1e597de155077611e3c789a0a2be02183821bba56cf61cc1b8ed610120820151527f174252324727c0d2ee5e50eb57a5231f67474ceed6932ad4ffe9bcf866aa3428602061012083015101527f28db289a4cfb73ba92961572f3185298ae366ed1a44971607bcbf801f120f561610140820151527f045cfe7ae2cd175508172e7d9c2e899bb1d216dfc31fe89fc6c917caaee877a2602061014083015101527f195f2eec8547727fc46ed01b79e8f666ded64ae54f57073874a5a2470380a785610160820151527f1527322e85da1aefbd839e65d11dc695aac16b0db6c62591d9813242d41cbe31602061016083015101527f10c8d7d7355f7e0f8c002f482cc3b98c90baa94261c59a17b424eecfe4e963b2610180820151527f2272e30178647167bbead3a2d7371988f2e198e65815029ded4c64bfc0850f1f602061018083015101527f15d56ea7ab2fa61265f551c2ae25389c8fe7bcb3bf6608082c36a201f225f77d6101a0820151527f0b58546887202e7273d3d0c55d65dd6132cac98ebf04efb1b52445c513c4a4df60206101a083015101527f050d6f43774e8dffaa868f2a7dc82f566c69d175d818d4517cc70ac5fcb2f1b16101c0820151527f2fff87bf605e998373bb64553f3a625dabcd12888692d678a8f44d136440c86360206101c083015101527f12d085608c602cfb5b8c03ec7bd13ac0ff9e64a9ac1e9aa746594a033e464bf26101e0820151527f18ac5a3536042eeb0b0c7c2f43f5e2ca3b2173daa4c2812ffca64787e8e956b260206101e083015101527f0f0f9891fc2b790e74dc253c8854df6392e010f4de6760b8423a3dd69bbe5dce610200820151527f16bed1d244a2fe3ab9a652c7feec5650161d8a75227dece7294f3c8fc542fd6c602061020083015101527f0fa36d00672fa6a1c44cd3c259212c1ada48c66bf7bb085f24471b15b17e6e51610220820151527f182088e56b64955232460891d2b279765325813aef1dae855e5f496c418afc41602061022083015101527f2baf5ae2dd832e1449facc611b6b80fd66d58c871d5827c5c8e2747064e29964610240820151527f29f543b543137e881804c989cd3b99934010002238e8ab3eec882e09d306681f602061024083015101527f2db0ddc7123b42f520e257466a0d92da8b564fe01ec665096c14119643012984610260820151527f1b7ab27a66966284d7fb29bce9d550eafba16c49fbc6267827cdfc8d0b16f94f602061026083015101527fb0838893ec1f237e8b07323b0744599f4e97b598b3b589bcc2bc37b8d5c418016102808201527fc18393c0fa30fe4e8b038e357ad851eae8de9107584effe7c7f1f651b2010e266102a082015290565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054600160401b900460ff1661093857604051631afcd79f60e31b815260040160405180910390fd5b610ff26120b1565b806001600160a01b03163b5f0361213757604051634c9c8ce360e01b81526001600160a01b0382166004820152602401611017565b5f8051602061317c83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516121819190613160565b5f60405180830381855af49150503d805f81146121b9576040519150601f19603f3d011682016040523d82523d5f602084013e6121be565b606091505b50915091506121ce8583836121f6565b95945050505050565b34156109385760405163b398979f60e01b815260040160405180910390fd5b60608261220657610fcd8261224d565b815115801561221d57506001600160a01b0384163b155b1561224657604051639996b31560e01b81526001600160a01b0385166004820152602401611017565b5092915050565b80511561225d5780518082602001fd5b604051630a12f52160e11b815260040160405180910390fd5b5080545f8255600202905f5260205f20908101906110299190612519565b604051806101600160405280600b906020820280368337509192915050565b604051806102c001604052805f81526020015f81526020016122e660405180604001604052805f81526020015f81525090565b815260200161230660405180604001604052805f81526020015f81525090565b815260200161232660405180604001604052805f81526020015f81525090565b815260200161234660405180604001604052805f81526020015f81525090565b815260200161236660405180604001604052805f81526020015f81525090565b815260200161238660405180604001604052805f81526020015f81525090565b81526020016123a660405180604001604052805f81526020015f81525090565b81526020016123c660405180604001604052805f81526020015f81525090565b81526020016123e660405180604001604052805f81526020015f81525090565b815260200161240660405180604001604052805f81526020015f81525090565b815260200161242660405180604001604052805f81526020015f81525090565b815260200161244660405180604001604052805f81526020015f81525090565b815260200161246660405180604001604052805f81526020015f81525090565b815260200161248660405180604001604052805f81526020015f81525090565b81526020016124a660405180604001604052805f81526020015f81525090565b81526020016124c660405180604001604052805f81526020015f81525090565b81526020016124e660405180604001604052805f81526020015f81525090565b815260200161250660405180604001604052805f81526020015f81525090565b81525f6020820181905260409091015290565b5b8082111561253e5780546001600160c01b03191681555f600182015560020161251a565b5090565b80356001600160a01b0381168114610fb2575f80fd5b5f60208284031215612568575f80fd5b610fe382612542565b5f60208284031215612581575f80fd5b5035919050565b80356001600160401b0381168114610fb2575f80fd5b5f602082840312156125ae575f80fd5b610fe382612588565b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b03811182821017156125ed576125ed6125b7565b60405290565b6040516102e081016001600160401b03811182821017156125ed576125ed6125b7565b604051601f8201601f191681016001600160401b038111828210171561263e5761263e6125b7565b604052919050565b5f8060408385031215612657575f80fd5b61266083612542565b91506020808401356001600160401b038082111561267c575f80fd5b818601915086601f83011261268f575f80fd5b8135818111156126a1576126a16125b7565b6126b3601f8201601f19168501612616565b915080825287848285010111156126c8575f80fd5b80848401858401375f848284010152508093505050509250929050565b5f606082840312156126f5575f80fd5b604051606081018181106001600160401b0382111715612717576127176125b7565b60405290508061272683612588565b815261273460208401612588565b6020820152604083013560408201525092915050565b5f6080828403121561275a575f80fd5b6127626125cb565b90508135815260208201356020820152604082013560408201526060820135606082015292915050565b5f6040828403121561279c575f80fd5b604051604081018181106001600160401b03821117156127be576127be6125b7565b604052823581526020928301359281019290925250919050565b5f805f8385036105608112156127ec575f80fd5b6127f686866126e5565b9350612805866060870161274a565b92506104808060df198301121561281a575f80fd5b6128226125f3565b91506128318760e0880161278c565b82526101206128428882890161278c565b602084015261016061285689828a0161278c565b60408501526101a061286a8a828b0161278c565b60608601526101e061287e8b828c0161278c565b60808701526102206128928c828d0161278c565b60a08801526102606128a68d828e0161278c565b60c08901526102a06128ba8e828f0161278c565b60e08a01526128cd8e6102e08f0161278c565b6101008a01526128e18e6103208f0161278c565b878a01526128f38e6103608f0161278c565b6101408a01526129078e6103a08f0161278c565b868a01526129198e6103e08f0161278c565b6101808a01526104208d0135858a01526104408d01356101c08a01526104608d0135848a0152878d01356102008a01526104a08d0135838a01526104c08d01356102408a01526104e08d0135828a01526105008d01356102808a01526105208d0135818a015250505050505050506105408501356102c0820152809150509250925092565b803563ffffffff81168114610fb2575f80fd5b5f805f805f61014086880312156129c6575f80fd5b6129d087876126e5565b94506129df876060880161274a565b93506129ed60e0870161299e565b92506129fc6101008701612542565b9150612a0b6101208701612588565b90509295509295909350565b5f8060408385031215612a28575f80fd5b612a3183612588565b9150612a3f60208401612588565b90509250929050565b5f60208284031215612a58575f80fd5b610fe38261299e565b5f5b83811015612a7b578181015183820152602001612a63565b50505f910152565b602081525f8251806020840152612aa1816040850160208701612a61565b601f01601f19169190910160400192915050565b5f60608284031215612ac5575f80fd5b610fe383836126e5565b5f8060408385031215612ae0575f80fd5b50508035926020909101359150565b5f6020808385031215612b00575f80fd5b82356001600160401b0380821115612b16575f80fd5b818501915085601f830112612b29575f80fd5b813581811115612b3b57612b3b6125b7565b612b49848260051b01612616565b818152848101925060079190911b830184019087821115612b68575f80fd5b928401925b81841015612bd25760808489031215612b84575f80fd5b612b8c6125cb565b612b9585612588565b8152612ba2868601612588565b868201526040612bb3818701612588565b9082015260608581013590820152835260809093019291840191612b6d565b979650505050505050565b634e487b7160e01b5f52601160045260245ffd5b6001600160401b0381811683821601908082111561224657612246612bdd565b81810381811115610ee657610ee6612bdd565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b5f6001600160401b0380841680612c6557612c65612c38565b92169190910692915050565b5f6001600160401b0380841680612c8a57612c8a612c38565b92169190910492915050565b5f60208284031215612ca6575f80fd5b5051919050565b805f5b600b811015612ccf578151845260209384019390910190600101612cb0565b50505050565b612cea82825180518252602090810151910152565b6020818101518051604085015290810151606084015250604081015180516080840152602081015160a0840152506060810151805160c0840152602081015160e0840152506080810151610100612d4d8185018380518252602090810151910152565b60a08301519150610140612d6d8186018480518252602090810151910152565b60c08401519250610180612d8d8187018580518252602090810151910152565b60e085015193506101c0612dad8188018680518252602090810151910152565b92850151935061020092612dcd8785018680518252602090810151910152565b6101208601519450610240612dee8189018780518252602090810151910152565b92860151945061028092612e0e8885018780518252602090810151910152565b61016087015195506102c0612e2f818a018880518252602090810151910152565b9287015180516103008a0152602001516103208901526101a0870151610340890152908601516103608801526101e0860151610380880152928501516103a08701526102208501516103c0870152918401516103e08601526102608401516104008601528301516104208501526102a0830151610440850152909101516104609092019190915250565b5f610ae08201905084518252602085015160208301526040850151612eeb604084018280518252602090810151910152565b50606085015180516080840152602081015160a0840152506080850151805160c0840152602081015160e08401525060a0850151610100612f388185018380518252602090810151910152565b60c08701519150610140612f588186018480518252602090810151910152565b60e08801519250610180612f788187018580518252602090810151910152565b9188015192506101c091612f988684018580518252602090810151910152565b6101208901519350610200612fb98188018680518252602090810151910152565b91890151935061024091612fd98784018680518252602090810151910152565b6101608a01519450610280612ffa8189018780518252602090810151910152565b918a015180516102c08901526020908101516102e08901526101a08b015180516103008a0152810151610320890152938a015180516103408901528401516103608801526101e08a015180516103808901528401516103a088015289015180516103c08801528301516103e087015261022089015180516104008801528301516104208701529088015180516104408701528201516104608601526102608801518051610480870152909101516104a08501528601516104c0840152506102a08501516104e08301526130d1610500830185612cad565b6130df610660830184612cd5565b949350505050565b5f602082840312156130f7575f80fd5b81518015158114610fe3575f80fd5b6001600160401b0382811682821603908082111561224657612246612bdd565b5f6001600160401b0380831681810361314157613141612bdd565b6001019392505050565b5f8161315957613159612bdd565b505f190190565b5f8251613171818460208701612a61565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca164736f6c6343000817000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[P`@Qb\x005\xBF8\x03\x80b\x005\xBF\x839\x81\x01`@\x81\x90Rb\0\x007\x91b\0\x03(V[b\0\0Ab\0\0YV[b\0\0O\x84\x84\x84\x84b\0\x01\rV[PPPPb\0\x03\xF0V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0\xAAW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\x01\nW\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[\x83Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80b\0\x012WP` \x84\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80b\0\x01@WP` \x83\x01Q\x15[\x80b\0\x01NWP`@\x83\x01Q\x15[\x80b\0\x01\\WP``\x83\x01Q\x15[\x80b\0\x01gWP\x82Q\x15[\x80b\0\x01zWPa\x0E\x10\x82c\xFF\xFF\xFF\xFF\x16\x10[\x80b\0\x01\x8FWPc\x01\xE13\x80\x82c\xFF\xFF\xFF\xFF\x16\x11[\x80b\0\x01\xA2WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15b\0\x01\xC1W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\t\x80T` \x80\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16h\x01\0\0\0\0\0\0\0\0\x02`\x01`\x01`\x80\x1B\x03\x19\x93\x84\x16\x95\x82\x16\x95\x86\x17\x81\x17\x90\x94U`@\x98\x89\x01Q`\n\x81\x90U`\x0B\x80T\x90\x94\x16\x90\x95\x17\x90\x93\x17\x90\x91U`\x0C\x92\x90\x92U\x84Q`\x01\x81\x90U\x91\x85\x01Q`\x02\x81\x90U\x95\x85\x01Q`\x03\x81\x90U``\x90\x95\x01Q`\x04\x81\x90U`\x05\x92\x90\x92U`\x06\x95\x90\x95U`\x07\x93\x90\x93U`\x08\x92\x90\x92U`\r\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U_\x80T\x91\x90\x92\x16`\x01`\x01`@\x1B\x03\x19\x91\x90\x91\x16\x17\x90UV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02\xC3WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x02\xC3WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x03\x0FW_\x80\xFD[\x91\x90PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x03\x0FW_\x80\xFD[_\x80_\x80\x84\x86\x03a\x01 \x81\x12\x15b\0\x03>W_\x80\xFD[``\x81\x12\x15b\0\x03LW_\x80\xFD[b\0\x03Vb\0\x02\x94V[b\0\x03a\x87b\0\x02\xF8V[\x81Rb\0\x03q` \x88\x01b\0\x02\xF8V[` \x82\x01R`@\x87\x81\x01Q\x90\x82\x01R\x94P`\x80`_\x19\x82\x01\x12\x15b\0\x03\x94W_\x80\xFD[Pb\0\x03\x9Fb\0\x02\xC9V[``\x86\x81\x01Q\x82R`\x80\x87\x01Q` \x83\x01R`\xA0\x87\x01Q`@\x83\x01R`\xC0\x87\x01Q\x90\x82\x01R\x92Pb\0\x03\xD4`\xE0\x86\x01b\0\x03\x14V[\x91Pb\0\x03\xE5a\x01\0\x86\x01b\0\x02\xF8V[\x90P\x92\x95\x91\x94P\x92PV[`\x80Qa1\xA8b\0\x04\x17_9_\x81\x81a\x11=\x01R\x81\x81a\x11f\x01Ra\x12\xE3\x01Ra1\xA8_\xF3\xFE`\x80`@R`\x046\x10a\x01\xF1W_5`\xE0\x1C\x80c\x82nA\xFC\x11a\x01\x08W\x80c\xB2BN?\x11a\0\x9DW\x80c\xD2M\x93=\x11a\0mW\x80c\xD2M\x93=\x14a\x06\xF6W\x80c\xE003\x01\x14a\x07%W\x80c\xF2\xFD\xE3\x8B\x14a\x07DW\x80c\xF5ga`\x14a\x07cW\x80c\xF9\xE5\r\x19\x14a\x07\x82W_\x80\xFD[\x80c\xB2BN?\x14a\x06.W\x80c\xB5\xAD\xEA<\x14a\x06LW\x80c\xC2;\x9E\x9E\x14a\x06\xA3W\x80c\xC8\xE5\xE4\x98\x14a\x06\xDBW_\x80\xFD[\x80c\x96\xC1\xCAa\x11a\0\xD8W\x80c\x96\xC1\xCAa\x14a\x05^W\x80c\x9F\xDBT\xA7\x14a\x05}W\x80c\xA1\xBE\x8DR\x14a\x05\xD2W\x80c\xAD<\xB1\xCC\x14a\x05\xF1W_\x80\xFD[\x80c\x82nA\xFC\x14a\x04\x9CW\x80c\x85\x84\xD2?\x14a\x04\xC7W\x80c\x8D\xA5\xCB[\x14a\x05\x03W\x80c\x90\xC1C\x90\x14a\x05?W_\x80\xFD[\x80c<#\xB6\xDB\x11a\x01\x89W\x80ci\xCCj\x04\x11a\x01YW\x80ci\xCCj\x04\x14a\x04\"W\x80cqP\x18\xA6\x14a\x046W\x80cu|7\xAD\x14a\x04JW\x80cvg\x18\x08\x14a\x04iW\x80c\x81\x1F\x85?\x14a\x04}W_\x80\xFD[\x80c<#\xB6\xDB\x14a\x03\x9DW\x80cBm1\x94\x14a\x03\xD9W\x80cO\x1E\xF2\x86\x14a\x03\xFBW\x80cR\xD1\x90-\x14a\x04\x0EW_\x80\xFD[\x80c-R\xAA\xD6\x11a\x01\xC4W\x80c-R\xAA\xD6\x14a\x02\xE0W\x80c/y\x88\x9D\x14a\x03\x0CW\x80c1=\xF7\xB1\x14a\x03JW\x80c7\x8E\xC2;\x14a\x03\x81W_\x80\xFD[\x80c\x01?\xA5\xFC\x14a\x01\xF5W\x80c\x02\xB5\x92\xF3\x14a\x02\x16W\x80c\x06%\xE1\x9B\x14a\x02sW\x80c\r\x8En,\x14a\x02\xB5W[_\x80\xFD[4\x80\x15a\x02\0W_\x80\xFD[Pa\x02\x14a\x02\x0F6`\x04a%XV[a\x07\x96V[\0[4\x80\x15a\x02!W_\x80\xFD[Pa\x025a\x0206`\x04a%qV[a\x08IV[`@Qa\x02j\x94\x93\x92\x91\x90`\x01`\x01`@\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R\x92\x16`@\x82\x01R``\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02~W_\x80\xFD[P`\x05T`\x06T`\x07T`\x08Ta\x02\x95\x93\x92\x91\x90\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02jV[4\x80\x15a\x02\xC0W_\x80\xFD[P`@\x80Q`\x01\x81R_` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x01a\x02jV[4\x80\x15a\x02\xEBW_\x80\xFD[Pa\x02\x14a\x02\xFA6`\x04a%qV[`\x0F\x80T`\xFF\x19\x16`\x01\x17\x90U`\x10UV[4\x80\x15a\x03\x17W_\x80\xFD[P`\rTa\x032\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02jV[4\x80\x15a\x03UW_\x80\xFD[P`\rTa\x03i\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02jV[4\x80\x15a\x03\x8CW_\x80\xFD[PC[`@Q\x90\x81R` \x01a\x02jV[4\x80\x15a\x03\xA8W_\x80\xFD[Pa\x02\x14a\x03\xB76`\x04a%\x9EV[_\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x03\xE4W_\x80\xFD[P`\x01T`\x02T`\x03T`\x04Ta\x02\x95\x93\x92\x91\x90\x84V[a\x02\x14a\x04\t6`\x04a&FV[a\x08\x92V[4\x80\x15a\x04\x19W_\x80\xFD[Pa\x03\x8Fa\x08\xB1V[4\x80\x15a\x04-W_\x80\xFD[Pa\x02\x14a\x08\xCCV[4\x80\x15a\x04AW_\x80\xFD[Pa\x02\x14a\t:V[4\x80\x15a\x04UW_\x80\xFD[Pa\x02\x14a\x04d6`\x04a'\xD8V[a\tKV[4\x80\x15a\x04tW_\x80\xFD[Pa\x032a\x0C\x16V[4\x80\x15a\x04\x88W_\x80\xFD[Pa\x02\x14a\x04\x976`\x04a)\xB1V[a\x0C@V[4\x80\x15a\x04\xA7W_\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[`@Q\x90\x15\x15\x81R` \x01a\x02jV[4\x80\x15a\x04\xD2W_\x80\xFD[Pa\x04\xE6a\x04\xE16`\x04a%qV[a\rdV[`@\x80Q\x92\x83R`\x01`\x01`@\x1B\x03\x90\x91\x16` \x83\x01R\x01a\x02jV[4\x80\x15a\x05\x0EW_\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16a\x03iV[4\x80\x15a\x05JW_\x80\xFD[Pa\x032a\x05Y6`\x04a*\x17V[a\x0E\x8FV[4\x80\x15a\x05iW_\x80\xFD[Pa\x02\x14a\x05x6`\x04a*HV[a\x0E\xECV[4\x80\x15a\x05\x88W_\x80\xFD[P`\x0BT`\x0CTa\x05\xAC\x91`\x01`\x01`@\x1B\x03\x80\x82\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x94\x85\x16\x81R\x93\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\x02jV[4\x80\x15a\x05\xDDW_\x80\xFD[Pa\x04\xB7a\x05\xEC6`\x04a%\x9EV[a\x0FuV[4\x80\x15a\x05\xFCW_\x80\xFD[Pa\x06!`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02j\x91\x90a*\x83V[4\x80\x15a\x069W_\x80\xFD[P_Ta\x032\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06WW_\x80\xFD[Pa\x02\x14a\x06f6`\x04a*\xB5V[\x80Q`\x0B\x80T` \x84\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x93\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90U`@\x01Q`\x0CUV[4\x80\x15a\x06\xAEW_\x80\xFD[P`\rTa\x06\xC6\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02jV[4\x80\x15a\x06\xE6W_\x80\xFD[Pa\x02\x14`\x0F\x80T`\xFF\x19\x16\x90UV[4\x80\x15a\x07\x01W_\x80\xFD[P`\tT`\nTa\x05\xAC\x91`\x01`\x01`@\x1B\x03\x80\x82\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x83V[4\x80\x15a\x070W_\x80\xFD[Pa\x04\xB7a\x07?6`\x04a*\xCFV[a\x0F\xB7V[4\x80\x15a\x07OW_\x80\xFD[Pa\x02\x14a\x07^6`\x04a%XV[a\x0F\xEAV[4\x80\x15a\x07nW_\x80\xFD[Pa\x02\x14a\x07}6`\x04a*\xEFV[a\x10,V[4\x80\x15a\x07\x8DW_\x80\xFD[P`\x0ETa\x03\x8FV[a\x07\x9Ea\x10\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xC5W`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x07\xF4W`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x80\x17\xBB\x88\x7F\xDF\x8F\xCAC\x14\xA9\xD4\x0Fns\xB3\xB8\x10\x02\xD6~\\\xFA\x85\xD8\x81s\xAFj\xA4`r\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x0E\x81\x81T\x81\x10a\x08XW_\x80\xFD[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x80\x83\x16\x93P`\x01`@\x1B\x83\x04\x81\x16\x92`\x01`\x80\x1B\x90\x04\x16\x90\x84V[a\x08\x9Aa\x112V[a\x08\xA3\x82a\x11\xD6V[a\x08\xAD\x82\x82a\x12\x17V[PPV[_a\x08\xBAa\x12\xD8V[P_\x80Q` a1|\x839\x81Q\x91R\x90V[a\x08\xD4a\x10\xD7V[`\rT`\x01`\x01`\xA0\x1B\x03\x16\x15a\t\x1FW`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`@Q\x7F\x9A_W\xDE\x85m\xD6h\xC5M\xD9^\\U\xDF\x93C!q\xCB\xCAI\xA8wmV \xEAY\xC0$P\x90_\x90\xA1V[`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\tBa\x10\xD7V[a\t8_a\x13!V[`\rT`\x01`\x01`\xA0\x1B\x03\x16\x15\x15\x80\x15a\tpWP`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\t\x8EW`@Qc\x01GL\x8F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x83Q`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x11\x15\x80a\t\xC7WP`\x0BT` \x84\x01Q`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x92\x04\x82\x16\x91\x16\x11\x15[\x15a\t\xE5W`@Qc\x05\x1CF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xF2\x83`@\x01Qa\x13\x91V[_a\t\xFBa\x0C\x16V[` \x85\x01Q_\x80T\x92\x93P\x91a\n\x1A\x91\x90`\x01`\x01`@\x1B\x03\x16a\x0E\x8FV[\x90Pa\n'\x82`\x01a+\xF1V[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14\x80\x15a\n`WP`\x0BTa\n^\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0FuV[\x15[\x80\x15a\ntWP_\x82`\x01`\x01`@\x1B\x03\x16\x11[\x15a\n\x92W`@QcqP\xDEE`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x9D\x82`\x02a+\xF1V[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x10a\n\xCEW`@QcqP\xDEE`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xDB\x84` \x01Qa\x13\x91V[a\n\xE8\x84`@\x01Qa\x13\x91V[a\n\xF5\x84``\x01Qa\x13\x91V[a\x0B\0\x85\x85\x85a\x14\x01V[\x84Q`\x0B\x80T` \x88\x01Q`\x01`\x01`@\x1B\x03\x81\x81\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x94\x16\x93\x90\x93\x17\x17\x90U`@\x86\x01Q`\x0CUa\x0BB\x90a\x0FuV[\x15a\x0B\xACW\x83Q`\x05U` \x84\x01Q`\x06U`@\x84\x01Q`\x07U``\x84\x01Q`\x08U\x7F1\xEA\xBD\x90\x99\xFD\xB2]\xAC\xDD\xD2\x06\xAB\xFF\x871\x1EU4A\xFC\x9D\x0F\xCD\xEF \x10b\xD7\xE7\x07\x1Ba\x0B\x90\x82`\x01a+\xF1V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1[a\x0B\xB7CB\x87a\x15YV[\x84` \x01Q`\x01`\x01`@\x1B\x03\x16\x85_\x01Q`\x01`\x01`@\x1B\x03\x16\x7F\xA0Jw9$PZA\x85d67%\xF5h2\xF5w.k\x8D\r\xBDn\xFC\xE7$\xDF\xE8\x03\xDA\xE6\x87`@\x01Q`@Qa\x0C\x07\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPV[`\x0BT_\x80T\x90\x91a\x0C;\x91`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x92\x04\x82\x16\x91\x16a\x0E\x8FV[\x90P\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x0C\x84WP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x0C\x9FWP0;\x15[\x90P\x81\x15\x80\x15a\x0C\xADWP\x80\x15[\x15a\x0C\xCBW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x0C\xF5W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x0C\xFE\x87a\x17BV[a\r\x06a\x17SV[a\r\x12\x8A\x8A\x8A\x89a\x17[V[\x83\x15a\rXW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\x0E\x80T_\x91\x82\x91\x90a\rx`\x01\x83a,\x11V[\x81T\x81\x10a\r\x88Wa\r\x88a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x84\x10a\r\xC7W`@Qc\x18V\xA4\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16[\x81\x81\x10\x15a\x0E\x88W\x84`\x0E\x82\x81T\x81\x10a\r\xF7Wa\r\xF7a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\x80W`\x0E\x81\x81T\x81\x10a\x0E0Wa\x0E0a,$V[\x90_R` _ \x90`\x02\x02\x01`\x01\x01T`\x0E\x82\x81T\x81\x10a\x0ESWa\x0ESa,$V[\x90_R` _ \x90`\x02\x02\x01_\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x93P\x93PPP\x91P\x91V[`\x01\x01a\r\xDBV[PP\x91P\x91V[_\x81`\x01`\x01`@\x1B\x03\x16_\x03a\x0E\xA7WP_a\x0E\xE6V[a\x0E\xB1\x82\x84a,LV[`\x01`\x01`@\x1B\x03\x16_\x03a\x0E\xD1Wa\x0E\xCA\x82\x84a,qV[\x90Pa\x0E\xE6V[a\x0E\xDB\x82\x84a,qV[a\x0E\xCA\x90`\x01a+\xF1V[\x92\x91PPV[a\x0E\xF4a\x10\xD7V[a\x0E\x10\x81c\xFF\xFF\xFF\xFF\x16\x10\x80a\x0F\x13WPc\x01\xE13\x80\x81c\xFF\xFF\xFF\xFF\x16\x11[\x80a\x0F1WP`\rTc\xFF\xFF\xFF\xFF`\x01`\xA0\x1B\x90\x91\x04\x81\x16\x90\x82\x16\x11\x15[\x15a\x0FOW`@Qc\x07\xA5\x07w`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\r\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x81`\x01`\x01`@\x1B\x03\x16_\x03a\x0F\x8DWP_\x91\x90PV[_Ta\x0F\xA2\x90`\x01`\x01`@\x1B\x03\x16\x83a,LV[`\x01`\x01`@\x1B\x03\x16\x15\x92\x91PPV[\x91\x90PV[`\x0FT_\x90`\xFF\x16a\x0F\xD2Wa\x0F\xCD\x83\x83a\x18\xD5V[a\x0F\xE3V[\x81`\x10T\x84a\x0F\xE1\x91\x90a,\x11V[\x11[\x93\x92PPPV[a\x0F\xF2a\x10\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10 W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x10)\x81a\x13!V[PV[a\x107`\x0E_a\"vV[_[\x81Q\x81\x10\x15a\x08\xADW`\x0E\x82\x82\x81Q\x81\x10a\x10VWa\x10Va,$V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x83\x90 \x82Q`\x02\x90\x92\x02\x01\x80T\x93\x83\x01Q`@\x84\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x92\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x97\x16\x91\x90\x94\x16\x17\x94\x90\x94\x17\x93\x90\x93\x16\x17\x82U``\x01Q\x90\x82\x01U\x01a\x109V[3a\x11\t\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t8W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x10\x17V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x11\xB8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x11\xAC_\x80Q` a1|\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\t8W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xDEa\x10\xD7V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x90` \x01a\x08>V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x12qWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12n\x91\x81\x01\x90a,\x96V[`\x01[a\x12\x99W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x10\x17V[_\x80Q` a1|\x839\x81Q\x91R\x81\x14a\x12\xC9W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x10\x17V[a\x12\xD3\x83\x83a\x1A-V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t8W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x81\x10\x80a\x08\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FBn254: invalid scalar field\0\0\0\0\0`D\x82\x01R`d\x01a\x10\x17V[_a\x14\na\x1A\x82V[\x90Pa\x14\x14a\"\x94V[\x84Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R` \x80\x87\x01\x80Q\x90\x92\x16\x90\x83\x01R`@\x80\x87\x01Q\x90\x83\x01R`\x06T``\x83\x01R`\x07T`\x80\x83\x01R`\x08T`\xA0\x83\x01R`\x05T`\xC0\x83\x01RQa\x14e\x90a\x0FuV[\x15a\x14\x97W` \x84\x01Q`\xE0\x82\x01R`@\x84\x01Qa\x01\0\x82\x01R``\x84\x01Qa\x01 \x82\x01R\x83Qa\x01@\x82\x01Ra\x14\xBBV[`\x06T`\xE0\x82\x01R`\x07Ta\x01\0\x82\x01R`\x08Ta\x01 \x82\x01R`\x05Ta\x01@\x82\x01R[`@Qc\xFC\x86`\xC7`\xE0\x1B\x81Rs_\xBD\xB21Vx\xAF\xEC\xB3g\xF02\xD9?d/d\x18\n\xA3\x90c\xFC\x86`\xC7\x90a\x14\xF6\x90\x85\x90\x85\x90\x88\x90`\x04\x01a.\xB9V[` `@Q\x80\x83\x03\x81\x86Z\xF4\x15\x80\x15a\x15\x11W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x155\x91\x90a0\xE7V[a\x15RW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x0ET\x15\x80\x15\x90a\x15\xCEWP`\rT`\x0E\x80T`\x01`\xA0\x1B\x83\x04c\xFF\xFF\xFF\xFF\x16\x92`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x81\x10a\x15\x99Wa\x15\x99a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01Ta\x15\xC3\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x84a1\x06V[`\x01`\x01`@\x1B\x03\x16\x11[\x15a\x16aW`\rT`\x0E\x80T\x90\x91`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x81\x10a\x15\xFBWa\x15\xFBa,$V[_\x91\x82R` \x82 `\x02\x90\x91\x02\x01\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x81U`\x01\x01U`\r\x80T`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90`\x18a\x16;\x83a1&V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPP[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`@\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x80\x85\x01\x91\x82R\x83\x01Q\x85\x16\x84\x83\x01\x90\x81R\x92\x90\x91\x01Q``\x84\x01\x90\x81R`\x0E\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x93Q`\x02\x90\x94\x02\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x81\x01\x80T\x93Q\x94Q\x87\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x96\x90\x97\x16\x95\x90\x95\x17\x92\x90\x92\x17\x92\x90\x92\x16\x93\x90\x93\x17\x90\x91UQ\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x90\x91\x01UV[a\x17Ja \xB1V[a\x10)\x81a \xFAV[a\t8a \xB1V[\x83Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80a\x17\x7FWP` \x84\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80a\x17\x8CWP` \x83\x01Q\x15[\x80a\x17\x99WP`@\x83\x01Q\x15[\x80a\x17\xA6WP``\x83\x01Q\x15[\x80a\x17\xB0WP\x82Q\x15[\x80a\x17\xC2WPa\x0E\x10\x82c\xFF\xFF\xFF\xFF\x16\x10[\x80a\x17\xD6WPc\x01\xE13\x80\x82c\xFF\xFF\xFF\xFF\x16\x11[\x80a\x17\xE8WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x18\x06W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\t\x80T` \x80\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x93\x84\x16\x95\x82\x16\x95\x86\x17\x81\x17\x90\x94U`@\x98\x89\x01Q`\n\x81\x90U`\x0B\x80T\x90\x94\x16\x90\x95\x17\x90\x93\x17\x90\x91U`\x0C\x92\x90\x92U\x84Q`\x01\x81\x90U\x91\x85\x01Q`\x02\x81\x90U\x95\x85\x01Q`\x03\x81\x90U``\x90\x95\x01Q`\x04\x81\x90U`\x05\x92\x90\x92U`\x06\x95\x90\x95U`\x07\x93\x90\x93U`\x08\x92\x90\x92U`\r\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U_\x80T\x91\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x90\x91\x16\x17\x90UV[`\x0ET_\x90C\x84\x11\x80a\x18\xE6WP\x80\x15[\x80a\x190WP`\rT`\x0E\x80T\x90\x91`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x81\x10a\x19\x14Wa\x19\x14a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x84\x10[\x15a\x19NW`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\x19\\`\x01\x85a,\x11V[\x90P[\x81a\x19\xF8W`\rT`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x10a\x19\xF8W\x86`\x0E\x82\x81T\x81\x10a\x19\x91Wa\x19\x91a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x11a\x19\xE6W`\x01\x91P`\x0E\x81\x81T\x81\x10a\x19\xC6Wa\x19\xC6a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x92Pa\x19\xF8V[\x80a\x19\xF0\x81a1KV[\x91PPa\x19_V[\x81a\x1A\x16W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x1A!\x84\x89a,\x11V[\x11\x97\x96PPPPPPPV[a\x1A6\x82a!\x02V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x1AzWa\x12\xD3\x82\x82a!eV[a\x08\xADa!\xD7V[a\x1A\x8Aa\"\xB3V[b\x01\0\0\x81R`\x0B` \x82\x01R\x7F/\xAFZ\x11>\xFD\x87\xD7X\x18\xE6?\xF9\xA6\x17\0\x07\xF2,\x89\xBB\xC4\xA8\xBD\x0F+H&\x87W\xB0\x14`@\x82\x01QR\x7F\x18Z\xEE\x05\xF8\xD3\xBA\xBF\xCEg\x93\x1F\x15\xDB9\xE6\x1F%\xF7\x94\xA4\x13M{\xEEn\x18\xC5\xAD\x1E\xC0W` `@\x83\x01Q\x01R\x7F\r\xCC\xF5\xDC\xF6g\xA3|\xA9;\x8Dr\x10\x91\xD8\xF3\xA8\x04\x9B=\x1E\x89\xA5mf\xE4'Q\xBB\xAF{^``\x82\x01QR\x7F,\xF1\tI\xFC[\xFC\xEC\xB3\xBCT\xDDA!\xE5X\x07C\x0F\x17\xF3\x04\x98\xA7\xEAj\x02`p\xB1\x91b` ``\x83\x01Q\x01R\x7F\x08\xD7\x0EN\x01\x84\xFES\xBDVo\r~\xDCL\xD7\xB0\xE39I\ts\xD0\xFA\xEC}\xAC \x89\xF58\xE5`\x80\x82\x01QR~\xF6e\xFE\x1F\xD1\x10\xD3}\x1D\xEADn\x84\0\xF0o\x06\xB9\xB5\x8A\xB3\xDF\x90\xFB\xAE|G\xEEX`A` `\x80\x83\x01Q\x01R\x7F\x08~\x14\xD7\x19$\xAC\x0F(\x80\xAD\xF0\xF1\x06\x92^Zo\xDDW\xD0 \xBB<\x8A\xA7\x0F\xA9\xFC\0\xCC\xF3`\xA0\x82\x01QR\x7F\x01\xDB~1x\xB3B\xF9\x1DT\xFC\x97,\xEErV\x9FB\x9A99\x88\xEEC\xC2\x89\xE2\xED\x96\x07qR` `\xA0\x83\x01Q\x01R\x7F\x19m\xD4-vr\x01\xF7\xF1\x96\xC4*\xAE\xF4\x85e`F1\x0FP\x83U\x95\x92\xBD\x13\x13\xE1iH\xB7`\xC0\x82\x01QR\x7F\x17\x88\x96\x80\x81\n\xAA\xBD\x1F\xF3\xACJlT\x92\x10\x05y\xE0Y\x17\x0C\xD2\xB7~+=\xA6\xD3|\xC2F` `\xC0\x83\x01Q\x01R\x7F$\x93^zw\xAC1?\xD3\xD6\x0F\xF3\xF1\xA0\xA7\x9E\xC3,}\xC5\x19\xB3\x9D\xA0\xAC\xB2\xC4\x9F6wq\xCC`\xE0\x82\x01QR\x7F\x16\x8E)B^\xF18\xCBiC\xC7SR\xF3<\x19\x0E_\x14\x88\xEBT\xA9\xE1\x1D\xEBtM\xA7\xFBk.` `\xE0\x83\x01Q\x01R\x7F\x1BX\xD5X\xB5RdS\xBD\x10(\xCA\x93\x8C\x94\x0B\xB8\x9Er?|5x|\x02\xF9\xF1y\xAE\x9A\x0C\xEAa\x01\0\x82\x01QR\x7F!\xAF\xC1!\xD9\x1D\x9D\x1C\x17\xDA\xFB\x926\xBC\x9B\x87,[C\xDF\x06L\x0B\x12\x86\x01/\xB4:v#$` a\x01\0\x83\x01Q\x01R\x7F\x10G\xFCUyM\x1EY}\xE1U\x07v\x11\xE3\xC7\x89\xA0\xA2\xBE\x02\x188!\xBB\xA5l\xF6\x1C\xC1\xB8\xEDa\x01 \x82\x01QR\x7F\x17BR2G'\xC0\xD2\xEE^P\xEBW\xA5#\x1FgGL\xEE\xD6\x93*\xD4\xFF\xE9\xBC\xF8f\xAA4(` a\x01 \x83\x01Q\x01R\x7F(\xDB(\x9AL\xFBs\xBA\x92\x96\x15r\xF3\x18R\x98\xAE6n\xD1\xA4Iq`{\xCB\xF8\x01\xF1 \xF5aa\x01@\x82\x01QR\x7F\x04\\\xFEz\xE2\xCD\x17U\x08\x17.}\x9C.\x89\x9B\xB1\xD2\x16\xDF\xC3\x1F\xE8\x9F\xC6\xC9\x17\xCA\xAE\xE8w\xA2` a\x01@\x83\x01Q\x01R\x7F\x19_.\xEC\x85Gr\x7F\xC4n\xD0\x1By\xE8\xF6f\xDE\xD6J\xE5OW\x078t\xA5\xA2G\x03\x80\xA7\x85a\x01`\x82\x01QR\x7F\x15'2.\x85\xDA\x1A\xEF\xBD\x83\x9Ee\xD1\x1D\xC6\x95\xAA\xC1k\r\xB6\xC6%\x91\xD9\x812B\xD4\x1C\xBE1` a\x01`\x83\x01Q\x01R\x7F\x10\xC8\xD7\xD75_~\x0F\x8C\0/H,\xC3\xB9\x8C\x90\xBA\xA9Ba\xC5\x9A\x17\xB4$\xEE\xCF\xE4\xE9c\xB2a\x01\x80\x82\x01QR\x7F\"r\xE3\x01xdqg\xBB\xEA\xD3\xA2\xD77\x19\x88\xF2\xE1\x98\xE6X\x15\x02\x9D\xEDLd\xBF\xC0\x85\x0F\x1F` a\x01\x80\x83\x01Q\x01R\x7F\x15\xD5n\xA7\xAB/\xA6\x12e\xF5Q\xC2\xAE%8\x9C\x8F\xE7\xBC\xB3\xBFf\x08\x08,6\xA2\x01\xF2%\xF7}a\x01\xA0\x82\x01QR\x7F\x0BXTh\x87 .rs\xD3\xD0\xC5]e\xDDa2\xCA\xC9\x8E\xBF\x04\xEF\xB1\xB5$E\xC5\x13\xC4\xA4\xDF` a\x01\xA0\x83\x01Q\x01R\x7F\x05\roCwN\x8D\xFF\xAA\x86\x8F*}\xC8/Vli\xD1u\xD8\x18\xD4Q|\xC7\n\xC5\xFC\xB2\xF1\xB1a\x01\xC0\x82\x01QR\x7F/\xFF\x87\xBF`^\x99\x83s\xBBdU?:b]\xAB\xCD\x12\x88\x86\x92\xD6x\xA8\xF4M\x13d@\xC8c` a\x01\xC0\x83\x01Q\x01R\x7F\x12\xD0\x85`\x8C`,\xFB[\x8C\x03\xEC{\xD1:\xC0\xFF\x9Ed\xA9\xAC\x1E\x9A\xA7FYJ\x03>FK\xF2a\x01\xE0\x82\x01QR\x7F\x18\xACZ56\x04.\xEB\x0B\x0C|/C\xF5\xE2\xCA;!s\xDA\xA4\xC2\x81/\xFC\xA6G\x87\xE8\xE9V\xB2` a\x01\xE0\x83\x01Q\x01R\x7F\x0F\x0F\x98\x91\xFC+y\x0Et\xDC%<\x88T\xDFc\x92\xE0\x10\xF4\xDEg`\xB8B:=\xD6\x9B\xBE]\xCEa\x02\0\x82\x01QR\x7F\x16\xBE\xD1\xD2D\xA2\xFE:\xB9\xA6R\xC7\xFE\xECVP\x16\x1D\x8Au\"}\xEC\xE7)O<\x8F\xC5B\xFDl` a\x02\0\x83\x01Q\x01R\x7F\x0F\xA3m\0g/\xA6\xA1\xC4L\xD3\xC2Y!,\x1A\xDAH\xC6k\xF7\xBB\x08_$G\x1B\x15\xB1~nQa\x02 \x82\x01QR\x7F\x18 \x88\xE5kd\x95R2F\x08\x91\xD2\xB2yvS%\x81:\xEF\x1D\xAE\x85^_IlA\x8A\xFCA` a\x02 \x83\x01Q\x01R\x7F+\xAFZ\xE2\xDD\x83.\x14I\xFA\xCCa\x1Bk\x80\xFDf\xD5\x8C\x87\x1DX'\xC5\xC8\xE2tpd\xE2\x99da\x02@\x82\x01QR\x7F)\xF5C\xB5C\x13~\x88\x18\x04\xC9\x89\xCD;\x99\x93@\x10\0\"8\xE8\xAB>\xEC\x88.\t\xD3\x06h\x1F` a\x02@\x83\x01Q\x01R\x7F-\xB0\xDD\xC7\x12;B\xF5 \xE2WFj\r\x92\xDA\x8BVO\xE0\x1E\xC6e\tl\x14\x11\x96C\x01)\x84a\x02`\x82\x01QR\x7F\x1Bz\xB2zf\x96b\x84\xD7\xFB)\xBC\xE9\xD5P\xEA\xFB\xA1lI\xFB\xC6&x'\xCD\xFC\x8D\x0B\x16\xF9O` a\x02`\x83\x01Q\x01R\x7F\xB0\x83\x88\x93\xEC\x1F#~\x8B\x072;\x07DY\x9FN\x97\xB5\x98\xB3\xB5\x89\xBC\xC2\xBC7\xB8\xD5\xC4\x18\x01a\x02\x80\x82\x01R\x7F\xC1\x83\x93\xC0\xFA0\xFEN\x8B\x03\x8E5z\xD8Q\xEA\xE8\xDE\x91\x07XN\xFF\xE7\xC7\xF1\xF6Q\xB2\x01\x0E&a\x02\xA0\x82\x01R\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\t8W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xF2a \xB1V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a!7W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x10\x17V[_\x80Q` a1|\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa!\x81\x91\x90a1`V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a!\xB9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\xBEV[``\x91P[P\x91P\x91Pa!\xCE\x85\x83\x83a!\xF6V[\x95\x94PPPPPV[4\x15a\t8W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\"\x06Wa\x0F\xCD\x82a\"MV[\x81Q\x15\x80\x15a\"\x1DWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\"FW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x10\x17V[P\x92\x91PPV[\x80Q\x15a\"]W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T_\x82U`\x02\x02\x90_R` _ \x90\x81\x01\x90a\x10)\x91\x90a%\x19V[`@Q\x80a\x01`\x01`@R\x80`\x0B\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01a\"\xE6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\x06`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#&`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#F`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#f`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\x86`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\xA6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\xC6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\xE6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\x06`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$&`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$F`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$f`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\x86`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\xA6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\xC6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\xE6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a%\x06`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[[\x80\x82\x11\x15a%>W\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x81U_`\x01\x82\x01U`\x02\x01a%\x1AV[P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xB2W_\x80\xFD[_` \x82\x84\x03\x12\x15a%hW_\x80\xFD[a\x0F\xE3\x82a%BV[_` \x82\x84\x03\x12\x15a%\x81W_\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0F\xB2W_\x80\xFD[_` \x82\x84\x03\x12\x15a%\xAEW_\x80\xFD[a\x0F\xE3\x82a%\x88V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xEDWa%\xEDa%\xB7V[`@R\x90V[`@Qa\x02\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xEDWa%\xEDa%\xB7V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&>Wa&>a%\xB7V[`@R\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a&WW_\x80\xFD[a&`\x83a%BV[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&|W_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a&\x8FW_\x80\xFD[\x815\x81\x81\x11\x15a&\xA1Wa&\xA1a%\xB7V[a&\xB3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a&\x16V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a&\xC8W_\x80\xFD[\x80\x84\x84\x01\x85\x84\x017_\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[_``\x82\x84\x03\x12\x15a&\xF5W_\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a'\x17Wa'\x17a%\xB7V[`@R\x90P\x80a'&\x83a%\x88V[\x81Ra'4` \x84\x01a%\x88V[` \x82\x01R`@\x83\x015`@\x82\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a'ZW_\x80\xFD[a'ba%\xCBV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_`@\x82\x84\x03\x12\x15a'\x9CW_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a'\xBEWa'\xBEa%\xB7V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x80_\x83\x85\x03a\x05`\x81\x12\x15a'\xECW_\x80\xFD[a'\xF6\x86\x86a&\xE5V[\x93Pa(\x05\x86``\x87\x01a'JV[\x92Pa\x04\x80\x80`\xDF\x19\x83\x01\x12\x15a(\x1AW_\x80\xFD[a(\"a%\xF3V[\x91Pa(1\x87`\xE0\x88\x01a'\x8CV[\x82Ra\x01 a(B\x88\x82\x89\x01a'\x8CV[` \x84\x01Ra\x01`a(V\x89\x82\x8A\x01a'\x8CV[`@\x85\x01Ra\x01\xA0a(j\x8A\x82\x8B\x01a'\x8CV[``\x86\x01Ra\x01\xE0a(~\x8B\x82\x8C\x01a'\x8CV[`\x80\x87\x01Ra\x02 a(\x92\x8C\x82\x8D\x01a'\x8CV[`\xA0\x88\x01Ra\x02`a(\xA6\x8D\x82\x8E\x01a'\x8CV[`\xC0\x89\x01Ra\x02\xA0a(\xBA\x8E\x82\x8F\x01a'\x8CV[`\xE0\x8A\x01Ra(\xCD\x8Ea\x02\xE0\x8F\x01a'\x8CV[a\x01\0\x8A\x01Ra(\xE1\x8Ea\x03 \x8F\x01a'\x8CV[\x87\x8A\x01Ra(\xF3\x8Ea\x03`\x8F\x01a'\x8CV[a\x01@\x8A\x01Ra)\x07\x8Ea\x03\xA0\x8F\x01a'\x8CV[\x86\x8A\x01Ra)\x19\x8Ea\x03\xE0\x8F\x01a'\x8CV[a\x01\x80\x8A\x01Ra\x04 \x8D\x015\x85\x8A\x01Ra\x04@\x8D\x015a\x01\xC0\x8A\x01Ra\x04`\x8D\x015\x84\x8A\x01R\x87\x8D\x015a\x02\0\x8A\x01Ra\x04\xA0\x8D\x015\x83\x8A\x01Ra\x04\xC0\x8D\x015a\x02@\x8A\x01Ra\x04\xE0\x8D\x015\x82\x8A\x01Ra\x05\0\x8D\x015a\x02\x80\x8A\x01Ra\x05 \x8D\x015\x81\x8A\x01RPPPPPPPPa\x05@\x85\x015a\x02\xC0\x82\x01R\x80\x91PP\x92P\x92P\x92V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xB2W_\x80\xFD[_\x80_\x80_a\x01@\x86\x88\x03\x12\x15a)\xC6W_\x80\xFD[a)\xD0\x87\x87a&\xE5V[\x94Pa)\xDF\x87``\x88\x01a'JV[\x93Pa)\xED`\xE0\x87\x01a)\x9EV[\x92Pa)\xFCa\x01\0\x87\x01a%BV[\x91Pa*\x0Ba\x01 \x87\x01a%\x88V[\x90P\x92\x95P\x92\x95\x90\x93PV[_\x80`@\x83\x85\x03\x12\x15a*(W_\x80\xFD[a*1\x83a%\x88V[\x91Pa*?` \x84\x01a%\x88V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a*XW_\x80\xFD[a\x0F\xE3\x82a)\x9EV[_[\x83\x81\x10\x15a*{W\x81\x81\x01Q\x83\x82\x01R` \x01a*cV[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra*\xA1\x81`@\x85\x01` \x87\x01a*aV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[_``\x82\x84\x03\x12\x15a*\xC5W_\x80\xFD[a\x0F\xE3\x83\x83a&\xE5V[_\x80`@\x83\x85\x03\x12\x15a*\xE0W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x80\x83\x85\x03\x12\x15a+\0W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a+\x16W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a+)W_\x80\xFD[\x815\x81\x81\x11\x15a+;Wa+;a%\xB7V[a+I\x84\x82`\x05\x1B\x01a&\x16V[\x81\x81R\x84\x81\x01\x92P`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a+hW_\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a+\xD2W`\x80\x84\x89\x03\x12\x15a+\x84W_\x80\xFD[a+\x8Ca%\xCBV[a+\x95\x85a%\x88V[\x81Ra+\xA2\x86\x86\x01a%\x88V[\x86\x82\x01R`@a+\xB3\x81\x87\x01a%\x88V[\x90\x82\x01R``\x85\x81\x015\x90\x82\x01R\x83R`\x80\x90\x93\x01\x92\x91\x84\x01\x91a+mV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\"FWa\"Fa+\xDDV[\x81\x81\x03\x81\x81\x11\x15a\x0E\xE6Wa\x0E\xE6a+\xDDV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a,eWa,ea,8V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a,\x8AWa,\x8Aa,8V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[_` \x82\x84\x03\x12\x15a,\xA6W_\x80\xFD[PQ\x91\x90PV[\x80_[`\x0B\x81\x10\x15a,\xCFW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a,\xB0V[PPPPV[a,\xEA\x82\x82Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[` \x81\x81\x01Q\x80Q`@\x85\x01R\x90\x81\x01Q``\x84\x01RP`@\x81\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP``\x81\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\x80\x81\x01Qa\x01\0a-M\x81\x85\x01\x83\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xA0\x83\x01Q\x91Pa\x01@a-m\x81\x86\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xC0\x84\x01Q\x92Pa\x01\x80a-\x8D\x81\x87\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xE0\x85\x01Q\x93Pa\x01\xC0a-\xAD\x81\x88\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x85\x01Q\x93Pa\x02\0\x92a-\xCD\x87\x85\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01 \x86\x01Q\x94Pa\x02@a-\xEE\x81\x89\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x86\x01Q\x94Pa\x02\x80\x92a.\x0E\x88\x85\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01`\x87\x01Q\x95Pa\x02\xC0a./\x81\x8A\x01\x88\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x87\x01Q\x80Qa\x03\0\x8A\x01R` \x01Qa\x03 \x89\x01Ra\x01\xA0\x87\x01Qa\x03@\x89\x01R\x90\x86\x01Qa\x03`\x88\x01Ra\x01\xE0\x86\x01Qa\x03\x80\x88\x01R\x92\x85\x01Qa\x03\xA0\x87\x01Ra\x02 \x85\x01Qa\x03\xC0\x87\x01R\x91\x84\x01Qa\x03\xE0\x86\x01Ra\x02`\x84\x01Qa\x04\0\x86\x01R\x83\x01Qa\x04 \x85\x01Ra\x02\xA0\x83\x01Qa\x04@\x85\x01R\x90\x91\x01Qa\x04`\x90\x92\x01\x91\x90\x91RPV[_a\n\xE0\x82\x01\x90P\x84Q\x82R` \x85\x01Q` \x83\x01R`@\x85\x01Qa.\xEB`@\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x85\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP`\x80\x85\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\xA0\x85\x01Qa\x01\0a/8\x81\x85\x01\x83\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xC0\x87\x01Q\x91Pa\x01@a/X\x81\x86\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xE0\x88\x01Q\x92Pa\x01\x80a/x\x81\x87\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x88\x01Q\x92Pa\x01\xC0\x91a/\x98\x86\x84\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01 \x89\x01Q\x93Pa\x02\0a/\xB9\x81\x88\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x89\x01Q\x93Pa\x02@\x91a/\xD9\x87\x84\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01`\x8A\x01Q\x94Pa\x02\x80a/\xFA\x81\x89\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x8A\x01Q\x80Qa\x02\xC0\x89\x01R` \x90\x81\x01Qa\x02\xE0\x89\x01Ra\x01\xA0\x8B\x01Q\x80Qa\x03\0\x8A\x01R\x81\x01Qa\x03 \x89\x01R\x93\x8A\x01Q\x80Qa\x03@\x89\x01R\x84\x01Qa\x03`\x88\x01Ra\x01\xE0\x8A\x01Q\x80Qa\x03\x80\x89\x01R\x84\x01Qa\x03\xA0\x88\x01R\x89\x01Q\x80Qa\x03\xC0\x88\x01R\x83\x01Qa\x03\xE0\x87\x01Ra\x02 \x89\x01Q\x80Qa\x04\0\x88\x01R\x83\x01Qa\x04 \x87\x01R\x90\x88\x01Q\x80Qa\x04@\x87\x01R\x82\x01Qa\x04`\x86\x01Ra\x02`\x88\x01Q\x80Qa\x04\x80\x87\x01R\x90\x91\x01Qa\x04\xA0\x85\x01R\x86\x01Qa\x04\xC0\x84\x01RPa\x02\xA0\x85\x01Qa\x04\xE0\x83\x01Ra0\xD1a\x05\0\x83\x01\x85a,\xADV[a0\xDFa\x06`\x83\x01\x84a,\xD5V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a0\xF7W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F\xE3W_\x80\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\"FWa\"Fa+\xDDV[_`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03a1AWa1Aa+\xDDV[`\x01\x01\x93\x92PPPV[_\x81a1YWa1Ya+\xDDV[P_\x19\x01\x90V[_\x82Qa1q\x81\x84` \x87\x01a*aV[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA1dsolcC\0\x08\x17\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101f1575f3560e01c8063826e41fc11610108578063b2424e3f1161009d578063d24d933d1161006d578063d24d933d146106f6578063e030330114610725578063f2fde38b14610744578063f567616014610763578063f9e50d1914610782575f80fd5b8063b2424e3f1461062e578063b5adea3c1461064c578063c23b9e9e146106a3578063c8e5e498146106db575f80fd5b806396c1ca61116100d857806396c1ca611461055e5780639fdb54a71461057d578063a1be8d52146105d2578063ad3cb1cc146105f1575f80fd5b8063826e41fc1461049c5780638584d23f146104c75780638da5cb5b1461050357806390c143901461053f575f80fd5b80633c23b6db1161018957806369cc6a041161015957806369cc6a0414610422578063715018a614610436578063757c37ad1461044a5780637667180814610469578063811f853f1461047d575f80fd5b80633c23b6db1461039d578063426d3194146103d95780634f1ef286146103fb57806352d1902d1461040e575f80fd5b80632d52aad6116101c45780632d52aad6146102e05780632f79889d1461030c578063313df7b11461034a578063378ec23b14610381575f80fd5b8063013fa5fc146101f557806302b592f3146102165780630625e19b146102735780630d8e6e2c146102b5575b5f80fd5b348015610200575f80fd5b5061021461020f366004612558565b610796565b005b348015610221575f80fd5b50610235610230366004612571565b610849565b60405161026a94939291906001600160401b039485168152928416602084015292166040820152606081019190915260800190565b60405180910390f35b34801561027e575f80fd5b506005546006546007546008546102959392919084565b60408051948552602085019390935291830152606082015260800161026a565b3480156102c0575f80fd5b5060408051600181525f602082018190529181019190915260600161026a565b3480156102eb575f80fd5b506102146102fa366004612571565b600f805460ff19166001179055601055565b348015610317575f80fd5b50600d5461033290600160c01b90046001600160401b031681565b6040516001600160401b03909116815260200161026a565b348015610355575f80fd5b50600d54610369906001600160a01b031681565b6040516001600160a01b03909116815260200161026a565b34801561038c575f80fd5b50435b60405190815260200161026a565b3480156103a8575f80fd5b506102146103b736600461259e565b5f805467ffffffffffffffff19166001600160401b0392909216919091179055565b3480156103e4575f80fd5b506001546002546003546004546102959392919084565b610214610409366004612646565b610892565b348015610419575f80fd5b5061038f6108b1565b34801561042d575f80fd5b506102146108cc565b348015610441575f80fd5b5061021461093a565b348015610455575f80fd5b506102146104643660046127d8565b61094b565b348015610474575f80fd5b50610332610c16565b348015610488575f80fd5b506102146104973660046129b1565b610c40565b3480156104a7575f80fd5b50600d546001600160a01b031615155b604051901515815260200161026a565b3480156104d2575f80fd5b506104e66104e1366004612571565b610d64565b604080519283526001600160401b0390911660208301520161026a565b34801561050e575f80fd5b507f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b0316610369565b34801561054a575f80fd5b50610332610559366004612a17565b610e8f565b348015610569575f80fd5b50610214610578366004612a48565b610eec565b348015610588575f80fd5b50600b54600c546105ac916001600160401b0380821692600160401b909204169083565b604080516001600160401b0394851681529390921660208401529082015260600161026a565b3480156105dd575f80fd5b506104b76105ec36600461259e565b610f75565b3480156105fc575f80fd5b50610621604051806040016040528060058152602001640352e302e360dc1b81525081565b60405161026a9190612a83565b348015610639575f80fd5b505f54610332906001600160401b031681565b348015610657575f80fd5b50610214610666366004612ab5565b8051600b805460208401516001600160401b03908116600160401b026001600160801b031990921693169290921791909117905560400151600c55565b3480156106ae575f80fd5b50600d546106c690600160a01b900463ffffffff1681565b60405163ffffffff909116815260200161026a565b3480156106e6575f80fd5b50610214600f805460ff19169055565b348015610701575f80fd5b50600954600a546105ac916001600160401b0380821692600160401b909204169083565b348015610730575f80fd5b506104b761073f366004612acf565b610fb7565b34801561074f575f80fd5b5061021461075e366004612558565b610fea565b34801561076e575f80fd5b5061021461077d366004612aef565b61102c565b34801561078d575f80fd5b50600e5461038f565b61079e6110d7565b6001600160a01b0381166107c55760405163e6c4247b60e01b815260040160405180910390fd5b600d546001600160a01b03908116908216036107f45760405163a863aec960e01b815260040160405180910390fd5b600d80546001600160a01b0319166001600160a01b0383169081179091556040519081527f8017bb887fdf8fca4314a9d40f6e73b3b81002d67e5cfa85d88173af6aa46072906020015b60405180910390a150565b600e8181548110610858575f80fd5b5f918252602090912060029091020180546001909101546001600160401b038083169350600160401b8304811692600160801b9004169084565b61089a611132565b6108a3826111d6565b6108ad8282611217565b5050565b5f6108ba6112d8565b505f8051602061317c83398151915290565b6108d46110d7565b600d546001600160a01b03161561091f57600d80546001600160a01b03191690556040517f9a5f57de856dd668c54dd95e5c55df93432171cbca49a8776d5620ea59c02450905f90a1565b60405163a863aec960e01b815260040160405180910390fd5b565b6109426110d7565b6109385f611321565b600d546001600160a01b0316151580156109705750600d546001600160a01b03163314155b1561098e576040516301474c8f60e71b815260040160405180910390fd5b600b5483516001600160401b0391821691161115806109c75750600b5460208401516001600160401b03600160401b9092048216911611155b156109e55760405163051c46ef60e01b815260040160405180910390fd5b6109f28360400151611391565b5f6109fb610c16565b60208501515f805492935091610a1a91906001600160401b0316610e8f565b9050610a27826001612bf1565b6001600160401b0316816001600160401b0316148015610a605750600b54610a5e90600160401b90046001600160401b0316610f75565b155b8015610a7457505f826001600160401b0316115b15610a9257604051637150de4560e01b815260040160405180910390fd5b610a9d826002612bf1565b6001600160401b0316816001600160401b031610610ace57604051637150de4560e01b815260040160405180910390fd5b610adb8460200151611391565b610ae88460400151611391565b610af58460600151611391565b610b00858585611401565b8451600b805460208801516001600160401b03818116600160401b026001600160801b03199093169416939093171790556040860151600c55610b4290610f75565b15610bac5783516005556020840151600655604084015160075560608401516008557f31eabd9099fdb25dacddd206abff87311e553441fc9d0fcdef201062d7e7071b610b90826001612bf1565b6040516001600160401b03909116815260200160405180910390a15b610bb7434287611559565b84602001516001600160401b0316855f01516001600160401b03167fa04a773924505a418564363725f56832f5772e6b8d0dbd6efce724dfe803dae68760400151604051610c0791815260200190565b60405180910390a35050505050565b600b545f80549091610c3b916001600160401b03600160401b90920482169116610e8f565b905090565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a008054600160401b810460ff1615906001600160401b03165f81158015610c845750825b90505f826001600160401b03166001148015610c9f5750303b155b905081158015610cad575080155b15610ccb5760405163f92ee8a960e01b815260040160405180910390fd5b845467ffffffffffffffff191660011785558315610cf557845460ff60401b1916600160401b1785555b610cfe87611742565b610d06611753565b610d128a8a8a8961175b565b8315610d5857845460ff60401b19168555604051600181527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50505050505050505050565b600e80545f91829190610d78600183612c11565b81548110610d8857610d88612c24565b5f918252602090912060029091020154600160801b90046001600160401b03168410610dc757604051631856a49960e21b815260040160405180910390fd5b600d54600160c01b90046001600160401b03165b81811015610e885784600e8281548110610df757610df7612c24565b5f918252602090912060029091020154600160801b90046001600160401b03161115610e8057600e8181548110610e3057610e30612c24565b905f5260205f20906002020160010154600e8281548110610e5357610e53612c24565b905f5260205f2090600202015f0160109054906101000a90046001600160401b0316935093505050915091565b600101610ddb565b5050915091565b5f816001600160401b03165f03610ea757505f610ee6565b610eb18284612c4c565b6001600160401b03165f03610ed157610eca8284612c71565b9050610ee6565b610edb8284612c71565b610eca906001612bf1565b92915050565b610ef46110d7565b610e108163ffffffff161080610f1357506301e133808163ffffffff16115b80610f315750600d5463ffffffff600160a01b909104811690821611155b15610f4f576040516307a5077760e51b815260040160405180910390fd5b600d805463ffffffff909216600160a01b0263ffffffff60a01b19909216919091179055565b5f816001600160401b03165f03610f8d57505f919050565b5f54610fa2906001600160401b031683612c4c565b6001600160401b03161592915050565b919050565b600f545f9060ff16610fd257610fcd83836118d5565b610fe3565b8160105484610fe19190612c11565b115b9392505050565b610ff26110d7565b6001600160a01b03811661102057604051631e4fbdf760e01b81525f60048201526024015b60405180910390fd5b61102981611321565b50565b611037600e5f612276565b5f5b81518110156108ad57600e82828151811061105657611056612c24565b6020908102919091018101518254600181810185555f94855293839020825160029092020180549383015160408401516001600160401b03908116600160801b0267ffffffffffffffff60801b19928216600160401b026001600160801b031990971691909416179490941793909316178255606001519082015501611039565b336111097f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300546001600160a01b031690565b6001600160a01b0316146109385760405163118cdaa760e01b8152336004820152602401611017565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806111b857507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166111ac5f8051602061317c833981519152546001600160a01b031690565b6001600160a01b031614155b156109385760405163703e46dd60e11b815260040160405180910390fd5b6111de6110d7565b6040516001600160a01b03821681527ff78721226efe9a1bb678189a16d1554928b9f2192e2cb93eeda83b79fa40007d9060200161083e565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611271575060408051601f3d908101601f1916820190925261126e91810190612c96565b60015b61129957604051634c9c8ce360e01b81526001600160a01b0383166004820152602401611017565b5f8051602061317c83398151915281146112c957604051632a87526960e21b815260048101829052602401611017565b6112d38383611a2d565b505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109385760405163703e46dd60e11b815260040160405180910390fd5b7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c19930080546001600160a01b031981166001600160a01b03848116918217845560405192169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a3505050565b7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018110806108ad5760405162461bcd60e51b815260206004820152601b60248201527f426e3235343a20696e76616c6964207363616c6172206669656c6400000000006044820152606401611017565b5f61140a611a82565b9050611414612294565b84516001600160401b0390811682526020808701805190921690830152604080870151908301526006546060830152600754608083015260085460a083015260055460c08301525161146590610f75565b1561149757602084015160e08201526040840151610100820152606084015161012082015283516101408201526114bb565b60065460e08201526007546101008201526008546101208201526005546101408201525b60405163fc8660c760e01b8152735fbdb2315678afecb367f032d93f642f64180aa39063fc8660c7906114f690859085908890600401612eb9565b602060405180830381865af4158015611511573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061153591906130e7565b611552576040516309bde33960e01b815260040160405180910390fd5b5050505050565b600e54158015906115ce5750600d54600e8054600160a01b830463ffffffff1692600160c01b90046001600160401b031690811061159957611599612c24565b5f9182526020909120600290910201546115c390600160401b90046001600160401b031684613106565b6001600160401b0316115b1561166157600d54600e80549091600160c01b90046001600160401b03169081106115fb576115fb612c24565b5f9182526020822060029091020180546001600160c01b031916815560010155600d8054600160c01b90046001600160401b031690601861163b83613126565b91906101000a8154816001600160401b0302191690836001600160401b03160217905550505b604080516080810182526001600160401b03948516815292841660208085019182528301518516848301908152929091015160608401908152600e80546001810182555f91909152935160029094027fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fd81018054935194518716600160801b0267ffffffffffffffff60801b19958816600160401b026001600160801b03199095169690971695909517929092179290921693909317909155517fbb7b4a454dc3493923482f07822329ed19e8244eff582cc204f8554c3620c3fe90910155565b61174a6120b1565b611029816120fa565b6109386120b1565b83516001600160401b031615158061177f575060208401516001600160401b031615155b8061178c57506020830151155b8061179957506040830151155b806117a657506060830151155b806117b057508251155b806117c25750610e108263ffffffff16105b806117d657506301e133808263ffffffff16115b806117e857506001600160401b038116155b15611806576040516350dd03f760e11b815260040160405180910390fd5b8351600980546020808801516001600160401b03908116600160401b026001600160801b03199384169582169586178117909455604098890151600a819055600b8054909416909517909317909155600c92909255845160018190559185015160028190559585015160038190556060909501516004819055600592909255600695909555600793909355600892909255600d805463ffffffff909216600160a01b0263ffffffff60a01b199092169190911790555f80549190921667ffffffffffffffff1991909116179055565b600e545f90438411806118e6575080155b806119305750600d54600e80549091600160c01b90046001600160401b031690811061191457611914612c24565b5f9182526020909120600290910201546001600160401b031684105b1561194e5760405163b0b4387760e01b815260040160405180910390fd5b5f808061195c600185612c11565b90505b816119f857600d54600160c01b90046001600160401b031681106119f85786600e828154811061199157611991612c24565b5f9182526020909120600290910201546001600160401b0316116119e65760019150600e81815481106119c6576119c6612c24565b5f9182526020909120600290910201546001600160401b031692506119f8565b806119f08161314b565b91505061195f565b81611a165760405163b0b4387760e01b815260040160405180910390fd5b85611a218489612c11565b11979650505050505050565b611a3682612102565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611a7a576112d38282612165565b6108ad6121d7565b611a8a6122b3565b620100008152600b60208201527f2faf5a113efd87d75818e63ff9a6170007f22c89bbc4a8bd0f2b48268757b0146040820151527f185aee05f8d3babfce67931f15db39e61f25f794a4134d7bee6e18c5ad1ec0576020604083015101527f0dccf5dcf667a37ca93b8d721091d8f3a8049b3d1e89a56d66e42751bbaf7b5e6060820151527f2cf10949fc5bfcecb3bc54dd4121e55807430f17f30498a7ea6a026070b191626020606083015101527f08d70e4e0184fe53bd566f0d7edc4cd7b0e339490973d0faec7dac2089f538e56080820151527ef665fe1fd110d37d1dea446e8400f06f06b9b58ab3df90fbae7c47ee5860416020608083015101527f087e14d71924ac0f2880adf0f106925e5a6fdd57d020bb3c8aa70fa9fc00ccf360a0820151527f01db7e3178b342f91d54fc972cee72569f429a393988ee43c289e2ed96077152602060a083015101527f196dd42d767201f7f196c42aaef485656046310f5083559592bd1313e16948b760c0820151527f17889680810aaabd1ff3ac4a6c5492100579e059170cd2b77e2b3da6d37cc246602060c083015101527f24935e7a77ac313fd3d60ff3f1a0a79ec32c7dc519b39da0acb2c49f367771cc60e0820151527f168e29425ef138cb6943c75352f33c190e5f1488eb54a9e11deb744da7fb6b2e602060e083015101527f1b58d558b5526453bd1028ca938c940bb89e723f7c35787c02f9f179ae9a0cea610100820151527f21afc121d91d9d1c17dafb9236bc9b872c5b43df064c0b1286012fb43a762324602061010083015101527f1047fc55794d1e597de155077611e3c789a0a2be02183821bba56cf61cc1b8ed610120820151527f174252324727c0d2ee5e50eb57a5231f67474ceed6932ad4ffe9bcf866aa3428602061012083015101527f28db289a4cfb73ba92961572f3185298ae366ed1a44971607bcbf801f120f561610140820151527f045cfe7ae2cd175508172e7d9c2e899bb1d216dfc31fe89fc6c917caaee877a2602061014083015101527f195f2eec8547727fc46ed01b79e8f666ded64ae54f57073874a5a2470380a785610160820151527f1527322e85da1aefbd839e65d11dc695aac16b0db6c62591d9813242d41cbe31602061016083015101527f10c8d7d7355f7e0f8c002f482cc3b98c90baa94261c59a17b424eecfe4e963b2610180820151527f2272e30178647167bbead3a2d7371988f2e198e65815029ded4c64bfc0850f1f602061018083015101527f15d56ea7ab2fa61265f551c2ae25389c8fe7bcb3bf6608082c36a201f225f77d6101a0820151527f0b58546887202e7273d3d0c55d65dd6132cac98ebf04efb1b52445c513c4a4df60206101a083015101527f050d6f43774e8dffaa868f2a7dc82f566c69d175d818d4517cc70ac5fcb2f1b16101c0820151527f2fff87bf605e998373bb64553f3a625dabcd12888692d678a8f44d136440c86360206101c083015101527f12d085608c602cfb5b8c03ec7bd13ac0ff9e64a9ac1e9aa746594a033e464bf26101e0820151527f18ac5a3536042eeb0b0c7c2f43f5e2ca3b2173daa4c2812ffca64787e8e956b260206101e083015101527f0f0f9891fc2b790e74dc253c8854df6392e010f4de6760b8423a3dd69bbe5dce610200820151527f16bed1d244a2fe3ab9a652c7feec5650161d8a75227dece7294f3c8fc542fd6c602061020083015101527f0fa36d00672fa6a1c44cd3c259212c1ada48c66bf7bb085f24471b15b17e6e51610220820151527f182088e56b64955232460891d2b279765325813aef1dae855e5f496c418afc41602061022083015101527f2baf5ae2dd832e1449facc611b6b80fd66d58c871d5827c5c8e2747064e29964610240820151527f29f543b543137e881804c989cd3b99934010002238e8ab3eec882e09d306681f602061024083015101527f2db0ddc7123b42f520e257466a0d92da8b564fe01ec665096c14119643012984610260820151527f1b7ab27a66966284d7fb29bce9d550eafba16c49fbc6267827cdfc8d0b16f94f602061026083015101527fb0838893ec1f237e8b07323b0744599f4e97b598b3b589bcc2bc37b8d5c418016102808201527fc18393c0fa30fe4e8b038e357ad851eae8de9107584effe7c7f1f651b2010e266102a082015290565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0054600160401b900460ff1661093857604051631afcd79f60e31b815260040160405180910390fd5b610ff26120b1565b806001600160a01b03163b5f0361213757604051634c9c8ce360e01b81526001600160a01b0382166004820152602401611017565b5f8051602061317c83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516121819190613160565b5f60405180830381855af49150503d805f81146121b9576040519150601f19603f3d011682016040523d82523d5f602084013e6121be565b606091505b50915091506121ce8583836121f6565b95945050505050565b34156109385760405163b398979f60e01b815260040160405180910390fd5b60608261220657610fcd8261224d565b815115801561221d57506001600160a01b0384163b155b1561224657604051639996b31560e01b81526001600160a01b0385166004820152602401611017565b5092915050565b80511561225d5780518082602001fd5b604051630a12f52160e11b815260040160405180910390fd5b5080545f8255600202905f5260205f20908101906110299190612519565b604051806101600160405280600b906020820280368337509192915050565b604051806102c001604052805f81526020015f81526020016122e660405180604001604052805f81526020015f81525090565b815260200161230660405180604001604052805f81526020015f81525090565b815260200161232660405180604001604052805f81526020015f81525090565b815260200161234660405180604001604052805f81526020015f81525090565b815260200161236660405180604001604052805f81526020015f81525090565b815260200161238660405180604001604052805f81526020015f81525090565b81526020016123a660405180604001604052805f81526020015f81525090565b81526020016123c660405180604001604052805f81526020015f81525090565b81526020016123e660405180604001604052805f81526020015f81525090565b815260200161240660405180604001604052805f81526020015f81525090565b815260200161242660405180604001604052805f81526020015f81525090565b815260200161244660405180604001604052805f81526020015f81525090565b815260200161246660405180604001604052805f81526020015f81525090565b815260200161248660405180604001604052805f81526020015f81525090565b81526020016124a660405180604001604052805f81526020015f81525090565b81526020016124c660405180604001604052805f81526020015f81525090565b81526020016124e660405180604001604052805f81526020015f81525090565b815260200161250660405180604001604052805f81526020015f81525090565b81525f6020820181905260409091015290565b5b8082111561253e5780546001600160c01b03191681555f600182015560020161251a565b5090565b80356001600160a01b0381168114610fb2575f80fd5b5f60208284031215612568575f80fd5b610fe382612542565b5f60208284031215612581575f80fd5b5035919050565b80356001600160401b0381168114610fb2575f80fd5b5f602082840312156125ae575f80fd5b610fe382612588565b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b03811182821017156125ed576125ed6125b7565b60405290565b6040516102e081016001600160401b03811182821017156125ed576125ed6125b7565b604051601f8201601f191681016001600160401b038111828210171561263e5761263e6125b7565b604052919050565b5f8060408385031215612657575f80fd5b61266083612542565b91506020808401356001600160401b038082111561267c575f80fd5b818601915086601f83011261268f575f80fd5b8135818111156126a1576126a16125b7565b6126b3601f8201601f19168501612616565b915080825287848285010111156126c8575f80fd5b80848401858401375f848284010152508093505050509250929050565b5f606082840312156126f5575f80fd5b604051606081018181106001600160401b0382111715612717576127176125b7565b60405290508061272683612588565b815261273460208401612588565b6020820152604083013560408201525092915050565b5f6080828403121561275a575f80fd5b6127626125cb565b90508135815260208201356020820152604082013560408201526060820135606082015292915050565b5f6040828403121561279c575f80fd5b604051604081018181106001600160401b03821117156127be576127be6125b7565b604052823581526020928301359281019290925250919050565b5f805f8385036105608112156127ec575f80fd5b6127f686866126e5565b9350612805866060870161274a565b92506104808060df198301121561281a575f80fd5b6128226125f3565b91506128318760e0880161278c565b82526101206128428882890161278c565b602084015261016061285689828a0161278c565b60408501526101a061286a8a828b0161278c565b60608601526101e061287e8b828c0161278c565b60808701526102206128928c828d0161278c565b60a08801526102606128a68d828e0161278c565b60c08901526102a06128ba8e828f0161278c565b60e08a01526128cd8e6102e08f0161278c565b6101008a01526128e18e6103208f0161278c565b878a01526128f38e6103608f0161278c565b6101408a01526129078e6103a08f0161278c565b868a01526129198e6103e08f0161278c565b6101808a01526104208d0135858a01526104408d01356101c08a01526104608d0135848a0152878d01356102008a01526104a08d0135838a01526104c08d01356102408a01526104e08d0135828a01526105008d01356102808a01526105208d0135818a015250505050505050506105408501356102c0820152809150509250925092565b803563ffffffff81168114610fb2575f80fd5b5f805f805f61014086880312156129c6575f80fd5b6129d087876126e5565b94506129df876060880161274a565b93506129ed60e0870161299e565b92506129fc6101008701612542565b9150612a0b6101208701612588565b90509295509295909350565b5f8060408385031215612a28575f80fd5b612a3183612588565b9150612a3f60208401612588565b90509250929050565b5f60208284031215612a58575f80fd5b610fe38261299e565b5f5b83811015612a7b578181015183820152602001612a63565b50505f910152565b602081525f8251806020840152612aa1816040850160208701612a61565b601f01601f19169190910160400192915050565b5f60608284031215612ac5575f80fd5b610fe383836126e5565b5f8060408385031215612ae0575f80fd5b50508035926020909101359150565b5f6020808385031215612b00575f80fd5b82356001600160401b0380821115612b16575f80fd5b818501915085601f830112612b29575f80fd5b813581811115612b3b57612b3b6125b7565b612b49848260051b01612616565b818152848101925060079190911b830184019087821115612b68575f80fd5b928401925b81841015612bd25760808489031215612b84575f80fd5b612b8c6125cb565b612b9585612588565b8152612ba2868601612588565b868201526040612bb3818701612588565b9082015260608581013590820152835260809093019291840191612b6d565b979650505050505050565b634e487b7160e01b5f52601160045260245ffd5b6001600160401b0381811683821601908082111561224657612246612bdd565b81810381811115610ee657610ee6612bdd565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b5f6001600160401b0380841680612c6557612c65612c38565b92169190910692915050565b5f6001600160401b0380841680612c8a57612c8a612c38565b92169190910492915050565b5f60208284031215612ca6575f80fd5b5051919050565b805f5b600b811015612ccf578151845260209384019390910190600101612cb0565b50505050565b612cea82825180518252602090810151910152565b6020818101518051604085015290810151606084015250604081015180516080840152602081015160a0840152506060810151805160c0840152602081015160e0840152506080810151610100612d4d8185018380518252602090810151910152565b60a08301519150610140612d6d8186018480518252602090810151910152565b60c08401519250610180612d8d8187018580518252602090810151910152565b60e085015193506101c0612dad8188018680518252602090810151910152565b92850151935061020092612dcd8785018680518252602090810151910152565b6101208601519450610240612dee8189018780518252602090810151910152565b92860151945061028092612e0e8885018780518252602090810151910152565b61016087015195506102c0612e2f818a018880518252602090810151910152565b9287015180516103008a0152602001516103208901526101a0870151610340890152908601516103608801526101e0860151610380880152928501516103a08701526102208501516103c0870152918401516103e08601526102608401516104008601528301516104208501526102a0830151610440850152909101516104609092019190915250565b5f610ae08201905084518252602085015160208301526040850151612eeb604084018280518252602090810151910152565b50606085015180516080840152602081015160a0840152506080850151805160c0840152602081015160e08401525060a0850151610100612f388185018380518252602090810151910152565b60c08701519150610140612f588186018480518252602090810151910152565b60e08801519250610180612f788187018580518252602090810151910152565b9188015192506101c091612f988684018580518252602090810151910152565b6101208901519350610200612fb98188018680518252602090810151910152565b91890151935061024091612fd98784018680518252602090810151910152565b6101608a01519450610280612ffa8189018780518252602090810151910152565b918a015180516102c08901526020908101516102e08901526101a08b015180516103008a0152810151610320890152938a015180516103408901528401516103608801526101e08a015180516103808901528401516103a088015289015180516103c08801528301516103e087015261022089015180516104008801528301516104208701529088015180516104408701528201516104608601526102608801518051610480870152909101516104a08501528601516104c0840152506102a08501516104e08301526130d1610500830185612cad565b6130df610660830184612cd5565b949350505050565b5f602082840312156130f7575f80fd5b81518015158114610fe3575f80fd5b6001600160401b0382811682821603908082111561224657612246612bdd565b5f6001600160401b0380831681810361314157613141612bdd565b6001019392505050565b5f8161315957613159612bdd565b505f190190565b5f8251613171818460208701612a61565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca164736f6c6343000817000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xF1W_5`\xE0\x1C\x80c\x82nA\xFC\x11a\x01\x08W\x80c\xB2BN?\x11a\0\x9DW\x80c\xD2M\x93=\x11a\0mW\x80c\xD2M\x93=\x14a\x06\xF6W\x80c\xE003\x01\x14a\x07%W\x80c\xF2\xFD\xE3\x8B\x14a\x07DW\x80c\xF5ga`\x14a\x07cW\x80c\xF9\xE5\r\x19\x14a\x07\x82W_\x80\xFD[\x80c\xB2BN?\x14a\x06.W\x80c\xB5\xAD\xEA<\x14a\x06LW\x80c\xC2;\x9E\x9E\x14a\x06\xA3W\x80c\xC8\xE5\xE4\x98\x14a\x06\xDBW_\x80\xFD[\x80c\x96\xC1\xCAa\x11a\0\xD8W\x80c\x96\xC1\xCAa\x14a\x05^W\x80c\x9F\xDBT\xA7\x14a\x05}W\x80c\xA1\xBE\x8DR\x14a\x05\xD2W\x80c\xAD<\xB1\xCC\x14a\x05\xF1W_\x80\xFD[\x80c\x82nA\xFC\x14a\x04\x9CW\x80c\x85\x84\xD2?\x14a\x04\xC7W\x80c\x8D\xA5\xCB[\x14a\x05\x03W\x80c\x90\xC1C\x90\x14a\x05?W_\x80\xFD[\x80c<#\xB6\xDB\x11a\x01\x89W\x80ci\xCCj\x04\x11a\x01YW\x80ci\xCCj\x04\x14a\x04\"W\x80cqP\x18\xA6\x14a\x046W\x80cu|7\xAD\x14a\x04JW\x80cvg\x18\x08\x14a\x04iW\x80c\x81\x1F\x85?\x14a\x04}W_\x80\xFD[\x80c<#\xB6\xDB\x14a\x03\x9DW\x80cBm1\x94\x14a\x03\xD9W\x80cO\x1E\xF2\x86\x14a\x03\xFBW\x80cR\xD1\x90-\x14a\x04\x0EW_\x80\xFD[\x80c-R\xAA\xD6\x11a\x01\xC4W\x80c-R\xAA\xD6\x14a\x02\xE0W\x80c/y\x88\x9D\x14a\x03\x0CW\x80c1=\xF7\xB1\x14a\x03JW\x80c7\x8E\xC2;\x14a\x03\x81W_\x80\xFD[\x80c\x01?\xA5\xFC\x14a\x01\xF5W\x80c\x02\xB5\x92\xF3\x14a\x02\x16W\x80c\x06%\xE1\x9B\x14a\x02sW\x80c\r\x8En,\x14a\x02\xB5W[_\x80\xFD[4\x80\x15a\x02\0W_\x80\xFD[Pa\x02\x14a\x02\x0F6`\x04a%XV[a\x07\x96V[\0[4\x80\x15a\x02!W_\x80\xFD[Pa\x025a\x0206`\x04a%qV[a\x08IV[`@Qa\x02j\x94\x93\x92\x91\x90`\x01`\x01`@\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R\x92\x16`@\x82\x01R``\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02~W_\x80\xFD[P`\x05T`\x06T`\x07T`\x08Ta\x02\x95\x93\x92\x91\x90\x84V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x02jV[4\x80\x15a\x02\xC0W_\x80\xFD[P`@\x80Q`\x01\x81R_` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R``\x01a\x02jV[4\x80\x15a\x02\xEBW_\x80\xFD[Pa\x02\x14a\x02\xFA6`\x04a%qV[`\x0F\x80T`\xFF\x19\x16`\x01\x17\x90U`\x10UV[4\x80\x15a\x03\x17W_\x80\xFD[P`\rTa\x032\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02jV[4\x80\x15a\x03UW_\x80\xFD[P`\rTa\x03i\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02jV[4\x80\x15a\x03\x8CW_\x80\xFD[PC[`@Q\x90\x81R` \x01a\x02jV[4\x80\x15a\x03\xA8W_\x80\xFD[Pa\x02\x14a\x03\xB76`\x04a%\x9EV[_\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x03\xE4W_\x80\xFD[P`\x01T`\x02T`\x03T`\x04Ta\x02\x95\x93\x92\x91\x90\x84V[a\x02\x14a\x04\t6`\x04a&FV[a\x08\x92V[4\x80\x15a\x04\x19W_\x80\xFD[Pa\x03\x8Fa\x08\xB1V[4\x80\x15a\x04-W_\x80\xFD[Pa\x02\x14a\x08\xCCV[4\x80\x15a\x04AW_\x80\xFD[Pa\x02\x14a\t:V[4\x80\x15a\x04UW_\x80\xFD[Pa\x02\x14a\x04d6`\x04a'\xD8V[a\tKV[4\x80\x15a\x04tW_\x80\xFD[Pa\x032a\x0C\x16V[4\x80\x15a\x04\x88W_\x80\xFD[Pa\x02\x14a\x04\x976`\x04a)\xB1V[a\x0C@V[4\x80\x15a\x04\xA7W_\x80\xFD[P`\rT`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[`@Q\x90\x15\x15\x81R` \x01a\x02jV[4\x80\x15a\x04\xD2W_\x80\xFD[Pa\x04\xE6a\x04\xE16`\x04a%qV[a\rdV[`@\x80Q\x92\x83R`\x01`\x01`@\x1B\x03\x90\x91\x16` \x83\x01R\x01a\x02jV[4\x80\x15a\x05\x0EW_\x80\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16a\x03iV[4\x80\x15a\x05JW_\x80\xFD[Pa\x032a\x05Y6`\x04a*\x17V[a\x0E\x8FV[4\x80\x15a\x05iW_\x80\xFD[Pa\x02\x14a\x05x6`\x04a*HV[a\x0E\xECV[4\x80\x15a\x05\x88W_\x80\xFD[P`\x0BT`\x0CTa\x05\xAC\x91`\x01`\x01`@\x1B\x03\x80\x82\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x83V[`@\x80Q`\x01`\x01`@\x1B\x03\x94\x85\x16\x81R\x93\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\x02jV[4\x80\x15a\x05\xDDW_\x80\xFD[Pa\x04\xB7a\x05\xEC6`\x04a%\x9EV[a\x0FuV[4\x80\x15a\x05\xFCW_\x80\xFD[Pa\x06!`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x02j\x91\x90a*\x83V[4\x80\x15a\x069W_\x80\xFD[P_Ta\x032\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06WW_\x80\xFD[Pa\x02\x14a\x06f6`\x04a*\xB5V[\x80Q`\x0B\x80T` \x84\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x93\x16\x92\x90\x92\x17\x91\x90\x91\x17\x90U`@\x01Q`\x0CUV[4\x80\x15a\x06\xAEW_\x80\xFD[P`\rTa\x06\xC6\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02jV[4\x80\x15a\x06\xE6W_\x80\xFD[Pa\x02\x14`\x0F\x80T`\xFF\x19\x16\x90UV[4\x80\x15a\x07\x01W_\x80\xFD[P`\tT`\nTa\x05\xAC\x91`\x01`\x01`@\x1B\x03\x80\x82\x16\x92`\x01`@\x1B\x90\x92\x04\x16\x90\x83V[4\x80\x15a\x070W_\x80\xFD[Pa\x04\xB7a\x07?6`\x04a*\xCFV[a\x0F\xB7V[4\x80\x15a\x07OW_\x80\xFD[Pa\x02\x14a\x07^6`\x04a%XV[a\x0F\xEAV[4\x80\x15a\x07nW_\x80\xFD[Pa\x02\x14a\x07}6`\x04a*\xEFV[a\x10,V[4\x80\x15a\x07\x8DW_\x80\xFD[P`\x0ETa\x03\x8FV[a\x07\x9Ea\x10\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xC5W`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x07\xF4W`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x80\x17\xBB\x88\x7F\xDF\x8F\xCAC\x14\xA9\xD4\x0Fns\xB3\xB8\x10\x02\xD6~\\\xFA\x85\xD8\x81s\xAFj\xA4`r\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x0E\x81\x81T\x81\x10a\x08XW_\x80\xFD[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T`\x01`\x01`@\x1B\x03\x80\x83\x16\x93P`\x01`@\x1B\x83\x04\x81\x16\x92`\x01`\x80\x1B\x90\x04\x16\x90\x84V[a\x08\x9Aa\x112V[a\x08\xA3\x82a\x11\xD6V[a\x08\xAD\x82\x82a\x12\x17V[PPV[_a\x08\xBAa\x12\xD8V[P_\x80Q` a1|\x839\x81Q\x91R\x90V[a\x08\xD4a\x10\xD7V[`\rT`\x01`\x01`\xA0\x1B\x03\x16\x15a\t\x1FW`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`@Q\x7F\x9A_W\xDE\x85m\xD6h\xC5M\xD9^\\U\xDF\x93C!q\xCB\xCAI\xA8wmV \xEAY\xC0$P\x90_\x90\xA1V[`@Qc\xA8c\xAE\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\tBa\x10\xD7V[a\t8_a\x13!V[`\rT`\x01`\x01`\xA0\x1B\x03\x16\x15\x15\x80\x15a\tpWP`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\t\x8EW`@Qc\x01GL\x8F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0BT\x83Q`\x01`\x01`@\x1B\x03\x91\x82\x16\x91\x16\x11\x15\x80a\t\xC7WP`\x0BT` \x84\x01Q`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x92\x04\x82\x16\x91\x16\x11\x15[\x15a\t\xE5W`@Qc\x05\x1CF\xEF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xF2\x83`@\x01Qa\x13\x91V[_a\t\xFBa\x0C\x16V[` \x85\x01Q_\x80T\x92\x93P\x91a\n\x1A\x91\x90`\x01`\x01`@\x1B\x03\x16a\x0E\x8FV[\x90Pa\n'\x82`\x01a+\xF1V[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14\x80\x15a\n`WP`\x0BTa\n^\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0FuV[\x15[\x80\x15a\ntWP_\x82`\x01`\x01`@\x1B\x03\x16\x11[\x15a\n\x92W`@QcqP\xDEE`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\x9D\x82`\x02a+\xF1V[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x10a\n\xCEW`@QcqP\xDEE`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xDB\x84` \x01Qa\x13\x91V[a\n\xE8\x84`@\x01Qa\x13\x91V[a\n\xF5\x84``\x01Qa\x13\x91V[a\x0B\0\x85\x85\x85a\x14\x01V[\x84Q`\x0B\x80T` \x88\x01Q`\x01`\x01`@\x1B\x03\x81\x81\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x94\x16\x93\x90\x93\x17\x17\x90U`@\x86\x01Q`\x0CUa\x0BB\x90a\x0FuV[\x15a\x0B\xACW\x83Q`\x05U` \x84\x01Q`\x06U`@\x84\x01Q`\x07U``\x84\x01Q`\x08U\x7F1\xEA\xBD\x90\x99\xFD\xB2]\xAC\xDD\xD2\x06\xAB\xFF\x871\x1EU4A\xFC\x9D\x0F\xCD\xEF \x10b\xD7\xE7\x07\x1Ba\x0B\x90\x82`\x01a+\xF1V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1[a\x0B\xB7CB\x87a\x15YV[\x84` \x01Q`\x01`\x01`@\x1B\x03\x16\x85_\x01Q`\x01`\x01`@\x1B\x03\x16\x7F\xA0Jw9$PZA\x85d67%\xF5h2\xF5w.k\x8D\r\xBDn\xFC\xE7$\xDF\xE8\x03\xDA\xE6\x87`@\x01Q`@Qa\x0C\x07\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPV[`\x0BT_\x80T\x90\x91a\x0C;\x91`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x92\x04\x82\x16\x91\x16a\x0E\x8FV[\x90P\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x0C\x84WP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x0C\x9FWP0;\x15[\x90P\x81\x15\x80\x15a\x0C\xADWP\x80\x15[\x15a\x0C\xCBW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x0C\xF5W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x0C\xFE\x87a\x17BV[a\r\x06a\x17SV[a\r\x12\x8A\x8A\x8A\x89a\x17[V[\x83\x15a\rXW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\x0E\x80T_\x91\x82\x91\x90a\rx`\x01\x83a,\x11V[\x81T\x81\x10a\r\x88Wa\r\x88a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x84\x10a\r\xC7W`@Qc\x18V\xA4\x99`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\rT`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16[\x81\x81\x10\x15a\x0E\x88W\x84`\x0E\x82\x81T\x81\x10a\r\xF7Wa\r\xF7a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0E\x80W`\x0E\x81\x81T\x81\x10a\x0E0Wa\x0E0a,$V[\x90_R` _ \x90`\x02\x02\x01`\x01\x01T`\x0E\x82\x81T\x81\x10a\x0ESWa\x0ESa,$V[\x90_R` _ \x90`\x02\x02\x01_\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x93P\x93PPP\x91P\x91V[`\x01\x01a\r\xDBV[PP\x91P\x91V[_\x81`\x01`\x01`@\x1B\x03\x16_\x03a\x0E\xA7WP_a\x0E\xE6V[a\x0E\xB1\x82\x84a,LV[`\x01`\x01`@\x1B\x03\x16_\x03a\x0E\xD1Wa\x0E\xCA\x82\x84a,qV[\x90Pa\x0E\xE6V[a\x0E\xDB\x82\x84a,qV[a\x0E\xCA\x90`\x01a+\xF1V[\x92\x91PPV[a\x0E\xF4a\x10\xD7V[a\x0E\x10\x81c\xFF\xFF\xFF\xFF\x16\x10\x80a\x0F\x13WPc\x01\xE13\x80\x81c\xFF\xFF\xFF\xFF\x16\x11[\x80a\x0F1WP`\rTc\xFF\xFF\xFF\xFF`\x01`\xA0\x1B\x90\x91\x04\x81\x16\x90\x82\x16\x11\x15[\x15a\x0FOW`@Qc\x07\xA5\x07w`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\r\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x81`\x01`\x01`@\x1B\x03\x16_\x03a\x0F\x8DWP_\x91\x90PV[_Ta\x0F\xA2\x90`\x01`\x01`@\x1B\x03\x16\x83a,LV[`\x01`\x01`@\x1B\x03\x16\x15\x92\x91PPV[\x91\x90PV[`\x0FT_\x90`\xFF\x16a\x0F\xD2Wa\x0F\xCD\x83\x83a\x18\xD5V[a\x0F\xE3V[\x81`\x10T\x84a\x0F\xE1\x91\x90a,\x11V[\x11[\x93\x92PPPV[a\x0F\xF2a\x10\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10 W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x10)\x81a\x13!V[PV[a\x107`\x0E_a\"vV[_[\x81Q\x81\x10\x15a\x08\xADW`\x0E\x82\x82\x81Q\x81\x10a\x10VWa\x10Va,$V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x83\x90 \x82Q`\x02\x90\x92\x02\x01\x80T\x93\x83\x01Q`@\x84\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x92\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x97\x16\x91\x90\x94\x16\x17\x94\x90\x94\x17\x93\x90\x93\x16\x17\x82U``\x01Q\x90\x82\x01U\x01a\x109V[3a\x11\t\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\t8W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x10\x17V[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x11\xB8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x11\xAC_\x80Q` a1|\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\t8W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xDEa\x10\xD7V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R\x7F\xF7\x87!\"n\xFE\x9A\x1B\xB6x\x18\x9A\x16\xD1UI(\xB9\xF2\x19.,\xB9>\xED\xA8;y\xFA@\0}\x90` \x01a\x08>V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x12qWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12n\x91\x81\x01\x90a,\x96V[`\x01[a\x12\x99W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x10\x17V[_\x80Q` a1|\x839\x81Q\x91R\x81\x14a\x12\xC9W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x10\x17V[a\x12\xD3\x83\x83a\x1A-V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t8W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x81\x10\x80a\x08\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FBn254: invalid scalar field\0\0\0\0\0`D\x82\x01R`d\x01a\x10\x17V[_a\x14\na\x1A\x82V[\x90Pa\x14\x14a\"\x94V[\x84Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R` \x80\x87\x01\x80Q\x90\x92\x16\x90\x83\x01R`@\x80\x87\x01Q\x90\x83\x01R`\x06T``\x83\x01R`\x07T`\x80\x83\x01R`\x08T`\xA0\x83\x01R`\x05T`\xC0\x83\x01RQa\x14e\x90a\x0FuV[\x15a\x14\x97W` \x84\x01Q`\xE0\x82\x01R`@\x84\x01Qa\x01\0\x82\x01R``\x84\x01Qa\x01 \x82\x01R\x83Qa\x01@\x82\x01Ra\x14\xBBV[`\x06T`\xE0\x82\x01R`\x07Ta\x01\0\x82\x01R`\x08Ta\x01 \x82\x01R`\x05Ta\x01@\x82\x01R[`@Qc\xFC\x86`\xC7`\xE0\x1B\x81Rs_\xBD\xB21Vx\xAF\xEC\xB3g\xF02\xD9?d/d\x18\n\xA3\x90c\xFC\x86`\xC7\x90a\x14\xF6\x90\x85\x90\x85\x90\x88\x90`\x04\x01a.\xB9V[` `@Q\x80\x83\x03\x81\x86Z\xF4\x15\x80\x15a\x15\x11W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x155\x91\x90a0\xE7V[a\x15RW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x0ET\x15\x80\x15\x90a\x15\xCEWP`\rT`\x0E\x80T`\x01`\xA0\x1B\x83\x04c\xFF\xFF\xFF\xFF\x16\x92`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x81\x10a\x15\x99Wa\x15\x99a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01Ta\x15\xC3\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x84a1\x06V[`\x01`\x01`@\x1B\x03\x16\x11[\x15a\x16aW`\rT`\x0E\x80T\x90\x91`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x81\x10a\x15\xFBWa\x15\xFBa,$V[_\x91\x82R` \x82 `\x02\x90\x91\x02\x01\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x81U`\x01\x01U`\r\x80T`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90`\x18a\x16;\x83a1&V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPP[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`@\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x80\x85\x01\x91\x82R\x83\x01Q\x85\x16\x84\x83\x01\x90\x81R\x92\x90\x91\x01Q``\x84\x01\x90\x81R`\x0E\x80T`\x01\x81\x01\x82U_\x91\x90\x91R\x93Q`\x02\x90\x94\x02\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x81\x01\x80T\x93Q\x94Q\x87\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x96\x90\x97\x16\x95\x90\x95\x17\x92\x90\x92\x17\x92\x90\x92\x16\x93\x90\x93\x17\x90\x91UQ\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x90\x91\x01UV[a\x17Ja \xB1V[a\x10)\x81a \xFAV[a\t8a \xB1V[\x83Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x80a\x17\x7FWP` \x84\x01Q`\x01`\x01`@\x1B\x03\x16\x15\x15[\x80a\x17\x8CWP` \x83\x01Q\x15[\x80a\x17\x99WP`@\x83\x01Q\x15[\x80a\x17\xA6WP``\x83\x01Q\x15[\x80a\x17\xB0WP\x82Q\x15[\x80a\x17\xC2WPa\x0E\x10\x82c\xFF\xFF\xFF\xFF\x16\x10[\x80a\x17\xD6WPc\x01\xE13\x80\x82c\xFF\xFF\xFF\xFF\x16\x11[\x80a\x17\xE8WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x18\x06W`@QcP\xDD\x03\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\t\x80T` \x80\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x93\x84\x16\x95\x82\x16\x95\x86\x17\x81\x17\x90\x94U`@\x98\x89\x01Q`\n\x81\x90U`\x0B\x80T\x90\x94\x16\x90\x95\x17\x90\x93\x17\x90\x91U`\x0C\x92\x90\x92U\x84Q`\x01\x81\x90U\x91\x85\x01Q`\x02\x81\x90U\x95\x85\x01Q`\x03\x81\x90U``\x90\x95\x01Q`\x04\x81\x90U`\x05\x92\x90\x92U`\x06\x95\x90\x95U`\x07\x93\x90\x93U`\x08\x92\x90\x92U`\r\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U_\x80T\x91\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x90\x91\x16\x17\x90UV[`\x0ET_\x90C\x84\x11\x80a\x18\xE6WP\x80\x15[\x80a\x190WP`\rT`\x0E\x80T\x90\x91`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90\x81\x10a\x19\x14Wa\x19\x14a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x84\x10[\x15a\x19NW`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\x19\\`\x01\x85a,\x11V[\x90P[\x81a\x19\xF8W`\rT`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81\x10a\x19\xF8W\x86`\x0E\x82\x81T\x81\x10a\x19\x91Wa\x19\x91a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x11a\x19\xE6W`\x01\x91P`\x0E\x81\x81T\x81\x10a\x19\xC6Wa\x19\xC6a,$V[_\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01T`\x01`\x01`@\x1B\x03\x16\x92Pa\x19\xF8V[\x80a\x19\xF0\x81a1KV[\x91PPa\x19_V[\x81a\x1A\x16W`@Qc\xB0\xB48w`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x1A!\x84\x89a,\x11V[\x11\x97\x96PPPPPPPV[a\x1A6\x82a!\x02V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x1AzWa\x12\xD3\x82\x82a!eV[a\x08\xADa!\xD7V[a\x1A\x8Aa\"\xB3V[b\x01\0\0\x81R`\x0B` \x82\x01R\x7F/\xAFZ\x11>\xFD\x87\xD7X\x18\xE6?\xF9\xA6\x17\0\x07\xF2,\x89\xBB\xC4\xA8\xBD\x0F+H&\x87W\xB0\x14`@\x82\x01QR\x7F\x18Z\xEE\x05\xF8\xD3\xBA\xBF\xCEg\x93\x1F\x15\xDB9\xE6\x1F%\xF7\x94\xA4\x13M{\xEEn\x18\xC5\xAD\x1E\xC0W` `@\x83\x01Q\x01R\x7F\r\xCC\xF5\xDC\xF6g\xA3|\xA9;\x8Dr\x10\x91\xD8\xF3\xA8\x04\x9B=\x1E\x89\xA5mf\xE4'Q\xBB\xAF{^``\x82\x01QR\x7F,\xF1\tI\xFC[\xFC\xEC\xB3\xBCT\xDDA!\xE5X\x07C\x0F\x17\xF3\x04\x98\xA7\xEAj\x02`p\xB1\x91b` ``\x83\x01Q\x01R\x7F\x08\xD7\x0EN\x01\x84\xFES\xBDVo\r~\xDCL\xD7\xB0\xE39I\ts\xD0\xFA\xEC}\xAC \x89\xF58\xE5`\x80\x82\x01QR~\xF6e\xFE\x1F\xD1\x10\xD3}\x1D\xEADn\x84\0\xF0o\x06\xB9\xB5\x8A\xB3\xDF\x90\xFB\xAE|G\xEEX`A` `\x80\x83\x01Q\x01R\x7F\x08~\x14\xD7\x19$\xAC\x0F(\x80\xAD\xF0\xF1\x06\x92^Zo\xDDW\xD0 \xBB<\x8A\xA7\x0F\xA9\xFC\0\xCC\xF3`\xA0\x82\x01QR\x7F\x01\xDB~1x\xB3B\xF9\x1DT\xFC\x97,\xEErV\x9FB\x9A99\x88\xEEC\xC2\x89\xE2\xED\x96\x07qR` `\xA0\x83\x01Q\x01R\x7F\x19m\xD4-vr\x01\xF7\xF1\x96\xC4*\xAE\xF4\x85e`F1\x0FP\x83U\x95\x92\xBD\x13\x13\xE1iH\xB7`\xC0\x82\x01QR\x7F\x17\x88\x96\x80\x81\n\xAA\xBD\x1F\xF3\xACJlT\x92\x10\x05y\xE0Y\x17\x0C\xD2\xB7~+=\xA6\xD3|\xC2F` `\xC0\x83\x01Q\x01R\x7F$\x93^zw\xAC1?\xD3\xD6\x0F\xF3\xF1\xA0\xA7\x9E\xC3,}\xC5\x19\xB3\x9D\xA0\xAC\xB2\xC4\x9F6wq\xCC`\xE0\x82\x01QR\x7F\x16\x8E)B^\xF18\xCBiC\xC7SR\xF3<\x19\x0E_\x14\x88\xEBT\xA9\xE1\x1D\xEBtM\xA7\xFBk.` `\xE0\x83\x01Q\x01R\x7F\x1BX\xD5X\xB5RdS\xBD\x10(\xCA\x93\x8C\x94\x0B\xB8\x9Er?|5x|\x02\xF9\xF1y\xAE\x9A\x0C\xEAa\x01\0\x82\x01QR\x7F!\xAF\xC1!\xD9\x1D\x9D\x1C\x17\xDA\xFB\x926\xBC\x9B\x87,[C\xDF\x06L\x0B\x12\x86\x01/\xB4:v#$` a\x01\0\x83\x01Q\x01R\x7F\x10G\xFCUyM\x1EY}\xE1U\x07v\x11\xE3\xC7\x89\xA0\xA2\xBE\x02\x188!\xBB\xA5l\xF6\x1C\xC1\xB8\xEDa\x01 \x82\x01QR\x7F\x17BR2G'\xC0\xD2\xEE^P\xEBW\xA5#\x1FgGL\xEE\xD6\x93*\xD4\xFF\xE9\xBC\xF8f\xAA4(` a\x01 \x83\x01Q\x01R\x7F(\xDB(\x9AL\xFBs\xBA\x92\x96\x15r\xF3\x18R\x98\xAE6n\xD1\xA4Iq`{\xCB\xF8\x01\xF1 \xF5aa\x01@\x82\x01QR\x7F\x04\\\xFEz\xE2\xCD\x17U\x08\x17.}\x9C.\x89\x9B\xB1\xD2\x16\xDF\xC3\x1F\xE8\x9F\xC6\xC9\x17\xCA\xAE\xE8w\xA2` a\x01@\x83\x01Q\x01R\x7F\x19_.\xEC\x85Gr\x7F\xC4n\xD0\x1By\xE8\xF6f\xDE\xD6J\xE5OW\x078t\xA5\xA2G\x03\x80\xA7\x85a\x01`\x82\x01QR\x7F\x15'2.\x85\xDA\x1A\xEF\xBD\x83\x9Ee\xD1\x1D\xC6\x95\xAA\xC1k\r\xB6\xC6%\x91\xD9\x812B\xD4\x1C\xBE1` a\x01`\x83\x01Q\x01R\x7F\x10\xC8\xD7\xD75_~\x0F\x8C\0/H,\xC3\xB9\x8C\x90\xBA\xA9Ba\xC5\x9A\x17\xB4$\xEE\xCF\xE4\xE9c\xB2a\x01\x80\x82\x01QR\x7F\"r\xE3\x01xdqg\xBB\xEA\xD3\xA2\xD77\x19\x88\xF2\xE1\x98\xE6X\x15\x02\x9D\xEDLd\xBF\xC0\x85\x0F\x1F` a\x01\x80\x83\x01Q\x01R\x7F\x15\xD5n\xA7\xAB/\xA6\x12e\xF5Q\xC2\xAE%8\x9C\x8F\xE7\xBC\xB3\xBFf\x08\x08,6\xA2\x01\xF2%\xF7}a\x01\xA0\x82\x01QR\x7F\x0BXTh\x87 .rs\xD3\xD0\xC5]e\xDDa2\xCA\xC9\x8E\xBF\x04\xEF\xB1\xB5$E\xC5\x13\xC4\xA4\xDF` a\x01\xA0\x83\x01Q\x01R\x7F\x05\roCwN\x8D\xFF\xAA\x86\x8F*}\xC8/Vli\xD1u\xD8\x18\xD4Q|\xC7\n\xC5\xFC\xB2\xF1\xB1a\x01\xC0\x82\x01QR\x7F/\xFF\x87\xBF`^\x99\x83s\xBBdU?:b]\xAB\xCD\x12\x88\x86\x92\xD6x\xA8\xF4M\x13d@\xC8c` a\x01\xC0\x83\x01Q\x01R\x7F\x12\xD0\x85`\x8C`,\xFB[\x8C\x03\xEC{\xD1:\xC0\xFF\x9Ed\xA9\xAC\x1E\x9A\xA7FYJ\x03>FK\xF2a\x01\xE0\x82\x01QR\x7F\x18\xACZ56\x04.\xEB\x0B\x0C|/C\xF5\xE2\xCA;!s\xDA\xA4\xC2\x81/\xFC\xA6G\x87\xE8\xE9V\xB2` a\x01\xE0\x83\x01Q\x01R\x7F\x0F\x0F\x98\x91\xFC+y\x0Et\xDC%<\x88T\xDFc\x92\xE0\x10\xF4\xDEg`\xB8B:=\xD6\x9B\xBE]\xCEa\x02\0\x82\x01QR\x7F\x16\xBE\xD1\xD2D\xA2\xFE:\xB9\xA6R\xC7\xFE\xECVP\x16\x1D\x8Au\"}\xEC\xE7)O<\x8F\xC5B\xFDl` a\x02\0\x83\x01Q\x01R\x7F\x0F\xA3m\0g/\xA6\xA1\xC4L\xD3\xC2Y!,\x1A\xDAH\xC6k\xF7\xBB\x08_$G\x1B\x15\xB1~nQa\x02 \x82\x01QR\x7F\x18 \x88\xE5kd\x95R2F\x08\x91\xD2\xB2yvS%\x81:\xEF\x1D\xAE\x85^_IlA\x8A\xFCA` a\x02 \x83\x01Q\x01R\x7F+\xAFZ\xE2\xDD\x83.\x14I\xFA\xCCa\x1Bk\x80\xFDf\xD5\x8C\x87\x1DX'\xC5\xC8\xE2tpd\xE2\x99da\x02@\x82\x01QR\x7F)\xF5C\xB5C\x13~\x88\x18\x04\xC9\x89\xCD;\x99\x93@\x10\0\"8\xE8\xAB>\xEC\x88.\t\xD3\x06h\x1F` a\x02@\x83\x01Q\x01R\x7F-\xB0\xDD\xC7\x12;B\xF5 \xE2WFj\r\x92\xDA\x8BVO\xE0\x1E\xC6e\tl\x14\x11\x96C\x01)\x84a\x02`\x82\x01QR\x7F\x1Bz\xB2zf\x96b\x84\xD7\xFB)\xBC\xE9\xD5P\xEA\xFB\xA1lI\xFB\xC6&x'\xCD\xFC\x8D\x0B\x16\xF9O` a\x02`\x83\x01Q\x01R\x7F\xB0\x83\x88\x93\xEC\x1F#~\x8B\x072;\x07DY\x9FN\x97\xB5\x98\xB3\xB5\x89\xBC\xC2\xBC7\xB8\xD5\xC4\x18\x01a\x02\x80\x82\x01R\x7F\xC1\x83\x93\xC0\xFA0\xFEN\x8B\x03\x8E5z\xD8Q\xEA\xE8\xDE\x91\x07XN\xFF\xE7\xC7\xF1\xF6Q\xB2\x01\x0E&a\x02\xA0\x82\x01R\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\t8W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xF2a \xB1V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a!7W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x10\x17V[_\x80Q` a1|\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa!\x81\x91\x90a1`V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a!\xB9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\xBEV[``\x91P[P\x91P\x91Pa!\xCE\x85\x83\x83a!\xF6V[\x95\x94PPPPPV[4\x15a\t8W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\"\x06Wa\x0F\xCD\x82a\"MV[\x81Q\x15\x80\x15a\"\x1DWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\"FW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x10\x17V[P\x92\x91PPV[\x80Q\x15a\"]W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T_\x82U`\x02\x02\x90_R` _ \x90\x81\x01\x90a\x10)\x91\x90a%\x19V[`@Q\x80a\x01`\x01`@R\x80`\x0B\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01a\"\xE6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\x06`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#&`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#F`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#f`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\x86`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\xA6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\xC6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a#\xE6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\x06`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$&`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$F`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$f`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\x86`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\xA6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\xC6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a$\xE6`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a%\x06`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[[\x80\x82\x11\x15a%>W\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x81U_`\x01\x82\x01U`\x02\x01a%\x1AV[P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xB2W_\x80\xFD[_` \x82\x84\x03\x12\x15a%hW_\x80\xFD[a\x0F\xE3\x82a%BV[_` \x82\x84\x03\x12\x15a%\x81W_\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0F\xB2W_\x80\xFD[_` \x82\x84\x03\x12\x15a%\xAEW_\x80\xFD[a\x0F\xE3\x82a%\x88V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xEDWa%\xEDa%\xB7V[`@R\x90V[`@Qa\x02\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\xEDWa%\xEDa%\xB7V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&>Wa&>a%\xB7V[`@R\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a&WW_\x80\xFD[a&`\x83a%BV[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&|W_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a&\x8FW_\x80\xFD[\x815\x81\x81\x11\x15a&\xA1Wa&\xA1a%\xB7V[a&\xB3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a&\x16V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a&\xC8W_\x80\xFD[\x80\x84\x84\x01\x85\x84\x017_\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[_``\x82\x84\x03\x12\x15a&\xF5W_\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a'\x17Wa'\x17a%\xB7V[`@R\x90P\x80a'&\x83a%\x88V[\x81Ra'4` \x84\x01a%\x88V[` \x82\x01R`@\x83\x015`@\x82\x01RP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a'ZW_\x80\xFD[a'ba%\xCBV[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R\x92\x91PPV[_`@\x82\x84\x03\x12\x15a'\x9CW_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a'\xBEWa'\xBEa%\xB7V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x80_\x83\x85\x03a\x05`\x81\x12\x15a'\xECW_\x80\xFD[a'\xF6\x86\x86a&\xE5V[\x93Pa(\x05\x86``\x87\x01a'JV[\x92Pa\x04\x80\x80`\xDF\x19\x83\x01\x12\x15a(\x1AW_\x80\xFD[a(\"a%\xF3V[\x91Pa(1\x87`\xE0\x88\x01a'\x8CV[\x82Ra\x01 a(B\x88\x82\x89\x01a'\x8CV[` \x84\x01Ra\x01`a(V\x89\x82\x8A\x01a'\x8CV[`@\x85\x01Ra\x01\xA0a(j\x8A\x82\x8B\x01a'\x8CV[``\x86\x01Ra\x01\xE0a(~\x8B\x82\x8C\x01a'\x8CV[`\x80\x87\x01Ra\x02 a(\x92\x8C\x82\x8D\x01a'\x8CV[`\xA0\x88\x01Ra\x02`a(\xA6\x8D\x82\x8E\x01a'\x8CV[`\xC0\x89\x01Ra\x02\xA0a(\xBA\x8E\x82\x8F\x01a'\x8CV[`\xE0\x8A\x01Ra(\xCD\x8Ea\x02\xE0\x8F\x01a'\x8CV[a\x01\0\x8A\x01Ra(\xE1\x8Ea\x03 \x8F\x01a'\x8CV[\x87\x8A\x01Ra(\xF3\x8Ea\x03`\x8F\x01a'\x8CV[a\x01@\x8A\x01Ra)\x07\x8Ea\x03\xA0\x8F\x01a'\x8CV[\x86\x8A\x01Ra)\x19\x8Ea\x03\xE0\x8F\x01a'\x8CV[a\x01\x80\x8A\x01Ra\x04 \x8D\x015\x85\x8A\x01Ra\x04@\x8D\x015a\x01\xC0\x8A\x01Ra\x04`\x8D\x015\x84\x8A\x01R\x87\x8D\x015a\x02\0\x8A\x01Ra\x04\xA0\x8D\x015\x83\x8A\x01Ra\x04\xC0\x8D\x015a\x02@\x8A\x01Ra\x04\xE0\x8D\x015\x82\x8A\x01Ra\x05\0\x8D\x015a\x02\x80\x8A\x01Ra\x05 \x8D\x015\x81\x8A\x01RPPPPPPPPa\x05@\x85\x015a\x02\xC0\x82\x01R\x80\x91PP\x92P\x92P\x92V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xB2W_\x80\xFD[_\x80_\x80_a\x01@\x86\x88\x03\x12\x15a)\xC6W_\x80\xFD[a)\xD0\x87\x87a&\xE5V[\x94Pa)\xDF\x87``\x88\x01a'JV[\x93Pa)\xED`\xE0\x87\x01a)\x9EV[\x92Pa)\xFCa\x01\0\x87\x01a%BV[\x91Pa*\x0Ba\x01 \x87\x01a%\x88V[\x90P\x92\x95P\x92\x95\x90\x93PV[_\x80`@\x83\x85\x03\x12\x15a*(W_\x80\xFD[a*1\x83a%\x88V[\x91Pa*?` \x84\x01a%\x88V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a*XW_\x80\xFD[a\x0F\xE3\x82a)\x9EV[_[\x83\x81\x10\x15a*{W\x81\x81\x01Q\x83\x82\x01R` \x01a*cV[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra*\xA1\x81`@\x85\x01` \x87\x01a*aV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[_``\x82\x84\x03\x12\x15a*\xC5W_\x80\xFD[a\x0F\xE3\x83\x83a&\xE5V[_\x80`@\x83\x85\x03\x12\x15a*\xE0W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x80\x83\x85\x03\x12\x15a+\0W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a+\x16W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a+)W_\x80\xFD[\x815\x81\x81\x11\x15a+;Wa+;a%\xB7V[a+I\x84\x82`\x05\x1B\x01a&\x16V[\x81\x81R\x84\x81\x01\x92P`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a+hW_\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a+\xD2W`\x80\x84\x89\x03\x12\x15a+\x84W_\x80\xFD[a+\x8Ca%\xCBV[a+\x95\x85a%\x88V[\x81Ra+\xA2\x86\x86\x01a%\x88V[\x86\x82\x01R`@a+\xB3\x81\x87\x01a%\x88V[\x90\x82\x01R``\x85\x81\x015\x90\x82\x01R\x83R`\x80\x90\x93\x01\x92\x91\x84\x01\x91a+mV[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\"FWa\"Fa+\xDDV[\x81\x81\x03\x81\x81\x11\x15a\x0E\xE6Wa\x0E\xE6a+\xDDV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a,eWa,ea,8V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[_`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a,\x8AWa,\x8Aa,8V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[_` \x82\x84\x03\x12\x15a,\xA6W_\x80\xFD[PQ\x91\x90PV[\x80_[`\x0B\x81\x10\x15a,\xCFW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a,\xB0V[PPPPV[a,\xEA\x82\x82Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[` \x81\x81\x01Q\x80Q`@\x85\x01R\x90\x81\x01Q``\x84\x01RP`@\x81\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP``\x81\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\x80\x81\x01Qa\x01\0a-M\x81\x85\x01\x83\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xA0\x83\x01Q\x91Pa\x01@a-m\x81\x86\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xC0\x84\x01Q\x92Pa\x01\x80a-\x8D\x81\x87\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xE0\x85\x01Q\x93Pa\x01\xC0a-\xAD\x81\x88\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x85\x01Q\x93Pa\x02\0\x92a-\xCD\x87\x85\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01 \x86\x01Q\x94Pa\x02@a-\xEE\x81\x89\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x86\x01Q\x94Pa\x02\x80\x92a.\x0E\x88\x85\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01`\x87\x01Q\x95Pa\x02\xC0a./\x81\x8A\x01\x88\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x87\x01Q\x80Qa\x03\0\x8A\x01R` \x01Qa\x03 \x89\x01Ra\x01\xA0\x87\x01Qa\x03@\x89\x01R\x90\x86\x01Qa\x03`\x88\x01Ra\x01\xE0\x86\x01Qa\x03\x80\x88\x01R\x92\x85\x01Qa\x03\xA0\x87\x01Ra\x02 \x85\x01Qa\x03\xC0\x87\x01R\x91\x84\x01Qa\x03\xE0\x86\x01Ra\x02`\x84\x01Qa\x04\0\x86\x01R\x83\x01Qa\x04 \x85\x01Ra\x02\xA0\x83\x01Qa\x04@\x85\x01R\x90\x91\x01Qa\x04`\x90\x92\x01\x91\x90\x91RPV[_a\n\xE0\x82\x01\x90P\x84Q\x82R` \x85\x01Q` \x83\x01R`@\x85\x01Qa.\xEB`@\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x85\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP`\x80\x85\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\xA0\x85\x01Qa\x01\0a/8\x81\x85\x01\x83\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xC0\x87\x01Q\x91Pa\x01@a/X\x81\x86\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xE0\x88\x01Q\x92Pa\x01\x80a/x\x81\x87\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x88\x01Q\x92Pa\x01\xC0\x91a/\x98\x86\x84\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01 \x89\x01Q\x93Pa\x02\0a/\xB9\x81\x88\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x89\x01Q\x93Pa\x02@\x91a/\xD9\x87\x84\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01`\x8A\x01Q\x94Pa\x02\x80a/\xFA\x81\x89\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x8A\x01Q\x80Qa\x02\xC0\x89\x01R` \x90\x81\x01Qa\x02\xE0\x89\x01Ra\x01\xA0\x8B\x01Q\x80Qa\x03\0\x8A\x01R\x81\x01Qa\x03 \x89\x01R\x93\x8A\x01Q\x80Qa\x03@\x89\x01R\x84\x01Qa\x03`\x88\x01Ra\x01\xE0\x8A\x01Q\x80Qa\x03\x80\x89\x01R\x84\x01Qa\x03\xA0\x88\x01R\x89\x01Q\x80Qa\x03\xC0\x88\x01R\x83\x01Qa\x03\xE0\x87\x01Ra\x02 \x89\x01Q\x80Qa\x04\0\x88\x01R\x83\x01Qa\x04 \x87\x01R\x90\x88\x01Q\x80Qa\x04@\x87\x01R\x82\x01Qa\x04`\x86\x01Ra\x02`\x88\x01Q\x80Qa\x04\x80\x87\x01R\x90\x91\x01Qa\x04\xA0\x85\x01R\x86\x01Qa\x04\xC0\x84\x01RPa\x02\xA0\x85\x01Qa\x04\xE0\x83\x01Ra0\xD1a\x05\0\x83\x01\x85a,\xADV[a0\xDFa\x06`\x83\x01\x84a,\xD5V[\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a0\xF7W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F\xE3W_\x80\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\"FWa\"Fa+\xDDV[_`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03a1AWa1Aa+\xDDV[`\x01\x01\x93\x92PPPV[_\x81a1YWa1Ya+\xDDV[P_\x19\x01\x90V[_\x82Qa1q\x81\x84` \x87\x01a*aV[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA1dsolcC\0\x08\x17\0\n",
    );
    /**Custom error with signature `AddressEmptyCode(address)` and selector `0x9996b315`.
    ```solidity
    error AddressEmptyCode(address target);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddressEmptyCode {
        pub target: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddressEmptyCode> for UnderlyingRustTuple<'_> {
            fn from(value: AddressEmptyCode) -> Self {
                (value.target,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddressEmptyCode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { target: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AddressEmptyCode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AddressEmptyCode(address)";
            const SELECTOR: [u8; 4] = [153u8, 150u8, 179u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`.
    ```solidity
    error ERC1967InvalidImplementation(address implementation);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967InvalidImplementation {
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967InvalidImplementation> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967InvalidImplementation) -> Self {
                (value.implementation,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967InvalidImplementation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    implementation: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967InvalidImplementation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967InvalidImplementation(address)";
            const SELECTOR: [u8; 4] = [76u8, 156u8, 140u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.implementation,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
    ```solidity
    error ERC1967NonPayable();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC1967NonPayable> for UnderlyingRustTuple<'_> {
            fn from(value: ERC1967NonPayable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC1967NonPayable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC1967NonPayable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC1967NonPayable()";
            const SELECTOR: [u8; 4] = [179u8, 152u8, 151u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `FailedInnerCall()` and selector `0x1425ea42`.
    ```solidity
    error FailedInnerCall();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedInnerCall {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FailedInnerCall> for UnderlyingRustTuple<'_> {
            fn from(value: FailedInnerCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedInnerCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedInnerCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedInnerCall()";
            const SELECTOR: [u8; 4] = [20u8, 37u8, 234u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientSnapshotHistory()` and selector `0xb0b43877`.
    ```solidity
    error InsufficientSnapshotHistory();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSnapshotHistory {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientSnapshotHistory> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSnapshotHistory) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSnapshotHistory {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSnapshotHistory {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSnapshotHistory()";
            const SELECTOR: [u8; 4] = [176u8, 180u8, 56u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidAddress()` and selector `0xe6c4247b`.
    ```solidity
    error InvalidAddress();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAddress {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAddress> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAddress()";
            const SELECTOR: [u8; 4] = [230u8, 196u8, 36u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidArgs()` and selector `0xa1ba07ee`.
    ```solidity
    error InvalidArgs();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidArgs {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidArgs> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidArgs) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidArgs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidArgs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidArgs()";
            const SELECTOR: [u8; 4] = [161u8, 186u8, 7u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidHotShotBlockForCommitmentCheck()` and selector `0x615a9264`.
    ```solidity
    error InvalidHotShotBlockForCommitmentCheck();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHotShotBlockForCommitmentCheck {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidHotShotBlockForCommitmentCheck> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHotShotBlockForCommitmentCheck) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHotShotBlockForCommitmentCheck {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHotShotBlockForCommitmentCheck {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHotShotBlockForCommitmentCheck()";
            const SELECTOR: [u8; 4] = [97u8, 90u8, 146u8, 100u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
    ```solidity
    error InvalidInitialization();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidInitialization> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInitialization) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInitialization {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInitialization {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInitialization()";
            const SELECTOR: [u8; 4] = [249u8, 46u8, 232u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidMaxStateHistory()` and selector `0xf4a0eee0`.
    ```solidity
    error InvalidMaxStateHistory();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidMaxStateHistory {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidMaxStateHistory> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidMaxStateHistory) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidMaxStateHistory {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidMaxStateHistory {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidMaxStateHistory()";
            const SELECTOR: [u8; 4] = [244u8, 160u8, 238u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidProof()` and selector `0x09bde339`.
    ```solidity
    error InvalidProof();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProof {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProof()";
            const SELECTOR: [u8; 4] = [9u8, 189u8, 227u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `MissingLastBlockInEpochUpdate()` and selector `0x7150de45`.
    ```solidity
    error MissingLastBlockInEpochUpdate();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MissingLastBlockInEpochUpdate {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MissingLastBlockInEpochUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: MissingLastBlockInEpochUpdate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MissingLastBlockInEpochUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MissingLastBlockInEpochUpdate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MissingLastBlockInEpochUpdate()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 222u8, 69u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `NoChangeRequired()` and selector `0xa863aec9`.
    ```solidity
    error NoChangeRequired();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoChangeRequired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NoChangeRequired> for UnderlyingRustTuple<'_> {
            fn from(value: NoChangeRequired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoChangeRequired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoChangeRequired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoChangeRequired()";
            const SELECTOR: [u8; 4] = [168u8, 99u8, 174u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
    ```solidity
    error NotInitializing();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotInitializing> for UnderlyingRustTuple<'_> {
            fn from(value: NotInitializing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotInitializing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotInitializing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotInitializing()";
            const SELECTOR: [u8; 4] = [215u8, 230u8, 188u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OutdatedState()` and selector `0x051c46ef`.
    ```solidity
    error OutdatedState();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutdatedState {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OutdatedState> for UnderlyingRustTuple<'_> {
            fn from(value: OutdatedState) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutdatedState {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutdatedState {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutdatedState()";
            const SELECTOR: [u8; 4] = [5u8, 28u8, 70u8, 239u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
    ```solidity
    error OwnableInvalidOwner(address owner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
    ```solidity
    error OwnableUnauthorizedAccount(address account);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableUnauthorizedAccount> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ProverNotPermissioned()` and selector `0xa3a64780`.
    ```solidity
    error ProverNotPermissioned();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProverNotPermissioned {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProverNotPermissioned> for UnderlyingRustTuple<'_> {
            fn from(value: ProverNotPermissioned) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProverNotPermissioned {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProverNotPermissioned {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProverNotPermissioned()";
            const SELECTOR: [u8; 4] = [163u8, 166u8, 71u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
    ```solidity
    error UUPSUnauthorizedCallContext();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnauthorizedCallContext> for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnauthorizedCallContext) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UUPSUnauthorizedCallContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnauthorizedCallContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnauthorizedCallContext()";
            const SELECTOR: [u8; 4] = [224u8, 124u8, 141u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`.
    ```solidity
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for UnderlyingRustTuple<'_> {
            fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
                (value.slot,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UUPSUnsupportedProxiableUUID {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { slot: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UUPSUnsupportedProxiableUUID {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UUPSUnsupportedProxiableUUID(bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 29u8, 73u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
        }
    };
    /**Custom error with signature `WrongStakeTableUsed()` and selector `0x51618089`.
    ```solidity
    error WrongStakeTableUsed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongStakeTableUsed {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WrongStakeTableUsed> for UnderlyingRustTuple<'_> {
            fn from(value: WrongStakeTableUsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongStakeTableUsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongStakeTableUsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongStakeTableUsed()";
            const SELECTOR: [u8; 4] = [81u8, 97u8, 128u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `Initialized(uint64)` and selector `0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2`.
    ```solidity
    event Initialized(uint64 version);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8, 19u8,
                    244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8, 33u8, 238u8,
                    209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NewEpoch(uint64)` and selector `0x31eabd9099fdb25dacddd206abff87311e553441fc9d0fcdef201062d7e7071b`.
    ```solidity
    event NewEpoch(uint64 epoch);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewEpoch {
        #[allow(missing_docs)]
        pub epoch: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewEpoch {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NewEpoch(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 234u8, 189u8, 144u8, 153u8, 253u8, 178u8, 93u8, 172u8, 221u8, 210u8, 6u8,
                    171u8, 255u8, 135u8, 49u8, 30u8, 85u8, 52u8, 65u8, 252u8, 157u8, 15u8, 205u8,
                    239u8, 32u8, 16u8, 98u8, 215u8, 231u8, 7u8, 27u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { epoch: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.epoch,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewEpoch {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewEpoch> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewEpoch) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NewState(uint64,uint64,uint256)` and selector `0xa04a773924505a418564363725f56832f5772e6b8d0dbd6efce724dfe803dae6`.
    ```solidity
    event NewState(uint64 indexed viewNum, uint64 indexed blockHeight, BN254.ScalarField blockCommRoot);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewState {
        #[allow(missing_docs)]
        pub viewNum: u64,
        #[allow(missing_docs)]
        pub blockHeight: u64,
        #[allow(missing_docs)]
        pub blockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewState {
            type DataTuple<'a> = (BN254::ScalarField,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "NewState(uint64,uint64,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    160u8, 74u8, 119u8, 57u8, 36u8, 80u8, 90u8, 65u8, 133u8, 100u8, 54u8, 55u8,
                    37u8, 245u8, 104u8, 50u8, 245u8, 119u8, 46u8, 107u8, 141u8, 13u8, 189u8, 110u8,
                    252u8, 231u8, 36u8, 223u8, 232u8, 3u8, 218u8, 230u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    viewNum: topics.1,
                    blockHeight: topics.2,
                    blockCommRoot: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<BN254::ScalarField as alloy_sol_types::SolType>::tokenize(
                    &self.blockCommRoot,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.viewNum.clone(),
                    self.blockHeight.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.viewNum);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blockHeight);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewState {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewState> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewState) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
    ```solidity
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PermissionedProverNotRequired()` and selector `0x9a5f57de856dd668c54dd95e5c55df93432171cbca49a8776d5620ea59c02450`.
    ```solidity
    event PermissionedProverNotRequired();
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PermissionedProverNotRequired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PermissionedProverNotRequired {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PermissionedProverNotRequired()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    154u8, 95u8, 87u8, 222u8, 133u8, 109u8, 214u8, 104u8, 197u8, 77u8, 217u8, 94u8,
                    92u8, 85u8, 223u8, 147u8, 67u8, 33u8, 113u8, 203u8, 202u8, 73u8, 168u8, 119u8,
                    109u8, 86u8, 32u8, 234u8, 89u8, 192u8, 36u8, 80u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PermissionedProverNotRequired {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PermissionedProverNotRequired> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PermissionedProverNotRequired) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PermissionedProverRequired(address)` and selector `0x8017bb887fdf8fca4314a9d40f6e73b3b81002d67e5cfa85d88173af6aa46072`.
    ```solidity
    event PermissionedProverRequired(address permissionedProver);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PermissionedProverRequired {
        #[allow(missing_docs)]
        pub permissionedProver: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PermissionedProverRequired {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PermissionedProverRequired(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    128u8, 23u8, 187u8, 136u8, 127u8, 223u8, 143u8, 202u8, 67u8, 20u8, 169u8,
                    212u8, 15u8, 110u8, 115u8, 179u8, 184u8, 16u8, 2u8, 214u8, 126u8, 92u8, 250u8,
                    133u8, 216u8, 129u8, 115u8, 175u8, 106u8, 164u8, 96u8, 114u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    permissionedProver: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.permissionedProver,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PermissionedProverRequired {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PermissionedProverRequired> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PermissionedProverRequired) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Upgrade(address)` and selector `0xf78721226efe9a1bb678189a16d1554928b9f2192e2cb93eeda83b79fa40007d`.
    ```solidity
    event Upgrade(address implementation);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgrade {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgrade {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Upgrade(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    247u8, 135u8, 33u8, 34u8, 110u8, 254u8, 154u8, 27u8, 182u8, 120u8, 24u8, 154u8,
                    22u8, 209u8, 85u8, 73u8, 40u8, 185u8, 242u8, 25u8, 46u8, 44u8, 185u8, 62u8,
                    237u8, 168u8, 59u8, 121u8, 250u8, 64u8, 0u8, 125u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    implementation: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.implementation,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgrade {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgrade> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgrade) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Upgraded(address)` and selector `0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b`.
    ```solidity
    event Upgraded(address indexed implementation);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Upgraded {
        #[allow(missing_docs)]
        pub implementation: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Upgraded {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Upgraded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                    179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8, 12u8,
                    192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    implementation: topics.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.implementation.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.implementation,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Upgraded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Upgraded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Upgraded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(LightClient.LightClientState genesis, LightClient.StakeTableState genesisStakeTableState, uint32 maxHistorySeconds, uint64 blocksPerEpoch);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
        pub genesisStakeTableState:
            <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
        pub maxHistorySeconds: u32,
        pub blocksPerEpoch: u64,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                LightClient::LightClientState,
                LightClient::StakeTableState,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
                <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
                u32,
                u64,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value.genesis,
                        value.genesisStakeTableState,
                        value.maxHistorySeconds,
                        value.blocksPerEpoch,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        genesis: tuple.0,
                        genesisStakeTableState: tuple.1,
                        maxHistorySeconds: tuple.2,
                        blocksPerEpoch: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                LightClient::LightClientState,
                LightClient::StakeTableState,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LightClient::LightClientState as alloy_sol_types::SolType>::tokenize(
                        &self.genesis,
                    ),
                    <LightClient::StakeTableState as alloy_sol_types::SolType>::tokenize(
                        &self.genesisStakeTableState,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.maxHistorySeconds,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.blocksPerEpoch,
                    ),
                )
            }
        }
    };
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
    ```solidity
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall {}
    ///Container type for the return parameters of the [`UPGRADE_INTERFACE_VERSION()`](UPGRADE_INTERFACE_VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONReturn {
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for UPGRADE_INTERFACE_VERSIONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UPGRADE_INTERFACE_VERSIONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: UPGRADE_INTERFACE_VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for UPGRADE_INTERFACE_VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UPGRADE_INTERFACE_VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = UPGRADE_INTERFACE_VERSIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UPGRADE_INTERFACE_VERSION()";
            const SELECTOR: [u8; 4] = [173u8, 60u8, 177u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `_blocksPerEpoch()` and selector `0xb2424e3f`.
    ```solidity
    function _blocksPerEpoch() external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _blocksPerEpochCall {}
    ///Container type for the return parameters of the [`_blocksPerEpoch()`](_blocksPerEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _blocksPerEpochReturn {
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<_blocksPerEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: _blocksPerEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _blocksPerEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<_blocksPerEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: _blocksPerEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _blocksPerEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _blocksPerEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = _blocksPerEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_blocksPerEpoch()";
            const SELECTOR: [u8; 4] = [178u8, 66u8, 78u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `currentBlockNumber()` and selector `0x378ec23b`.
    ```solidity
    function currentBlockNumber() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentBlockNumberCall {}
    ///Container type for the return parameters of the [`currentBlockNumber()`](currentBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentBlockNumberReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentBlockNumber()";
            const SELECTOR: [u8; 4] = [55u8, 142u8, 194u8, 59u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `currentEpoch()` and selector `0x76671808`.
    ```solidity
    function currentEpoch() external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochCall {}
    ///Container type for the return parameters of the [`currentEpoch()`](currentEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochReturn {
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentEpoch()";
            const SELECTOR: [u8; 4] = [118u8, 103u8, 24u8, 8u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `disablePermissionedProverMode()` and selector `0x69cc6a04`.
    ```solidity
    function disablePermissionedProverMode() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disablePermissionedProverModeCall {}
    ///Container type for the return parameters of the [`disablePermissionedProverMode()`](disablePermissionedProverModeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disablePermissionedProverModeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<disablePermissionedProverModeCall> for UnderlyingRustTuple<'_> {
                fn from(value: disablePermissionedProverModeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disablePermissionedProverModeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<disablePermissionedProverModeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disablePermissionedProverModeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disablePermissionedProverModeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disablePermissionedProverModeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = disablePermissionedProverModeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disablePermissionedProverMode()";
            const SELECTOR: [u8; 4] = [105u8, 204u8, 106u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `epochFromBlockNumber(uint64,uint64)` and selector `0x90c14390`.
    ```solidity
    function epochFromBlockNumber(uint64 blockNum, uint64 blocksPerEpoch) external pure returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct epochFromBlockNumberCall {
        pub blockNum: u64,
        pub blocksPerEpoch: u64,
    }
    ///Container type for the return parameters of the [`epochFromBlockNumber(uint64,uint64)`](epochFromBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct epochFromBlockNumberReturn {
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<epochFromBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: epochFromBlockNumberCall) -> Self {
                    (value.blockNum, value.blocksPerEpoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for epochFromBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNum: tuple.0,
                        blocksPerEpoch: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<epochFromBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: epochFromBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for epochFromBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for epochFromBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = epochFromBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "epochFromBlockNumber(uint64,uint64)";
            const SELECTOR: [u8; 4] = [144u8, 193u8, 67u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNum,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.blocksPerEpoch,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `finalizedState()` and selector `0x9fdb54a7`.
    ```solidity
    function finalizedState() external view returns (uint64 viewNum, uint64 blockHeight, BN254.ScalarField blockCommRoot);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizedStateCall {}
    ///Container type for the return parameters of the [`finalizedState()`](finalizedStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct finalizedStateReturn {
        pub viewNum: u64,
        pub blockHeight: u64,
        pub blockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<finalizedStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: finalizedStateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for finalizedStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                BN254::ScalarField,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<finalizedStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: finalizedStateReturn) -> Self {
                    (value.viewNum, value.blockHeight, value.blockCommRoot)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for finalizedStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        viewNum: tuple.0,
                        blockHeight: tuple.1,
                        blockCommRoot: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for finalizedStateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = finalizedStateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                BN254::ScalarField,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "finalizedState()";
            const SELECTOR: [u8; 4] = [159u8, 219u8, 84u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `genesisStakeTableState()` and selector `0x426d3194`.
    ```solidity
    function genesisStakeTableState() external view returns (uint256 threshold, BN254.ScalarField blsKeyComm, BN254.ScalarField schnorrKeyComm, BN254.ScalarField amountComm);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct genesisStakeTableStateCall {}
    ///Container type for the return parameters of the [`genesisStakeTableState()`](genesisStakeTableStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct genesisStakeTableStateReturn {
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
        pub blsKeyComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub schnorrKeyComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub amountComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<genesisStakeTableStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: genesisStakeTableStateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for genesisStakeTableStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BN254::ScalarField,
                BN254::ScalarField,
                BN254::ScalarField,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<genesisStakeTableStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: genesisStakeTableStateReturn) -> Self {
                    (
                        value.threshold,
                        value.blsKeyComm,
                        value.schnorrKeyComm,
                        value.amountComm,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for genesisStakeTableStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        threshold: tuple.0,
                        blsKeyComm: tuple.1,
                        schnorrKeyComm: tuple.2,
                        amountComm: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for genesisStakeTableStateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = genesisStakeTableStateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BN254::ScalarField,
                BN254::ScalarField,
                BN254::ScalarField,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "genesisStakeTableState()";
            const SELECTOR: [u8; 4] = [66u8, 109u8, 49u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `genesisState()` and selector `0xd24d933d`.
    ```solidity
    function genesisState() external view returns (uint64 viewNum, uint64 blockHeight, BN254.ScalarField blockCommRoot);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct genesisStateCall {}
    ///Container type for the return parameters of the [`genesisState()`](genesisStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct genesisStateReturn {
        pub viewNum: u64,
        pub blockHeight: u64,
        pub blockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<genesisStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: genesisStateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for genesisStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                BN254::ScalarField,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<genesisStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: genesisStateReturn) -> Self {
                    (value.viewNum, value.blockHeight, value.blockCommRoot)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for genesisStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        viewNum: tuple.0,
                        blockHeight: tuple.1,
                        blockCommRoot: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for genesisStateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = genesisStateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                BN254::ScalarField,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "genesisState()";
            const SELECTOR: [u8; 4] = [210u8, 77u8, 147u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getHotShotCommitment(uint256)` and selector `0x8584d23f`.
    ```solidity
    function getHotShotCommitment(uint256 hotShotBlockHeight) external view returns (BN254.ScalarField hotShotBlockCommRoot, uint64 hotshotBlockHeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHotShotCommitmentCall {
        pub hotShotBlockHeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getHotShotCommitment(uint256)`](getHotShotCommitmentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHotShotCommitmentReturn {
        pub hotShotBlockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub hotshotBlockHeight: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHotShotCommitmentCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHotShotCommitmentCall) -> Self {
                    (value.hotShotBlockHeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHotShotCommitmentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        hotShotBlockHeight: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (BN254::ScalarField, alloy::sol_types::sol_data::Uint<64>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
                u64,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHotShotCommitmentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getHotShotCommitmentReturn) -> Self {
                    (value.hotShotBlockCommRoot, value.hotshotBlockHeight)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHotShotCommitmentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        hotShotBlockCommRoot: tuple.0,
                        hotshotBlockHeight: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHotShotCommitmentCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHotShotCommitmentReturn;
            type ReturnTuple<'a> = (BN254::ScalarField, alloy::sol_types::sol_data::Uint<64>);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHotShotCommitment(uint256)";
            const SELECTOR: [u8; 4] = [133u8, 132u8, 210u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.hotShotBlockHeight,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getStateHistoryCount()` and selector `0xf9e50d19`.
    ```solidity
    function getStateHistoryCount() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateHistoryCountCall {}
    ///Container type for the return parameters of the [`getStateHistoryCount()`](getStateHistoryCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateHistoryCountReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStateHistoryCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStateHistoryCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStateHistoryCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStateHistoryCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStateHistoryCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStateHistoryCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStateHistoryCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStateHistoryCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStateHistoryCount()";
            const SELECTOR: [u8; 4] = [249u8, 229u8, 13u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getVersion()` and selector `0x0d8e6e2c`.
    ```solidity
    function getVersion() external pure returns (uint8 majorVersion, uint8 minorVersion, uint8 patchVersion);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVersionCall {}
    ///Container type for the return parameters of the [`getVersion()`](getVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVersionReturn {
        pub majorVersion: u8,
        pub minorVersion: u8,
        pub patchVersion: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVersionCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVersionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u8, u8);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVersionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVersionReturn) -> Self {
                    (value.majorVersion, value.minorVersion, value.patchVersion)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        majorVersion: tuple.0,
                        minorVersion: tuple.1,
                        patchVersion: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVersionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVersionReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVersion()";
            const SELECTOR: [u8; 4] = [13u8, 142u8, 110u8, 44u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),uint32,address,uint64)` and selector `0x811f853f`.
    ```solidity
    function initialize(LightClient.LightClientState memory _genesis, LightClient.StakeTableState memory _genesisStakeTableState, uint32 _stateHistoryRetentionPeriod, address owner, uint64 blocksPerEpoch) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
        pub _genesisStakeTableState:
            <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
        pub _stateHistoryRetentionPeriod: u32,
        pub owner: alloy::sol_types::private::Address,
        pub blocksPerEpoch: u64,
    }
    ///Container type for the return parameters of the [`initialize((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),uint32,address,uint64)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                LightClient::LightClientState,
                LightClient::StakeTableState,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
                <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
                u32,
                alloy::sol_types::private::Address,
                u64,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value._genesis,
                        value._genesisStakeTableState,
                        value._stateHistoryRetentionPeriod,
                        value.owner,
                        value.blocksPerEpoch,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _genesis: tuple.0,
                        _genesisStakeTableState: tuple.1,
                        _stateHistoryRetentionPeriod: tuple.2,
                        owner: tuple.3,
                        blocksPerEpoch: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                LightClient::LightClientState,
                LightClient::StakeTableState,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),uint32,address,uint64)";
            const SELECTOR: [u8; 4] = [129u8, 31u8, 133u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LightClient::LightClientState as alloy_sol_types::SolType>::tokenize(
                        &self._genesis,
                    ),
                    <LightClient::StakeTableState as alloy_sol_types::SolType>::tokenize(
                        &self._genesisStakeTableState,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._stateHistoryRetentionPeriod,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.blocksPerEpoch,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isLastBlockInEpoch(uint64)` and selector `0xa1be8d52`.
    ```solidity
    function isLastBlockInEpoch(uint64 blockHeight) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLastBlockInEpochCall {
        pub blockHeight: u64,
    }
    ///Container type for the return parameters of the [`isLastBlockInEpoch(uint64)`](isLastBlockInEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLastBlockInEpochReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isLastBlockInEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: isLastBlockInEpochCall) -> Self {
                    (value.blockHeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isLastBlockInEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockHeight: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isLastBlockInEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isLastBlockInEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isLastBlockInEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isLastBlockInEpochCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isLastBlockInEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isLastBlockInEpoch(uint64)";
            const SELECTOR: [u8; 4] = [161u8, 190u8, 141u8, 82u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.blockHeight,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isPermissionedProverEnabled()` and selector `0x826e41fc`.
    ```solidity
    function isPermissionedProverEnabled() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPermissionedProverEnabledCall {}
    ///Container type for the return parameters of the [`isPermissionedProverEnabled()`](isPermissionedProverEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPermissionedProverEnabledReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isPermissionedProverEnabledCall> for UnderlyingRustTuple<'_> {
                fn from(value: isPermissionedProverEnabledCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPermissionedProverEnabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isPermissionedProverEnabledReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isPermissionedProverEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPermissionedProverEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPermissionedProverEnabledCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isPermissionedProverEnabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isPermissionedProverEnabled()";
            const SELECTOR: [u8; 4] = [130u8, 110u8, 65u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `lagOverEscapeHatchThreshold(uint256,uint256)` and selector `0xe0303301`.
    ```solidity
    function lagOverEscapeHatchThreshold(uint256 blockNumber, uint256 threshold) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lagOverEscapeHatchThresholdCall {
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`lagOverEscapeHatchThreshold(uint256,uint256)`](lagOverEscapeHatchThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lagOverEscapeHatchThresholdReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lagOverEscapeHatchThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: lagOverEscapeHatchThresholdCall) -> Self {
                    (value.blockNumber, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lagOverEscapeHatchThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        threshold: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lagOverEscapeHatchThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lagOverEscapeHatchThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lagOverEscapeHatchThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lagOverEscapeHatchThresholdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lagOverEscapeHatchThresholdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lagOverEscapeHatchThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [224u8, 48u8, 51u8, 1u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.threshold,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `newFinalizedState((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x757c37ad`.
    ```solidity
    function newFinalizedState(LightClient.LightClientState memory newState, LightClient.StakeTableState memory nextStakeTable, IPlonkVerifier.PlonkProof memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newFinalizedStateCall {
        pub newState: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
        pub nextStakeTable: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
        pub proof: <IPlonkVerifier::PlonkProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`newFinalizedState((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))`](newFinalizedStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newFinalizedStateReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                LightClient::LightClientState,
                LightClient::StakeTableState,
                IPlonkVerifier::PlonkProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
                <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
                <IPlonkVerifier::PlonkProof as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<newFinalizedStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: newFinalizedStateCall) -> Self {
                    (value.newState, value.nextStakeTable, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newFinalizedStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newState: tuple.0,
                        nextStakeTable: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<newFinalizedStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: newFinalizedStateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newFinalizedStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for newFinalizedStateCall {
            type Parameters<'a> = (
                LightClient::LightClientState,
                LightClient::StakeTableState,
                IPlonkVerifier::PlonkProof,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = newFinalizedStateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "newFinalizedState((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256),uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [117u8, 124u8, 55u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LightClient::LightClientState as alloy_sol_types::SolType>::tokenize(
                        &self.newState,
                    ),
                    <LightClient::StakeTableState as alloy_sol_types::SolType>::tokenize(
                        &self.nextStakeTable,
                    ),
                    <IPlonkVerifier::PlonkProof as alloy_sol_types::SolType>::tokenize(&self.proof),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `permissionedProver()` and selector `0x313df7b1`.
    ```solidity
    function permissionedProver() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionedProverCall {}
    ///Container type for the return parameters of the [`permissionedProver()`](permissionedProverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionedProverReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permissionedProverCall> for UnderlyingRustTuple<'_> {
                fn from(value: permissionedProverCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionedProverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<permissionedProverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permissionedProverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionedProverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionedProverCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = permissionedProverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permissionedProver()";
            const SELECTOR: [u8; 4] = [49u8, 61u8, 247u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
    ```solidity
    function proxiableUUID() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall {}
    ///Container type for the return parameters of the [`proxiableUUID()`](proxiableUUIDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDCall> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proxiableUUIDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proxiableUUIDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proxiableUUIDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proxiableUUIDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = proxiableUUIDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proxiableUUID()";
            const SELECTOR: [u8; 4] = [82u8, 209u8, 144u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setBlocksPerEpoch(uint64)` and selector `0x3c23b6db`.
    ```solidity
    function setBlocksPerEpoch(uint64 newBlocksPerEpoch) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlocksPerEpochCall {
        pub newBlocksPerEpoch: u64,
    }
    ///Container type for the return parameters of the [`setBlocksPerEpoch(uint64)`](setBlocksPerEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setBlocksPerEpochReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBlocksPerEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: setBlocksPerEpochCall) -> Self {
                    (value.newBlocksPerEpoch,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlocksPerEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newBlocksPerEpoch: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setBlocksPerEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setBlocksPerEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setBlocksPerEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setBlocksPerEpochCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setBlocksPerEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setBlocksPerEpoch(uint64)";
            const SELECTOR: [u8; 4] = [60u8, 35u8, 182u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.newBlocksPerEpoch,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setFinalizedState((uint64,uint64,uint256))` and selector `0xb5adea3c`.
    ```solidity
    function setFinalizedState(LightClient.LightClientState memory state) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFinalizedStateCall {
        pub state: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setFinalizedState((uint64,uint64,uint256))`](setFinalizedStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFinalizedStateReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (LightClient::LightClientState,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<LightClient::LightClientState as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setFinalizedStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: setFinalizedStateCall) -> Self {
                    (value.state,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFinalizedStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { state: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setFinalizedStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setFinalizedStateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFinalizedStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setFinalizedStateCall {
            type Parameters<'a> = (LightClient::LightClientState,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setFinalizedStateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setFinalizedState((uint64,uint64,uint256))";
            const SELECTOR: [u8; 4] = [181u8, 173u8, 234u8, 60u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LightClient::LightClientState as alloy_sol_types::SolType>::tokenize(
                        &self.state,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setHotShotDownSince(uint256)` and selector `0x2d52aad6`.
    ```solidity
    function setHotShotDownSince(uint256 l1Height) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setHotShotDownSinceCall {
        pub l1Height: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setHotShotDownSince(uint256)`](setHotShotDownSinceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setHotShotDownSinceReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setHotShotDownSinceCall> for UnderlyingRustTuple<'_> {
                fn from(value: setHotShotDownSinceCall) -> Self {
                    (value.l1Height,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setHotShotDownSinceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { l1Height: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setHotShotDownSinceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setHotShotDownSinceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setHotShotDownSinceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setHotShotDownSinceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setHotShotDownSinceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setHotShotDownSince(uint256)";
            const SELECTOR: [u8; 4] = [45u8, 82u8, 170u8, 214u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.l1Height,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setHotShotUp()` and selector `0xc8e5e498`.
    ```solidity
    function setHotShotUp() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setHotShotUpCall {}
    ///Container type for the return parameters of the [`setHotShotUp()`](setHotShotUpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setHotShotUpReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setHotShotUpCall> for UnderlyingRustTuple<'_> {
                fn from(value: setHotShotUpCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setHotShotUpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setHotShotUpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setHotShotUpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setHotShotUpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setHotShotUpCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setHotShotUpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setHotShotUp()";
            const SELECTOR: [u8; 4] = [200u8, 229u8, 228u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setPermissionedProver(address)` and selector `0x013fa5fc`.
    ```solidity
    function setPermissionedProver(address prover) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPermissionedProverCall {
        pub prover: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPermissionedProver(address)`](setPermissionedProverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPermissionedProverReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPermissionedProverCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPermissionedProverCall) -> Self {
                    (value.prover,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPermissionedProverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { prover: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPermissionedProverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setPermissionedProverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPermissionedProverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPermissionedProverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPermissionedProverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPermissionedProver(address)";
            const SELECTOR: [u8; 4] = [1u8, 63u8, 165u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prover,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setStateHistory((uint64,uint64,uint64,uint256)[])` and selector `0xf5676160`.
    ```solidity
    function setStateHistory(LightClient.StateHistoryCommitment[] memory _stateHistoryCommitments) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStateHistoryCall {
        pub _stateHistoryCommitments: alloy::sol_types::private::Vec<
            <LightClient::StateHistoryCommitment as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`setStateHistory((uint64,uint64,uint64,uint256)[])`](setStateHistoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStateHistoryReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<LightClient::StateHistoryCommitment>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <LightClient::StateHistoryCommitment as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setStateHistoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: setStateHistoryCall) -> Self {
                    (value._stateHistoryCommitments,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStateHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _stateHistoryCommitments: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setStateHistoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setStateHistoryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStateHistoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStateHistoryCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<LightClient::StateHistoryCommitment>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStateHistoryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setStateHistory((uint64,uint64,uint64,uint256)[])";
            const SELECTOR: [u8; 4] = [245u8, 103u8, 97u8, 96u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    LightClient::StateHistoryCommitment,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._stateHistoryCommitments,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setstateHistoryRetentionPeriod(uint32)` and selector `0x96c1ca61`.
    ```solidity
    function setstateHistoryRetentionPeriod(uint32 historySeconds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setstateHistoryRetentionPeriodCall {
        pub historySeconds: u32,
    }
    ///Container type for the return parameters of the [`setstateHistoryRetentionPeriod(uint32)`](setstateHistoryRetentionPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setstateHistoryRetentionPeriodReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setstateHistoryRetentionPeriodCall> for UnderlyingRustTuple<'_> {
                fn from(value: setstateHistoryRetentionPeriodCall) -> Self {
                    (value.historySeconds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setstateHistoryRetentionPeriodCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        historySeconds: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setstateHistoryRetentionPeriodReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setstateHistoryRetentionPeriodReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setstateHistoryRetentionPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setstateHistoryRetentionPeriodCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setstateHistoryRetentionPeriodReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setstateHistoryRetentionPeriod(uint32)";
            const SELECTOR: [u8; 4] = [150u8, 193u8, 202u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.historySeconds,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stateHistoryCommitments(uint256)` and selector `0x02b592f3`.
    ```solidity
    function stateHistoryCommitments(uint256) external view returns (uint64 l1BlockHeight, uint64 l1BlockTimestamp, uint64 hotShotBlockHeight, BN254.ScalarField hotShotBlockCommRoot);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateHistoryCommitmentsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`stateHistoryCommitments(uint256)`](stateHistoryCommitmentsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateHistoryCommitmentsReturn {
        pub l1BlockHeight: u64,
        pub l1BlockTimestamp: u64,
        pub hotShotBlockHeight: u64,
        pub hotShotBlockCommRoot: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateHistoryCommitmentsCall> for UnderlyingRustTuple<'_> {
                fn from(value: stateHistoryCommitmentsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateHistoryCommitmentsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                BN254::ScalarField,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                u64,
                u64,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateHistoryCommitmentsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stateHistoryCommitmentsReturn) -> Self {
                    (
                        value.l1BlockHeight,
                        value.l1BlockTimestamp,
                        value.hotShotBlockHeight,
                        value.hotShotBlockCommRoot,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateHistoryCommitmentsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        l1BlockHeight: tuple.0,
                        l1BlockTimestamp: tuple.1,
                        hotShotBlockHeight: tuple.2,
                        hotShotBlockCommRoot: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stateHistoryCommitmentsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stateHistoryCommitmentsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                BN254::ScalarField,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stateHistoryCommitments(uint256)";
            const SELECTOR: [u8; 4] = [2u8, 181u8, 146u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stateHistoryFirstIndex()` and selector `0x2f79889d`.
    ```solidity
    function stateHistoryFirstIndex() external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateHistoryFirstIndexCall {}
    ///Container type for the return parameters of the [`stateHistoryFirstIndex()`](stateHistoryFirstIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateHistoryFirstIndexReturn {
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateHistoryFirstIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: stateHistoryFirstIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateHistoryFirstIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateHistoryFirstIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stateHistoryFirstIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateHistoryFirstIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stateHistoryFirstIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stateHistoryFirstIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stateHistoryFirstIndex()";
            const SELECTOR: [u8; 4] = [47u8, 121u8, 136u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stateHistoryRetentionPeriod()` and selector `0xc23b9e9e`.
    ```solidity
    function stateHistoryRetentionPeriod() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateHistoryRetentionPeriodCall {}
    ///Container type for the return parameters of the [`stateHistoryRetentionPeriod()`](stateHistoryRetentionPeriodCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stateHistoryRetentionPeriodReturn {
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateHistoryRetentionPeriodCall> for UnderlyingRustTuple<'_> {
                fn from(value: stateHistoryRetentionPeriodCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateHistoryRetentionPeriodCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stateHistoryRetentionPeriodReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stateHistoryRetentionPeriodReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stateHistoryRetentionPeriodReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stateHistoryRetentionPeriodCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stateHistoryRetentionPeriodReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stateHistoryRetentionPeriod()";
            const SELECTOR: [u8; 4] = [194u8, 59u8, 158u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`.
    ```solidity
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallCall {
        pub newImplementation: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`upgradeToAndCall(address,bytes)`](upgradeToAndCallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct upgradeToAndCallReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallCall> for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallCall) -> Self {
                    (value.newImplementation, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for upgradeToAndCallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newImplementation: tuple.0,
                        data: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<upgradeToAndCallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: upgradeToAndCallReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for upgradeToAndCallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for upgradeToAndCallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = upgradeToAndCallReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "upgradeToAndCall(address,bytes)";
            const SELECTOR: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newImplementation,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `votingStakeTableState()` and selector `0x0625e19b`.
    ```solidity
    function votingStakeTableState() external view returns (uint256 threshold, BN254.ScalarField blsKeyComm, BN254.ScalarField schnorrKeyComm, BN254.ScalarField amountComm);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votingStakeTableStateCall {}
    ///Container type for the return parameters of the [`votingStakeTableState()`](votingStakeTableStateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct votingStakeTableStateReturn {
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
        pub blsKeyComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub schnorrKeyComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
        pub amountComm: <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<votingStakeTableStateCall> for UnderlyingRustTuple<'_> {
                fn from(value: votingStakeTableStateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votingStakeTableStateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BN254::ScalarField,
                BN254::ScalarField,
                BN254::ScalarField,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
                <BN254::ScalarField as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<votingStakeTableStateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: votingStakeTableStateReturn) -> Self {
                    (
                        value.threshold,
                        value.blsKeyComm,
                        value.schnorrKeyComm,
                        value.amountComm,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for votingStakeTableStateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        threshold: tuple.0,
                        blsKeyComm: tuple.1,
                        schnorrKeyComm: tuple.2,
                        amountComm: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for votingStakeTableStateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = votingStakeTableStateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                BN254::ScalarField,
                BN254::ScalarField,
                BN254::ScalarField,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "votingStakeTableState()";
            const SELECTOR: [u8; 4] = [6u8, 37u8, 225u8, 155u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`LightClientMock`](self) function calls.
    pub enum LightClientMockCalls {
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        _blocksPerEpoch(_blocksPerEpochCall),
        currentBlockNumber(currentBlockNumberCall),
        currentEpoch(currentEpochCall),
        disablePermissionedProverMode(disablePermissionedProverModeCall),
        epochFromBlockNumber(epochFromBlockNumberCall),
        finalizedState(finalizedStateCall),
        genesisStakeTableState(genesisStakeTableStateCall),
        genesisState(genesisStateCall),
        getHotShotCommitment(getHotShotCommitmentCall),
        getStateHistoryCount(getStateHistoryCountCall),
        getVersion(getVersionCall),
        initialize(initializeCall),
        isLastBlockInEpoch(isLastBlockInEpochCall),
        isPermissionedProverEnabled(isPermissionedProverEnabledCall),
        lagOverEscapeHatchThreshold(lagOverEscapeHatchThresholdCall),
        newFinalizedState(newFinalizedStateCall),
        owner(ownerCall),
        permissionedProver(permissionedProverCall),
        proxiableUUID(proxiableUUIDCall),
        renounceOwnership(renounceOwnershipCall),
        setBlocksPerEpoch(setBlocksPerEpochCall),
        setFinalizedState(setFinalizedStateCall),
        setHotShotDownSince(setHotShotDownSinceCall),
        setHotShotUp(setHotShotUpCall),
        setPermissionedProver(setPermissionedProverCall),
        setStateHistory(setStateHistoryCall),
        setstateHistoryRetentionPeriod(setstateHistoryRetentionPeriodCall),
        stateHistoryCommitments(stateHistoryCommitmentsCall),
        stateHistoryFirstIndex(stateHistoryFirstIndexCall),
        stateHistoryRetentionPeriod(stateHistoryRetentionPeriodCall),
        transferOwnership(transferOwnershipCall),
        upgradeToAndCall(upgradeToAndCallCall),
        votingStakeTableState(votingStakeTableStateCall),
    }
    #[automatically_derived]
    impl LightClientMockCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 63u8, 165u8, 252u8],
            [2u8, 181u8, 146u8, 243u8],
            [6u8, 37u8, 225u8, 155u8],
            [13u8, 142u8, 110u8, 44u8],
            [45u8, 82u8, 170u8, 214u8],
            [47u8, 121u8, 136u8, 157u8],
            [49u8, 61u8, 247u8, 177u8],
            [55u8, 142u8, 194u8, 59u8],
            [60u8, 35u8, 182u8, 219u8],
            [66u8, 109u8, 49u8, 148u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [105u8, 204u8, 106u8, 4u8],
            [113u8, 80u8, 24u8, 166u8],
            [117u8, 124u8, 55u8, 173u8],
            [118u8, 103u8, 24u8, 8u8],
            [129u8, 31u8, 133u8, 63u8],
            [130u8, 110u8, 65u8, 252u8],
            [133u8, 132u8, 210u8, 63u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 193u8, 67u8, 144u8],
            [150u8, 193u8, 202u8, 97u8],
            [159u8, 219u8, 84u8, 167u8],
            [161u8, 190u8, 141u8, 82u8],
            [173u8, 60u8, 177u8, 204u8],
            [178u8, 66u8, 78u8, 63u8],
            [181u8, 173u8, 234u8, 60u8],
            [194u8, 59u8, 158u8, 158u8],
            [200u8, 229u8, 228u8, 152u8],
            [210u8, 77u8, 147u8, 61u8],
            [224u8, 48u8, 51u8, 1u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 103u8, 97u8, 96u8],
            [249u8, 229u8, 13u8, 25u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for LightClientMockCalls {
        const NAME: &'static str = "LightClientMockCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::_blocksPerEpoch(_) => {
                    <_blocksPerEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentBlockNumber(_) => {
                    <currentBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentEpoch(_) => <currentEpochCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::disablePermissionedProverMode(_) => {
                    <disablePermissionedProverModeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::epochFromBlockNumber(_) => {
                    <epochFromBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::finalizedState(_) => {
                    <finalizedStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::genesisStakeTableState(_) => {
                    <genesisStakeTableStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::genesisState(_) => <genesisStateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getHotShotCommitment(_) => {
                    <getHotShotCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStateHistoryCount(_) => {
                    <getStateHistoryCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => <getVersionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isLastBlockInEpoch(_) => {
                    <isLastBlockInEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isPermissionedProverEnabled(_) => {
                    <isPermissionedProverEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lagOverEscapeHatchThreshold(_) => {
                    <lagOverEscapeHatchThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::newFinalizedState(_) => {
                    <newFinalizedStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::permissionedProver(_) => {
                    <permissionedProverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setBlocksPerEpoch(_) => {
                    <setBlocksPerEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setFinalizedState(_) => {
                    <setFinalizedStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setHotShotDownSince(_) => {
                    <setHotShotDownSinceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setHotShotUp(_) => <setHotShotUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setPermissionedProver(_) => {
                    <setPermissionedProverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStateHistory(_) => {
                    <setStateHistoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setstateHistoryRetentionPeriod(_) => {
                    <setstateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stateHistoryCommitments(_) => {
                    <stateHistoryCommitmentsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stateHistoryFirstIndex(_) => {
                    <stateHistoryFirstIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stateHistoryRetentionPeriod(_) => {
                    <stateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::votingStakeTableState(_) => {
                    <votingStakeTableStateCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<LightClientMockCalls>] = &[
                {
                    fn setPermissionedProver(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setPermissionedProverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::setPermissionedProver)
                    }
                    setPermissionedProver
                },
                {
                    fn stateHistoryCommitments(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <stateHistoryCommitmentsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::stateHistoryCommitments)
                    }
                    stateHistoryCommitments
                },
                {
                    fn votingStakeTableState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <votingStakeTableStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::votingStakeTableState)
                    }
                    votingStakeTableState
                },
                {
                    fn getVersion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(LightClientMockCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn setHotShotDownSince(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setHotShotDownSinceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::setHotShotDownSince)
                    }
                    setHotShotDownSince
                },
                {
                    fn stateHistoryFirstIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <stateHistoryFirstIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::stateHistoryFirstIndex)
                    }
                    stateHistoryFirstIndex
                },
                {
                    fn permissionedProver(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <permissionedProverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::permissionedProver)
                    }
                    permissionedProver
                },
                {
                    fn currentBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <currentBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::currentBlockNumber)
                    }
                    currentBlockNumber
                },
                {
                    fn setBlocksPerEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setBlocksPerEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::setBlocksPerEpoch)
                    }
                    setBlocksPerEpoch
                },
                {
                    fn genesisStakeTableState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <genesisStakeTableStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::genesisStakeTableState)
                    }
                    genesisStakeTableState
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn disablePermissionedProverMode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <disablePermissionedProverModeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LightClientMockCalls::disablePermissionedProverMode)
                    }
                    disablePermissionedProverMode
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn newFinalizedState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <newFinalizedStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::newFinalizedState)
                    }
                    newFinalizedState
                },
                {
                    fn currentEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <currentEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::currentEpoch)
                    }
                    currentEpoch
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(LightClientMockCalls::initialize)
                    }
                    initialize
                },
                {
                    fn isPermissionedProverEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <isPermissionedProverEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LightClientMockCalls::isPermissionedProverEnabled)
                    }
                    isPermissionedProverEnabled
                },
                {
                    fn getHotShotCommitment(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <getHotShotCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::getHotShotCommitment)
                    }
                    getHotShotCommitment
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(LightClientMockCalls::owner)
                    }
                    owner
                },
                {
                    fn epochFromBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <epochFromBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::epochFromBlockNumber)
                    }
                    epochFromBlockNumber
                },
                {
                    fn setstateHistoryRetentionPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setstateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LightClientMockCalls::setstateHistoryRetentionPeriod)
                    }
                    setstateHistoryRetentionPeriod
                },
                {
                    fn finalizedState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <finalizedStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::finalizedState)
                    }
                    finalizedState
                },
                {
                    fn isLastBlockInEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <isLastBlockInEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::isLastBlockInEpoch)
                    }
                    isLastBlockInEpoch
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn _blocksPerEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <_blocksPerEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::_blocksPerEpoch)
                    }
                    _blocksPerEpoch
                },
                {
                    fn setFinalizedState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setFinalizedStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::setFinalizedState)
                    }
                    setFinalizedState
                },
                {
                    fn stateHistoryRetentionPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <stateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LightClientMockCalls::stateHistoryRetentionPeriod)
                    }
                    stateHistoryRetentionPeriod
                },
                {
                    fn setHotShotUp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setHotShotUpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::setHotShotUp)
                    }
                    setHotShotUp
                },
                {
                    fn genesisState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <genesisStateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::genesisState)
                    }
                    genesisState
                },
                {
                    fn lagOverEscapeHatchThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <lagOverEscapeHatchThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LightClientMockCalls::lagOverEscapeHatchThreshold)
                    }
                    lagOverEscapeHatchThreshold
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn setStateHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <setStateHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::setStateHistory)
                    }
                    setStateHistory
                },
                {
                    fn getStateHistoryCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockCalls> {
                        <getStateHistoryCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockCalls::getStateHistoryCount)
                    }
                    getStateHistoryCount
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::_blocksPerEpoch(inner) => {
                    <_blocksPerEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentBlockNumber(inner) => {
                    <currentBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disablePermissionedProverMode(inner) => {
                    <disablePermissionedProverModeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::epochFromBlockNumber(inner) => {
                    <epochFromBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::finalizedState(inner) => {
                    <finalizedStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::genesisStakeTableState(inner) => {
                    <genesisStakeTableStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::genesisState(inner) => {
                    <genesisStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHotShotCommitment(inner) => {
                    <getHotShotCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStateHistoryCount(inner) => {
                    <getStateHistoryCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVersion(inner) => {
                    <getVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isLastBlockInEpoch(inner) => {
                    <isLastBlockInEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isPermissionedProverEnabled(inner) => {
                    <isPermissionedProverEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lagOverEscapeHatchThreshold(inner) => {
                    <lagOverEscapeHatchThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::newFinalizedState(inner) => {
                    <newFinalizedStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::permissionedProver(inner) => {
                    <permissionedProverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setBlocksPerEpoch(inner) => {
                    <setBlocksPerEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setFinalizedState(inner) => {
                    <setFinalizedStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setHotShotDownSince(inner) => {
                    <setHotShotDownSinceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setHotShotUp(inner) => {
                    <setHotShotUpCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPermissionedProver(inner) => {
                    <setPermissionedProverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStateHistory(inner) => {
                    <setStateHistoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setstateHistoryRetentionPeriod(inner) => {
                    <setstateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stateHistoryCommitments(inner) => {
                    <stateHistoryCommitmentsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stateHistoryFirstIndex(inner) => {
                    <stateHistoryFirstIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stateHistoryRetentionPeriod(inner) => {
                    <stateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::votingStakeTableState(inner) => {
                    <votingStakeTableStateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::_blocksPerEpoch(inner) => {
                    <_blocksPerEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::currentBlockNumber(inner) => {
                    <currentBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::disablePermissionedProverMode(inner) => {
                    <disablePermissionedProverModeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::epochFromBlockNumber(inner) => {
                    <epochFromBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::finalizedState(inner) => {
                    <finalizedStateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::genesisStakeTableState(inner) => {
                    <genesisStakeTableStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::genesisState(inner) => {
                    <genesisStateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getHotShotCommitment(inner) => {
                    <getHotShotCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getStateHistoryCount(inner) => {
                    <getStateHistoryCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getVersion(inner) => {
                    <getVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isLastBlockInEpoch(inner) => {
                    <isLastBlockInEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isPermissionedProverEnabled(inner) => {
                    <isPermissionedProverEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::lagOverEscapeHatchThreshold(inner) => {
                    <lagOverEscapeHatchThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::newFinalizedState(inner) => {
                    <newFinalizedStateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::permissionedProver(inner) => {
                    <permissionedProverCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setBlocksPerEpoch(inner) => {
                    <setBlocksPerEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setFinalizedState(inner) => {
                    <setFinalizedStateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setHotShotDownSince(inner) => {
                    <setHotShotDownSinceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::setHotShotUp(inner) => {
                    <setHotShotUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setPermissionedProver(inner) => {
                    <setPermissionedProverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::setStateHistory(inner) => {
                    <setStateHistoryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setstateHistoryRetentionPeriod(inner) => {
                    <setstateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stateHistoryCommitments(inner) => {
                    <stateHistoryCommitmentsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stateHistoryFirstIndex(inner) => {
                    <stateHistoryFirstIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stateHistoryRetentionPeriod(inner) => {
                    <stateHistoryRetentionPeriodCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::votingStakeTableState(inner) => {
                    <votingStakeTableStateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
            }
        }
    }
    ///Container for all the [`LightClientMock`](self) custom errors.
    pub enum LightClientMockErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedInnerCall(FailedInnerCall),
        InsufficientSnapshotHistory(InsufficientSnapshotHistory),
        InvalidAddress(InvalidAddress),
        InvalidArgs(InvalidArgs),
        InvalidHotShotBlockForCommitmentCheck(InvalidHotShotBlockForCommitmentCheck),
        InvalidInitialization(InvalidInitialization),
        InvalidMaxStateHistory(InvalidMaxStateHistory),
        InvalidProof(InvalidProof),
        MissingLastBlockInEpochUpdate(MissingLastBlockInEpochUpdate),
        NoChangeRequired(NoChangeRequired),
        NotInitializing(NotInitializing),
        OutdatedState(OutdatedState),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        ProverNotPermissioned(ProverNotPermissioned),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        WrongStakeTableUsed(WrongStakeTableUsed),
    }
    #[automatically_derived]
    impl LightClientMockErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [5u8, 28u8, 70u8, 239u8],
            [9u8, 189u8, 227u8, 57u8],
            [17u8, 140u8, 218u8, 167u8],
            [20u8, 37u8, 234u8, 66u8],
            [30u8, 79u8, 189u8, 247u8],
            [76u8, 156u8, 140u8, 227u8],
            [81u8, 97u8, 128u8, 137u8],
            [97u8, 90u8, 146u8, 100u8],
            [113u8, 80u8, 222u8, 69u8],
            [153u8, 150u8, 179u8, 21u8],
            [161u8, 186u8, 7u8, 238u8],
            [163u8, 166u8, 71u8, 128u8],
            [168u8, 99u8, 174u8, 201u8],
            [170u8, 29u8, 73u8, 164u8],
            [176u8, 180u8, 56u8, 119u8],
            [179u8, 152u8, 151u8, 159u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 196u8, 36u8, 123u8],
            [244u8, 160u8, 238u8, 224u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for LightClientMockErrors {
        const NAME: &'static str = "LightClientMockErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 21usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedInnerCall(_) => {
                    <FailedInnerCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientSnapshotHistory(_) => {
                    <InsufficientSnapshotHistory as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAddress(_) => <InvalidAddress as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidArgs(_) => <InvalidArgs as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidHotShotBlockForCommitmentCheck(_) => {
                    <InvalidHotShotBlockForCommitmentCheck as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidMaxStateHistory(_) => {
                    <InvalidMaxStateHistory as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProof(_) => <InvalidProof as alloy_sol_types::SolError>::SELECTOR,
                Self::MissingLastBlockInEpochUpdate(_) => {
                    <MissingLastBlockInEpochUpdate as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoChangeRequired(_) => {
                    <NoChangeRequired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutdatedState(_) => <OutdatedState as alloy_sol_types::SolError>::SELECTOR,
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProverNotPermissioned(_) => {
                    <ProverNotPermissioned as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WrongStakeTableUsed(_) => {
                    <WrongStakeTableUsed as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<LightClientMockErrors>] = &[
                {
                    fn OutdatedState(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <OutdatedState as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(LightClientMockErrors::OutdatedState)
                    }
                    OutdatedState
                },
                {
                    fn InvalidProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(LightClientMockErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn FailedInnerCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <FailedInnerCall as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::FailedInnerCall)
                    }
                    FailedInnerCall
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn WrongStakeTableUsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <WrongStakeTableUsed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::WrongStakeTableUsed)
                    }
                    WrongStakeTableUsed
                },
                {
                    fn InvalidHotShotBlockForCommitmentCheck(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InvalidHotShotBlockForCommitmentCheck as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                LightClientMockErrors::InvalidHotShotBlockForCommitmentCheck,
                            )
                    }
                    InvalidHotShotBlockForCommitmentCheck
                },
                {
                    fn MissingLastBlockInEpochUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <MissingLastBlockInEpochUpdate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LightClientMockErrors::MissingLastBlockInEpochUpdate)
                    }
                    MissingLastBlockInEpochUpdate
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn InvalidArgs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InvalidArgs as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(LightClientMockErrors::InvalidArgs)
                    }
                    InvalidArgs
                },
                {
                    fn ProverNotPermissioned(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <ProverNotPermissioned as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::ProverNotPermissioned)
                    }
                    ProverNotPermissioned
                },
                {
                    fn NoChangeRequired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <NoChangeRequired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::NoChangeRequired)
                    }
                    NoChangeRequired
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn InsufficientSnapshotHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InsufficientSnapshotHistory as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::InsufficientSnapshotHistory)
                    }
                    InsufficientSnapshotHistory
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn InvalidAddress(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InvalidAddress as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::InvalidAddress)
                    }
                    InvalidAddress
                },
                {
                    fn InvalidMaxStateHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InvalidMaxStateHistory as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::InvalidMaxStateHistory)
                    }
                    InvalidMaxStateHistory
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LightClientMockErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(LightClientMockErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedInnerCall(inner) => {
                    <FailedInnerCall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientSnapshotHistory(inner) => {
                    <InsufficientSnapshotHistory as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAddress(inner) => {
                    <InvalidAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidArgs(inner) => {
                    <InvalidArgs as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidHotShotBlockForCommitmentCheck(inner) => {
                    <InvalidHotShotBlockForCommitmentCheck as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidMaxStateHistory(inner) => {
                    <InvalidMaxStateHistory as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MissingLastBlockInEpochUpdate(inner) => {
                    <MissingLastBlockInEpochUpdate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoChangeRequired(inner) => {
                    <NoChangeRequired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutdatedState(inner) => {
                    <OutdatedState as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProverNotPermissioned(inner) => {
                    <ProverNotPermissioned as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WrongStakeTableUsed(inner) => {
                    <WrongStakeTableUsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967InvalidImplementation(inner) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC1967NonPayable(inner) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedInnerCall(inner) => {
                    <FailedInnerCall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientSnapshotHistory(inner) => {
                    <InsufficientSnapshotHistory as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAddress(inner) => {
                    <InvalidAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidArgs(inner) => {
                    <InvalidArgs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHotShotBlockForCommitmentCheck(inner) => {
                    <InvalidHotShotBlockForCommitmentCheck as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidMaxStateHistory(inner) => {
                    <InvalidMaxStateHistory as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MissingLastBlockInEpochUpdate(inner) => {
                    <MissingLastBlockInEpochUpdate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoChangeRequired(inner) => {
                    <NoChangeRequired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutdatedState(inner) => {
                    <OutdatedState as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProverNotPermissioned(inner) => {
                    <ProverNotPermissioned as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnauthorizedCallContext(inner) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UUPSUnsupportedProxiableUUID(inner) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WrongStakeTableUsed(inner) => {
                    <WrongStakeTableUsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`LightClientMock`](self) events.
    pub enum LightClientMockEvents {
        Initialized(Initialized),
        NewEpoch(NewEpoch),
        NewState(NewState),
        OwnershipTransferred(OwnershipTransferred),
        PermissionedProverNotRequired(PermissionedProverNotRequired),
        PermissionedProverRequired(PermissionedProverRequired),
        Upgrade(Upgrade),
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl LightClientMockEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                49u8, 234u8, 189u8, 144u8, 153u8, 253u8, 178u8, 93u8, 172u8, 221u8, 210u8, 6u8,
                171u8, 255u8, 135u8, 49u8, 30u8, 85u8, 52u8, 65u8, 252u8, 157u8, 15u8, 205u8,
                239u8, 32u8, 16u8, 98u8, 215u8, 231u8, 7u8, 27u8,
            ],
            [
                128u8, 23u8, 187u8, 136u8, 127u8, 223u8, 143u8, 202u8, 67u8, 20u8, 169u8, 212u8,
                15u8, 110u8, 115u8, 179u8, 184u8, 16u8, 2u8, 214u8, 126u8, 92u8, 250u8, 133u8,
                216u8, 129u8, 115u8, 175u8, 106u8, 164u8, 96u8, 114u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                154u8, 95u8, 87u8, 222u8, 133u8, 109u8, 214u8, 104u8, 197u8, 77u8, 217u8, 94u8,
                92u8, 85u8, 223u8, 147u8, 67u8, 33u8, 113u8, 203u8, 202u8, 73u8, 168u8, 119u8,
                109u8, 86u8, 32u8, 234u8, 89u8, 192u8, 36u8, 80u8,
            ],
            [
                160u8, 74u8, 119u8, 57u8, 36u8, 80u8, 90u8, 65u8, 133u8, 100u8, 54u8, 55u8, 37u8,
                245u8, 104u8, 50u8, 245u8, 119u8, 46u8, 107u8, 141u8, 13u8, 189u8, 110u8, 252u8,
                231u8, 36u8, 223u8, 232u8, 3u8, 218u8, 230u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8, 179u8,
                32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8, 12u8, 192u8, 34u8,
                91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8, 19u8,
                244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8, 33u8, 238u8,
                209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                247u8, 135u8, 33u8, 34u8, 110u8, 254u8, 154u8, 27u8, 182u8, 120u8, 24u8, 154u8,
                22u8, 209u8, 85u8, 73u8, 40u8, 185u8, 242u8, 25u8, 46u8, 44u8, 185u8, 62u8, 237u8,
                168u8, 59u8, 121u8, 250u8, 64u8, 0u8, 125u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for LightClientMockEvents {
        const NAME: &'static str = "LightClientMockEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<NewEpoch as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewEpoch as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::NewEpoch)
                }
                Some(<NewState as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewState as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::NewState)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(
                    <PermissionedProverNotRequired as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <PermissionedProverNotRequired as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::PermissionedProverNotRequired),
                Some(<PermissionedProverRequired as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PermissionedProverRequired as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::PermissionedProverRequired)
                }
                Some(<Upgrade as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgrade as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Upgrade)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Upgraded)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for LightClientMockEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewEpoch(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::NewState(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PermissionedProverNotRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PermissionedProverRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgrade(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::Upgraded(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewEpoch(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewState(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PermissionedProverNotRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PermissionedProverRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgrade(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LightClientMock`](self) contract instance.

    See the [wrapper's documentation](`LightClientMockInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LightClientMockInstance<T, P, N> {
        LightClientMockInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
        genesisStakeTableState: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
        maxHistorySeconds: u32,
        blocksPerEpoch: u64,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<LightClientMockInstance<T, P, N>>>
    {
        LightClientMockInstance::<T, P, N>::deploy(
            provider,
            genesis,
            genesisStakeTableState,
            maxHistorySeconds,
            blocksPerEpoch,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
        genesisStakeTableState: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
        maxHistorySeconds: u32,
        blocksPerEpoch: u64,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        LightClientMockInstance::<T, P, N>::deploy_builder(
            provider,
            genesis,
            genesisStakeTableState,
            maxHistorySeconds,
            blocksPerEpoch,
        )
    }
    /**A [`LightClientMock`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`LightClientMock`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LightClientMockInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LightClientMockInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LightClientMockInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > LightClientMockInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`LightClientMock`](self) contract instance.

        See the [wrapper's documentation](`LightClientMockInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
            genesisStakeTableState: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
            maxHistorySeconds: u32,
            blocksPerEpoch: u64,
        ) -> alloy_contract::Result<LightClientMockInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                genesis,
                genesisStakeTableState,
                maxHistorySeconds,
                blocksPerEpoch,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
            genesisStakeTableState: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
            maxHistorySeconds: u32,
            blocksPerEpoch: u64,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        genesis,
                        genesisStakeTableState,
                        maxHistorySeconds,
                        blocksPerEpoch,
                    })[..],
                ]
                .concat()
                .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> LightClientMockInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LightClientMockInstance<T, P, N> {
            LightClientMockInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > LightClientMockInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall {})
        }
        ///Creates a new call builder for the [`_blocksPerEpoch`] function.
        pub fn _blocksPerEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, _blocksPerEpochCall, N> {
            self.call_builder(&_blocksPerEpochCall {})
        }
        ///Creates a new call builder for the [`currentBlockNumber`] function.
        pub fn currentBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentBlockNumberCall, N> {
            self.call_builder(&currentBlockNumberCall {})
        }
        ///Creates a new call builder for the [`currentEpoch`] function.
        pub fn currentEpoch(&self) -> alloy_contract::SolCallBuilder<T, &P, currentEpochCall, N> {
            self.call_builder(&currentEpochCall {})
        }
        ///Creates a new call builder for the [`disablePermissionedProverMode`] function.
        pub fn disablePermissionedProverMode(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, disablePermissionedProverModeCall, N> {
            self.call_builder(&disablePermissionedProverModeCall {})
        }
        ///Creates a new call builder for the [`epochFromBlockNumber`] function.
        pub fn epochFromBlockNumber(
            &self,
            blockNum: u64,
            blocksPerEpoch: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, epochFromBlockNumberCall, N> {
            self.call_builder(&epochFromBlockNumberCall {
                blockNum,
                blocksPerEpoch,
            })
        }
        ///Creates a new call builder for the [`finalizedState`] function.
        pub fn finalizedState(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, finalizedStateCall, N> {
            self.call_builder(&finalizedStateCall {})
        }
        ///Creates a new call builder for the [`genesisStakeTableState`] function.
        pub fn genesisStakeTableState(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, genesisStakeTableStateCall, N> {
            self.call_builder(&genesisStakeTableStateCall {})
        }
        ///Creates a new call builder for the [`genesisState`] function.
        pub fn genesisState(&self) -> alloy_contract::SolCallBuilder<T, &P, genesisStateCall, N> {
            self.call_builder(&genesisStateCall {})
        }
        ///Creates a new call builder for the [`getHotShotCommitment`] function.
        pub fn getHotShotCommitment(
            &self,
            hotShotBlockHeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHotShotCommitmentCall, N> {
            self.call_builder(&getHotShotCommitmentCall { hotShotBlockHeight })
        }
        ///Creates a new call builder for the [`getStateHistoryCount`] function.
        pub fn getStateHistoryCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStateHistoryCountCall, N> {
            self.call_builder(&getStateHistoryCountCall {})
        }
        ///Creates a new call builder for the [`getVersion`] function.
        pub fn getVersion(&self) -> alloy_contract::SolCallBuilder<T, &P, getVersionCall, N> {
            self.call_builder(&getVersionCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _genesis: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
            _genesisStakeTableState: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
            _stateHistoryRetentionPeriod: u32,
            owner: alloy::sol_types::private::Address,
            blocksPerEpoch: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _genesis,
                _genesisStakeTableState,
                _stateHistoryRetentionPeriod,
                owner,
                blocksPerEpoch,
            })
        }
        ///Creates a new call builder for the [`isLastBlockInEpoch`] function.
        pub fn isLastBlockInEpoch(
            &self,
            blockHeight: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, isLastBlockInEpochCall, N> {
            self.call_builder(&isLastBlockInEpochCall { blockHeight })
        }
        ///Creates a new call builder for the [`isPermissionedProverEnabled`] function.
        pub fn isPermissionedProverEnabled(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, isPermissionedProverEnabledCall, N> {
            self.call_builder(&isPermissionedProverEnabledCall {})
        }
        ///Creates a new call builder for the [`lagOverEscapeHatchThreshold`] function.
        pub fn lagOverEscapeHatchThreshold(
            &self,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, lagOverEscapeHatchThresholdCall, N> {
            self.call_builder(&lagOverEscapeHatchThresholdCall {
                blockNumber,
                threshold,
            })
        }
        ///Creates a new call builder for the [`newFinalizedState`] function.
        pub fn newFinalizedState(
            &self,
            newState: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
            nextStakeTable: <LightClient::StakeTableState as alloy::sol_types::SolType>::RustType,
            proof: <IPlonkVerifier::PlonkProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, newFinalizedStateCall, N> {
            self.call_builder(&newFinalizedStateCall {
                newState,
                nextStakeTable,
                proof,
            })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`permissionedProver`] function.
        pub fn permissionedProver(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionedProverCall, N> {
            self.call_builder(&permissionedProverCall {})
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(&self) -> alloy_contract::SolCallBuilder<T, &P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setBlocksPerEpoch`] function.
        pub fn setBlocksPerEpoch(
            &self,
            newBlocksPerEpoch: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, setBlocksPerEpochCall, N> {
            self.call_builder(&setBlocksPerEpochCall { newBlocksPerEpoch })
        }
        ///Creates a new call builder for the [`setFinalizedState`] function.
        pub fn setFinalizedState(
            &self,
            state: <LightClient::LightClientState as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setFinalizedStateCall, N> {
            self.call_builder(&setFinalizedStateCall { state })
        }
        ///Creates a new call builder for the [`setHotShotDownSince`] function.
        pub fn setHotShotDownSince(
            &self,
            l1Height: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setHotShotDownSinceCall, N> {
            self.call_builder(&setHotShotDownSinceCall { l1Height })
        }
        ///Creates a new call builder for the [`setHotShotUp`] function.
        pub fn setHotShotUp(&self) -> alloy_contract::SolCallBuilder<T, &P, setHotShotUpCall, N> {
            self.call_builder(&setHotShotUpCall {})
        }
        ///Creates a new call builder for the [`setPermissionedProver`] function.
        pub fn setPermissionedProver(
            &self,
            prover: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPermissionedProverCall, N> {
            self.call_builder(&setPermissionedProverCall { prover })
        }
        ///Creates a new call builder for the [`setStateHistory`] function.
        pub fn setStateHistory(
            &self,
            _stateHistoryCommitments: alloy::sol_types::private::Vec<
                <LightClient::StateHistoryCommitment as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStateHistoryCall, N> {
            self.call_builder(&setStateHistoryCall {
                _stateHistoryCommitments,
            })
        }
        ///Creates a new call builder for the [`setstateHistoryRetentionPeriod`] function.
        pub fn setstateHistoryRetentionPeriod(
            &self,
            historySeconds: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setstateHistoryRetentionPeriodCall, N> {
            self.call_builder(&setstateHistoryRetentionPeriodCall { historySeconds })
        }
        ///Creates a new call builder for the [`stateHistoryCommitments`] function.
        pub fn stateHistoryCommitments(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, stateHistoryCommitmentsCall, N> {
            self.call_builder(&stateHistoryCommitmentsCall { _0 })
        }
        ///Creates a new call builder for the [`stateHistoryFirstIndex`] function.
        pub fn stateHistoryFirstIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stateHistoryFirstIndexCall, N> {
            self.call_builder(&stateHistoryFirstIndexCall {})
        }
        ///Creates a new call builder for the [`stateHistoryRetentionPeriod`] function.
        pub fn stateHistoryRetentionPeriod(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stateHistoryRetentionPeriodCall, N> {
            self.call_builder(&stateHistoryRetentionPeriodCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, upgradeToAndCallCall, N> {
            self.call_builder(&upgradeToAndCallCall {
                newImplementation,
                data,
            })
        }
        ///Creates a new call builder for the [`votingStakeTableState`] function.
        pub fn votingStakeTableState(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, votingStakeTableStateCall, N> {
            self.call_builder(&votingStakeTableStateCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > LightClientMockInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NewEpoch`] event.
        pub fn NewEpoch_filter(&self) -> alloy_contract::Event<T, &P, NewEpoch, N> {
            self.event_filter::<NewEpoch>()
        }
        ///Creates a new event filter for the [`NewState`] event.
        pub fn NewState_filter(&self) -> alloy_contract::Event<T, &P, NewState, N> {
            self.event_filter::<NewState>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PermissionedProverNotRequired`] event.
        pub fn PermissionedProverNotRequired_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PermissionedProverNotRequired, N> {
            self.event_filter::<PermissionedProverNotRequired>()
        }
        ///Creates a new event filter for the [`PermissionedProverRequired`] event.
        pub fn PermissionedProverRequired_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PermissionedProverRequired, N> {
            self.event_filter::<PermissionedProverRequired>()
        }
        ///Creates a new event filter for the [`Upgrade`] event.
        pub fn Upgrade_filter(&self) -> alloy_contract::Event<T, &P, Upgrade, N> {
            self.event_filter::<Upgrade>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<T, &P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
