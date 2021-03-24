<p align="center">
<img src="./logo.png" alt="A box with the stellar logo going into a hole, a visual pun on drop-in">
</p>

> ⚠️ This Project is a Work-in-Progress!

# Drop-in Federated Byzantine Agreement
This is a general-purpose library that ~~implements~~ *will implement*⁰ Stellar-Consensus-Protocol-style Federated Byzantine Agreement.

> **0**: It's WIP. There's even a banner at the top of this README!

this project is loosely based on existing implementations¹ as well as the original technical papers. Not much literature exists on the subject, but I think that SCP - and FBA in general - is a great idea and could benefit from an easy-to-use library so more people can mess around with it and build stuff on it.

> **1:** All right, all right - it's *actually* just a simplified port of [bobg/scp (Golang)](https://github.com/bobg/scp) to Rust. Huge thanks to [Bob Glickstein](https://github.com/bobg) for the nice clean implementation!

## Why should you care?
Imagine you have a huge bundle of computers, some working, others not, and you want to make sure they all reach the same shared state - but here's the catch - the computers can only communicate by sending messages between each other.

> This 'state' could be anything, really - could be values in a database cluster, transactions in a blockchain, filesystems, etc.

Synchronizing state in this message-passing manner is a bit tricky. So tricky, in fact, that it even has a name: *The Byzantine Generals Problem*. Lower-and-upper bounds have been proved for many arrangements of this problem, and much effort has gone into proving solutions for the problem, yet it still remains an active area of research.

Why? The reason so many have researched solutions for it is a simple one: we have a *massive* message-passing-centric communications network (the internet), and we like to hook computers up to it. Solutions to this problem - systems that are said to be *Byzantine Fault Tolerant* - have practical and useful implications.

Right now, there are generally two ways we solve this problem:

1. If we own most of the computers and know about all of them before hand (like a company running a database cluster, perhaps), we can use a **PAXOS**-like system. This system is fast, but, this system assumes *centralization*: someone has to make a list of all the computers and ensure they all work together. This is the traditional centralized **Byzantine Agreement (BA)** solution.

2. If we do *not* own most of the computers or don't know about all of them before-hand, we can use a *decentralized* solution, like Bitcoin's **Proof-of-Work (PoW)** or Ethereum's **Proof-of-Stake (PoS)** (supposedly, we'll see how 2.0 goes). However, these systems, in assuming zero trust, tend to be a waste of energy (PoW), slow (PoW & PoS), or expensive (PoW).

These aren't the only solutions, however.

**Federated Byzantine Agreement (FBA)** generalizes Byzantine Agreement to distributed systems. We can have our cake and eat it too: FBA is as quick as a PAXOS-like system, but allows for open-membership and is completely decentralized. FBA is efficient because it doesn't assume zero trust - rather, computers in the network choose a *subset* of all computers in the network to trust (a *Quorum Slice*), and by taking the transitive closure over this *Quorum Slice* with respect to a specific computer, we can generate a trusted subset of the network (a *Quorum*) through which we can undergo fairly standard PAXOS-style voting to arrive at a shared state.

The **Stellar Consensus Protocol (SCP)²** is a specific construction that fulfills FBA. If I were to write an analogy, SCP is to FBA as PAXOS is to BA. For proof that SCP works in practice, look no further than [Stellar](https://stellar.org), a distributed transactions system.

> **2:** No, no, not *that* SCP.

## So the purpose of this project is... ?
Although it's fairly easy to find drop-in implementations and explanations of Proof-of-Work, PAXOS, RAFT, etc., outside of [Stellar-Core](https://github.com/stellar/stellar-core), not many production-ready FBA/SCP libraries exist.

Hence the name of this project: `drop-in-fba` aims to be, well, **a drop-in Federated Byzantine Agreement library based on the Stellar Consensus protocol³.**

That's all folks. Stay tuned!

— Isaac Clayton

> **3:** I have a few goals:
>
> 1. This library should be *rocket-fast*. period. This means I'll be keeping it fairly low-level on the back-end, while still exposing a nice high-level API. I'm going to try my best to minimize unnecessary allocations and duplications.
> 2. This library should be composable, and asynchronous-friendly. FBA usually takes place in a network-based environment, so this is a no-brainer. Despite being async-friendly, `drop-in-fba` will not pull in any async dependencies.
> 3. This library should be composable. You should be able to use `drop-in-fba` to, well, drop in FBA. It should be general enough, at least, to be the basis for a Stellar-compatible SCP implementation written in Rust, while still also being flexible for other applications.
