# FarmGate Pay

Instant escrow payments for farmers and produce buyers using Stellar.

---

## Problem

Vegetable farmers in Benguet wait several days or weeks before receiving payments from wholesale buyers, causing delays in buying seeds, fertilizer, and fuel for the next harvest.

---

## Solution

Wholesale buyers lock USDC into a Soroban escrow contract before pickup, and payment automatically releases once produce delivery is confirmed.

---

## Timeline

### Week 1
- Smart contract development
- Local testing

### Week 2
- Frontend integration
- Wallet connection

### Week 3
- Testnet deployment
- Demo preparation

---

## Stellar Features Used

- USDC transfers
- Soroban smart contracts
- Trustlines
- Escrow logic

---

## Vision and Purpose

FarmGate Pay helps farmers receive faster and more transparent payments while reducing disputes between buyers and suppliers.

---

## Prerequisites

- Rust
- Cargo
- Soroban CLI v22+

---

## Build Contract

```bash
soroban contract build
```

---

## Run Tests

```bash
cargo test
```

---

## Deploy to Stellar Testnet

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/farmgate_pay.wasm \
--source alice \
--network testnet
```

---

## Sample Contract Invocation

### Create Escrow

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- create_escrow \
--buyer GBUYER123 \
--farmer GFARMER456 \
--amount 5000
```

### Confirm Delivery

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- confirm_delivery \
--escrow_id 1 \
--buyer GBUYER123
```

### Release Payment

```bash
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- release_payment \
--escrow_id 1
```

---

## License

MIT