use super::ExecutionEnvironment;
use async_compatibility_layer::art::async_spawn;
use futures::TryStreamExt;
use netlink_packet_route::DecodeError;
use nix::{
    errno::Errno,
    sched::{setns, CloneFlags},
};
use rtnetlink::{
    new_connection_with_socket, sys::SmolSocket, AddressHandle, Handle, NetemQdisc,
    NetworkNamespace, RouteHandle, TcNetemCorrelations, TcNetemCorrupt, TcNetemDelay, TcNetemQopt,
    NETNS_PATH,
};
use snafu::{ResultExt, Snafu};
use std::{
    fs::File,
    net::{AddrParseError, Ipv4Addr},
    os::unix::{io::IntoRawFd, prelude::AsRawFd},
    path::Path,
    process::Command,
};
use tracing::{error, info};

/// hardcoded default values
pub const LOSSY_QDISC: NetemQdisc = NetemQdisc {
    config: TcNetemQopt {
        limit: 10240,
        loss: 2,
        gap: 0,
        duplicate: 0,
    },
    delay: Some(TcNetemDelay {
        delay: 500000,
        stddev: 0,
    }),
    correlations: Some(TcNetemCorrelations {
        delay_corr: 0,
        loss_corr: 0,
        dup_corr: 0,
    }),
    corruption: Some(TcNetemCorrupt { prob: 2, corr: 0 }),
    reorder: None,
};

async fn del_link(handle: Handle, name: String) -> Result<(), LossyNetworkError> {
    let mut links = handle.link().get().match_name(name.clone()).execute();
    if let Some(link) = links.try_next().await.context(RtNetlinkSnafu)? {
        Ok(handle
            .link()
            .del(link.header.index)
            .execute()
            .await
            .context(RtNetlinkSnafu)?)
    } else {
        error!("link {} not found", name);
        Ok(())
    }
}

/// represent the current network namespace
/// (useful if returning)
struct Netns {
    cur: File,
}

impl Netns {
    /// creates new network namespace
    /// and enters namespace
    async fn new(path: &str) -> Result<Self, LossyNetworkError> {
        // create new ns
        NetworkNamespace::add(path.to_string())
            .await
            .context(RtNetlinkSnafu)?;

        // entry new ns
        let ns_path = Path::new(NETNS_PATH);
        let file = File::open(ns_path.join(path)).context(IoSnafu)?;

        Ok(Self { cur: file })
    }
}

/// A description of a lossy network
#[derive(Clone, derive_builder::Builder, custom_debug::Debug)]
pub struct LossyNetwork {
    /// Ethernet interface that is connected to WAN
    eth_name: String,
    /// metadata describing how to isolate. Only used when `env_type` is `Metal`
    isolation_config: Option<IsolationConfig>,
    /// The network loss conditions
    netem_config: NetemQdisc,
    /// the execution environment
    env_type: ExecutionEnvironment,
}

impl LossyNetwork {
    /// Create isolated environment in separate network namespace via network bridge
    pub async fn isolate(&self) -> Result<(), LossyNetworkError> {
        if let Some(ref isolation_config) = self.isolation_config {
            isolation_config.isolate_netlink(&self.eth_name).await?
        }
        Ok(())
    }

    /// Delete isolated environment and network bridge
    pub async fn undo_isolate(&self) -> Result<(), LossyNetworkError> {
        if let Some(ref isolation_config) = self.isolation_config {
            isolation_config
                .undo_isolate_netlink(self.eth_name.clone())
                .await?
        }
        Ok(())
    }

    /// Create a network qdisc
    pub async fn create_qdisc(&self) -> Result<(), LossyNetworkError> {
        match self.env_type {
            ExecutionEnvironment::Docker => {
                self.create_qdisc_netlink(&self.eth_name).await?;
            }
            ExecutionEnvironment::Metal => match self.isolation_config {
                Some(ref isolation_config) => {
                    self.create_qdisc_netlink(&isolation_config.veth2_name.clone())
                        .await?;
                }
                None => return Err(LossyNetworkError::InvalidConfig),
            },
        }
        Ok(())
    }

    /// Internal invocation to netlink library
    /// to create the qdisc
    async fn create_qdisc_netlink(&self, veth: &str) -> Result<(), LossyNetworkError> {
        let (connection, handle, _) =
            new_connection_with_socket::<SmolSocket>().context(IoSnafu)?;
        async_spawn(connection);
        let mut links = handle.link().get().match_name(veth.to_string()).execute();
        if let Some(link) = links.try_next().await.context(RtNetlinkSnafu)? {
            handle
                .qdisc()
                .add(link.header.index as i32)
                .netem(self.netem_config.clone())
                .context(DecodeSnafu)?
                .execute()
                .await
                .context(RtNetlinkSnafu)?
        } else {
            return Err(LossyNetworkError::InvalidConfig);
        }
        Ok(())
    }
}

