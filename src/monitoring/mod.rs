#[cfg(feature = "monitoring_prom")]
mod prometheus;

pub fn increment_rpc_calls_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::RPC_CALL_COUNTER.inc();
}

pub fn increment_p2p_msg_unauthenticated_handshake_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_UNAUTHENTICATED_HANDSHAKE_RECEIVED_COUNTER.inc();
}

pub fn increment_p2p_msg_authenticated_handshake_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_AUTHENTICATED_HANDSHAKE_RECEIVED_COUNTER.inc();
}

pub fn increment_p2p_msg_get_neighbors_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_GET_NEIGHBORS_RECEIVED_COUNTER.inc();
}

pub fn increment_p2p_msg_get_blocks_inv_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_GET_BLOCKS_INV_RECEIVED_COUNTER.inc();
}

pub fn increment_p2p_msg_nack_sent_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_NACK_SENT_COUNTER.inc();
}

pub fn increment_p2p_msg_ping_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_PING_RECEIVED_COUNTER.inc();
}

pub fn increment_p2p_msg_nat_punch_request_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::P2P_MSG_NAT_PUNCH_REQUEST_RECEIVED_COUNTER.inc();
}

pub fn increment_stx_blocks_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_BLOCKS_RECEIVED_COUNTER.inc();
}

pub fn increment_stx_micro_blocks_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_MICRO_BLOCKS_RECEIVED_COUNTER.inc();
}

pub fn increment_stx_blocks_served_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_BLOCKS_SERVED_COUNTER.inc();
}

pub fn increment_stx_micro_blocks_served_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_MICRO_BLOCKS_SERVED_COUNTER.inc();
}

pub fn increment_stx_confirmed_micro_blocks_served_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_CONFIRMED_MICRO_BLOCKS_SERVED_COUNTER.inc();
}

pub fn increment_txs_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::TXS_RECEIVED_COUNTER.inc();
}

pub fn increment_btc_blocks_received_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::BTC_BLOCKS_RECEIVED_COUNTER.inc();
}

pub fn increment_btc_ops_sent_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::BTC_OPS_SENT_COUNTER.inc();
}

pub fn increment_stx_blocks_processed_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_BLOCKS_PROCESSED_COUNTER.inc();
}

pub fn increment_stx_blocks_mined_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::STX_BLOCKS_MINED_COUNTER.inc();
}

pub fn increment_warning_emitted_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::WARNING_EMITTED_COUNTER.inc();
}

pub fn increment_errors_emitted_counter() {
    #[cfg(feature = "monitoring_prom")]
    prometheus::ERRORS_EMITTED_COUNTER.inc();
}

#[allow(unused_variables)]
pub fn update_active_miners_count_gauge(value: i64) {
    #[cfg(feature = "monitoring_prom")]
    prometheus::ACTIVE_MINERS_COUNT_GAUGE.set(value);
}
