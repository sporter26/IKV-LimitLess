# LimitLess - Lisanssız IKV Otomasyonu

İstanbul Kıyamet Vakti (IKV) oyunu için geliştirilmiş, lisanssız ve açık kaynaklı bir çoklu istemci (multi-client) otomasyon ve bot kontrol aracı.

## Özellikler

- **Çoklu Hesap Yönetimi:** Birden fazla hesabı aynı anda yönetin
- **Bot Kontrol:** Farm ve Boss modlarını hesap bazında kontrol edin
- **Toplu İşlem:** Tüm hesapları aynı anda başlatın
- **Tauri Tabanlı:** Masaüstü uygulaması olarak çalışır

## Teknoloji Yığını

- **Frontend:** React 18 + Vite + TailwindCSS
- **Backend:** Rust + Tauri 2.0
- **UI Bileşenleri:** Radix UI + Lucide Icons
- **Bildirim:** Sonner Toast

## Kurulum

### Ön Gereksinimler

- Node.js 18+ 
- Rust 1.60+
- npm veya pnpm

### Adımlar

1. **Bağımlılıkları yükleyin:**
```bash
npm install
# veya
pnpm install
```

2. **Geliştirme modunda çalıştırın:**
```bash
npm run dev
# veya
pnpm dev
```

3. **Üretim için derleyin:**
```bash
npm run build
# veya
pnpm build
```

## Proje Yapısı

```
ikvlimitless-tauri/
├── src/                          # React Frontend
│   ├── components/              # React bileşenleri
│   │   ├── AccountForm.jsx
│   │   ├── AccountList.jsx
│   │   ├── BotControl.jsx
│   │   └── SettingsPanel.jsx
│   ├── pages/
│   │   └── Dashboard.jsx
│   ├── App.jsx
│   ├── main.jsx
│   └── index.css
├── src-tauri/                    # Rust Backend
│   ├── src/
│   │   ├── main.rs             # Tauri ana dosyası
│   │   ├── commands.rs         # Tauri invoke komutları
│   │   ├── db.rs               # Veritabanı modülü
│   │   └── models.rs           # Veri modelleri
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── index.html
├── package.json
├── vite.config.js
├── tailwind.config.js
└── README.md
```

## Tauri Komutları

Uygulama aşağıdaki Tauri invoke komutlarını sağlar:

- `get_accounts` - Tüm hesapları listele
- `add_account` - Yeni hesap ekle
- `remove_account` - Hesabı sil
- `launch_game` - Oyunu başlat
- `launch_all_accounts` - Tüm oyunları başlat
- `toggle_farm_mode` - Farm modunu aç/kapat
- `toggle_boss_mode` - Boss modunu aç/kapat

## Geliştirme

### Frontend Geliştirme

Frontend, Vite ile sıcak yenileme (hot reload) desteğiyle geliştirilir:

```bash
npm run dev
```

### Backend Geliştirme

Rust backend kodunu `src-tauri/src/` dizininde düzenleyin. Değişiklikler otomatik olarak derlenir.

### Yeni Komut Ekleme

1. `src-tauri/src/commands.rs` dosyasında yeni komut fonksiyonu yazın
2. `src-tauri/src/main.rs` dosyasında `invoke_handler` listesine ekleyin
3. Frontend'de `invoke()` kullanarak çağırın

Örnek:
```rust
#[tauri::command]
pub async fn my_command(db: State<'_, Database>) -> Result<ApiResponse<String>, String> {
    // İşlem yap
    Ok(ApiResponse {
        success: true,
        data: Some("Başarılı".to_string()),
        error: None,
    })
}
```

## Lisans

Bu proje açık kaynak ve lisanssızdır. Özgürce kullanabilir, değiştirebilir ve dağıtabilirsiniz.

## Katkıda Bulunma

Katkılarınız hoş karşılanır! Lütfen:

1. Projeyi fork edin
2. Özellik dalı oluşturun (`git checkout -b feature/YeniÖzellik`)
3. Değişiklikleri commit edin (`git commit -m 'Yeni özellik ekle'`)
4. Dalı push edin (`git push origin feature/YeniÖzellik`)
5. Pull Request oluşturun

## Destek

Sorunlarla karşılaşırsanız, lütfen GitHub Issues'de bir hata raporu oluşturun.

## Geliştirici

LimitLess Contributors