/// Hardcoded default values for current AWS setup
impl Default for IsolationConfig {
    fn default() -> Self {
        Self {
            counter_ns: "COUNTER_NS".to_string(),
            bridge_addr: "172.13.0.1".to_string(),
            bridge_name: "br0".to_string(),
            veth_name: "veth1".to_string(),
            veth2_name: "veth2".to_string(),
            veth2_addr: "172.13.0.2".to_string(),
        }
    }
}

/// A description of how the network should be isolated
#[derive(Clone, derive_builder::Builder, custom_debug::Debug)]
#[builder(default)]
pub struct IsolationConfig {
    /// the network namespace name to create
    counter_ns: String,
    /// the bridge ip address
    bridge_addr: String,
    /// the bridge name
    bridge_name: String,
    /// the virtual ethernet interface name
    /// that lives in the default/root network namespace
    veth_name: String,
    /// the virtual ethernet interface name
    /// that lives in `counter_ns`
    veth2_name: String,
    /// the virtual ethernet interface ip address
    /// that lives in `counter_ns`
    veth2_addr: String,
}

impl IsolationConfig {
    /// Prepares server for latency by:
    /// - creating a separate network namespace denoted `counter_ns`
    /// - creating a virtual ethernet device (veth2) in this namespace
    /// - bridging the virtual ethernet device within COUNTER_NS to the default/root network namespace
    /// - adding firewall rules to allow traffic to flow between the network bridge and outside world
    /// - execute the demo inside network namespace
    async fn isolate_netlink(&self, eth_name: &str) -> Result<(), LossyNetworkError> {
        let (connection, handle, _) =
            new_connection_with_socket::<SmolSocket>().context(IoSnafu)?;
        async_spawn(connection);

        // create new netns
        let counter_ns_name = self.counter_ns.clone();
        let counter_ns = Netns::new(&counter_ns_name).await?;

        // create veth interfaces
        let veth = self.veth_name.clone();
        let veth_2 = self.veth2_name.clone();

        handle
            .link()
            .add()
            .veth(veth.clone(), veth_2.clone())
            .execute()
            .await
            .context(RtNetlinkSnafu)?;
        let veth_idx = handle
            .link()
            .get()
            .match_name(veth.clone())
            .execute()
            .try_next()
            .await
            .context(RtNetlinkSnafu)?
            .ok_or(LossyNetworkError::InvalidConfig)?
            .header
            .index;
        let veth_2_idx = handle
            .link()
            .get()
            .match_name(veth_2.clone())
            .execute()
            .try_next()
            .await
            .context(RtNetlinkSnafu)?
            .ok_or(LossyNetworkError::InvalidConfig)?
            .header
            .index;

        // set interfaces up
        handle
            .link()
            .set(veth_idx)
            .up()
            .execute()
            .await
            .context(RtNetlinkSnafu)?;
        handle
            .link()
            .set(veth_2_idx)
            .up()
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // move veth_2 to counter_ns
        handle
            .link()
            .set(veth_2_idx)
            .setns_by_fd(counter_ns.cur.as_raw_fd())
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        let bridge_name = self.bridge_name.clone();

        handle
            .link()
            .add()
            .bridge(bridge_name.clone())
            .execute()
            .await
            .context(RtNetlinkSnafu)?;
        let bridge_idx = handle
            .link()
            .get()
            .match_name(bridge_name.clone())
            .execute()
            .try_next()
            .await
            .context(RtNetlinkSnafu)?
            .ok_or(LossyNetworkError::InvalidConfig)?
            .header
            .index;

        // set bridge up
        handle
            .link()
            .set(bridge_idx)
            .up()
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // set veth master to bridge
        handle
            .link()
            .set(veth_idx)
            .master(bridge_idx)
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // add ip address to bridge
        let bridge_addr = self
            .bridge_addr
            .parse::<Ipv4Addr>()
            .context(AddrParseSnafu)?;
        let bridge_range = 16;
        AddressHandle::new(handle)
            .add(bridge_idx, std::net::IpAddr::V4(bridge_addr), bridge_range)
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // switch to counter_ns
        setns(counter_ns.cur.as_raw_fd(), CloneFlags::CLONE_NEWNET).context(SetNsSnafu)?;

        // get connection metadata in new net namespace
        let (connection, handle, _) =
            new_connection_with_socket::<SmolSocket>().context(IoSnafu)?;
        async_spawn(connection);

        // set lo interface to up
        let lo_idx = handle
            .link()
            .get()
            .match_name("lo".to_string())
            .execute()
            .try_next()
            .await
            .context(RtNetlinkSnafu)?
            .ok_or(LossyNetworkError::InvalidConfig)?
            .header
            .index;
        handle
            .link()
            .set(lo_idx)
            .up()
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // set veth2 to up
        let veth_2_idx = handle
            .link()
            .get()
            .match_name(veth_2)
            .execute()
            .try_next()
            .await
            .context(RtNetlinkSnafu)?
            .ok_or(LossyNetworkError::InvalidConfig)?
            .header
            .index;
        handle
            .link()
            .set(veth_2_idx)
            .up()
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // set veth2 address
        let veth_2_addr = self
            .veth2_addr
            .parse::<std::net::IpAddr>()
            .context(AddrParseSnafu)?;
        let veth_2_range = 16;
        AddressHandle::new(handle.clone())
            .add(veth_2_idx, veth_2_addr, veth_2_range)
            .execute()
            .await
            .context(RtNetlinkSnafu)?;

        // add route
        let route = RouteHandle::new(handle).add();
        route
            .v4()
            .gateway(bridge_addr)
            .execute()
            .await
            .context(RtNetlinkSnafu)?;
        self.enable_firewall(eth_name).await?;

        Ok(())
    }

