use std::time::{SystemTime, UNIX_EPOCH};

// 1. Struktur untuk mencatat satu titik data grafik (Candlestick Data)
#[derive(Debug)]
struct ChartPoint {
    waktu: u64,
    harga: f64,
}

struct LiquidityPool {
    prsp_reserve: u64,
    usdt_reserve: f64,
    coin_price: f64,
    riwayat_grafik: Vec<ChartPoint>, // Tempat menyimpan daftar harga dari waktu ke waktu
}

impl LiquidityPool {
    fn beli_coin(&mut self, jumlah_usdt: f64) -> u64 {
        let koin_didapat = (jumlah_usdt / self.coin_price) as u64;
        self.prsp_reserve -= koin_didapat;
        self.usdt_reserve += jumlah_usdt;
        
        // Harga naik setelah ada yang beli
        self.coin_price *= 1.05; 

        // CATAT KE GRAFIK: Simpan harga baru beserta detiknya saat transaksi terjadi
        let detik_ini = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.riwayat_grafik.push(ChartPoint {
            waktu: detik_ini,
            harga: self.coin_price,
        });

        koin_didapat
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/beli", post(proses_beli_dari_web));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let mut pool = LiquidityPool {
        prsp_reserve: 12_000_000,
        usdt_reserve: 0.0,
        coin_price: 0.01, // Harga awal $0.01
        riwayat_grafik: Vec::new(),
    };

    println!("====================================================");
    println!("     PROSPERA ($PRSP) REAL-TIME CHART ENGINE        ");
    println!("====================================================");

    // Simulasi Transaksi Beruntun agar grafik bergerak naik naik ke langit
    println!("[Pembeli 1] Membeli koin sebesar $100 USDT...");
    pool.beli_coin(100.0);

    println!("[Pembeli 2] Membeli koin sebesar $250 USDT...");
    pool.beli_coin(250.0);

    println!("[Pembeli 3] Membeli koin sebesar $500 USDT...");
    pool.beli_coin(500.0);

    println!("\n[Sukses] Seluruh Riwayat Harga Berhasil Dicatat!");
    println!("----------------------------------------------------");
    println!("Daftar Data yang Dikirim ke Grafik Tampilan:");
    
    // Menampilkan isi database grafik yang siap digambar jadi chart TradingView
    for (index, point) in pool.riwayat_grafik.iter().enumerate() {
        println!("Titik Ke-{} -> Waktu: {} | Harga Koin: ${:.4} USDT", index + 1, point.waktu, point.harga);
    }
    
    println!("----------------------------------------------------");
    println!("Status Kolam Akhir: Sisa {} PRSP | Total ${} USDT", pool.prsp_reserve, pool.usdt_reserve);
    println!("====================================================");
}




use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct OrderBeli {
    jumlah_usdt: f64,
    alamat_wallet_pembeli: String,
}

async fn proses_beli_dari_web(Json(payload): Json<OrderBeli>) -> String {
    format!(
        "Sukses! {} USDT berhasil dikonversi ke $PRSP dan dikirim ke wallet: {}",
        payload.jumlah_usdt, payload.alamat_wallet_pembeli
    )
}
