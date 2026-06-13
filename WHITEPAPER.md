<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Prospera Network - 100% Fair Launch Quantum-Resistant L1</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0f172a; color: #f8fafc; margin: 0; padding: 20px; line-height: 1.6; }
        .container { max-width: 850px; margin: auto; background: #1e293b; padding: 35px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.3); }
        h1 { color: #38bdf8; border-bottom: 2px solid #334155; padding-bottom: 10px; margin-top: 0; }
        h2 { color: #38bdf8; margin-top: 30px; border-bottom: 1px solid #334155; padding-bottom: 5px; }
        h3 { color: #f43f5e; margin-top: 25px; }
        .badge { background: #0369a1; color: #e0f2fe; padding: 5px 12px; border-radius: 20px; font-size: 0.85em; font-weight: bold; display: inline-block; margin-bottom: 20px; margin-right: 5px; }
        .code-box { background: #020617; border-left: 4px solid #38bdf8; padding: 15px; font-family: 'Courier New', Courier, monospace; border-radius: 6px; overflow-x: auto; font-size: 0.9em; color: #34d399; margin: 15px 0; }
        .nav-tabs { display: flex; gap: 10px; margin-bottom: 25px; border-bottom: 2px solid #334155; padding-bottom: 10px; }
        .tab-btn { background: #334155; color: #94a3b8; border: none; padding: 10px 20px; border-radius: 6px; cursor: pointer; font-weight: bold; font-size: 0.95em; }
        .tab-btn.active { background: #38bdf8; color: #0f172a; }
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        .highlight-box { background: rgba(244, 63, 94, 0.1); border: 1px solid #f43f5e; padding: 20px; border-radius: 8px; margin: 20px 0; }
        footer { margin-top: 40px; text-align: center; color: #64748b; font-size: 0.85em; border-top: 1px solid #334155; padding-top: 20px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Prospera Network</h1>
        <div style="margin-bottom: 15px;">
            <span class="badge">Layer 1 Network</span>
            <span class="badge" style="background: #15803d;">Quantum-Resistant</span>
            <span class="badge" style="background: #b45309;">100% Fair Launch</span>
        </div>

        <!-- Navigation Tabs -->
        <div class="nav-tabs">
            <button class="tab-btn active" onclick="openTab('home')">Overview</button>
            <button class="tab-btn" onclick="openTab('whitepaper')">Whitepaper (EN)</button>
        </div>

        <!-- Tab 1: Home Overview -->
        <div id="home" class="tab-content active">
            <p>Prospera is a next-generation Layer 1 (L1) blockchain network engineered in <strong>Rust</strong>. It is designed from the ground up for maximum computational performance and cryptographic security against upcoming quantum computing threats.</p>
            
            <h2>Key Network Features</h2>
            <ul>
                <li><strong>Post-Quantum Security:</strong> Securing digital assets using state-of-the-art lattice-based cryptographic algorithms.</li>
                <li><strong>Rust Core Engine:</strong> Highly optimized memory management without the overhead of garbage collection.</li>
                <li><strong>Absolute Transparency:</strong> Open-source framework verifiable by anyone globally on GitHub.</li>
            </ul>

            <h2>How to Run a Node (via Termux / Linux)</h2>
            <p>Compile and deploy your node directly onto the Prospera Network using these commands:</p>
            <div class="code-box">
                git clone https://github.com<br>
                cd prospera-blockchain<br>
                cargo run
            </div>
        </div>

        <!-- Tab 2: Whitepaper -->
        <div id="whitepaper" class="tab-content">
            <h2>Whitepaper: Quantum-Resistant & Pure Fair Launch Ecosystem</h2>
            <p><strong>Version:</strong> 1.0 (June 2026) | <strong>Core Language:</strong> Rust</p>
            
            <h3>1. Introduction</h3>
            <p>The dawn of quantum computing poses an existential threat to traditional ECDSA-based blockchains (such as Bitcoin and Ethereum). Prospera Network mitigates this risk by integrating NIST-approved post-quantum cryptographic primitives right into the Genesis block.</p>

            <h3>2. Technical Framework</h3>
            <p>Built exclusively in Rust, Prospera ensures high-throughput peer-to-peer data replication. The network adopts hash-based and lattice-based signature schemes, rendering public keys immune to Shor's algorithm-driven private key extraction.</p>

            <div class="highlight-box">
                <h3>3. 100% Fair Launch Commitment (Zero Scam / Pure Decentralization)</h3>
                <p>To establish absolute trust within the Web3 community, Prospera operates on a <strong>Strict Fair Launch</strong> economic model:</p>
                <ul>
                    <li><strong>0% Developer Allocation:</strong> The core team or creators do not hold, reserve, or pre-allocate any supply for themselves.</li>
                    <li><strong>100% Public Distribution:</strong> Every single token ($PRSP) is minted through active network validation, node operations, or mining.</li>
                    <li><strong>No Pre-Mine / No ICO / No VC:</strong> There are no early-stage token sales or hidden venture capital allocations. If developers want to acquire $PRSP, they must run nodes and mine alongside the community on equal terms.</li>
                </ul>
            </div>

            <h3>4. Network Consensus</h3>
            <p>Prospera features an optimized lightweight consensus protocol designed to run smoothly on low-power devices, including Android environments via Termux. Token mechanics are hardcoded directly into the open-source GitHub tree (`src/main.rs`), eliminating arbitrary supply inflation.</p>
        </div>

        <footer>
            &copy; 2026 Prospera Network. Built for the community, by the community.
        </footer>
    </div>

    <script>
        function openTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));
            document.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
            document.getElementById(tabId).add('active');
            event.currentTarget.classList.add('active');
        }
    </script>
</body>
</html>
0