    /// Enables firewall rules to allow network bridge to function properly (e.g. no packets
    /// dropped). Assumes firewall is via iptables
    async fn enable_firewall(&self, eth_name: &str) -> Result<(), LossyNetworkError> {
        // accept traffic on bridge
        // iptables -A FORWARD -o ens5 -i br0 -j ACCEPT
        info!(
            "{:?}",
            Command::new("iptables")
                .args(["-A", "FORWARD", "-o", eth_name, "-i", "br0", "-j", "ACCEPT"])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -A FORWARD -i ens5 -o br0 -j ACCEPT
        info!(
            "{:?}",
            Command::new("iptables")
                .args(["-A", "FORWARD", "-i", eth_name, "-o", "br0", "-j", "ACCEPT"])
                .output()
                .context(IoSnafu)?
                .status
        );
        // NAT
        // iptables -t nat -A POSTROUTING -s 172.20.0.1 -o ens5 -j MASQUERADE
        info!(
            "{:?}",
            Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-A",
                    "POSTROUTING",
                    "-s",
                    &self.bridge_addr,
                    "-o",
                    "ens5",
                    "-j",
                    "MASQUERADE"
                ])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -t nat -A POSTROUTING -o br0 -j MASQUERADE
        info!(
            "{:?}",
            Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-A",
                    "POSTROUTING",
                    "-o",
                    &self.bridge_name,
                    "-j",
                    "MASQUERADE"
                ])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -t nat -A POSTROUTING -s 172.20.0.0/16 -j MASQUERADE
        info!(
            "{:?}",
            Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-A",
                    "POSTROUTING",
                    "-s",
                    &format!("{}/16", &self.bridge_addr),
                    "-j",
                    "MASQUERADE"
                ])
                .output()
                .context(IoSnafu)?
                .status
        );

        Ok(())
    }

    /// tears down all created interfaces
    /// deletes all iptables rules
    /// deletes namespace
    async fn undo_isolate_netlink(&self, eth_name: String) -> Result<(), LossyNetworkError> {
        let root_ns_fd = File::open("/proc/1/ns/net").context(IoSnafu)?.into_raw_fd();
        setns(root_ns_fd, CloneFlags::CLONE_NEWNET).context(SetNsSnafu)?;
        NetworkNamespace::del(self.counter_ns.to_string())
            .await
            .context(RtNetlinkSnafu)?;
        let (connection, handle, _) =
            new_connection_with_socket::<SmolSocket>().context(IoSnafu)?;
        async_spawn(connection);
        del_link(handle, self.bridge_name.clone()).await?;
        // delete creates iptables rules
        self.undo_firewall(eth_name).await?;
        Ok(())
    }

    /// deletes created iptables rules
    async fn undo_firewall(&self, eth_name: String) -> Result<(), LossyNetworkError> {
        // iptables -D FORWARD -o ens5 -i br0 -j ACCEPT
        info!(
            "{:?}",
            Command::new("iptables")
                .args(["-D", "FORWARD", "-o", &eth_name, "-i", "br0", "-j", "ACCEPT"])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -D FORWARD -i ens5 -o br0 -j ACCEPT
        info!(
            "{:?}",
            Command::new("iptables")
                .args(["-D", "FORWARD", "-i", &eth_name, "-o", "br0", "-j", "ACCEPT"])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -t nat -D POSTROUTING -s $bridge_addr -o ens5 -j MASQUERADE
        info!(
            "{:?}",
            Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-D",
                    "POSTROUTING",
                    "-s",
                    &self.bridge_addr,
                    "-o",
                    "ens5",
                    "-j",
                    "MASQUERADE"
                ])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -t nat -D POSTROUTING -o br0 -j MASQUERADE
        info!(
            "{:?}",
            Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-D",
                    "POSTROUTING",
                    "-o",
                    &self.bridge_name,
                    "-j",
                    "MASQUERADE"
                ])
                .output()
                .context(IoSnafu)?
                .status
        );
        // iptables -t nat -D POSTROUTING -s $bridge_addr/16 -j MASQUERADE
        info!(
            "{:?}",
            Command::new("iptables")
                .args([
                    "-t",
                    "nat",
                    "-D",
                    "POSTROUTING",
                    "-s",
                    &format!("{}/16", self.bridge_addr),
                    "-j",
                    "MASQUERADE"
                ])
                .output()
                .context(IoSnafu)?
                .status
        );
        Ok(())
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum LossyNetworkError {
    RtNetlink { source: rtnetlink::Error },
    Io { source: std::io::Error },
    SetNs { source: Errno },
    InvalidConfig,
    Decode { source: DecodeError },
    AddrParse { source: AddrParseError },
}
