

## 🚀 Özellikler

- **Çoklu Hesap Yönetimi (Multi-Client):** Aynı anda sınırsız sayıda hesabı yönetebilir, hepsini tek tuşla oyuna sokabilirsiniz.
- **Hesap Giriş İpucu:** Eğer uygulamanız oyuna giriş yapamıyorsa veya hata veriyorsa, hesabınızı kaydederken yazdığınız telefon numarasının başındaki "0" (sıfır) rakamını silerek veya ekleyerek tekrar deneyin.
- **Donanım Kimliği Gizleme (HWID Spoofer):** Sisteme entegre edilen `ikv_spoofer` sayesinde her hesap oyuna farklı bir donanım kimliğiyle girer. Ban riski en aza indirilir.
- **Modern Arayüz (UI):** React ve TailwindCSS ile kodlanmış, karanlık mod destekli, göz yormayan modern yönetim paneli.

---

## 🔒 Güvenlik & Veri Politikası

LimitLess, tamamen **Yerel (Local) Çalışma Prensibi**'ne dayanır. 
- Uygulama içerisine girdiğiniz karakter adları, kullanıcı bilgileri ve şifreler, şifrelenmiş (AES 256) olarak yalnızca **kendi bilgisayarınızda** (`AppData/Roaming/com.limitless.app`) yerel SQLite veritabanında saklanır. 
- Hiçbir hesap bilginiz internete veya harici bir sunucuya **asla gönderilmez.** 
- Proje açık kaynaklı olduğu için kaynak kodlarını dilediğiniz gibi inceleyebilir, dışarıya veri akışı olmadığını kendi gözlerinizle görebilirsiniz.

---

## ⚙️ Kurulum ve Geliştirme (Nasıl Çalıştırılır?)

Bu uygulamayı kaynak kodundan kendi bilgisayarınızda çalıştırabilmek için aşağıdaki gereksinimleri kurmalısınız.

### 3. Çalıştırma (Dev Mode)
Terminali (CMD veya PowerShell) açın ve şu kodları sırasıyla yazın:
```bash
# LimitLess klasörünün içinde sağ tıklayarak terminali aç diyin. 

# Proje klasörüne girin
cd ikvlimitless-tauri

# Kütüphaneleri indirin
npm install

# Geliştirici modunda uygulamayı başlatın
npm run tauri dev
```

---

## 📦 Nasıl .EXE Haline Getirilir? (Build)

Uygulamanın kodlarında değişiklik yaptınız ve bunu normal bir Windows programı (.exe) olarak paketlemek istiyorsunuz:

1. Terminalde `ikvlimitless-tauri` dizininde olduğunuzdan emin olun.
2. Şu komutu çalıştırın:
```bash
npm run tauri build
```
3. Derleme işlemi (bilgisayarınızın hızına göre) birkaç dakika sürebilir. Rust tüm kodları sıkıştırıp tek bir exe dosyasına gömecektir.
4. **Çıkan EXE Dosyasının Konumu:**
Derleme bittiğinde oluşturulan nihai LimitLess kurulum dosyası (veya direkt çalıştırılabilir dosya) şu klasöre gidecektir:
👉 `ikvlimitless-tauri/src-tauri/target/release/`
veya kurulum (setup) dosyası için:
👉 `ikvlimitless-tauri/src-tauri/target/release/bundle/nsis/`

Oluşan bu EXE dosyasını alıp istediğiniz bilgisayarda kullanabilirsiniz.

---

## 📜 Lisans & Yasal Uyarı
Bu yazılım tamamen yapay zeka ile yapılmış olup açık kaynaklı ve lisanssız olarak paylaşılmaktadır. Eğitim ve oyun mekaniklerini inceleme (Reverse Engineering) amacıyla yapılmıştır. Sorumluluk tamamen kullanıcıya aittir.
